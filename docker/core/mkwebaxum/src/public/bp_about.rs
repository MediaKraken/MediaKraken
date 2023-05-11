use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;

#[derive(Template)]
#[template(path = "bss_public/bss_public_about.html")]
struct AboutTemplate;

pub async fn public_about() -> impl IntoResponse {
    let template = AboutTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
