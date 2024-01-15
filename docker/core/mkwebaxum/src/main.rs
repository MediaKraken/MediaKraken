#[macro_use]
extern crate lazy_static;

use axum::http::{Method, Uri};
use axum::{
    body::Body,
    extract::{Request, State},
    response::{IntoResponse, Response},
    //http::StatusCode,
    routing::{get, post},
    BoxError,
    Extension,
    Json,
    Router,
};
use axum_csrf::{CsrfConfig, CsrfToken};
use axum_extra::routing::RouterExt;
use axum_handle_error_extract::HandleErrorLayer;
use axum_prometheus::{EndpointLabel, PrometheusMetricLayerBuilder};
use axum_server::tls_rustls::RustlsConfig;
use axum_session::{
    Key, SessionConfig, SessionLayer, SessionPgPool, SessionPgSessionStore, SessionRedisPool,
    SessionStore,
};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use mk_lib_database;
use rcgen::generate_simple_self_signed;
// use reverse_proxy_service::ReusedServiceBuilder;
// use reverse_proxy_service::{AppendSuffix, Static, TrimPrefix};
use hyper::StatusCode;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use ring::digest;
use serde_json::json;
use sqlx::PgPool;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use std::{net::SocketAddr, path::PathBuf};
use tokio::signal;
use tower::timeout::TimeoutLayer;
use tower::ServiceExt;
use tower::{timeout::error::Elapsed, ServiceBuilder};
use tower_http::services::{ServeDir, ServeFile};

type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;
mod axum_custom_filters;
mod error_handling;
mod guard;

