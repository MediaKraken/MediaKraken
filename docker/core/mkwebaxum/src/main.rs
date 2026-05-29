use axum::http::header;
use axum::http::Method;
use axum::{
    Router,
    body::Body,
    extract::FromRef,
    extract::Request,
    extract::ConnectInfo,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::routing::RouterExt;
use axum_prometheus::PrometheusMetricLayer;
use axum_session::{Key, SameSitePolicy, SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use mk_lib_database;
use mk_lib_logging;
use ring::digest;
use serde_json::json;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use tokio::net::TcpListener;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;
mod axum_custom_filters;
mod user_preferences;

#[path = "admin"]
pub mod admin {
    pub mod bp_backup;
    pub mod bp_cron;
    pub mod bp_database;
    pub mod bp_game_servers;
    pub mod bp_hardware;
    pub mod bp_home;
    pub mod bp_library;
    pub mod bp_logging;
    pub mod bp_reports;
    pub mod bp_s3;
    pub mod bp_server_links;
    pub mod bp_settings;
    pub mod bp_torrent;
    pub mod bp_user;
}

#[path = "api"]
pub mod api {
    pub mod bp_api_title_search;
}

#[path = "error/bp_error.rs"]
mod bp_error;

#[path = "public"]
pub mod public {
    pub mod bp_about;
    pub mod bp_forgot_password;
    pub mod bp_health_check;
    pub mod bp_login;
    pub mod bp_logout;
    pub mod bp_register;
}

#[path = "user"]
pub mod user {
    pub mod bp_hardware;
    pub mod bp_home;
    pub mod bp_profile;
    pub mod bp_queue;
    pub mod bp_search;
    pub mod bp_sync;
}

#[path = "user/internet"]
pub mod user_internet {
    pub mod bp_inter_flickr;
    pub mod bp_inter_home;
    pub mod bp_inter_twitchtv;
    pub mod bp_inter_vimeo;
    pub mod bp_inter_youtube;
}

#[path = "user/media"]
pub mod user_media {
    pub mod bp_media_book;
    pub mod bp_media_collection;
    pub mod bp_media_game;
    pub mod bp_media_game_servers;
    pub mod bp_media_genre;
    pub mod bp_media_home_media;
    pub mod bp_media_image;
    pub mod bp_media_iradio;
    pub mod bp_media_movie;
    pub mod bp_media_music;
    pub mod bp_media_music_video;
    pub mod bp_media_physical;
    pub mod bp_media_sports;
    pub mod bp_media_tv;
}

#[path = "user/metadata"]
pub mod user_metadata {
    pub mod bp_meta_book;
    pub mod bp_meta_game;
    pub mod bp_meta_game_system;
    pub mod bp_meta_movie;
    pub mod bp_meta_music;
    pub mod bp_meta_music_video;
    pub mod bp_meta_object;
    pub mod bp_meta_openlibrary;
    pub mod bp_meta_person;
    pub mod bp_meta_sports;
    pub mod bp_meta_tv;
}

#[path = "user/playback"]
pub mod user_playback {
    pub mod bp_audio;
    pub mod bp_comic;
    pub mod bp_video;
}

#[derive(Clone)]
struct AppState {
    flash_config: axum_flash::Config,
    pub sqlx_pool_rw: PgPool,
    pub sqlx_pool_ro: PgPool,
    client: Client,
    login_limiter: Arc<RateLimiter>,
    register_limiter: Arc<RateLimiter>,
}

// Our state type must implement this trait. That is how the config
// is passed to axum-flash in a type safe way.
impl FromRef<AppState> for axum_flash::Config {
    fn from_ref(state: &AppState) -> axum_flash::Config {
        state.flash_config.clone()
    }
}

// Signing key for flash/session cookies. Derives a stable 64-byte key from
// MKWEBAPP_SIGNING_KEY so cookies survive restarts and are consistent across
// replicas. Falls back to an ephemeral key with a loud warning for dev.
fn load_signing_key() -> Key {
    match std::env::var("MKWEBAPP_SIGNING_KEY") {
        Ok(secret) if secret.len() >= 32 => {
            let hash = digest::digest(&digest::SHA512, secret.as_bytes());
            Key::from(hash.as_ref())
        }
        _ => {
            tracing::warn!(
                "MKWEBAPP_SIGNING_KEY not set or shorter than 32 bytes; \
                 generating ephemeral key. Existing sessions will be invalidated on restart \
                 and replicas will not share cookies."
            );
            Key::generate()
        }
    }
}

/// Simple in-memory rate limiter keyed by IP address.
struct RateLimiter {
    inner: RwLock<HashMap<String, Vec<Instant>>>,
    window: Duration,
    max_requests: usize,
}

impl RateLimiter {
    fn new(window: Duration, max_requests: usize) -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
            window,
            max_requests,
        }
    }

    async fn is_allowed(&self, key: &str) -> bool {
        let mut map = self.inner.write().await;
        let now = Instant::now();
        let entries = map.entry(key.to_string()).or_insert_with(Vec::new);
        entries.retain(|t| now.duration_since(*t) < self.window);
        if entries.len() >= self.max_requests {
            false
        } else {
            entries.push(now);
            true
        }
    }
}

