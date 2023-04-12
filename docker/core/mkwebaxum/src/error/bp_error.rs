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

// https://docs.rs/http/latest/http/status/struct.StatusCode.html#
// possible status codes

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

pub async fn general_not_authorized() -> impl IntoResponse {
    let template = TemplateError401Context {};
    let reply_html = template.render().unwrap();
    (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

pub async fn general_not_administrator() -> impl IntoResponse {
    let template = TemplateError403Context {};
    let reply_html = template.render().unwrap();
    (StatusCode::FORBIDDEN, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_error/bss_error_404.html")]
struct TemplateError404Context {}

pub async fn general_not_found() -> impl IntoResponse {
    let template = TemplateError404Context {};
    let reply_html = template.render().unwrap();
    (StatusCode::NOT_FOUND, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_error/bss_error_500.html")]
struct TemplateError500Context {}

pub async fn general_security() -> impl IntoResponse {
    let template = TemplateError500Context {};
    let reply_html = template.render().unwrap();
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(reply_html).into_response(),
    )
}
