#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_settings.html")]
struct AdminSettingsTemplate;

pub async fn admin_settings(
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>) -> impl IntoResponse {
    let template = AdminSettingsTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
