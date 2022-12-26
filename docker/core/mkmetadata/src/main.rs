#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Uuid;
use sqlx::Row;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use tokio::time::{sleep, Duration};

#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_media.rs"]
mod mk_lib_database_media;
#[path = "mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;
#[path = "mk_lib_database_option_status.rs"]
mod mk_lib_database_option_status;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_network.rs"]
mod mk_lib_network;

#[path = "identification.rs"]
mod metadata_identification;

#[path = "metadata/base.rs"]
mod metadata_base;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkmetadata";
    mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}), LOGGING_INDEX_NAME)
        .await;

    // open the database
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false).await;

    // pull options/api keys and set structs to contain the data
    let option_json: serde_json::Value =
        mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();

    // launch thread per provider
    let tmdb_api_key = option_json["API"]["themoviedb"].to_string();
    let handle_tmdb = tokio::spawn(async move {
        loop {
            println!("Before themoviedb read");
            let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
            let metadata_to_process = mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool, "themoviedb").await.unwrap();
            for download_data in metadata_to_process {
                metadata_base::metadata_process(
                    &sqlx_pool,
                    "themoviedb".to_string(),
                    download_data,
                    &tmdb_api_key,
                )
                .await
                .unwrap();
            }
            sleep(Duration::from_secs(1)).await;
        }
    });
    if !option_json["API"]["musicbrainz"].is_null() {
        let musicbrainz_api_key = option_json["API"]["musicbrainz"].to_string();
        let handle_musicbrainz = tokio::spawn(async move {
            loop {
                println!("Before musicbrainz read");
                let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
                let metadata_to_process = mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool, "musicbrainz").await.unwrap();
                for download_data in metadata_to_process {
                    metadata_base::metadata_process(
                        &sqlx_pool,
                        "musicbrainz".to_string(),
                        download_data,
                        &musicbrainz_api_key,
                    )
                    .await
                    .unwrap();
                }
                sleep(Duration::from_secs(1)).await;
            }
        });
    };
    let thesportsdb_api_key = option_json["API"]["thesportsdb"].to_string();
    let handle_thesportsdb = tokio::spawn(async move {
        loop {
            println!("Before thesportsdb read");
            let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
            let metadata_to_process = mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool, "thesportsdb").await.unwrap();
            for download_data in metadata_to_process {
                metadata_base::metadata_process(
                    &sqlx_pool,
                    "thesportsdb".to_string(),
                    download_data,
                    &thesportsdb_api_key,
                )
                .await
                .unwrap();
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    // process all the "Z" records
    loop {
        println!("Before Z read");
        // grab new batch of records to process by content provider
        let metadata_to_process =
            mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(
                &sqlx_pool, "Z",
            )
            .await
            .unwrap();
        for download_data in metadata_to_process {
            // begin id process
            let mut metadata_uuid: uuid::Uuid = uuid::Uuid::nil();
            metadata_uuid = metadata_identification::metadata_identification(
                &sqlx_pool,
                &download_data,
            )
            .await
            .unwrap();
            // // guessit processing which includes identification
            // let metadata_uuid: uuid::Uuid =
            //     metadata_guessit::metadata_guessit(&sqlx_pool, download_data)
            //         .await
            //         .unwrap();
            // update the media row with the json media id and the proper name
            if metadata_uuid != uuid::Uuid::nil() {
                mk_lib_database_media::mk_lib_database_media_update_metadata_guid(
                    &sqlx_pool,
                    &download_data.mm_download_provider_id,
                    metadata_uuid,
                    &download_data.mm_download_guid,
                )
                .await
                .unwrap();
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
    // TODO unreachable....so, do I care
    //handle_tmdb.join().unwrap();
    //handle_tmdb.take().map(JoinHandle::join);
    //handle_musicbrainz.join().unwrap();
    //handle_musicbrainz.take().map(JoinHandle::join);
    //handle_thesportsdb.join().unwrap();
    //handle_thesportsdb.take().map(JoinHandle::join);
}
