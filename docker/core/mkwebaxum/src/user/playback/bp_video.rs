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

#[get("/playback/video")]
pub async fn user_playback_video(user: User) -> Template {
    Template::render(
        "bss_user/playback/bss_user_playback_video",
        tera::Context::new().into_json(),
    )
}
