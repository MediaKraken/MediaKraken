#![cfg_attr(debug_assertions, allow(dead_code))]

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

use crate::mk_lib_logging;

#[derive(Template)]
#[template(path = "bss_public/bss_public_about.html")]
struct AboutTemplate;

pub async fn public_about() -> impl IntoResponse {
    let template = AboutTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
