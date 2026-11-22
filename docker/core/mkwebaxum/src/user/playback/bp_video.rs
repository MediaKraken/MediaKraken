use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    body::{Body, Bytes},
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::{Html, IntoResponse, Json, Response},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use futures::Stream;
use serde::Deserialize;
use serde_json::json;
use sqlx::postgres::PgPool;
use std::io::SeekFrom;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncSeekExt, ReadBuf};

/// Media files are streamed to the browser in 1 MiB chunks.
const STREAM_CHUNK_SIZE: usize = 1024 * 1024;
/// Extensions the browser can decode natively; everything else goes
/// through the mktranscode HLS path.
const DIRECT_PLAYBACK_EXTENSIONS: [&str; 5] = ["mp4", "m4v", "webm", "ogv", "ogg"];
/// Root of the shared HLS working volume (see k8s chart): mktranscode
/// writes segments here, the web app serves them read-only.
const HLS_ROOT: &str = "/mediakraken/hls";
/// Segment/playlist names are restricted to this length.
const HLS_SEGMENT_NAME_MAX_LEN: usize = 128;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/playback/bss_user_playback_video.html")]
struct UserPlaybackVideoTemplate {
    page_title: Option<String>,
    media_name: Option<String>,
    /// `mm_media_guid` of the file to play; `None` on the parameter-less
    /// shell.
    media_guid: Option<String>,
    /// `direct` = native `<video>` stream, `hls` = mktranscode ABR ladder.
    playback_mode: Option<String>,
    media_source: Option<String>,
    media_mime: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlaybackVideoQuery {
    /// media guid (`mm_media_guid`) of the file to play in the browser
    media: Option<uuid::Uuid>,
}

pub async fn user_playback_video(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(query): Query<PlaybackVideoQuery>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        return (StatusCode::UNAUTHORIZED, Html(reply_html).into_response());
    }

    let (media_name, media_guid, playback_mode, media_source, media_mime) = match query.media {
        Some(media_guid) => {
            let media_path =
                match mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_path_by_uuid(
                    &state.sqlx_pool_ro,
                    media_guid,
                )
                .await
                {
                    Ok(media_path) => media_path,
                    Err(sqlx::Error::RowNotFound) => {
                        return (
                            StatusCode::NOT_FOUND,
                            Html(String::from("Not found")).into_response(),
                        );
                    }
                    Err(_) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Html(String::from("Internal server error")).into_response(),
                        );
                    }
                };
            let file_name = media_path
                .rsplit('/')
                .next()
                .unwrap_or(&media_path)
                .to_string();
            (
                Some(file_name),
                Some(media_guid.to_string()),
                Some(playback_mode_for_path(&media_path)),
                Some(media_source_for_path(&media_path, media_guid)),
                Some(mime_type_for_path(&media_path).to_string()),
            )
        }
        // the parameter-less form keeps working: plain player shell
        None => (None, None, None, None, None),
    };

    let template = UserPlaybackVideoTemplate {
        page_title: Some("MediaKraken Video Playback".to_string()),
        media_name,
        media_guid,
        playback_mode,
        media_source,
        media_mime,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/// `direct` when the browser can decode the file natively, `hls` when it
/// must be transcoded into an ABR ladder first.
fn playback_mode_for_path(media_path: &str) -> String {
    let extension = media_path
        .rsplit('.')
        .next()
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    if DIRECT_PLAYBACK_EXTENSIONS.contains(&extension.as_str()) {
        "direct".to_string()
    } else {
        "hls".to_string()
    }
}

/// The player source URL for a media file, by playback mode.
fn media_source_for_path(media_path: &str, media_guid: uuid::Uuid) -> String {
    if playback_mode_for_path(media_path) == "hls" {
        format!("/user/playback/hls/{media_guid}/master.m3u8")
    } else {
        format!("/user/playback/stream/{media_guid}")
    }
}

/// Only the playlist/segment names the transcode pod generates are served:
/// `master.m3u8`, `<height>.m3u8` and `<height>_seg_%05d.ts`.
fn valid_hls_segment_name(name: &str) -> bool {
    (1..=HLS_SEGMENT_NAME_MAX_LEN).contains(&name.len())
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
        && (name.ends_with(".m3u8") || name.ends_with(".ts"))
}

/// Resolves the on-disk path for `media_guid`, or a ready-made error
/// response. Shared by the start/segment handlers.
async fn resolve_media_path(
    state: &AppState,
    media_guid: uuid::Uuid,
) -> Result<String, (StatusCode, Json<serde_json::Value>)> {
    match mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_path_by_uuid(
        &state.sqlx_pool_ro,
        media_guid,
    )
    .await
    {
        Ok(media_path) => Ok(media_path),
        Err(sqlx::Error::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Media not found"})),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal server error"})),
        )),
    }
}

