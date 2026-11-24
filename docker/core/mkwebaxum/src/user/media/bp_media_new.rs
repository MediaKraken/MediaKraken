use crate::AppState;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::State;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use chrono::{Duration, Utc};
use mk_lib_common::mk_lib_common_enum_media_type::DLMediaType;
use mk_lib_common::mk_lib_common_pagination;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

/// One recently added media file (all media types).
#[derive(Debug, sqlx::FromRow)]
struct MediaNewListRow {
    mm_media_guid: uuid::Uuid,
    mm_media_metadata_guid: Option<uuid::Uuid>,
    mm_media_path: String,
    mm_media_class_enum: i16,
    date_added: String,
}

/// One row on the new releases page. An empty `template_detail_link`
/// means the media type has no detail page.
#[derive(Debug, Deserialize, Serialize)]
struct TemplateMediaNewList {
    template_title: String,
    template_type: String,
    template_date: String,
    template_detail_link: String,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_new.html")]
struct TemplateMediaNewContext<'a> {
    template_data: &'a Vec<TemplateMediaNewList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    #[allow(dead_code)]
    page: &'a usize,
    page_title: Option<String>,
}

/// Count media of any type added within the last `days_old` days.
async fn media_new_count(sqlx_pool: &PgPool, date_added: &str) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*)
           from mm_media
           where mm_media_json->>'DateAdded' >= $1"#,
    )
    .bind(date_added)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/// Read media of any type added within the last `days_old` days, newest first.
async fn media_new_read(
    sqlx_pool: &PgPool,
    date_added: &str,
    offset: i64,
    limit: i64,
) -> Result<Vec<MediaNewListRow>, sqlx::Error> {
    let table_rows: Vec<MediaNewListRow> = sqlx::query_as(
        r#"select mm_media_guid,
             mm_media_metadata_guid,
             coalesce(mm_media_path, '') as mm_media_path,
             coalesce(mm_media_class_enum, 0) as mm_media_class_enum,
             coalesce(mm_media_json->>'DateAdded', '') as date_added
             from mm_media
             where mm_media_json->>'DateAdded' >= $1
             order by date_added desc, mm_media_path
             offset $2 limit $3"#,
    )
    .bind(date_added)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

/// The file name (without extension) of a media path, used as the display
/// title; the media table itself carries no name column.
fn media_title_from_path(path: &str) -> String {
    let file_name = path.rsplit('/').next().unwrap_or(path);
    match file_name.rfind('.') {
        Some(pos) if pos > 0 => file_name[..pos].to_string(),
        _ => file_name.to_string(),
    }
}

/// Short display label for a media class (subtypes map to their parent type).
fn media_type_label(class: i16) -> &'static str {
    match class {
        DLMediaType::MOVIE | DLMediaType::MOVIE_HOME..=DLMediaType::MOVIE_TRAILER => "Movie",
        DLMediaType::TV | DLMediaType::TV_EPISODE..=DLMediaType::TV_TRAILER => "TV",
        DLMediaType::PERSON => "Person",
        DLMediaType::SPORTS => "Sports",
        DLMediaType::GAME | DLMediaType::GAME_CHD..=DLMediaType::GAME_SUPERPLAY => "Game",
        DLMediaType::PUBLICATION
        | DLMediaType::PUBLICATION_BOOK..=DLMediaType::PUBLICATION_GRAPHIC_NOVEL => "Publication",
        DLMediaType::PICTURE => "Picture",
        DLMediaType::ANIME => "Anime",
        DLMediaType::MUSIC | DLMediaType::MUSIC_ALBUM..=DLMediaType::MUSIC_VIDEO => "Music",
        DLMediaType::ADULT | DLMediaType::ADULT_IMAGE..=DLMediaType::ADULT_SCENE => "Adult",
        DLMediaType::COLLECTION => "Collection",
        _ => "Other",
    }
}

/// Detail page link for a media row, where one exists.
///
/// The movie detail route takes a media guid; every other media detail route
/// takes the metadata guid the media row points at. Types without a media
/// detail route (games, pictures, ...) return None.
fn media_detail_link(
    class: i16,
    media_guid: uuid::Uuid,
    metadata_guid: Option<uuid::Uuid>,
) -> Option<String> {
    let route = match class {
        DLMediaType::MOVIE | DLMediaType::MOVIE_HOME..=DLMediaType::MOVIE_TRAILER => {
            return Some(format!("/user/media/movie_detail/{media_guid}"));
        }
        DLMediaType::TV | DLMediaType::TV_EPISODE..=DLMediaType::TV_TRAILER => {
            "/user/media/tv_detail"
        }
        DLMediaType::SPORTS => "/user/media/sports_detail",
        DLMediaType::PUBLICATION
        | DLMediaType::PUBLICATION_BOOK..=DLMediaType::PUBLICATION_GRAPHIC_NOVEL => {
            "/user/media/book_detail"
        }
        DLMediaType::MUSIC | DLMediaType::MUSIC_ALBUM..=DLMediaType::MUSIC_SONG => {
            "/user/media/music_detail"
        }
        DLMediaType::MUSIC_VIDEO => "/user/media/music_video_detail",
        _ => return None,
    };
    metadata_guid.map(|guid| format!("{route}/{guid}"))
}

/// Bare sidebar URL: redirect to the first page.
pub async fn user_media_new_root() -> impl IntoResponse {
    Redirect::to("/user/media/new/1")
}

pub async fn user_media_new(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
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
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        // same DateAdded idiom as the dashboard "new media" badge
        let date_added = (Utc::now() - Duration::days(7))
            .format("%Y-%m-%d")
            .to_string();
        let total_items: i64 = match media_new_count(&state.sqlx_pool_ro, &date_added).await {
            Ok(total) => total,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let pagination_html = match mk_lib_common_pagination::mk_lib_common_paginate(
            total_items,
            page,
            "/user/media/new".to_string(),
            None,
            pagination_count,
        )
        .await
        {
            Ok(html) => html,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let new_media_list = match media_new_read(
            &state.sqlx_pool_ro,
            &date_added,
            db_offset,
            pagination_count,
        )
        .await
        {
            Ok(list) => list,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let template_data_vec: Vec<TemplateMediaNewList> = new_media_list
            .iter()
            .map(|row| TemplateMediaNewList {
                template_title: media_title_from_path(&row.mm_media_path),
                template_type: media_type_label(row.mm_media_class_enum).to_string(),
                template_date: row.date_added.clone(),
                template_detail_link: media_detail_link(
                    row.mm_media_class_enum,
                    row.mm_media_guid,
                    row.mm_media_metadata_guid,
                )
                .unwrap_or_default(),
            })
            .collect();
        let template_data_exists = !template_data_vec.is_empty();
        let page_usize = page as usize;
        let template = TemplateMediaNewContext {
            template_data: &template_data_vec,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            #[allow(dead_code)]
            page: &page_usize,
            page_title: Some("MediaKraken New Releases".to_string()),
        };
        let reply_html = match template.render() {
            Ok(html) => html,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
