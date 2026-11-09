use askama::Template;
use axum::{
    extract::{Query, Request},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;

// https://docs.rs/http/latest/http/status/struct.StatusCode.html#
// possible status codes

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {
    is_admin: bool,
}

#[derive(Template)]
#[template(path = "bss_error/bss_error_404.html")]
struct TemplateError404Context {
    is_admin: bool,
}

#[derive(Template)]
#[template(path = "bss_error/bss_error_500.html")]
struct TemplateError500Context {
    is_admin: bool,
}

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

#[derive(Debug, Deserialize)]
pub struct ErrorPageQuery {
    /// Carried by redirects out of the admin area so the error page can
    /// keep the admin navigation bar (`?admin=true`).
    admin: Option<String>,
}

fn error_page_is_admin(query: &Query<ErrorPageQuery>) -> bool {
    query.admin.as_deref() == Some("true")
}

pub async fn general_not_administrator(query: Query<ErrorPageQuery>) -> Response {
    tracing::debug!("rendering 403 page");
    let is_admin = error_page_is_admin(&query);
    render_with_fallback(
        StatusCode::FORBIDDEN,
        TemplateError403Context { is_admin }.render(),
        "Forbidden",
    )
}

pub async fn general_not_found(request: Request) -> Response {
    // The fallback sits outside the session layer, so the user session cannot
    // be consulted; the request path is the only way to tell whether the
    // missing page was an admin page and the admin nav should be kept.
    let is_admin = request.uri().path().starts_with("/admin/");
    render_with_fallback(
        StatusCode::NOT_FOUND,
        TemplateError404Context { is_admin }.render(),
        "Not Found",
    )
}

pub async fn general_error(query: Query<ErrorPageQuery>) -> Response {
    tracing::debug!("rendering 500 page");
    let is_admin = error_page_is_admin(&query);
    render_with_fallback(
        StatusCode::INTERNAL_SERVER_ERROR,
        TemplateError500Context { is_admin }.render(),
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
                TemplateError403Context { is_admin: false }.render(),
                "Forbidden",
            ),
            MKAxumError::Error500 => render_with_fallback(
                StatusCode::INTERNAL_SERVER_ERROR,
                TemplateError500Context { is_admin: false }.render(),
                "Internal Server Error",
            ),
        }
    }
}
