use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Multipart, Query, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{FromRow, postgres::PgPool};

const DEFAULT_PROFILE_IMAGE_URL: &str = "/static/image/K3.png";
const PROFILE_IMAGE_DIR: &str = "static/image/user_profile";
const MAX_PROFILE_IMAGE_BYTES: usize = 5 * 1024 * 1024;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/bss_user_profile.html")]
struct UserProfileTemplate {
    page_title: Option<String>,
    username: String,
    profile_image_url: String,
    success_message: Option<String>,
    error_message: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct UserProfileQuery {
    status: Option<String>,
    error: Option<String>,
}

#[derive(FromRow)]
struct UserProfileRow {
    mm_user_profile_guid: uuid::Uuid,
    mm_user_profile_json: Option<Value>,
}

pub async fn user_profile(
    State(state): State<AppState>,
    Query(query): Query<UserProfileQuery>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let template = UserProfileTemplate {
            page_title: Some("MediaKraken User Profile".to_string()),
            username: current_user.username,
            profile_image_url: load_profile_image_url(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or_else(|_| DEFAULT_PROFILE_IMAGE_URL.to_string()),
            success_message: match query.status.as_deref() {
                Some("updated") => Some("Profile picture updated.".to_string()),
                _ => None,
            },
            error_message: match query.error.as_deref() {
                Some("missing-file") => {
                    Some("Choose an image before submitting the form.".to_string())
                }
                Some("invalid-type") => {
                    Some("Profile pictures must be a PNG, JPEG, GIF, or WebP image.".to_string())
                }
                Some("too-large") => Some("Profile pictures must be 5 MB or smaller.".to_string()),
                Some("save-failed") => {
                    Some("MediaKraken could not save that profile picture right now.".to_string())
                }
                _ => None,
            },
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

pub async fn user_profile_photo_post(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    mut multipart: Multipart,
) -> Redirect {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::POST],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        return Redirect::to("/error/401");
    }

    let mut image_bytes = None;
    let mut image_extension = None;

    loop {
        let next_field = match multipart.next_field().await {
            Ok(field) => field,
            Err(_) => return Redirect::to("/user/profile?error=save-failed"),
        };

        let Some(field) = next_field else {
            break;
        };

        if field.name() != Some("profile_image") {
            continue;
        }

        let Some(extension) = content_type_to_extension(field.content_type()) else {
            return Redirect::to("/user/profile?error=invalid-type");
        };

        let bytes = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return Redirect::to("/user/profile?error=save-failed"),
        };

        if bytes.is_empty() {
            return Redirect::to("/user/profile?error=missing-file");
        }

        if bytes.len() > MAX_PROFILE_IMAGE_BYTES {
            return Redirect::to("/user/profile?error=too-large");
        }

        image_bytes = Some(bytes);
        image_extension = Some(extension.to_string());
        break;
    }

    let Some(image_bytes) = image_bytes else {
        return Redirect::to("/user/profile?error=missing-file");
    };
    let Some(image_extension) = image_extension else {
        return Redirect::to("/user/profile?error=invalid-type");
    };

    let public_image_url = format!(
        "/static/image/user_profile/user-{}.{}",
        current_user.id, image_extension
    );

    if save_profile_image(current_user.id, &image_extension, &image_bytes)
        .await
        .is_err()
    {
        return Redirect::to("/user/profile?error=save-failed");
    }

    if upsert_profile_image_url(&state.sqlx_pool_rw, current_user.id, &public_image_url)
        .await
        .is_err()
    {
        return Redirect::to("/user/profile?error=save-failed");
    }

    Redirect::to("/user/profile?status=updated")
}

async fn load_profile_image_url(sqlx_pool: &PgPool, user_id: i64) -> Result<String, sqlx::Error> {
    let profile_name = profile_name_for_user(user_id);
    let row = sqlx::query_scalar::<_, Option<String>>(
        r#"
        select mm_user_profile_json ->> 'profile_image_url'
        from mm_user_profile
        where mm_user_profile_name = $1
        order by mm_user_profile_guid
        limit 1
        "#,
    )
    .bind(profile_name)
    .fetch_optional(sqlx_pool)
    .await?;

    Ok(row
        .flatten()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_PROFILE_IMAGE_URL.to_string()))
}

async fn upsert_profile_image_url(
    sqlx_pool: &PgPool,
    user_id: i64,
    profile_image_url: &str,
) -> Result<(), sqlx::Error> {
    let profile_name = profile_name_for_user(user_id);
    let existing_profile = sqlx::query_as::<_, UserProfileRow>(
        r#"
        select mm_user_profile_guid, mm_user_profile_json
        from mm_user_profile
        where mm_user_profile_name = $1
        order by mm_user_profile_guid
        limit 1
        "#,
    )
    .bind(&profile_name)
    .fetch_optional(sqlx_pool)
    .await?;

    let mut transaction = sqlx_pool.begin().await?;

    if let Some(existing_profile) = existing_profile {
        let mut profile_json = existing_profile
            .mm_user_profile_json
            .unwrap_or_else(|| json!({}));

        if !profile_json.is_object() {
            profile_json = json!({});
        }

        if let Some(profile_object) = profile_json.as_object_mut() {
            profile_object.insert(
                "profile_image_url".to_string(),
                Value::String(profile_image_url.to_string()),
            );
        }

        sqlx::query(
            r#"
            update mm_user_profile
            set mm_user_profile_json = $2
            where mm_user_profile_guid = $1
            "#,
        )
        .bind(existing_profile.mm_user_profile_guid)
        .bind(profile_json)
        .execute(&mut *transaction)
        .await?;
    } else {
        sqlx::query(
            r#"
            insert into mm_user_profile (
                mm_user_profile_guid,
                mm_user_profile_name,
                mm_user_profile_json
            )
            values ($1, $2, $3)
            "#,
        )
        .bind(uuid::Uuid::now_v7())
        .bind(profile_name)
        .bind(json!({ "profile_image_url": profile_image_url }))
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}

async fn save_profile_image(
    user_id: i64,
    image_extension: &str,
    image_bytes: &[u8],
) -> Result<(), std::io::Error> {
    tokio::fs::create_dir_all(PROFILE_IMAGE_DIR).await?;

    for extension in ["png", "jpg", "jpeg", "gif", "webp"] {
        let existing_path = format!("{}/user-{}.{}", PROFILE_IMAGE_DIR, user_id, extension);
        if extension != image_extension {
            match tokio::fs::remove_file(&existing_path).await {
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }
    }

    let disk_path = format!("{}/user-{}.{}", PROFILE_IMAGE_DIR, user_id, image_extension);
    tokio::fs::write(disk_path, image_bytes).await
}

fn content_type_to_extension(content_type: Option<&str>) -> Option<&'static str> {
    match content_type {
        Some("image/png") => Some("png"),
        Some("image/jpeg") | Some("image/jpg") => Some("jpg"),
        Some("image/gif") => Some("gif"),
        Some("image/webp") => Some("webp"),
        _ => None,
    }
}

fn profile_name_for_user(user_id: i64) -> String {
    format!("axum_user_{}", user_id)
}
