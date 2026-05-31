use crate::AppState;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderValue, Request, Response, StatusCode, Uri, header},
    response::IntoResponse,
};
use std::time::Duration;

const UPSTREAM_TIMEOUT: Duration = Duration::from_secs(15);
// Upstream metadata objects (images, thumbnails, small JSON) should all fit
// comfortably under this cap. Reject anything larger to limit memory pressure.
const MAX_UPSTREAM_BYTES: u64 = 32 * 1024 * 1024;

fn text_response(status: StatusCode, message: &str) -> Response<Body> {
    let mut response = Response::new(Body::from(message.as_bytes().to_vec()));
    *response.status_mut() = status;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response
}

fn is_valid_object_key(key: &str) -> bool {
    if key.is_empty() || key.len() > 1024 {
        return false;
    }
    // Accept: ascii alnum plus -_./ (no leading slash handled by trim below).
    // Forbid: backslash, control chars, null, ".." path components.
    if key.split('/').any(|segment| segment == "..") {
        return false;
    }
    key.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || matches!(c, '-' | '_' | '.' | '/' | '=' | '+' | '%' | '~' | ':')
    })
}

pub async fn metadata_object_proxy(
    State(state): State<AppState>,
    Path(object_key): Path<String>,
) -> impl IntoResponse {
    let trimmed_key = object_key.trim_start_matches('/');
    if !is_valid_object_key(trimmed_key) {
        return text_response(StatusCode::BAD_REQUEST, "invalid object key");
    }

    let garage_base_url = std::env::var("MK_GARAGE_METADATA_BASE_URL").unwrap_or_default();
    if garage_base_url.trim().is_empty() {
        return text_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "metadata storage is not configured",
        );
    }

    let upstream_url = format!("{}/{}", garage_base_url.trim_end_matches('/'), trimmed_key);

    let upstream_uri = match upstream_url.parse::<Uri>() {
        Ok(uri) => uri,
        Err(_) => {
            return text_response(
                StatusCode::SERVICE_UNAVAILABLE,
                "metadata storage URL is invalid",
            );
        }
    };

    let mut request_builder = Request::builder().method("GET").uri(upstream_uri);
    if let Ok(token) = std::env::var("MK_GARAGE_METADATA_TOKEN") {
        let trimmed = token.trim();
        if !trimmed.is_empty() {
            request_builder =
                request_builder.header(header::AUTHORIZATION, format!("Bearer {trimmed}"));
        }
    }

    let upstream_request = match request_builder.body(Body::empty()) {
        Ok(request) => request,
        Err(_) => {
            return text_response(StatusCode::BAD_GATEWAY, "unable to build metadata request");
        }
    };

    let upstream_response = match tokio::time::timeout(
        UPSTREAM_TIMEOUT,
        state.client.request(upstream_request),
    )
    .await
    {
        Ok(Ok(response)) => response,
        Ok(Err(_)) => {
            return text_response(StatusCode::BAD_GATEWAY, "unable to reach metadata storage");
        }
        Err(_) => {
            return text_response(StatusCode::GATEWAY_TIMEOUT, "metadata storage timed out");
        }
    };

    let status = upstream_response.status();
    if !status.is_success() {
        return text_response(
            StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
            "metadata storage returned an error",
        );
    }

    // Reject advertised oversize bodies up-front so we never even start
    // streaming gigabyte-sized payloads into memory.
    if let Some(len) = upstream_response
        .headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
    {
        if len > MAX_UPSTREAM_BYTES {
            return text_response(
                StatusCode::PAYLOAD_TOO_LARGE,
                "metadata object exceeds maximum size",
            );
        }
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

    let content_type_header = HeaderValue::from_str(&content_type)
        .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));
    let cache_control_header = HeaderValue::from_str(&cache_control)
        .unwrap_or_else(|_| HeaderValue::from_static("public, max-age=3600"));

    // Stream the upstream body to the client and cap the total bytes so a
    // lying Content-Length cannot sneak past the advertised-size check above.
    let limited =
        http_body_util::Limited::new(upstream_response.into_body(), MAX_UPSTREAM_BYTES as usize);
    let streamed_body = Body::new(limited);

    let mut response = Response::new(streamed_body);
    *response.status_mut() = StatusCode::OK;
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, content_type_header);
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, cache_control_header);

    response
}
