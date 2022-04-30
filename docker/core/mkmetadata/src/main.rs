#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::path::Path;
use tokio::time::{Duration, sleep};
use sqlx::types::Uuid;
use sqlx::Row;
use torrent_name_parser::Metadata;

#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_media.rs"]
mod mk_lib_database_media;
#[path = "mk_lib_database_metadata_movie.rs"]
mod mk_lib_database_metadata_movie;
#[path = "mk_lib_database_metadata_tv.rs"]
mod mk_lib_database_metadata_tv;
#[path = "mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_database_option_status.rs"]
mod mk_lib_database_option_status;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_network.rs"]
mod mk_lib_network;

#[path = "identification.rs"]
mod metadata_identification;

#[path = "metadata/provider/tmdb.rs"]
mod metadata_provider_tmdb;

#[path = "metadata/base.rs"]
mod metadata_base;

// #[derive(Serialize, Deserialize)]
// struct MediaTitleYear {
//     title: String,
//     year: Option<i8>,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkmetadata";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // open the database
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           false).await;

    // pull options/api keys and set structs to contain the data
    let option_json: serde_json::Value = mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool).await.unwrap();

    // launch thread per provider
    let handle_tmdb = tokio::spawn(async move {
        metadata_provider_tmdb::TMDBAPI.tmdb_api_key = option_json["API"]["themoviedb"];
        loop {
            let metadata_to_process = mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool, "themoviedb").await.unwrap();
            for row_data in metadata_to_process {
                metadata_base::metadata_process(&sqlx_pool, "themoviedb".to_string(), row_data)
            };
        }
    });

    // setup last used id's per thread
    let mut metadata_last_uuid: Uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000")?;
    let mut metadata_last_title: String = "".to_string();
    let mut metadata_last_year: i32 = 0;

    // process all the "Z" records
    loop {
        // grab new batch of records to process by content provider
        let metadata_to_process = mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool, "Z").await.unwrap();
        for row_data in metadata_to_process {
            let mut metadata_uuid: Uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000")?;
            // check for dupes by name/year
            let row_data_path: String = row_data.get("mm_download_path");
            println!("Path: {:?}", row_data_path);
            let file_name = Path::new(&row_data_path).file_name().unwrap().to_os_string().into_string().unwrap();
            println!("File: {:?}", file_name);

            let guessit_data: Metadata = Metadata::from(&file_name).unwrap();
            if guessit_data.title().len() > 0 {
                if guessit_data.year().is_some() {
                    if guessit_data.title().to_lowercase() == metadata_last_title
                        && guessit_data.year().unwrap() == metadata_last_year {
                        // matches last media scanned, so set with that metadata id
                        metadata_uuid = metadata_last_uuid
                    }
                } else if guessit_data.title().to_lowercase() == metadata_last_title {
                    // matches last media scanned, so set with that metadata id
                    metadata_uuid = metadata_last_uuid
                }
                if metadata_uuid == Uuid::parse_str("00000000-0000-0000-0000-000000000000")? {
                    // begin id process
                    metadata_uuid = metadata_identification::metadata_identification(&sqlx_pool,
                                                                                       row_data,
                                                                                       guessit_data);
                }
                // allow none to be set so unmatched stuff can work for skipping
                metadata_last_uuid = metadata_uuid;
                metadata_last_title = guessit_data.title().to_lowercase();
                if guessit_data.year().is_some() {
                    metadata_last_year = guessit_data.year().unwrap();
                } else {
                    metadata_last_year = 0;
                }
            } else {
                mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_update_provider(&sqlx_pool,
                                                                                                                 "ZZ".to_string(),
                                                                                                                 row_data.get("mdq_id")).await.unwrap();
            }
            // update the media row with the json media id and the proper name
            if metadata_uuid != Uuid::parse_str("00000000-0000-0000-0000-000000000000")? {
                mk_lib_database_media::mk_lib_database_media_update_metadata_guid(&sqlx_pool,
                                                                                  row_data["mdq_provider_id"],
                                                                                  metadata_uuid,
                                                                                  row_data["mdq_id"]);
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
    handle_tmdb.join().unwrap();
}