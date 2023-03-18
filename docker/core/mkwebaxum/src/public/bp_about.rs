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

#[derive(Template)]
#[template(path = "bss_public/bss_public_about.html")]
struct AboutTemplate;

pub async fn public_about() -> impl IntoResponse {
    let template = AboutTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

// #[get("/about")]
// pub async fn public_about() -> Template {
//     Template::render(
//         "bss_public/bss_public_about",
//         tera::Context::new().into_json(),
//     )
// }
