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
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::Row;
use sqlx::postgres::PgPool;
use tokio::task;

#[derive(Template)]
#[template(path = "bss_public/bss_public_login.html")]
struct LoginTemplate {
    flash_messages: Vec<String>,
}

#[derive(Deserialize)]
struct AuthyVerifyResponse {
    success: Option<String>,
}

pub async fn public_login(flashes: IncomingFlashes) -> impl IntoResponse {
    let template = LoginTemplate {
        flash_messages: flashes
            .iter()
            .map(|(_, message)| message.to_string())
            .collect(),
    };

    match template.render() {
        Ok(reply_html) => (flashes, Html(reply_html)).into_response(),
        Err(_) => Redirect::to("/public/login").into_response(),
    }
}

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
    authy_token: Option<String>,
}

fn env_flag_enabled(var_name: &str) -> bool {
    match std::env::var(var_name) {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            matches!(normalized.as_str(), "1" | "true" | "yes" | "on")
        }
        Err(_) => false,
    }
}

async fn try_ad_login(sqlx_pool: &PgPool, username: &str, password: &str) -> Option<i64> {
    if password.trim().is_empty() || !env_flag_enabled("MKWEBAPP_AD_LOGIN_ENABLED") {
        return None;
    }
    let password_owned = password.to_string();

    let ldap_ip = match std::env::var("MKWEBAPP_AD_LDAP_IP") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return None,
    };
    let ldap_port = std::env::var("MKWEBAPP_AD_LDAP_PORT").unwrap_or_else(|_| "389".to_string());
    let ldap_domain = match std::env::var("MKWEBAPP_AD_DOMAIN") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return None,
    };

    let bind_username = if username.contains('@') {
        username.to_string()
    } else {
        format!("{}@{}", username, ldap_domain)
    };

    let bind_result = task::spawn_blocking(move || {
        mk_lib_network::mk_lib_network_ldap::ldap_bind_blocking(
            ldap_ip,
            ldap_port,
            &bind_username,
            &password_owned,
        )
    })
    .await;

    if !matches!(bind_result, Ok(Ok(_))) {
        return None;
    }

    let query_result = sqlx::query(r#"select id from mm_axum_users where username = $1 limit 1"#)
        .bind(username)
        .fetch_optional(sqlx_pool)
        .await
        .ok()
        .flatten();

    match query_result {
        Some(row) => row.try_get::<i64, _>("id").ok(),
        None => None,
    }
}

async fn verify_authy_token(authy_id: &str, authy_token: &str, api_key: &str) -> bool {
    // Authy tokens are 6-8 numeric digits; the user id is numeric as well.
    // Reject obviously malformed values before making a network call so that
    // log messages (and any error paths) don't echo user-supplied junk.
    if authy_token.is_empty()
        || authy_token.len() > 16
        || !authy_token.chars().all(|c| c.is_ascii_digit())
    {
        return false;
    }
    if authy_id.is_empty() || !authy_id.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let verify_url = format!(
        "https://api.authy.com/protected/json/verify/{}/{}",
        authy_token, authy_id
    );

    let response = match reqwest::Client::new()
        .get(&verify_url)
        // Pass the API key via header instead of query string so it does not
        // end up in reqwest/proxy/middlebox logs.
        .header("X-Authy-API-Key", api_key)
        .send()
        .await
    {
        Ok(value) => value,
        Err(_) => return false,
    };

    if response.status() != StatusCode::OK {
        return false;
    }

    match response.json::<AuthyVerifyResponse>().await {
        Ok(payload) => payload.success.as_deref() == Some("true"),
        Err(_) => false,
    }
}

pub async fn public_login_post(
    State(state): State<AppState>,
    mut auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    flash: Flash,
    Form(input_data): Form<LoginInput>,
) -> (Flash, Redirect) {
    let mut user_id: i64 =
        match mk_lib_database::mk_lib_database_user::mk_lib_database_user_login_verification(
            &state.sqlx_pool_rw,
            &input_data.username,
            &input_data.password,
        )
        .await
        {
            Ok(value) => value,
            Err(_) => {
                return (
                    flash.error("Unable to verify login request. Please try again."),
                    Redirect::to("/public/login"),
                );
            }
        };

    if user_id <= 0 {
        user_id = try_ad_login(
            &state.sqlx_pool_rw,
            &input_data.username,
            &input_data.password,
        )
        .await
        .unwrap_or(0);
    }

    if user_id > 0 {
        let authy_id = mk_lib_database::mk_lib_database_user::mk_lib_database_user_authy_id(
            &state.sqlx_pool_rw,
            user_id,
        )
        .await
        .ok()
        .flatten();

        if let Some(authy_user_id) = authy_id {
            // Fail closed: if the user has 2FA configured, require a valid
            // token. A missing API key is a misconfiguration, not a downgrade.
            let Ok(api_key) = std::env::var("MKWEBAPP_AUTHY_API_KEY") else {
                tracing::error!(
                    user_id,
                    "user has authy_id configured but MKWEBAPP_AUTHY_API_KEY is not set; refusing login"
                );
                return (
                    flash.error("2FA is misconfigured on this server. Contact an administrator."),
                    Redirect::to("/public/login"),
                );
            };
            if api_key.trim().is_empty() {
                tracing::error!(
                    user_id,
                    "MKWEBAPP_AUTHY_API_KEY is empty; refusing 2FA-enabled login"
                );
                return (
                    flash.error("2FA is misconfigured on this server. Contact an administrator."),
                    Redirect::to("/public/login"),
                );
            }

            let submitted_token = input_data.authy_token.unwrap_or_default();
            if submitted_token.trim().is_empty() {
                return (
                    flash.error("2FA code is required for this account."),
                    Redirect::to("/public/login"),
                );
            }

            let token_valid =
                verify_authy_token(&authy_user_id, submitted_token.trim(), api_key.trim()).await;
            if !token_valid {
                return (
                    flash.error("Invalid 2FA code."),
                    Redirect::to("/public/login"),
                );
            }
        }

        let _result = mk_lib_database::mk_lib_database_user::mk_lib_database_user_login(
            &state.sqlx_pool_rw,
            user_id,
        )
        .await;
        auth.login_user(user_id);
        (flash, Redirect::to("/user/home"))
    } else {
        (
            flash.error("Unknown user or password incorrect!"),
            Redirect::to("/public/login"),
        )
    }
}
