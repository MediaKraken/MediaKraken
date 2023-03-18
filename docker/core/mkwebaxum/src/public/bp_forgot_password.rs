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

#[get("/forgot_password")]
pub async fn public_forgot_password() -> Template {
    Template::render(
        "bss_public/bss_public_forgot_password",
        tera::Context::new().into_json(),
    )
}
