use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use serde_json::json;
use sqlx::Row;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_common/src/mk_lib_common.rs"]
mod mk_lib_common;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_compression/src/mk_lib_compression.rs"]
mod mk_lib_compression;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database/src/mk_lib_database.rs"]
mod mk_lib_database;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database/src/mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database/src/mk_lib_database_metadata.rs"]
mod mk_lib_database_metadata;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database/src/mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_logging/src/mk_lib_logging.rs"]
mod mk_lib_logging;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_network/src/mk_lib_network.rs"]
mod mk_lib_network;

#[cfg(not(debug_assertions))]
#[path = "mk_lib_common.rs"]
mod mk_lib_common;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_compression.rs"]
mod mk_lib_compression;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_database_metadata.rs"]
mod mk_lib_database_metadata;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_network.rs"]
mod mk_lib_network;

#[derive(Serialize, Deserialize)]
struct MetadataMovie {
    adult: bool,
    id: i32,
    original_title: String,
    popularity: f32,
    video: bool,
}

#[derive(Serialize, Deserialize)]
struct MetadataTV {
    id: i32,
    original_name: String,
    popularity: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mktmdbnetfetchbulk";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    let fetch_date: String = "08_10_2021".to_string();

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           false).await;

    // grab the movie id's
    // files.tmdb.org = 13.227.42.62
    let _fetch_result_movie = mk_lib_network::mk_download_file_from_url(
        format!("http://files.tmdb.org/p/exports/movie_ids_{}.json.gz", fetch_date),
        "/mediakraken/movie.gz".to_string()).await;
    let json_result = mk_lib_compression::mk_decompress_gz_data("/mediakraken/movie.gz").unwrap();
    // Please note that the data is NOT in id order
    for json_item in json_result.split('\n') {
        if !json_item.trim().is_empty() {
            let metadata_struct: MetadataMovie = serde_json::from_str(json_item)?;
            let result = mk_lib_database_metadata::mk_lib_database_metadata_exists_movie(&sqlx_pool,
                                                                                         metadata_struct.id).await.unwrap();
            if result == false {
                let download_result = mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
                                                                                                                              "themoviedb".to_string(),
                                                                                                                              mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                              metadata_struct.id).await.unwrap();
                if download_result == false {
                    mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                            "themoviedb".to_string(),
                                                                                                            mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                            Uuid::new_v4(),
                                                                                                            metadata_struct.id,
                                                                                                            "Fetch".to_string()).await;
                }
            }
        }
    }

    // grab the TV id's
    let _fetch_result_tv = mk_lib_network::mk_download_file_from_url(
        format!("http://files.tmdb.org/p/exports/tv_series_ids_{}.json.gz", fetch_date),
        "/mediakraken/tv.gz".to_string()).await;
    let json_result = mk_lib_compression::mk_decompress_gz_data("/mediakraken/tv.gz").unwrap();
    for json_item in json_result.split('\n') {
        if !json_item.trim().is_empty() {
            let metadata_struct: MetadataTV = serde_json::from_str(json_item)?;
            let result = mk_lib_database_metadata::mk_lib_database_metadata_exists_tv(&sqlx_pool,
                                                                                      metadata_struct.id).await.unwrap();
            if result == false {
                let download_result = mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
                                                                                                                              "themoviedb".to_string(),
                                                                                                                              mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                              metadata_struct.id).await.unwrap();
                if download_result == false {
                    mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                            "themoviedb".to_string(),
                                                                                                            mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                            Uuid::new_v4(),
                                                                                                            metadata_struct.id,
                                                                                                            "Fetch".to_string()).await;
                }
            }
        }
    }

    // stop logging
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"STOP": "STOP"}),
                                        LOGGING_INDEX_NAME).await;
    Ok(())
}