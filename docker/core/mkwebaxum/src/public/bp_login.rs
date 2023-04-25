#![cfg_attr(debug_assertions, allow(dead_code))]

use askama::Template;
use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_session::SessionPgPool;
use axum_session_auth::*;
use serde::Deserialize;
use sqlx::PgPool;
use stdext::function_name;
use validator::Validate;

use crate::mk_lib_logging;

use crate::database::mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_public/bss_public_login.html")]
struct LoginTemplate;

pub async fn public_login() -> impl IntoResponse {
    let template = LoginTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

pub async fn public_login_post(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Form(input_data): Form<LoginInput>,
) -> Redirect {
    let user_id: i64 = mk_lib_database_user::mk_lib_database_user_login_verification(
        &sqlx_pool,
        &input_data.username,
        &input_data.password,
    )
    .await
    .unwrap();
    // TODO show error when not found
    if user_id > 0 {
        mk_lib_database_user::mk_lib_database_user_login(&sqlx_pool, user_id).await;
        auth.login_user(user_id);
    }
    Redirect::to("/user/home")
}