/// Publishes an `FFMPEG/HlsStart` message to the mktranscode queue, asking
/// the running consumer to begin (or restart) the HLS transcode session.
pub async fn user_playback_hls_start(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(query): Query<PlaybackVideoQuery>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::POST],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Not authorized"})),
        )
            .into_response();
    }
    let Some(media_guid) = query.media else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Missing media guid"})),
        )
            .into_response();
    };
    let media_path = match resolve_media_path(&state, media_guid).await {
        Ok(media_path) => media_path,
        Err(response) => return response.into_response(),
    };

    let (rabbit_connection, rabbit_channel) =
        match mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkwebapp").await {
            Ok(connection) => connection,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Transcoder unavailable"})),
                )
                    .into_response();
            }
        };
    // Collapse the publish result before the next await: the error type
    // (Box<dyn std::error::Error>) is not Send and must not live across it.
    let publish_ok = matches!(
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish_with_retry(
            &rabbit_connection,
            "mktranscode",
            json!({
                "Type": "FFMPEG",
                "Subtype": "HlsStart",
                "Media UUID": media_guid,
                "Media Path": media_path,
            })
            .to_string(),
        )
        .await,
        Ok(())
    );
    let _ =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(rabbit_channel, rabbit_connection).await;
    if !publish_ok {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Unable to queue the transcode"})),
        )
            .into_response();
    }
    (StatusCode::OK, Json(json!({"ok": true}))).into_response()
}

/// Publishes an `FFMPEG/HlsStop` message. Best-effort by design: the
/// browser sends this as a `sendBeacon` on page close, so the response is
/// never read. A failed publish only delays cleanup (the transcode pod
/// sweeps stale session directories by age).
pub async fn user_playback_hls_stop(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(query): Query<PlaybackVideoQuery>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::POST],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Not authorized"})),
        )
            .into_response();
    }
    let Some(media_guid) = query.media else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Missing media guid"})),
        )
            .into_response();
    };

    let (rabbit_connection, rabbit_channel) =
        match mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkwebapp").await {
            Ok(connection) => connection,
            Err(_) => return (StatusCode::OK, Json(json!({"ok": false}))).into_response(),
        };
    let publish_ok = matches!(
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish_with_retry(
            &rabbit_connection,
            "mktranscode",
            json!({
                "Type": "FFMPEG",
                "Subtype": "HlsStop",
                "Media UUID": media_guid,
            })
            .to_string(),
        )
        .await,
        Ok(())
    );
    let _ =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(rabbit_channel, rabbit_connection).await;
    (StatusCode::OK, Json(json!({"ok": publish_ok}))).into_response()
}

/// Serves one playlist or segment of an HLS session from the shared
/// working volume. No Range support: hls.js fetches both whole.
pub async fn user_playback_hls_segment(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path((media_guid, file_name)): Path<(uuid::Uuid, String)>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        return (StatusCode::UNAUTHORIZED, Html(reply_html)).into_response();
    }

    if !valid_hls_segment_name(&file_name) {
        return (StatusCode::BAD_REQUEST, Html(String::from("Bad request"))).into_response();
    }

    let file_path = format!("{HLS_ROOT}/{media_guid}/{file_name}");
    let Ok(file_metadata) = tokio::fs::metadata(&file_path).await else {
        return (StatusCode::NOT_FOUND, Html(String::from("Not found"))).into_response();
    };
    let file_size = file_metadata.len();
    let content_type = if file_name.ends_with(".m3u8") {
        "application/vnd.apple.mpegurl"
    } else {
        "video/mp2t"
    };

    let stream = match MediaFileStream::open(&file_path, 0, file_size).await {
        Ok(stream) => stream,
        Err(_) => {
            return (StatusCode::NOT_FOUND, Html(String::from("Not found"))).into_response();
        }
    };

    let mut response = Response::new(Body::from_stream(stream));
    *response.status_mut() = StatusCode::OK;
    let headers = response.headers_mut();
    headers.insert(
        "content-type",
        HeaderValue::from_str(content_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );
    headers.insert("content-length", HeaderValue::from(file_size));
    headers.insert("cache-control", HeaderValue::from_static("no-store"));
    response
}

