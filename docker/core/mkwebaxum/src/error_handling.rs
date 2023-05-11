// https://docs.rs/http/latest/http/status/struct.StatusCode.html#
// possible status codes

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_404.html")]
struct TemplateError404Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_500.html")]
struct TemplateError500Context {}

pub enum MKAxumError {
    Error401,
    Error403,
    Error500,
}

impl IntoResponse for MKAxumError {
    fn into_response(self) -> Response {
        let mut reply_html = String::new();
        let status_code = match self {
            MKAxumError::Error401 => {
                println!("401");
                let template = TemplateError401Context {};
                reply_html = template.render().unwrap();
                StatusCode::UNAUTHORIZED
            }
            MKAxumError::Error403 => {
                println!("403");
                let template = TemplateError403Context {};
                reply_html = template.render().unwrap();
                StatusCode::FORBIDDEN
            }
            MKAxumError::Error500 => {
                println!("500");
                let template = TemplateError500Context {};
                reply_html = template.render().unwrap();
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        (status_code, Html(reply_html)).into_response()
    }
}
