#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use]
extern crate lazy_static;

use axum::ServiceExt;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use rcgen::generate_simple_self_signed;
use ring::digest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::path::Path;
use stdext::function_name;
use tower::layer::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
// #[path = "mk_lib_file.rs"]
// mod mk_lib_file;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "admin/bp_backup.rs"]
mod bp_admin_backup;
#[path = "admin/bp_cron.rs"]
mod bp_admin_cron;
#[path = "admin/bp_database.rs"]
mod bp_admin_database;
#[path = "admin/bp_docker.rs"]
mod bp_admin_docker;
#[path = "admin/bp_game_servers.rs"]
mod bp_admin_game_servers;
#[path = "admin/bp_hardware.rs"]
mod bp_admin_hardware;
#[path = "admin/bp_home.rs"]
mod bp_admin_home;
#[path = "admin/bp_library.rs"]
mod bp_admin_library;
#[path = "admin/bp_settings.rs"]
mod bp_admin_settings;
#[path = "admin/bp_share.rs"]
mod bp_admin_share;
#[path = "admin/bp_torrent.rs"]
mod bp_admin_torrent;
#[path = "admin/bp_user.rs"]
mod bp_admin_user;

#[path = "error/bp_error.rs"]
mod bp_error;

#[path = "public/bp_about.rs"]
mod bp_public_about;
#[path = "public/bp_forgot_password.rs"]
mod bp_public_forgot_password;
#[path = "public/bp_login.rs"]
mod bp_public_login;
#[path = "public/bp_register.rs"]
mod bp_public_register;

#[path = "user/internet/bp_inter_flickr.rs"]
mod bp_user_internet_bp_inter_flickr;
#[path = "user/internet/bp_inter_home.rs"]
mod bp_user_internet_bp_inter_home;
#[path = "user/internet/bp_inter_twitchtv.rs"]
mod bp_user_internet_bp_inter_twitchtv;
#[path = "user/internet/bp_inter_vimeo.rs"]
mod bp_user_internet_bp_inter_vimeo;
#[path = "user/internet/bp_inter_youtube.rs"]
mod bp_user_internet_bp_inter_youtube;

#[path = "user/media/bp_media_book.rs"]
mod bp_user_media_bp_media_book;
#[path = "user/media/bp_media_collection.rs"]
mod bp_user_media_bp_media_collection;
#[path = "user/media/bp_media_game.rs"]
mod bp_user_media_bp_media_game;
#[path = "user/media/bp_media_game_servers.rs"]
mod bp_user_media_bp_media_game_servers;
#[path = "user/media/bp_media_genre.rs"]
mod bp_user_media_bp_media_genre;
#[path = "user/media/bp_media_home_media.rs"]
mod bp_user_media_bp_media_home_media;
#[path = "user/media/bp_media_image.rs"]
mod bp_user_media_bp_media_image;
#[path = "user/media/bp_media_movie.rs"]
mod bp_user_media_bp_media_movie;
#[path = "user/media/bp_media_music.rs"]
mod bp_user_media_bp_media_music;
#[path = "user/media/bp_media_music_video.rs"]
mod bp_user_media_bp_media_music_video;
#[path = "user/media/bp_media_sports.rs"]
mod bp_user_media_bp_media_sports;
#[path = "user/media/bp_media_tv.rs"]
mod bp_user_media_bp_media_tv;

#[path = "user/metadata/bp_meta_book.rs"]
mod bp_user_metadata_bp_meta_book;
#[path = "user/metadata/bp_meta_game.rs"]
mod bp_user_metadata_bp_meta_game;
#[path = "user/metadata/bp_meta_game_system.rs"]
mod bp_user_metadata_bp_meta_game_system;
#[path = "user/metadata/bp_meta_movie.rs"]
mod bp_user_metadata_bp_meta_movie;
#[path = "user/metadata/bp_meta_music.rs"]
mod bp_user_metadata_bp_meta_music;
#[path = "user/metadata/bp_meta_music_video.rs"]
mod bp_user_metadata_bp_meta_music_video;
#[path = "user/metadata/bp_meta_person.rs"]
mod bp_user_metadata_bp_meta_person;
#[path = "user/metadata/bp_meta_sports.rs"]
mod bp_user_metadata_bp_meta_sports;
#[path = "user/metadata/bp_meta_tv.rs"]
mod bp_user_metadata_bp_meta_tv;

