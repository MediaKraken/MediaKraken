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
use sqlx::postgres::PgPool;

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

async fn verify_authy_token(authy_id: &str, authy_token: &str, api_key: &str) -> bool {
    let verify_url = format!(
        "https://api.authy.com/protected/json/verify/{}/{}/?api_key={}",
        authy_token, authy_id, api_key
    );

    let response = match reqwest::Client::new().get(&verify_url).send().await {
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
    let user_id: i64 =
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

    if user_id > 0 {
        let authy_id = mk_lib_database::mk_lib_database_user::mk_lib_database_user_authy_id(
            &state.sqlx_pool_rw,
            user_id,
        )
        .await
        .ok()
        .flatten();

        if let Some(authy_user_id) = authy_id {
            let authy_api_key = std::env::var("MKWEBAPP_AUTHY_API_KEY").ok();
            if let Some(api_key) = authy_api_key {
                let submitted_token = input_data.authy_token.unwrap_or_default();
                if submitted_token.trim().is_empty() {
                    return (
                        flash.error("2FA code is required for this account."),
                        Redirect::to("/public/login"),
                    );
                }

                let token_valid =
                    verify_authy_token(&authy_user_id, submitted_token.trim(), &api_key).await;
                if !token_valid {
                    return (
                        flash.error("Invalid 2FA code."),
                        Redirect::to("/public/login"),
                    );
                }
            }
        }

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
