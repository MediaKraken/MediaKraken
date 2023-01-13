#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use rcgen::generate_simple_self_signed;
use ring::digest;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::{Build, Request, Rocket};
use rocket_auth::{prelude::Error, *};
use rocket_dyn_templates::Template;
use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_file.rs"]
mod mk_lib_file;
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

#[rocket::main]
async fn main() -> Result<(), Error> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({"START": "START"})).await.unwrap();
    }

    // check for and create ssl certs if needed
    if Path::new("/mediakraken/key/cacert.pem").exists() == false {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({"stuff": "Cert not found, generating."}),
            )
            .await.unwrap();
        }
        // generate certs/keys
        let subject_alt_names = vec!["www.mediakraken.org".to_string(), "localhost".to_string()];
        let cert = generate_simple_self_signed(subject_alt_names).unwrap();
        let mut file_pem = File::create("/mediakraken/key/cacert.pem").unwrap();
        file_pem
            .write_all(cert.serialize_pem().unwrap().as_bytes())
            .unwrap();
        let mut file_key_pem = File::create("/mediakraken/key/privkey.pem").unwrap();
        file_key_pem
            .write_all(cert.serialize_private_key_pem().as_bytes())
            .unwrap();
    }

    // create crypto salt if needed
    if Path::new("/mediakraken/secure/data.zip").exists() == false {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({"stuff": "data.zip not found, generating."}),
            )
            .await.unwrap();
        }
        // create the hash salt
        if Path::new("/mediakraken/secure/data.zip").exists() == false {
            let mut file_salt = File::create("/mediakraken/secure/data.zip").unwrap();
            const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
            let salt = [0u8; CREDENTIAL_LEN];
            file_salt.write_all(&salt);
        }
        let salt = mk_lib_file::mk_read_file_data("/mediakraken/secure/data.zip");
    }

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, true).await;

    // setup auth
    let users: Users = sqlx_pool.clone().into();
    users.create_table().await?;

    // setup rocket
    rocket::build()
        .mount("/static", FileServer::from(relative!("static")))
        .mount(
            "/admin",
            routes![
                bp_admin_backup::admin_backup,
                bp_admin_cron::admin_cron,
                bp_admin_database::admin_database,
                bp_admin_docker::admin_docker,
                bp_admin_game_servers::admin_game_servers,
                bp_admin_hardware::admin_hardware,
                bp_admin_home::admin_home,
                bp_admin_library::admin_library,
                bp_admin_settings::admin_settings,
                bp_admin_torrent::admin_torrent,
                bp_admin_user::admin_user,
            ],
        )
        .mount(
            "/public",
            routes![
                bp_public_about::public_about,
                bp_public_forgot_password::public_forgot_password,
                bp_public_login::public_login,
                bp_public_login::public_login_post,
                bp_public_register::public_register,
                bp_public_register::public_register_post,
            ],
        )
        .mount(
            "/user",
            routes![
                bp_user_internet_bp_inter_flickr::user_inter_flickr,
                bp_user_internet_bp_inter_home::user_inter_home,
                bp_user_internet_bp_inter_twitchtv::user_inter_twitchtv,
                bp_user_internet_bp_inter_vimeo::user_inter_vimeo,
                bp_user_internet_bp_inter_youtube::user_inter_youtube,
                bp_user_media_bp_media_book::user_media_book,
                bp_user_media_bp_media_book::user_media_book_detail,
                bp_user_media_bp_media_collection::user_media_collection,
                bp_user_media_bp_media_collection::user_media_collection_detail,
                bp_user_media_bp_media_game::user_media_game,
                bp_user_media_bp_media_game::user_media_game_detail,
                bp_user_media_bp_media_game_servers::user_media_game_servers,
                bp_user_media_bp_media_genre::user_media_genre,
                bp_user_media_bp_media_home_media::user_media_home_media,
                bp_user_media_bp_media_image::user_media_image,
                bp_user_media_bp_media_movie::user_media_movie,
                bp_user_media_bp_media_movie::user_media_movie_detail,
                bp_user_media_bp_media_music::user_media_music,
                bp_user_media_bp_media_music::user_media_music_detail,
                bp_user_media_bp_media_music_video::user_media_music_video,
                bp_user_media_bp_media_music_video::user_media_music_video_detail,
                bp_user_media_bp_media_sports::user_media_sports,
                bp_user_media_bp_media_sports::user_media_sports_detail,
                bp_user_media_bp_media_tv::user_media_tv,
                bp_user_media_bp_media_tv::user_media_tv_detail,
                bp_user_metadata_bp_meta_book::user_metadata_book,
                bp_user_metadata_bp_meta_book::user_metadata_book_detail,
                bp_user_metadata_bp_meta_game::user_metadata_game,
                bp_user_metadata_bp_meta_game::user_metadata_game_detail,
                bp_user_metadata_bp_meta_game_system::user_metadata_game_system,
                bp_user_metadata_bp_meta_game_system::user_metadata_game_system_detail,
                bp_user_metadata_bp_meta_movie::user_metadata_movie,
                bp_user_metadata_bp_meta_movie::user_metadata_movie_detail,
                bp_user_metadata_bp_meta_music::user_metadata_music,
                bp_user_metadata_bp_meta_music::user_metadata_music_detail,
                bp_user_metadata_bp_meta_music_video::user_metadata_music_video,
                bp_user_metadata_bp_meta_music_video::user_metadata_music_video_detail,
                bp_user_metadata_bp_meta_person::user_metadata_person,
                bp_user_metadata_bp_meta_person::user_metadata_person_detail,
                bp_user_metadata_bp_meta_sports::user_metadata_sports,
                bp_user_metadata_bp_meta_sports::user_metadata_sports_detail,
                bp_user_metadata_bp_meta_tv::user_metadata_tv,
                bp_user_metadata_bp_meta_tv::user_metadata_tv_detail,
                bp_user_playback_bp_audio::user_playback_audio,
                bp_user_playback_bp_comic::user_playback_comic,
                bp_user_playback_bp_video::user_playback_video,
                bp_user_hardware::user_hardware,
                bp_user_home::user_home,
                bp_user_profile::user_profile,
                bp_user_queue::user_queue,
                bp_user_search::user_search,
                bp_user_sync::user_sync,
            ],
        )
        .register(
            "/",
            catchers![
                bp_error::general_not_authorized,
                bp_error::general_not_administrator,
                bp_error::general_not_found,
                bp_error::general_security,
                bp_error::default_catcher,
            ],
        )
        .manage::<sqlx::PgPool>(sqlx_pool)
        .manage(users)
        //.attach(rocket_csrf::Fairing::default())
        .attach(Template::fairing())
        .launch()
        .await;
    Ok(())
}