/// Security headers middleware. Adds standard HTTP security headers to every response.
async fn security_headers(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        header::HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::X_FRAME_OPTIONS,
        header::HeaderValue::from_static("DENY"),
    );
    headers.insert(
        header::X_XSS_PROTECTION,
        header::HeaderValue::from_static("1; mode=block"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        header::HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    // Prevent client-side caching of pages that require auth.
    headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store, no-cache, must-revalidate"),
    );
    headers.remove(header::SERVER);
    response
}

// CSRF defense via strict same-origin check on state-changing requests.
// Rejects POST/PUT/PATCH/DELETE whose Origin (or Referer fallback) does not
// match MKWEBAPP_ALLOWED_ORIGINS (comma-separated list of scheme://host[:port]).
// An empty allowed-origins list is a production hard-fail.
async fn require_same_origin(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let is_state_changing =
        matches!(method, Method::POST | Method::PUT | Method::PATCH | Method::DELETE);
    if !is_state_changing {
        return next.run(req).await;
    }

    let allowed_raw = std::env::var("MKWEBAPP_ALLOWED_ORIGINS").unwrap_or_default();
    let allowed: Vec<&str> = allowed_raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if allowed.is_empty() {
        tracing::error!(
            "MKWEBAPP_ALLOWED_ORIGINS is empty; rejecting cross-origin requests \
             with 403. This is a production safety measure."
        );
        return (StatusCode::FORBIDDEN, "cross-origin request rejected").into_response();
    }

    let headers = req.headers();
    let matches_allowed = |value: &str| -> bool {
        allowed.iter().any(|a| {
            value == *a
                || value
                    .strip_prefix(*a)
                    .map(|rest| rest.starts_with('/') || rest.is_empty())
                    .unwrap_or(false)
        })
    };

    let origin_ok = headers
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(matches_allowed)
        .or_else(|| {
            headers
                .get(header::REFERER)
                .and_then(|v| v.to_str().ok())
                .map(matches_allowed)
        })
        .unwrap_or(false);

    if !origin_ok {
        tracing::warn!(method = %method, uri = %req.uri(), "cross-origin state-changing request rejected");
        return (StatusCode::FORBIDDEN, "cross-origin request rejected").into_response();
    }

    next.run(req).await
}

/// Rate-limiting middleware for login and registration endpoints.
/// Only applies to POST requests on /public/login, /public/register, and
/// /public/forgot_password. All other requests pass through unmodified.
async fn rate_limit_auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    // Only apply rate limiting to auth POST endpoints.
    if req.method() != Method::POST {
        return next.run(req).await;
    }
    let path = req.uri().path();
    let limiter = match path {
        "/public/login" | "/public/forgot_password" => &state.login_limiter,
        "/public/register" => &state.register_limiter,
        _ => return next.run(req).await,
    };

    let addr = match req.extensions().get::<ConnectInfo<SocketAddr>>() {
        Some(addr) => addr.0.ip().to_string(),
        None => return next.run(req).await,
    };

    if !limiter.is_allowed(&addr).await {
        tracing::warn!(ip = %addr, path, "rate limit exceeded on auth endpoint");
        return (StatusCode::TOO_MANY_REQUESTS, "too many requests, please try again later").into_response();
    }

    next.run(req).await
}

