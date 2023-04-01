#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session::{
    DatabasePool, Session, SessionConfig, SessionLayer, SessionPgPool, SessionStore,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use serde_json::json;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_public/bss_public_login.html")]
struct LoginTemplate;

// #[get("/logout")]
// pub async fn public_logout(
//     auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
// ) -> Result<Template, Error> {
//     auth.logout_user()?;
//     Ok(Template::render("logout", tera::Context::new().into_json()))
// }

pub async fn public_logout(auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>) -> impl IntoResponse {
    let template = LoginTemplate {};
    let reply_html = template.render().unwrap();
    auth.logout_user();
    (StatusCode::OK, Html(reply_html).into_response())
}
