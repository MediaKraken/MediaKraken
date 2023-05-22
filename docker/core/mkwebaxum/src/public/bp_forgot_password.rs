use askama::Template;
use axum::{
    extract::Form,
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::{AuthSession, SessionPgPool};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use stdext::function_name;

#[derive(Template)]
#[template(path = "bss_public/bss_public_forgot_password.html")]
struct ForgotPasswordTemplate;

pub async fn public_forgot_password() -> impl IntoResponse {
    let template = ForgotPasswordTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