#[path = "user/playback/bp_audio.rs"]
mod bp_user_playback_bp_audio;
#[path = "user/playback/bp_comic.rs"]
mod bp_user_playback_bp_comic;
#[path = "user/playback/bp_video.rs"]
mod bp_user_playback_bp_video;

#[path = "user/bp_hardware.rs"]
mod bp_user_hardware;
#[path = "user/bp_home.rs"]
mod bp_user_home;
#[path = "user/bp_profile.rs"]
mod bp_user_profile;
#[path = "user/bp_queue.rs"]
mod bp_user_queue;
#[path = "user/bp_search.rs"]
mod bp_user_search;
#[path = "user/bp_sync.rs"]
mod bp_user_sync;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({"START": "START"}))
            .await
            .unwrap();
    }

    // check for and create ssl certs if needed
    if Path::new("/mediakraken/certs/cacert.pem").exists() == false {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({"stuff": "Cert not found, generating."}),
            )
            .await
            .unwrap();
        }
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
    // if Path::new("/secure/data.zip").exists() == false {
    //     #[cfg(debug_assertions)]
    //     {
    //         mk_lib_logging::mk_logging_post_elk(
    //             std::module_path!(),
    //             json!({"stuff": "data.zip not found, generating."}),
    //         )
    //         .await.unwrap();
    //     }
    //     // create the hash salt
    //     if Path::new("/secure/data.zip").exists() == false {
    //         let mut file_salt = File::create("/secure/data.zip").unwrap();
    //         const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    //         let salt = [0u8; CREDENTIAL_LEN];
    //         file_salt.write_all(&salt);
    //     }
    //     let salt = mk_lib_file::mk_read_file_data("/secure/data.zip");
    // }

    // create metadata paths, as before the db update will let it finish before
    // other containers can use them
    // if !Path::new(&"/mediakraken/static/meta").exists() {
    //     fs::create_dir("/mediakraken/static/meta")?;
    //     let vec_of_metadata = vec!["poster", "backdrop", "trailer"];
    //     for metadata_type in vec_of_metadata.iter() {
    //         let file_name = format!("/mediakraken/static/meta/{}/", metadata_type);
    //         fs::create_dir(&file_name)?;
    //         for c in b'a'..=b'z' {
    //             for d in b'a'..=b'z' {
    //                 for e in b'a'..=b'z' {
    //                     for f in b'a'..=b'z' {
    //                         fs::create_dir_all(format!(
    //                             "{}{}{}/{}{}",
    //                             file_name, c as char, d as char, e as char, f as char
    //                         ))?;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool(50)
        .await
        .unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, true).await;

    // build our application with a routes
    let app = NormalizePathLayer::trim_trailing_slash().layer(
        Router::new()
            .route("/about", get(bp_public_about::public_about))
            .route("/admin/backup", get(bp_admin_backup::admin_backup))
            .route("/admin/cron", get(bp_admin_cron::admin_cron))
            .route("/admin/database", get(bp_admin_database::admin_database))
            .route("/admin/docker", get(bp_admin_docker::admin_docker))
            .route(
                "/admin//game_servers/:page",
                get(bp_admin_game_servers::admin_game_servers),
            )
            .route("/admin/hardware", get(bp_admin_hardware::admin_hardware))
            .route("/admin/home", get(bp_admin_home::admin_home))
            .route(
                "/admin/library/:page",
                get(bp_admin_library::admin_library),
            )
            .route("/admin/share", get(bp_admin_share::admin_share))
            .route("/admin/settings", get(bp_admin_settings::admin_settings))
            .route("/admin/torrent", get(bp_admin_torrent::admin_torrent))
            .route("/admin/user/:page", get(bp_admin_user::admin_user))
            .route(
                "/public/forgot_password",
                get(bp_public_forgot_password::public_forgot_password),
            )
            .route("/public/login", get(bp_public_login::public_login))
            .route("/public/login", post(bp_public_login::public_login_post))
            .route("/public/register", get(bp_public_register::public_register))
            .route(
                "/public/register",
                post(bp_public_register::public_register_post),
            )
            .route(
                "/user/internet/flickr",
                get(bp_user_internet_bp_inter_flickr::user_inter_flickr),
            )
            .route(
                "/user/internet/flickr/:guid",
                get(bp_user_internet_bp_inter_flickr::user_inter_flickr_detail),
            )
            .route(
                "/user/internet",
                get(bp_user_internet_bp_inter_home::user_inter_home),
            )
            .route(
                "/user/internet/twitchtv",
                get(bp_user_internet_bp_inter_twitchtv::user_inter_twitchtv),
            )
            .route(
                "/user/internet/vimeo",
                get(bp_user_internet_bp_inter_vimeo::user_inter_vimeo),
            )
            .route(
                "/user/internet/youtube",
                get(bp_user_internet_bp_inter_youtube::user_inter_youtube),
            )
            .route(
                "/user/media/book/:page",
                get(bp_user_media_bp_media_book::user_media_book),
            )
            .route(
                "/user/media/book_detail/:guid",
                get(bp_user_media_bp_media_book::user_media_book_detail),
            )
            .route(
                "/user/media/collection/:page",
                get(bp_user_media_bp_media_collection::user_media_collection),
            )
            .route(
                "/user/media/collection_detail/:guid",
                get(bp_user_media_bp_media_collection::user_media_collection_detail),
            )
            .route(
                "/user/media/game/:page",
                get(bp_user_media_bp_media_game::user_media_game),
            )
            .route(
                "/user/media/game_detail/:guid",
                get(bp_user_media_bp_media_game::user_media_game_detail),
            )
            .route(
                "/user/media/game_servers/:page",
                get(bp_user_media_bp_media_game_servers::user_media_game_servers),
            )
            .route(
                "/user/media/genre",
                get(bp_user_media_bp_media_genre::user_media_genre),
            )
            .route(
                "/user/media/home_media/:page",
                get(bp_user_media_bp_media_home_media::user_media_home_media),
            )
            .route(
                "/user/media/image",
                get(bp_user_media_bp_media_image::user_media_image),
            )
            .route(
                "/user/media/movie/:page",
                get(bp_user_media_bp_media_movie::user_media_movie),
            )
            .route(
                "/user/media/movie_detail/:guid",
                get(bp_user_media_bp_media_movie::user_media_movie_detail),
            )
            .route(
                "/user/media/music/:page",
                get(bp_user_media_bp_media_music::user_media_music),
            )
            .route(
                "/user/media/music_detail/:guid",
                get(bp_user_media_bp_media_music::user_media_music_detail),
            )
            .route(
                "/user/media/music_video/:page",
                get(bp_user_media_bp_media_music_video::user_media_music_video),
            )
            .route(
                "/user/media/music_video_detail/:guid",
                get(bp_user_media_bp_media_music_video::user_media_music_video_detail),
            )
            .route(
                "/user/media/sports/:page",
                get(bp_user_media_bp_media_sports::user_media_sports),
            )
            .route(
                "/user/media/sports_detail/:guid",
                get(bp_user_media_bp_media_sports::user_media_sports_detail),
            )
            .route(
                "/user/media/tv/:page",
                get(bp_user_media_bp_media_tv::user_media_tv),
            )
            .route(
                "/user/media/tv_detail/:guid",
                get(bp_user_media_bp_media_tv::user_media_tv_detail),
            )
            .route(
                "/user/metadata/book/:page",
                get(bp_user_metadata_bp_meta_book::user_metadata_book),
            )
            .route(
                "/user/metadata/book_detail/:guid",
                get(bp_user_metadata_bp_meta_book::user_metadata_book_detail),
            )
            .route(
                "/user/metadata/game/:page",
                get(bp_user_metadata_bp_meta_game::user_metadata_game),
            )
            .route(
                "/user/metadata/game_detail/:guid",
                get(bp_user_metadata_bp_meta_game::user_metadata_game_detail),
            )
            .route(
                "/user/metadata/game_system/:page",
                get(bp_user_metadata_bp_meta_game_system::user_metadata_game_system),
            )
            .route(
                "/user/metadata/game_system_detail/:guid",
                get(bp_user_metadata_bp_meta_game_system::user_metadata_game_system_detail),
            )
            .route(
                "/user/metadata/movie/:page",
                get(bp_user_metadata_bp_meta_movie::user_metadata_movie),
            )
            .route(
                "/user/metadata/movie_detail/:guid",
                get(bp_user_metadata_bp_meta_movie::user_metadata_movie_detail),
            )
            .route(
                "/user/metadata/music/:page",
                get(bp_user_metadata_bp_meta_music::user_metadata_music),
            )
            .route(
                "/user/metadata/music_detail/:guid",
                get(bp_user_metadata_bp_meta_music::user_metadata_music_detail),
            )
            .route(
                "/user/metadata/music_video/:page",
                get(bp_user_metadata_bp_meta_music_video::user_metadata_music_video),
            )
            .route(
                "/user/metadata/music_video_detail/:guid",
                get(bp_user_metadata_bp_meta_music_video::user_metadata_music_video_detail),
            )
            .route(
                "/user/metadata/person/:page",
                get(bp_user_metadata_bp_meta_person::user_metadata_person),
            )
            .route(
                "/user/metadata/person_detail/:guid",
                get(bp_user_metadata_bp_meta_person::user_metadata_person_detail),
            )
            .route(
                "/user/metadata/sports/:page",
                get(bp_user_metadata_bp_meta_sports::user_metadata_sports),
            )
            .route(
                "/user/metadata/sports_detail/:guid",
                get(bp_user_metadata_bp_meta_sports::user_metadata_sports_detail),
            )
            .route(
                "/user/metadata/tv/:page",
                get(bp_user_metadata_bp_meta_tv::user_metadata_tv),
            )
            .route(
                "/user/metadata/tv_detail/:guid",
                get(bp_user_metadata_bp_meta_tv::user_metadata_tv_detail),
            )
            .route(
                "/user/playback/audio",
                get(bp_user_playback_bp_audio::user_playback_audio),
            )
            .route(
                "/user/playback/comic",
                get(bp_user_playback_bp_comic::user_playback_comic),
            )
            .route(
                "/user/playback/video",
                get(bp_user_playback_bp_video::user_playback_video),
            )
            .route("/user/hardware", get(bp_user_hardware::user_hardware))
            .route("/user/home", get(bp_user_home::user_home))
            .route("/user/profile", get(bp_user_profile::user_profile))
            .route("/user/queue", get(bp_user_queue::user_queue))
            .route("/user/search", get(bp_user_search::user_search))
            .route("/user/sync", get(bp_user_sync::user_sync))
            .nest("/static", axum_static::static_router("static"))
            .layer(Extension(sqlx_pool)),
    );

    // run our app with hyper
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    // setup rocket
    //     .register(
    //         "/",
    //         catchers![
    //             bp_error::general_not_authorized,
    //             bp_error::general_not_administrator,
    //             bp_error::general_not_found,
    //             bp_error::general_security,
    //             bp_error::default_catcher,
    //         ],
    //     )
    //     .manage(users)
    //     .attach(Template::fairing())
    //    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