#[tokio::main]
async fn main() {
    if let Err(error) = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(json!({
        "level": "info",
        "message": "App start",
        "module": module_path!(),
    }))
    .await
    {
        eprintln!("loki push error: {error}");
    }
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
            .await
            .unwrap();
    let _result = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(
        &sqlx_pool_ro,
        false,
    )
    .await;

    // Session config with security flags.
    // - secure_cookies: sent only over HTTPS (nginx ingress handles TLS).
    // - http_only: not accessible via JavaScript (mitigates XSS cookie theft).
    // - same_site: Lax allows same-site navigations but blocks cross-site POST.
    // - max_age: 1 hour of inactivity or absolute expiration, whichever comes first.
    let session_config = SessionConfig::default()
        .with_table_name("mm_session")
        .with_secure_cookies(true)
        .with_http_only(true)
        .with_same_site_policy(SameSitePolicy::Lax)
        .with_max_age(Duration::from_secs(3600));
    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(sqlx_pool_rw.clone().into()), session_config)
            .await
            .unwrap();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    // build our application with routes
    let client: Client =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

    let signing_key = load_signing_key();

    // Rate limiters: 10 requests per minute for login, 5 per minute for registration.
    let login_limiter = Arc::new(RateLimiter::new(Duration::from_secs(60), 10));
    let register_limiter = Arc::new(RateLimiter::new(Duration::from_secs(60), 5));

    let app_state = AppState {
        flash_config: axum_flash::Config::new(signing_key),
        sqlx_pool_rw: sqlx_pool_rw.clone(),
        sqlx_pool_ro: sqlx_pool_ro.clone(),
        client: client.clone(),
        login_limiter: login_limiter.clone(),
        register_limiter: register_limiter.clone(),
    };

    // route_with_tsr creates two routes.....one with trailing slash
    let app = Router::new()
        .route_with_tsr("/admin", get(admin::bp_home::admin_home))
        .route_with_tsr("/admin/backup", get(admin::bp_backup::admin_backup))
        .route_with_tsr("/admin/cron", get(admin::bp_cron::admin_cron))
        .route_with_tsr(
            "/admin/cron_run/{guid}",
            get(admin::bp_cron::admin_cron_run),
        )
        .route_with_tsr(
            "/admin/cron_update",
            post(admin::bp_cron::admin_cron_update),
        )
        .route_with_tsr("/admin/database", get(admin::bp_database::admin_database))
        .route_with_tsr(
            "/admin/game_servers/{page}",
            get(admin::bp_game_servers::admin_game_servers),
        )
        .route_with_tsr("/admin/hardware", get(admin::bp_hardware::admin_hardware))
        .route_with_tsr("/admin/home", get(admin::bp_home::admin_home))
        .route_with_tsr("/admin/logging", get(admin::bp_logging::admin_logging))
        .route_with_tsr(
            "/admin/library",
            get(admin::bp_library::admin_library).post(admin::bp_library::admin_library_share_add),
        )
        .route_with_tsr(
            "/admin/library_media_scan",
            get(admin::bp_library::admin_library_media_scan),
        )
        .route_with_tsr(
            "/admin/library_share_scan",
            get(admin::bp_library::admin_library_share_scan),
        )
        .route_with_tsr(
            "/admin/library/share_directories",
            get(admin::bp_library::admin_library_share_directories),
        )
        .route_with_tsr(
            "/admin/server_links",
            get(admin::bp_server_links::admin_server_links)
                .post(admin::bp_server_links::admin_server_links_post),
        )
        .route_with_tsr("/admin/s3", get(admin::bp_s3::admin_s3))
        .route_with_tsr("/admin/settings", get(admin::bp_settings::admin_settings))
        .route_with_tsr(
            "/admin/report_known_media/{page}",
            get(admin::bp_reports::admin_report_known_media),
        )
        .route_with_tsr("/admin/torrent", get(admin::bp_torrent::admin_torrent))
        .route_with_tsr(
            "/admin/torrent_add",
            post(admin::bp_torrent::admin_torrent_add),
        )
        .route_with_tsr(
            "/admin/torrent_start/{torrent_id}",
            post(admin::bp_torrent::admin_torrent_start),
        )
        .route_with_tsr(
            "/admin/torrent_stop/{torrent_id}",
            post(admin::bp_torrent::admin_torrent_stop),
        )
        .route_with_tsr(
            "/admin/torrent_delete/{torrent_id}",
            post(admin::bp_torrent::admin_torrent_delete),
        )
        .route_with_tsr("/admin/user/{page}", get(admin::bp_user::admin_user))
        .route_with_tsr(
            "/user/internet/flickr",
            get(user_internet::bp_inter_flickr::user_inter_flickr),
        )
        .route_with_tsr(
            "/user/internet/flickr/{guid}",
            get(user_internet::bp_inter_flickr::user_inter_flickr_detail),
        )
        .route_with_tsr(
            "/user/internet",
            get(user_internet::bp_inter_home::user_inter_home),
        )
        .route_with_tsr(
            "/user/metadata/openlibrary/{page}",
            get(user_metadata::bp_meta_openlibrary::user_inter_openlibrary),
        )
        .route_with_tsr(
            "/user/metadata/openlibrary_detail/{work_id}",
            get(user_metadata::bp_meta_openlibrary::user_inter_openlibrary_detail),
        )
        .route_with_tsr(
            "/user/internet/twitchtv",
            get(user_internet::bp_inter_twitchtv::user_inter_twitchtv),
        )
        .route_with_tsr(
            "/user/internet/twitchtv/{stream_name}",
            get(user_internet::bp_inter_twitchtv::user_inter_twitchtv_detail),
        )
        .route_with_tsr(
            "/user/internet/vimeo",
            get(user_internet::bp_inter_vimeo::user_inter_vimeo),
        )
        .route_with_tsr(
            "/user/internet/vimeo/{video_id}",
            get(user_internet::bp_inter_vimeo::user_inter_vimeo_detail),
        )
        .route_with_tsr(
            "/user/internet/youtube",
            get(user_internet::bp_inter_youtube::user_inter_youtube),
        )
        .route_with_tsr(
            "/user/internet/youtube/{guid}",
            get(user_internet::bp_inter_youtube::user_inter_youtube_detail),
        )
        .route_with_tsr(
            "/user/media/book/{page}",
            get(user_media::bp_media_book::user_media_book),
        )
        .route_with_tsr(
            "/user/media/book_detail/{guid}",
            get(user_media::bp_media_book::user_media_book_detail),
        )
        .route_with_tsr(
            "/user/media/game/{page}",
            get(user_media::bp_media_game::user_media_game),
        )
        .route_with_tsr(
            "/user/media/game_detail/{guid}",
            get(user_media::bp_media_game::user_media_game_detail),
        )
        .route_with_tsr(
            "/user/media/game_servers/{page}",
            get(user_media::bp_media_game_servers::user_media_game_servers),
        )
        .route_with_tsr(
            "/user/media/genre",
            get(user_media::bp_media_genre::user_media_genre),
        )
        .route_with_tsr(
            "/user/media/home_media/{page}",
            get(user_media::bp_media_home_media::user_media_home_media),
        )
        .route_with_tsr(
            "/user/media/image",
            get(user_media::bp_media_image::user_media_image),
        )
        .route_with_tsr(
            "/user/media/iradio/{page}",
            get(user_media::bp_media_iradio::user_media_iradio),
        )
        .route(
            "/user/metadata/object/{*object_key}",
            get(user_metadata::bp_meta_object::metadata_object_proxy),
        )
        .route_with_tsr(
            "/user/media/movie/{page}",
            get(user_media::bp_media_movie::user_media_movie),
        )
        .route_with_tsr(
            "/user/media/movie_detail/{guid}",
            get(user_media::bp_media_movie::user_media_movie_detail),
        )
        .route_with_tsr(
            "/user/media/music/{page}",
            get(user_media::bp_media_music::user_media_music),
        )
        .route_with_tsr(
            "/user/media/music_detail/{guid}",
            get(user_media::bp_media_music::user_media_music_detail),
        )
        .route_with_tsr(
            "/user/media/music_video/{page}",
            get(user_media::bp_media_music_video::user_media_music_video),
        )
        .route_with_tsr(
            "/user/media/music_video_detail/{guid}",
            get(user_media::bp_media_music_video::user_media_music_video_detail),
        )
        .route_with_tsr(
            "/user/media/physical/{page}",
            get(user_media::bp_media_physical::user_media_physical_post),
        )
        .route_with_tsr(
            "/user/media/sports/{page}",
            get(user_media::bp_media_sports::user_media_sports),
        )
        .route_with_tsr(
            "/user/media/sports_detail/{guid}",
            get(user_media::bp_media_sports::user_media_sports_detail),
        )
        .route_with_tsr(
            "/user/media/tv/{page}",
            get(user_media::bp_media_tv::user_media_tv),
        )
        .route_with_tsr(
            "/user/media/tv_detail/{guid}",
            get(user_media::bp_media_tv::user_media_tv_detail),
        )
        .route_with_tsr(
            "/user/metadata/book/{page}",
            get(user_metadata::bp_meta_book::user_metadata_book),
        )
        .route_with_tsr(
            "/user/metadata/book_detail/{guid}",
            get(user_metadata::bp_meta_book::user_metadata_book_detail),
        )
        .route_with_tsr(
            "/user/metadata/collection/{page}",
            get(user_media::bp_media_collection::user_media_collection),
        )
        .route_with_tsr(
            "/user/metadata/collection_detail/{guid}",
            get(user_media::bp_media_collection::user_media_collection_detail),
        )
        .route_with_tsr(
            "/user/metadata/game/{page}",
            get(user_metadata::bp_meta_game::user_metadata_game),
        )
        .route_with_tsr(
            "/user/metadata/game_detail/{guid}",
            get(user_metadata::bp_meta_game::user_metadata_game_detail),
        )
        .route_with_tsr(
            "/user/metadata/game_system/{page}",
            get(user_metadata::bp_meta_game_system::user_metadata_game_system),
        )
        .route_with_tsr(
            "/user/metadata/game_system_detail/{guid}",
            get(user_metadata::bp_meta_game_system::user_metadata_game_system_detail),
        )
        .route_with_tsr(
            "/user/metadata/movie/{page}",
            get(user_metadata::bp_meta_movie::user_metadata_movie),
        )
        .route_with_tsr(
            "/user/metadata/movie_detail/{guid}",
            get(user_metadata::bp_meta_movie::user_metadata_movie_detail),
        )
        .route_with_tsr(
            "/user/metadata/music/{page}",
            get(user_metadata::bp_meta_music::user_metadata_music),
        )
        .route_with_tsr(
            "/user/metadata/music_detail/{guid}",
            get(user_metadata::bp_meta_music::user_metadata_music_detail),
        )
        .route_with_tsr(
            "/user/metadata/music_video/{page}",
            get(user_metadata::bp_meta_music_video::user_metadata_music_video),
        )
        .route_with_tsr(
            "/user/metadata/music_video_detail/{guid}",
            get(user_metadata::bp_meta_music_video::user_metadata_music_video_detail),
        )
        .route_with_tsr(
            "/user/metadata/person/{page}",
            get(user_metadata::bp_meta_person::user_metadata_person),
        )
        .route_with_tsr(
            "/user/metadata/person_detail/{guid}",
            get(user_metadata::bp_meta_person::user_metadata_person_detail),
        )
        .route_with_tsr(
            "/user/metadata/sports/{page}",
            get(user_metadata::bp_meta_sports::user_metadata_sports),
        )
        .route_with_tsr(
            "/user/metadata/sports_detail/{guid}",
            get(user_metadata::bp_meta_sports::user_metadata_sports_detail),
        )
        .route_with_tsr(
            "/user/metadata/tv/{page}",
            get(user_metadata::bp_meta_tv::user_metadata_tv),
        )
        .route_with_tsr(
            "/user/metadata/tv_detail/{guid}",
            get(user_metadata::bp_meta_tv::user_metadata_tv_detail),
        )
        .route_with_tsr(
            "/user/playback/audio",
            get(user_playback::bp_audio::user_playback_audio),
        )
        .route_with_tsr(
            "/user/playback/comic",
            get(user_playback::bp_comic::user_playback_comic),
        )
        .route_with_tsr(
            "/user/playback/video",
            get(user_playback::bp_video::user_playback_video),
        )
        .route_with_tsr("/user/hardware", get(user::bp_hardware::user_hardware))
        .route_with_tsr("/user/home", get(user::bp_home::user_home))
        .route_with_tsr("/user/profile", get(user::bp_profile::user_profile))
        .route(
            "/user/profile/photo",
            post(user::bp_profile::user_profile_photo_post),
        )
        .route(
            "/user/profile/pagination",
            post(user::bp_profile::user_profile_pagination_post),
        )
        .route(
            "/user/profile/number-format-language",
            post(user::bp_profile::user_profile_number_format_language_post),
        )
        .route_with_tsr("/user/queue", get(user::bp_queue::user_queue))
        .route("/user/search", get(user::bp_search::search_handler))
        .route_with_tsr("/user/sync/{page}", get(user::bp_sync::user_sync))
        .route_with_tsr(
            "/user/user_media_movie_status",
            post(user_media::bp_media_movie::user_media_movie_status),
        )
        .route_with_tsr(
            "/user/user_metadata_movie_status",
            post(user_metadata::bp_meta_movie::user_metadata_movie_status),
        )
        .route_with_tsr(
            "/user/user_media_tv_status/{uuid}/{key}",
            post(user_media::bp_media_tv::user_media_tv_status),
        )
        .route_with_tsr(
            "/user/user_metadata_tv_status/{uuid}/{key}",
            post(user_metadata::bp_meta_tv::user_metadata_tv_status),
        )
        .route_with_tsr("/public/logout", get(public::bp_logout::public_logout))
        .route_with_tsr(
            "/public/login",
            get(public::bp_login::public_login)
                .post(public::bp_login::public_login_post),
        )
        .nest_service(
            "/static",
            ServiceBuilder::new()
                .layer(SetResponseHeaderLayer::overriding(
                    header::CACHE_CONTROL,
                    axum::http::HeaderValue::from_static("public, max-age=31536000, immutable"),
                ))
                .service(ServeDir::new("static")),
        )
        .nest_service(
            "/metadata",
            ServiceBuilder::new()
                .layer(SetResponseHeaderLayer::overriding(
                    header::CACHE_CONTROL,
                    axum::http::HeaderValue::from_static("public, max-age=3600"),
                ))
                .service(ServeDir::new("metadata")),
        )
        .layer(
            AuthSessionLayer::<
                mk_lib_database::mk_lib_database_user::User,
                i64,
                SessionPgPool,
                PgPool,
            >::new(Some(sqlx_pool_rw.clone().into()))
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        // after authsessionlayer so anyone can access
        .route_with_tsr(
            "/api/titlesearch/{title}",
            get(api::bp_api_title_search::api_title_search),
        )
        .route_with_tsr("/public/about", get(public::bp_about::public_about))
        .route_with_tsr("/error/401", get(bp_error::general_not_authorized))
        .route_with_tsr("/error/403", get(bp_error::general_not_administrator))
        .route_with_tsr("/error/500", get(bp_error::general_error))
        .route_with_tsr(
            "/public/forgot_password",
            get(public::bp_forgot_password::public_forgot_password)
                .post(public::bp_forgot_password::public_forgot_password_post),
        )
        .route_with_tsr(
            "/public/register",
            get(public::bp_register::public_register)
                .post(public::bp_register::public_register_post),
        )
        .route_with_tsr(
            "/health_check",
            get(public::bp_health_check::public_health_check),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .layer(middleware::from_fn(security_headers))
        .layer(middleware::from_fn(rate_limit_auth))
        .layer(middleware::from_fn(require_same_origin))
        .with_state(app_state);
    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(bp_error::general_not_found);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
