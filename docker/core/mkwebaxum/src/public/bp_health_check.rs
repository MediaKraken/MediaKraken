#![cfg_attr(debug_assertions, allow(dead_code))]

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn public_health_check() -> impl IntoResponse {
    (StatusCode::OK, Html("").into_response())
}
