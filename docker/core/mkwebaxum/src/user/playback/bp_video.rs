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
/// ffprobe `format.format_name` values (the demuxer names) of the
/// containers the browser can demux natively. The Matroska family needs
/// the WebM-restricted codec check below: ffprobe reports the same
/// `format_name` for WebM and MKV, so the codecs separate them.
const MP4_FAMILY_FORMAT_NAME: &str = "mov,mp4,m4a,3gp,3g2,mj2";
const MATROSKA_FORMAT_NAME: &str = "matroska,webm";
const OGG_FORMAT_NAME: &str = "Ogg";
/// WebM-restricted codecs: the subset the browser decodes in Matroska.
const WEBM_VIDEO_CODECS: [&str; 3] = ["vp8", "vp9", "av1"];
const WEBM_AUDIO_CODECS: [&str; 2] = ["opus", "vorbis"];
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
            // The share header (smb:// / nfs://) is reconstructed from the
            // share row so the playback mode matches the path the
            // transcoder will actually open.
            let (media_path, probe) = match resolve_media_path(&state, media_guid).await {
                Ok(value) => value,
                Err((status, body)) => return (status, body.into_response()),
            };
            let file_name = media_path
                .rsplit('/')
                .next()
                .unwrap_or(&media_path)
                .to_string();
            (
                Some(file_name),
                Some(media_guid.to_string()),
                Some(playback_mode_for_media(&media_path, probe.as_ref())),
                Some(media_source_for_media(
                    &media_path,
                    probe.as_ref(),
                    media_guid,
                )),
                direct_mime_for_probe(probe.as_ref()).map(str::to_string),
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

/// `direct` when the browser can decode the probed container natively,
/// `hls` when it must be transcoded into an ABR ladder first. The probed
/// format is authoritative — the file extension is never consulted, so a
/// mislabelled file (e.g. Matroska in an `.mp4` name) takes the transcode
/// path. Network-share paths (`smb://` / `nfs://`) are not local files the
/// web app can open, so they always transcode — ffmpeg reaches them
/// directly, the same way the ffprobe pass does.
fn playback_mode_for_media(media_path: &str, probe: Option<&serde_json::Value>) -> String {
    if media_path.starts_with("smb://") || media_path.starts_with("nfs://") {
        return "hls".to_string();
    }
    if direct_mime_for_probe(probe).is_some() {
        "direct".to_string()
    } else {
        "hls".to_string()
    }
}

/// The player source URL for a media file, by playback mode.
fn media_source_for_media(
    media_path: &str,
    probe: Option<&serde_json::Value>,
    media_guid: uuid::Uuid,
) -> String {
    if playback_mode_for_media(media_path, probe) == "hls" {
        format!("/user/playback/hls/{media_guid}/master.m3u8")
    } else {
        format!("/user/playback/stream/{media_guid}")
    }
}

/// The content type the browser can demux natively for the probed
/// container, or `None` when the file must take the mktranscode HLS path.
/// This is the single direct-vs-HLS decision: the mode is `direct` exactly
/// when this returns `Some`, so the `<source type>` and the stream
/// `content-type` can never name a container the browser rejects.
///
/// ffprobe's `format.format_name` is the demuxer name; the Matroska family
/// is only directly playable when its streams use the WebM-restricted
/// codecs, because ffprobe cannot tell WebM from MKV by name. No probe
/// (or no `format_name` / `streams`) means the container is unknown, so
/// the safe answer is `None` (transcode).
fn direct_mime_for_probe(probe: Option<&serde_json::Value>) -> Option<&'static str> {
    let probe = probe?;
    let format_name = probe.get("format")?.get("format_name")?.as_str()?;
    if format_name.eq_ignore_ascii_case(MP4_FAMILY_FORMAT_NAME) {
        return Some("video/mp4");
    }
    if format_name.eq_ignore_ascii_case(MATROSKA_FORMAT_NAME) {
        return is_webm_subset(probe.get("streams")?).then_some("video/webm");
    }
    if format_name.eq_ignore_ascii_case(OGG_FORMAT_NAME) {
        return Some("video/ogg");
    }
    None
}

