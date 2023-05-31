use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

// https://docs.rs/http/latest/http/status/struct.StatusCode.html#
// possible status codes

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

pub async fn general_not_authorized() -> impl IntoResponse {
    println!("general_not_authorized");
    let template = TemplateError401Context {};
    let reply_html = template.render().unwrap();
    (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

pub async fn general_not_administrator() -> impl IntoResponse {
    println!("general_not_administrator");
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

pub async fn general_error() -> impl IntoResponse {
    println!("general_error");
    let template = TemplateError500Context {};
    let reply_html = template.render().unwrap();
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(reply_html).into_response(),
    )
}