/// A `Range: bytes=...` request present on the stream endpoint.
enum RangeRequest {
    /// Satisfiable range, inclusive bounds.
    Partial { start: u64, end: u64 },
    /// Header is present but not satisfiable.
    Unsatisfiable,
}

/// Parses a `Range: bytes=start-end` header against a file of `size` bytes.
/// Returns `None` when no (usable) Range header is present. Multiple ranges
/// are collapsed to the first one; the rest are not served.
fn parse_range_header(range: Option<&HeaderValue>, size: u64) -> Option<RangeRequest> {
    let raw = range.and_then(|header| header.to_str().ok())?;
    let Some(spec) = raw.trim().strip_prefix("bytes=") else {
        return Some(RangeRequest::Unsatisfiable);
    };
    let first = spec.split(',').next().unwrap_or(spec).trim();
    let Some((start_raw, end_raw)) = first.split_once('-') else {
        return Some(RangeRequest::Unsatisfiable);
    };
    let start_raw = start_raw.trim();
    let end_raw = end_raw.trim();
    if start_raw.is_empty() {
        // suffix form: `bytes=-N` asks for the last N bytes
        let Ok(suffix) = end_raw.parse::<u64>() else {
            return Some(RangeRequest::Unsatisfiable);
        };
        if suffix == 0 || size == 0 {
            return Some(RangeRequest::Unsatisfiable);
        }
        return Some(RangeRequest::Partial {
            start: size - suffix,
            end: size - 1,
        });
    }
    let Ok(start) = start_raw.parse::<u64>() else {
        return Some(RangeRequest::Unsatisfiable);
    };
    if start >= size {
        return Some(RangeRequest::Unsatisfiable);
    }
    let end = if end_raw.is_empty() {
        size - 1
    } else {
        match end_raw.parse::<u64>() {
            Ok(end) => end.min(size - 1),
            Err(_) => return Some(RangeRequest::Unsatisfiable),
        }
    };
    if start > end {
        return Some(RangeRequest::Unsatisfiable);
    }
    Some(RangeRequest::Partial { start, end })
}

/// Serves the on-disk file of `media_guid` with HTTP Range support so the
/// browser player can seek.
pub async fn user_playback_stream(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    headers: HeaderMap,
    Path(media_guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        return (StatusCode::UNAUTHORIZED, Html(reply_html)).into_response();
    }

    let media_path = match mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_path_by_uuid(
        &state.sqlx_pool_ro,
        media_guid,
    )
    .await
    {
        Ok(media_path) => media_path,
        Err(sqlx::Error::RowNotFound) => {
            return (StatusCode::NOT_FOUND, Html(String::from("Not found")))
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(String::from("Internal server error")),
            )
                .into_response();
        }
    };

    let Ok(file_metadata) = tokio::fs::metadata(&media_path).await else {
        return (StatusCode::NOT_FOUND, Html(String::from("Not found"))).into_response();
    };
    let file_size = file_metadata.len();
    let mime_type = mime_type_for_path(&media_path);

    let (status, content_range, length, start) =
        match parse_range_header(headers.get("range"), file_size) {
            None => (StatusCode::OK, None, file_size, 0u64),
            Some(RangeRequest::Partial { start, end }) => (
                StatusCode::PARTIAL_CONTENT,
                Some(format!("bytes {start}-{end}/{file_size}")),
                end - start + 1,
                start,
            ),
            Some(RangeRequest::Unsatisfiable) => {
                let mut response = (
                    StatusCode::RANGE_NOT_SATISFIABLE,
                    Html(String::from("Range not satisfiable")),
                )
                    .into_response();
                if let Ok(value) = HeaderValue::from_str(&format!("bytes */{file_size}")) {
                    response.headers_mut().insert("content-range", value);
                }
                return response;
            }
        };

    let stream = match MediaFileStream::open(&media_path, start, length).await {
        Ok(stream) => stream,
        Err(_) => {
            return (StatusCode::NOT_FOUND, Html(String::from("Not found"))).into_response();
        }
    };

    let mut response = Response::new(Body::from_stream(stream));
    *response.status_mut() = status;
    let headers = response.headers_mut();
    headers.insert(
        "content-type",
        HeaderValue::from_str(mime_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );
    headers.insert("accept-ranges", HeaderValue::from_static("bytes"));
    headers.insert("content-length", HeaderValue::from(length));
    if let Some(content_range) = &content_range
        && let Ok(value) = HeaderValue::from_str(content_range)
    {
        headers.insert("content-range", value);
    }
    response
}