/// True when there is at least one video stream and every video/audio
/// stream uses a WebM-restricted codec — the subset the browser decodes
/// in Matroska. Subtitle/data streams are ignored.
fn is_webm_subset(streams: &serde_json::Value) -> bool {
    let Some(streams) = streams.as_array() else {
        return false;
    };
    let mut has_video = false;
    for stream in streams {
        let codec_name = stream
            .get("codec_name")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        match stream.get("codec_type").and_then(serde_json::Value::as_str) {
            Some("video") => {
                has_video = true;
                if !WEBM_VIDEO_CODECS
                    .iter()
                    .any(|codec| codec_name.eq_ignore_ascii_case(codec))
                {
                    return false;
                }
            }
            Some("audio")
                if !WEBM_AUDIO_CODECS
                    .iter()
                    .any(|codec| codec_name.eq_ignore_ascii_case(codec)) =>
            {
                return false;
            }
            _ => {}
        }
    }
    has_video
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

/// Resolves the stored path and the scanner's ffprobe document for
/// `media_guid`, or a ready-made error response. `mm_media_path` is stored
/// relative to the share root; the connection header (`smb://` / `nfs://`)
/// is rebuilt from the share row so the returned path is the exact string
/// the transcoder hands to ffmpeg. The ffprobe document
/// (`mm_media_ffprobe_json`) carries the probed container the playback
/// routing decides on and the source dimensions the transcode pod plans
/// from, so neither has to probe the source over the share again.
async fn resolve_media_path(
    state: &AppState,
    media_guid: uuid::Uuid,
) -> Result<(String, Option<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let row: (String, Option<serde_json::Value>, uuid::Uuid) = match sqlx::query_as(
        r#"select mm_media_path, mm_media_ffprobe_json, mm_media_share_guid
        from mm_media
        where mm_media_guid = $1"#,
    )
    .bind(media_guid)
    .fetch_one(&state.sqlx_pool_ro)
    .await
    {
        Ok(row) => row,
        Err(sqlx::Error::RowNotFound) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Media not found"})),
            ));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error"})),
            ));
        }
    };
    let media_path = match resolve_share_media_path(state, &row.0, row.2).await {
        Ok(path) => path,
        Err(response) => return Err(response),
    };
    Ok((media_path, row.1))
}

/// Rebuilds the connection header the scanner's `mk_media_path_prefix`
/// attaches to its messages: `mm_media_path` is stored relative to the
/// share root, and the header comes from the share row — the protocol
/// from `mm_network_share_version`, the host from `mm_network_share_ip`.
/// Paths that already carry a header pass through unchanged.
async fn resolve_share_media_path(
    state: &AppState,
    media_path: &str,
    share_guid: uuid::Uuid,
) -> Result<String, (StatusCode, Json<serde_json::Value>)> {
    if media_path.starts_with("smb://") || media_path.starts_with("nfs://") {
        return Ok(media_path.to_string());
    }
    let share =
        match mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_detail(
            &state.sqlx_pool_ro,
            share_guid,
        )
        .await
        {
            Ok(share) => share,
            Err(sqlx::Error::RowNotFound) => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Media share not found"})),
                ));
            }
            Err(_) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal server error"})),
                ));
            }
        };
    match share_media_path(
        share.mm_network_share_version,
        share.mm_network_share_ip,
        &share.mm_network_share_path,
        media_path,
    ) {
        Ok(path) => Ok(path),
        Err(error) => {
            eprintln!(
                "mkwebaxum: media share header resolution failed for share {share_guid} \
                 (path {media_path}): {error}"
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Unsupported media share path"})),
            ))
        }
    }
}

/// The full media path from the share row's protocol and host.
///
/// `mm_network_share_version` is authoritative (protocol table in
/// `mk_lib_network_share`): 0-2 is SMB, where the share path is a UNC
/// name and only the share segment is used (host comes from
/// `mm_network_share_ip`); 5-9 is NFS, where the share path is
/// server-relative. Any other value is rejected rather than guessed.
fn share_media_path(
    share_version: i16,
    share_ip: std::net::IpAddr,
    share_path: &str,
    media_path: &str,
) -> Result<String, String> {
    let header = match share_version {
        0..=2 => {
            let share_name =
                mk_lib_database::mk_lib_database_network_share::parse_share_name(share_path)
                    .ok_or_else(|| format!("share path '{share_path}' is not a valid UNC path"))?;
            format!("smb://{share_ip}/{share_name}")
        }
        5..=9 => {
            let share_root = share_path.trim_start_matches('/');
            format!("nfs://{share_ip}/{share_root}")
        }
        version => return Err(format!("unsupported share protocol version {version}")),
    };
    Ok(format!("{header}{media_path}"))
}