#[path = "admin"]
pub mod admin {
    pub mod bp_backup;
    pub mod bp_cron;
    pub mod bp_database;
    pub mod bp_docker;
    pub mod bp_game_servers;
    pub mod bp_hardware;
    pub mod bp_home;
    pub mod bp_library;
    pub mod bp_reports;
    pub mod bp_settings;
    pub mod bp_torrent;
    pub mod bp_user;
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
    pub mod bp_media_movie;
    pub mod bp_media_music;
    pub mod bp_media_music_video;
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

#[tokio::main]
async fn main() {
    // TODO this needs to move to another container that doesn't start multiples
    // check for and create ssl certs if needed
    if Path::new("/mediakraken/certs/cacert.pem").exists() == false {
        // generate certs/keys
        let subject_alt_names = vec!["www.mediakraken.org".to_string(), "localhost".to_string()];
        let cert = generate_simple_self_signed(subject_alt_names).unwrap();
        let mut file_pem = File::create("/mediakraken/certs/cacert.pem").unwrap();
        file_pem
            .write_all(cert.serialize_pem().unwrap().as_bytes())
            .unwrap();
        let mut file_key_pem = File::create("/mediakraken/certs/privkey.pem").unwrap();
        file_key_pem
            .write_all(cert.serialize_private_key_pem().as_bytes())
            .unwrap();
    }

    // create crypto salt if needed
    // TODO what was this for?
    // this was for the db user password salt?
    // but using gen_salt in postgresql to let it pick the salt
    // if Path::new("/secure/data.zip").exists() == false {
    //     // create the hash salt
    //     if Path::new("/secure/data.zip").exists() == false {
    //         let mut file_salt = File::create("/secure/data.zip").unwrap();
    //         const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    //         let salt = [0u8; CREDENTIAL_LEN];
    //         file_salt.write_all(&salt);
    //     }
    //     let salt = mk_lib_file::mk_read_file_data("/secure/data.zip");
    // }

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50)
        .await
        .unwrap();
    let _result =
        mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
            .await;

    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));

    // let client = redis::Client::open("redis://default:@mkstack_redis:6379/0")
    //     .expect("Error while trying to open the redis connection");
    // let session_config = SessionConfig::default();
    // let session_store =
    //     SessionStore::<SessionRedisPool>::new(Some(client.clone().into()), session_config).await
    //     .unwrap();

    let session_config = SessionConfig::default().with_table_name("mm_session");
    let session_store = SessionPgSessionStore::new(Some(sqlx_pool.clone().into()), session_config)
        .await
        .unwrap();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
        .with_endpoint_label_type(EndpointLabel::MatchedPathWithFallbackFn(|path| {
            format!("{}_changed", path)
        }))
        .with_default_metrics()
        .build_pair();

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(
        PathBuf::from("/mediakraken/certs/cacert.pem"),
        PathBuf::from("/mediakraken/certs/privkey.pem"),
    )
    .await
    .unwrap();

    // build our application with routes
    let docker_results = mk_lib_common::mk_lib_common_docker::mk_common_docker_info()
        .await
        .unwrap();
    // TODO let transmission_host: reverse_proxy_service::ReusedServiceBuilder =
    //     reverse_proxy_service::builder_http(format!("{}:9091", docker_results.name.unwrap()))
    //         .unwrap();
    let client: Client =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());
    // route_with_tsr creates two routes.....one with trailing slash
    let app = Router::new()
        .route_with_tsr("/admin", get(admin::bp_home::admin_home))
        .route_with_tsr("/admin/backup", get(admin::bp_backup::admin_backup))
        .route_with_tsr("/admin/cron", get(admin::bp_cron::admin_cron))
        .route_with_tsr("/admin/database", get(admin::bp_database::admin_database))
        .route_with_tsr("/admin/docker", get(admin::bp_docker::admin_docker))
        .route_with_tsr(
            "/admin/game_servers/:page",
            get(admin::bp_game_servers::admin_game_servers),
        )
        .route_with_tsr("/admin/hardware", get(admin::bp_hardware::admin_hardware))
        .route_with_tsr("/admin/home", get(admin::bp_home::admin_home))
        .route_with_tsr(
            "/admin/library/:page",
            get(admin::bp_library::admin_library),
        )
        .route_with_tsr("/admin/settings", get(admin::bp_settings::admin_settings))
        .route_with_tsr(
            "/admin/report_known_media/:page",
            get(admin::bp_reports::admin_report_known_media),
        )
        .route_with_tsr("/admin/torrent", get(admin::bp_torrent::admin_torrent))
        .route_with_tsr("/admin/torrent/web", get(proxy_transmission_handler))
        .with_state(client)
        .route_with_tsr("/admin/user/:page", get(admin::bp_user::admin_user))
        .route_with_tsr(
            "/user/internet/flickr",
            get(user_internet::bp_inter_flickr::user_inter_flickr),
        )
        .route_with_tsr(
            "/user/internet/flickr/:guid",
            get(user_internet::bp_inter_flickr::user_inter_flickr_detail),
        )
        .route_with_tsr(
            "/user/internet",
            get(user_internet::bp_inter_home::user_inter_home),
        )
        .route_with_tsr(
            "/user/internet/twitchtv",
            get(user_internet::bp_inter_twitchtv::user_inter_twitchtv),
        )
        .route_with_tsr(
            "/user/internet/vimeo",
            get(user_internet::bp_inter_vimeo::user_inter_vimeo),
        )
        .route_with_tsr(
            "/user/internet/youtube",
            get(user_internet::bp_inter_youtube::user_inter_youtube),
        )
        .route_with_tsr(
            "/user/media/book/:page",
            get(user_media::bp_media_book::user_media_book),
        )
        .route_with_tsr(
            "/user/media/book_detail/:guid",
            get(user_media::bp_media_book::user_media_book_detail),
        )
        .route_with_tsr(
            "/user/media/collection/:page",
            get(user_media::bp_media_collection::user_media_collection),
        )
        .route_with_tsr(
            "/user/media/collection_detail/:guid",
            get(user_media::bp_media_collection::user_media_collection_detail),
        )
        .route_with_tsr(
            "/user/media/game/:page",
            get(user_media::bp_media_game::user_media_game),
        )
        .route_with_tsr(
            "/user/media/game_detail/:guid",
            get(user_media::bp_media_game::user_media_game_detail),
        )
        .route_with_tsr(
            "/user/media/game_servers/:page",
            get(user_media::bp_media_game_servers::user_media_game_servers),
        )
        .route_with_tsr(
            "/user/media/genre",
            get(user_media::bp_media_genre::user_media_genre),
        )
        .route_with_tsr(
            "/user/media/home_media/:page",
            get(user_media::bp_media_home_media::user_media_home_media),
        )
        .route_with_tsr(
            "/user/media/image",
            get(user_media::bp_media_image::user_media_image),
        )
        .route_with_tsr(
            "/user/media/movie/:page",
            get(user_media::bp_media_movie::user_media_movie),
        )
        .route_with_tsr(
            "/user/media/movie_detail/:guid",
            get(user_media::bp_media_movie::user_media_movie_detail),
        )
        .route_with_tsr(
            "/user/media/music/:page",
            get(user_media::bp_media_music::user_media_music),
        )
        .route_with_tsr(
            "/user/media/music_detail/:guid",
            get(user_media::bp_media_music::user_media_music_detail),
        )
        .route_with_tsr(
            "/user/media/music_video/:page",
            get(user_media::bp_media_music_video::user_media_music_video),
        )
        .route_with_tsr(
            "/user/media/music_video_detail/:guid",
            get(user_media::bp_media_music_video::user_media_music_video_detail),
        )
        .route_with_tsr(
            "/user/media/sports/:page",
            get(user_media::bp_media_sports::user_media_sports),
        )
        .route_with_tsr(
            "/user/media/sports_detail/:guid",
            get(user_media::bp_media_sports::user_media_sports_detail),
        )
        .route_with_tsr(
            "/user/media/tv/:page",
            get(user_media::bp_media_tv::user_media_tv),
        )
        .route_with_tsr(
            "/user/media/tv_detail/:guid",
            get(user_media::bp_media_tv::user_media_tv_detail),
        )
        .route_with_tsr(
            "/user/metadata/book/:page",
            get(user_metadata::bp_meta_book::user_metadata_book),
        )
        .route_with_tsr(
            "/user/metadata/book_detail/:guid",
            get(user_metadata::bp_meta_book::user_metadata_book_detail),
        )
        .route_with_tsr(
            "/user/metadata/game/:page",
            get(user_metadata::bp_meta_game::user_metadata_game),
        )
        .route_with_tsr(
            "/user/metadata/game_detail/:guid",
            get(user_metadata::bp_meta_game::user_metadata_game_detail),
        )
        .route_with_tsr(
            "/user/metadata/game_system/:page",
            get(user_metadata::bp_meta_game_system::user_metadata_game_system),
        )
        .route_with_tsr(
            "/user/metadata/game_system_detail/:guid",
            get(user_metadata::bp_meta_game_system::user_metadata_game_system_detail),
        )
        .route_with_tsr(
            "/user/metadata/movie/:page",
            get(user_metadata::bp_meta_movie::user_metadata_movie),
        )
        .route_with_tsr(
            "/user/metadata/movie_detail/:guid",
            get(user_metadata::bp_meta_movie::user_metadata_movie_detail),
        )
        .route_with_tsr(
            "/user/metadata/music/:page",
            get(user_metadata::bp_meta_music::user_metadata_music),
        )
        .route_with_tsr(
            "/user/metadata/music_detail/:guid",
            get(user_metadata::bp_meta_music::user_metadata_music_detail),
        )
        .route_with_tsr(
            "/user/metadata/music_video/:page",
            get(user_metadata::bp_meta_music_video::user_metadata_music_video),
        )
        .route_with_tsr(
            "/user/metadata/music_video_detail/:guid",
            get(user_metadata::bp_meta_music_video::user_metadata_music_video_detail),
        )
        .route_with_tsr(
            "/user/metadata/person/:page",
            get(user_metadata::bp_meta_person::user_metadata_person),
        )
        .route_with_tsr(
            "/user/metadata/person_detail/:guid",
            get(user_metadata::bp_meta_person::user_metadata_person_detail),
        )
        .route_with_tsr(
            "/user/metadata/sports/:page",
            get(user_metadata::bp_meta_sports::user_metadata_sports),
        )
        .route_with_tsr(
            "/user/metadata/sports_detail/:guid",
            get(user_metadata::bp_meta_sports::user_metadata_sports_detail),
        )
        .route_with_tsr(
            "/user/metadata/tv/:page",
            get(user_metadata::bp_meta_tv::user_metadata_tv),
        )
        .route_with_tsr(
            "/user/metadata/tv_detail/:guid",
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
        .route_with_tsr("/user/queue", get(user::bp_queue::user_queue))
        .route_with_tsr("/user/search", get(user::bp_search::user_search))
        .route_with_tsr("/user/sync", get(user::bp_sync::user_sync))
        .route_with_tsr("/logout", get(public::bp_logout::public_logout))
        .route_with_tsr(
            "/public/login",
            get(public::bp_login::public_login).post(public::bp_login::public_login_post),
        )
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            AuthSessionLayer::<
                mk_lib_database::mk_lib_database_user::User,
                i64,
                SessionPgPool,
                PgPool,
            >::new(Some(sqlx_pool.clone().into()))
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        // after authsessionlayer so anyone can access
        .route_with_tsr("/public/about", get(public::bp_about::public_about))
        .route_with_tsr("/error/401", get(bp_error::general_not_authorized))
        .route_with_tsr("/error/403", get(bp_error::general_not_administrator))
        .route_with_tsr("/error/500", get(bp_error::general_error))
        .route_with_tsr(
            "/public/forgot_password",
            get(public::bp_forgot_password::public_forgot_password),
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
        .layer(Extension(sqlx_pool));
    // TODO .layer(
    //     ServiceBuilder::new()
    //         .layer(HandleErrorLayer::new(|_: BoxError| async {
    //             StatusCode::REQUEST_TIMEOUT
    //         }))
    //         .layer(TimeoutLayer::new(Duration::from_secs(10))),
    // );
    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(bp_error::general_not_found);

    // run our app with hyper
    axum_server::bind_rustls("0.0.0.0:8080".parse().unwrap(), config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    //             bp_error::general_not_authorized,        401
    //             bp_error::general_not_administrator,     403
    //             bp_error::general_security,              401?
    //             bp_error::default_catcher,               500
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

async fn proxy_transmission_handler(
    State(client): State<Client>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    let uri = format!("https://mkstack_transmission:9091{}", path_query);
    *req.uri_mut() = Uri::try_from(uri).unwrap();
    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
