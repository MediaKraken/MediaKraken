use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

// https://docs.rs/http/latest/http/status/struct.StatusCode.html#
// possible status codes

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

fn render_with_fallback(
    status: StatusCode,
    rendered: Result<String, askama::Error>,
    fallback_text: &'static str,
) -> Response {
    match rendered {
        Ok(html) => (status, Html(html)).into_response(),
        Err(error) => {
            tracing::error!(?error, %status, "error template render failed");
            (status, fallback_text).into_response()
        }
    }
}

pub async fn general_not_authorized() -> Response {
    tracing::debug!("rendering 401 page");
    render_with_fallback(
        StatusCode::UNAUTHORIZED,
        TemplateError401Context {}.render(),
        "Unauthorized",
    )
}

pub async fn general_not_administrator() -> Response {
    tracing::debug!("rendering 403 page");
    render_with_fallback(
        StatusCode::FORBIDDEN,
        TemplateError403Context {}.render(),
        "Forbidden",
    )
}

pub async fn general_not_found() -> Response {
    render_with_fallback(
        StatusCode::NOT_FOUND,
        TemplateError404Context {}.render(),
        "Not Found",
    )
}

pub async fn general_error() -> Response {
    tracing::debug!("rendering 500 page");
    render_with_fallback(
        StatusCode::INTERNAL_SERVER_ERROR,
        TemplateError500Context {}.render(),
        "Internal Server Error",
    )
}

/// Error type usable from handlers via `Result<_, MKAxumError>`.
pub enum MKAxumError {
    Error401,
    Error403,
    Error500,
}

impl IntoResponse for MKAxumError {
    fn into_response(self) -> Response {
        match self {
            MKAxumError::Error401 => render_with_fallback(
                StatusCode::UNAUTHORIZED,
                TemplateError401Context {}.render(),
                "Unauthorized",
            ),
            MKAxumError::Error403 => render_with_fallback(
                StatusCode::FORBIDDEN,
                TemplateError403Context {}.render(),
                "Forbidden",
            ),
            MKAxumError::Error500 => render_with_fallback(
                StatusCode::INTERNAL_SERVER_ERROR,
                TemplateError500Context {}.render(),
                "Internal Server Error",
            ),
        }
    }
}