/// First video stream dimensions from a scanner ffprobe document.
fn source_dimensions_from_probe(probe: Option<&serde_json::Value>) -> Option<(u32, u32)> {
    let streams = probe?.get("streams")?.as_array()?;
    for stream in streams {
        if stream.get("codec_type").and_then(serde_json::Value::as_str) == Some("video")
            && let Some(width) = stream.get("width").and_then(serde_json::Value::as_u64)
            && let Some(height) = stream.get("height").and_then(serde_json::Value::as_u64)
        {
            return match (width.try_into(), height.try_into()) {
                (Ok(width), Ok(height)) => Some((width, height)),
                _ => None,
            };
        }
    }
    None
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
    let (media_path, probe) = match resolve_media_path(&state, media_guid).await {
        Ok(value) => value,
        Err(response) => return response.into_response(),
    };
    // The source dimensions come from the scanner's ffprobe pass, so the
    // transcode pod plans the ladder without probing the share itself.
    let source_dimensions = source_dimensions_from_probe(probe.as_ref());

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
    let mut message = json!({
        "Type": "FFMPEG",
        "Subtype": "HlsStart",
        "Media UUID": media_guid,
        "Media Path": media_path,
    });
    if let Some((width, height)) = source_dimensions {
        message["Source Width"] = json!(width);
        message["Source Height"] = json!(height);
    }
    // Collapse the publish result before the next await: the error type
    // (Box<dyn std::error::Error>) is not Send and must not live across it.
    let publish_ok = matches!(
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish_with_retry(
            &rabbit_connection,
            "mktranscode",
            message.to_string(),
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

    // The ffprobe document selects the content type by probed container,
    // the same way the playback page picks the playback mode.
    let row: Option<(String, Option<serde_json::Value>)> = match sqlx::query_as(
        r#"select mm_media_path, mm_media_ffprobe_json
        from mm_media
        where mm_media_guid = $1"#,
    )
    .bind(media_guid)
    .fetch_optional(&state.sqlx_pool_ro)
    .await
    {
        Ok(row) => row,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(String::from("Internal server error")),
            )
                .into_response();
        }
    };
    let Some((media_path, probe)) = row else {
        return (StatusCode::NOT_FOUND, Html(String::from("Not found"))).into_response();
    };

    let Ok(file_metadata) = tokio::fs::metadata(&media_path).await else {
        return (StatusCode::NOT_FOUND, Html(String::from("Not found"))).into_response();
    };
    let file_size = file_metadata.len();
    // Containers the browser cannot demux (or no probe at all) are served
    // as opaque bytes: the playback page never routes them here.
    let mime_type = direct_mime_for_probe(probe.as_ref()).unwrap_or("application/octet-stream");

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

    /// A minimal ffprobe document with the given `format_name` and streams.
    fn probe_with_format(format_name: &str, streams: serde_json::Value) -> serde_json::Value {
        json!({
            "format": { "format_name": format_name },
            "streams": streams,
        })
    }

    fn probe_video_audio(video: &str, audio: &str) -> serde_json::Value {
        json!([
            { "codec_type": "video", "codec_name": video },
            { "codec_type": "audio", "codec_name": audio },
        ])
    }

    #[test]
    fn direct_mime_by_probe() {
        // The MP4/MOV family is demuxed natively.
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "mov,mp4,m4a,3gp,3g2,mj2",
                probe_video_audio("h264", "aac")
            ))),
            Some("video/mp4")
        );
        // Matroska family: only the WebM-restricted codec subset is
        // directly playable — ffprobe cannot tell WebM from MKV by name.
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "matroska,webm",
                probe_video_audio("vp9", "opus")
            ))),
            Some("video/webm")
        );
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "matroska,webm",
                probe_video_audio("vp8", "vorbis")
            ))),
            Some("video/webm")
        );
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "matroska,webm",
                probe_video_audio("av1", "opus")
            ))),
            Some("video/webm")
        );
        // A typical MKV (H.264 + AC-3) is not a playable WebM.
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "matroska,webm",
                probe_video_audio("h264", "ac3")
            ))),
            None
        );
        // WebM video with a non-WebM audio codec is not directly playable.
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "matroska,webm",
                probe_video_audio("vp9", "aac")
            ))),
            None
        );
        // Audio-only Matroska has no video to show in the player.
        assert_eq!(
            direct_mime_for_probe(Some(&json!({
                "format": { "format_name": "matroska,webm" },
                "streams": [{ "codec_type": "audio", "codec_name": "opus" }],
            }))),
            None
        );
        // A missing codec name fails the WebM check, never the match.
        assert_eq!(
            direct_mime_for_probe(Some(&json!({
                "format": { "format_name": "matroska,webm" },
                "streams": [{ "codec_type": "video" }],
            }))),
            None
        );
        // Ogg is demuxed natively.
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "Ogg",
                probe_video_audio("theora", "vorbis")
            ))),
            Some("video/ogg")
        );
        // Other containers (AVI, MPEG-TS, ...) transcode.
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "avi",
                probe_video_audio("mpeg4", "mp3")
            ))),
            None
        );
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "mpegts",
                probe_video_audio("h264", "ac3")
            ))),
            None
        );
        // No probe, no format name, or no streams: the container is
        // unknown, so it transcodes.
        assert_eq!(direct_mime_for_probe(None), None);
        assert_eq!(direct_mime_for_probe(Some(&json!({}))), None);
        assert_eq!(direct_mime_for_probe(Some(&json!({ "format": {} }))), None);
        assert_eq!(
            direct_mime_for_probe(Some(&json!({
                "format": { "format_name": "matroska,webm" },
            }))),
            None
        );
        // Demuxer names are matched case-insensitively.
        assert_eq!(
            direct_mime_for_probe(Some(&probe_with_format(
                "MOV,MP4,M4A,3GP,3G2,MJ2",
                probe_video_audio("h264", "aac")
            ))),
            Some("video/mp4")
        );
    }

    #[test]
    fn playback_mode_by_probe() {
        let mp4 = probe_with_format("mov,mp4,m4a,3gp,3g2,mj2", probe_video_audio("h264", "aac"));
        let mkv = probe_with_format("matroska,webm", probe_video_audio("h264", "ac3"));
        let webm = probe_with_format("matroska,webm", probe_video_audio("vp9", "opus"));

        // The probe decides, not the name: a true MP4 plays direct even
        // under an .mkv name; a Matroska file transcodes under any name.
        assert_eq!(
            playback_mode_for_media("/mediakraken/metadata/m/x.mkv", Some(&mp4)),
            "direct"
        );
        assert_eq!(
            playback_mode_for_media("/mediakraken/metadata/m/x.mp4", Some(&mkv)),
            "hls"
        );
        assert_eq!(
            playback_mode_for_media("/mediakraken/metadata/m/x.webm", Some(&webm)),
            "direct"
        );
        // A .webm-named file that is really a full MKV still transcodes.
        assert_eq!(
            playback_mode_for_media("/mediakraken/metadata/m/x.webm", Some(&mkv)),
            "hls"
        );
        // No probe data: the container is unknown, so it transcodes.
        assert_eq!(
            playback_mode_for_media("/mediakraken/metadata/m/x.mp4", None),
            "hls"
        );
        // Share paths always transcode, even for directly playable
        // containers: the web app cannot open them as local files.
        assert_eq!(
            playback_mode_for_media("smb://192.168.50.50/video/m/x.mp4", Some(&mp4)),
            "hls"
        );
        assert_eq!(
            playback_mode_for_media("nfs://192.168.50.50/media/m/x.mkv", Some(&mkv)),
            "hls"
        );
    }

    #[test]
    fn share_media_path_header() {
        let ip: std::net::IpAddr = "192.168.50.50".parse().unwrap();
        // SMB (version 1 = smb2): UNC share path, only the share segment
        // is used; the host comes from the share row.
        assert_eq!(
            share_media_path(1, ip, "\\\\192.168.50.50\\video", "/movies/a.mkv").unwrap(),
            "smb://192.168.50.50/video/movies/a.mkv"
        );
        // Forward-slash UNC with a deeper subpath: the subpath is dropped.
        assert_eq!(
            share_media_path(2, ip, "//192.168.50.50/video/sub", "/a.mkv").unwrap(),
            "smb://192.168.50.50/video/a.mkv"
        );
        // NFS (version 8 = nfs4.1): server-relative share path.
        assert_eq!(
            share_media_path(8, ip, "/media", "/movies/a.mkv").unwrap(),
            "nfs://192.168.50.50/media/movies/a.mkv"
        );
        // Unknown protocol versions are rejected, never guessed.
        assert!(share_media_path(3, ip, "\\\\h\\video", "/a.mkv").is_err());
        assert!(share_media_path(4, ip, "\\\\h\\video", "/a.mkv").is_err());
        // SMB share paths that do not yield a share name are rejected.
        assert!(share_media_path(1, ip, "192.168.50.50", "/a.mkv").is_err());
    }

    #[test]
    fn media_source_by_mode() {
        let guid = uuid::Uuid::new_v4();
        let mp4 = probe_with_format("mov,mp4,m4a,3gp,3g2,mj2", probe_video_audio("h264", "aac"));
        let mkv = probe_with_format("matroska,webm", probe_video_audio("h264", "ac3"));
        // A .mkv that probes as MP4 plays direct; a .mkv that probes as
        // Matroska transcodes — the extension never decides.
        assert_eq!(
            media_source_for_media("/mediakraken/metadata/m/x.mkv", Some(&mp4), guid),
            format!("/user/playback/stream/{guid}")
        );
        assert_eq!(
            media_source_for_media("/mediakraken/metadata/m/x.mkv", Some(&mkv), guid),
            format!("/user/playback/hls/{guid}/master.m3u8")
        );
    }
}