/// Sequential reader streaming `length` bytes of `path` starting at `start`,
/// in fixed-size chunks.
struct MediaFileStream {
    file: tokio::fs::File,
    /// bytes left to serve
    remaining: u64,
    buffer: Vec<u8>,
}

impl MediaFileStream {
    async fn open(path: &str, start: u64, length: u64) -> std::io::Result<Self> {
        let mut file = tokio::fs::File::open(path).await?;
        file.seek(SeekFrom::Start(start)).await?;
        Ok(Self {
            file,
            remaining: length,
            buffer: vec![0u8; STREAM_CHUNK_SIZE],
        })
    }
}

impl Stream for MediaFileStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.remaining == 0 {
            return Poll::Ready(None);
        }
        let read_limit = std::cmp::min(this.buffer.len(), this.remaining as usize);
        let mut read_buf = ReadBuf::new(&mut this.buffer[..read_limit]);
        let outcome = AsyncRead::poll_read(Pin::new(&mut this.file), cx, &mut read_buf);
        match outcome {
            Poll::Ready(Ok(())) => {
                let filled = read_buf.filled();
                let n = filled.len();
                if n == 0 {
                    return Poll::Ready(None);
                }
                let bytes = Bytes::copy_from_slice(filled);
                this.remaining -= n as u64;
                Poll::Ready(Some(Ok(bytes)))
            }
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(err))),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Best-effort content type for a media file, by extension.
fn mime_type_for_path(path: &str) -> &'static str {
    let extension = path
        .rsplit('.')
        .next()
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    match extension.as_str() {
        "mp4" | "m4v" | "m4p" => "video/mp4",
        "webm" => "video/webm",
        "ogv" | "ogg" => "video/ogg",
        "mkv" => "video/x-matroska",
        "avi" => "video/x-msvideo",
        "mov" | "qt" => "video/quicktime",
        "mpeg" | "mpg" | "mpe" | "vob" | "m2v" => "video/mpeg",
        "flv" => "video/x-flv",
        "wmv" => "video/x-ms-wmv",
        "asf" => "video/x-ms-asf",
        "rm" | "rmvb" => "application/vnd.rn-realmedia",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_name_validation() {
        assert!(valid_hls_segment_name("master.m3u8"));
        assert!(valid_hls_segment_name("1080.m3u8"));
        assert!(valid_hls_segment_name("720_seg_00001.ts"));
        assert!(valid_hls_segment_name("src_seg_12345.ts"));
        assert!(!valid_hls_segment_name(""));
        assert!(!valid_hls_segment_name("playlist.m3u"));
        assert!(!valid_hls_segment_name("a.ts.bak"));
        assert!(!valid_hls_segment_name("../escape.ts"));
        assert!(!valid_hls_segment_name("a/b.ts"));
        assert!(!valid_hls_segment_name("seg-00001.tsX"));
        assert!(!valid_hls_segment_name(
            &"a".repeat(HLS_SEGMENT_NAME_MAX_LEN + 1)
        ));
    }

    #[test]
    fn playback_mode_by_extension() {
        assert_eq!(
            playback_mode_for_path("/mediakraken/metadata/m/x.mp4"),
            "direct"
        );
        assert_eq!(
            playback_mode_for_path("/mediakraken/metadata/m/x.MP4"),
            "direct"
        );
        assert_eq!(
            playback_mode_for_path("/mediakraken/metadata/m/x.mkv"),
            "hls"
        );
        assert_eq!(
            playback_mode_for_path("/mediakraken/metadata/m/x.avi"),
            "hls"
        );
        assert_eq!(
            playback_mode_for_path("/mediakraken/metadata/m/noext"),
            "hls"
        );
    }

    #[test]
    fn media_source_by_mode() {
        let guid = uuid::Uuid::new_v4();
        assert_eq!(
            media_source_for_path("/mediakraken/metadata/m/x.mp4", guid),
            format!("/user/playback/stream/{guid}")
        );
        assert_eq!(
            media_source_for_path("/mediakraken/metadata/m/x.mkv", guid),
            format!("/user/playback/hls/{guid}/master.m3u8")
        );
    }
}
