#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use stdext::function_name;
use serde_json::json;
use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/settings")]
pub async fn admin_settings(sqlx_pool: &rocket::State<sqlx::PgPool>, user: AdminUser) -> Template {
    Template::render(
        "bss_admin/bss_admin_settings",
        tera::Context::new().into_json(),
    )
}
