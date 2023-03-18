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

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/internet")]
pub async fn user_inter_home(user: User) -> Template {
    Template::render(
        "bss_user/internet/bss_user_internet",
        tera::Context::new().into_json(),
    )
}
