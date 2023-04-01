#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use serde_json::json;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[derive(Template)]
#[template(path = "bss_user/playback/bss_user_playback_video.html")]
struct UserPlaybackVideoTemplate;

pub async fn user_playback_video() -> impl IntoResponse {
    let template = UserPlaybackVideoTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
