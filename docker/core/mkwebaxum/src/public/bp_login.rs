use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
};
use axum_flash::{Flash, IncomingFlashes};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use ldap3::LdapConnAsync;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_public/bss_public_login.html")]
struct LoginTemplate {
    flash_messages: Vec<String>,
}

pub async fn public_login(flashes: IncomingFlashes) -> impl IntoResponse {
    let template = LoginTemplate {
        flash_messages: flashes
            .iter()
            .map(|(_, message)| message.to_string())
            .collect(),
    };
    let reply_html = template.render().unwrap();
    (flashes, Html(reply_html))
}

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
    #[serde(default)]
    use_ms_ad: bool,
}

fn ms_ad_enabled() -> bool {
    matches!(
        std::env::var("MKWEBAPP_MS_AD_ENABLED")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes" | "on"
    )
}

fn ms_ad_bind_username(username: &str) -> String {
    if username.contains('\\') || username.contains('@') {
        return username.to_string();
    }

    let domain = std::env::var("MKWEBAPP_MS_AD_DOMAIN").unwrap_or_default();
    if domain.is_empty() {
        username.to_string()
    } else {
        format!("{username}@{domain}")
    }
}

fn local_username_for_ad(username: &str) -> String {
    if let Some((_, suffix)) = username.split_once('\\') {
        return suffix.to_string();
    }

    if let Some((prefix, _)) = username.split_once('@') {
        return prefix.to_string();
    }

    username.to_string()
}

async fn ms_ad_login_verification(username: &str, password: &str) -> bool {
    if !ms_ad_enabled() || username.is_empty() || password.is_empty() {
        return false;
    }

    let ad_url = std::env::var("MKWEBAPP_MS_AD_URL").unwrap_or_default();
    if ad_url.is_empty() {
        return false;
    }

    let bind_username = ms_ad_bind_username(username);

    let Ok((conn, mut ldap)) = LdapConnAsync::new(&ad_url).await else {
        return false;
    };
    ldap3::drive!(conn);

    let login_valid = match ldap.simple_bind(&bind_username, password).await {
        Ok(result) => result.success().is_ok(),
        Err(_) => false,
    };

    let _ = ldap.unbind().await;

    login_valid
}

fn ms_ad_auto_provision_enabled() -> bool {
    matches!(
        std::env::var("MKWEBAPP_MS_AD_AUTO_PROVISION")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes" | "on"
    )
}

async fn fetch_local_user_id(
    sqlx_pool: &sqlx::PgPool,
    username: &str,
) -> Result<Option<i64>, sqlx::Error> {
    let row: Option<(i64,)> =
        sqlx::query_as("select id from mm_axum_users where username = $1 limit 1")
            .bind(username)
            .fetch_optional(sqlx_pool)
            .await?;

    Ok(row.map(|(id,)| id))
}

async fn provision_local_ms_ad_user(
    sqlx_pool: &sqlx::PgPool,
    username: &str,
) -> Result<i64, sqlx::Error> {
    let username_owned = username.to_string();
    let generated_password = uuid::Uuid::new_v4().to_string();
    let user_id = mk_lib_database::mk_lib_database_user::mk_lib_database_user_insert(
        sqlx_pool,
        &username_owned,
        &generated_password,
    )
    .await?;

    sqlx::query(
        r#"insert into mm_axum_user_permissions (user_id, token)
            select $1, $2
            where not exists (
                select 1 from mm_axum_user_permissions where user_id = $1 and token = $2
            )"#,
    )
    .bind(user_id)
    .bind("User::View")
    .execute(sqlx_pool)
    .await?;

    Ok(user_id)
}

async fn resolve_local_ms_ad_user(
    sqlx_pool: &sqlx::PgPool,
    username: &str,
) -> Result<Option<i64>, sqlx::Error> {
    if let Some(user_id) = fetch_local_user_id(sqlx_pool, username).await? {
        return Ok(Some(user_id));
    }

    if ms_ad_auto_provision_enabled() {
        let user_id = provision_local_ms_ad_user(sqlx_pool, username).await?;
        Ok(Some(user_id))
    } else {
        Ok(None)
    }
}

pub async fn public_login_post(
    State(state): State<AppState>,
    mut auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    flash: Flash,
    Form(input_data): Form<LoginInput>,
) -> (Flash, Redirect) {
    let local_login_result =
        mk_lib_database::mk_lib_database_user::mk_lib_database_user_login_verification(
            &state.sqlx_pool_rw,
            &input_data.username,
            &input_data.password,
        )
        .await
        .unwrap_or(0);

    let mut user_id = local_login_result;

    if user_id <= 0 && input_data.use_ms_ad {
        let ad_login_valid =
            ms_ad_login_verification(&input_data.username, &input_data.password).await;
        if ad_login_valid {
            let local_username = local_username_for_ad(&input_data.username);
            user_id = resolve_local_ms_ad_user(&state.sqlx_pool_rw, &local_username)
                .await
                .ok()
                .flatten()
                .unwrap_or(0);
        }
    }

    if user_id > 0 {
        println!("Login User {:?}", user_id);
        let _result = mk_lib_database::mk_lib_database_user::mk_lib_database_user_login(
            &state.sqlx_pool_rw,
            user_id,
        )
        .await;
        auth.login_user(user_id);
        auth.remember_user(true);
        (flash, Redirect::to("/user/home"))
    } else {
        (
            flash.error("Unknown user or password incorrect!"),
            Redirect::to("/public/login"),
        )
    }
}
