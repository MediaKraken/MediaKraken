use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "bss_public/bss_public_about.html")]
struct AboutTemplate;

pub async fn public_about() -> impl IntoResponse {
    let template = AboutTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
