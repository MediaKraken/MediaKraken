use crate::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderValue, StatusCode, header},
    response::IntoResponse,
};

pub async fn metadata_object_proxy(
    State(state): State<AppState>,
    Path(object_key): Path<String>,
) -> impl IntoResponse {
    if object_key.trim().is_empty() || object_key.contains("..") {
        return (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            "invalid object key".as_bytes().to_vec(),
        );
    }

    let garage_base_url = std::env::var("MK_GARAGE_METADATA_BASE_URL").unwrap_or_default();
    if garage_base_url.trim().is_empty() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            "metadata storage is not configured".as_bytes().to_vec(),
        );
    }

    let upstream_url = format!(
        "{}/{}",
        garage_base_url.trim_end_matches('/'),
        object_key.trim_start_matches('/')
    );

    let mut request = state.client.get(upstream_url.as_str());
    if let Ok(token) = std::env::var("MK_GARAGE_METADATA_TOKEN") {
        let trimmed = token.trim();
        if !trimmed.is_empty() {
            request = request.header(header::AUTHORIZATION, format!("Bearer {trimmed}"));
        }
    }

    let upstream_response = match request.send().await {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::BAD_GATEWAY,
                [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
                "unable to reach metadata storage".as_bytes().to_vec(),
            );
        }
    };

    let status = upstream_response.status();
    if !status.is_success() {
        return (
            StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            "metadata storage returned an error".as_bytes().to_vec(),
        );
    }

    let content_type = upstream_response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let cache_control = upstream_response
        .headers()
        .get(header::CACHE_CONTROL)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("public, max-age=3600")
        .to_string();

    let body = match upstream_response.bytes().await {
        Ok(bytes) => bytes,
        Err(_) => {
            return (
                StatusCode::BAD_GATEWAY,
                [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
                "invalid metadata response".as_bytes().to_vec(),
            );
        }
    };

    let content_type_header = match HeaderValue::from_str(&content_type) {
        Ok(value) => value,
        Err(_) => HeaderValue::from_static("application/octet-stream"),
    };
    let cache_control_header = match HeaderValue::from_str(&cache_control) {
        Ok(value) => value,
        Err(_) => HeaderValue::from_static("public, max-age=3600"),
    };

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type_header),
            (header::CACHE_CONTROL, cache_control_header),
        ],
        body,
    )
}
