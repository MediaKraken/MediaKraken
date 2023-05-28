use mk_lib_common;
use mk_lib_compression;
use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use mk_lib_network;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use stdext::function_name;

#[derive(Serialize, Deserialize)]
struct MetadataMovie {
    adult: bool,
    id: Option<i32>,
    original_title: String,
    popularity: f32,
    video: bool,
}

#[derive(Serialize, Deserialize)]
struct MetadataTV {
    id: Option<i32>,
    original_name: String,
    popularity: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}))
            .await
            .unwrap();
    }

    let fetch_date: String = "05_10_2023".to_string();

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();

    // grab the movie id's
    let _fetch_result_movie = mk_lib_network::mk_lib_network::mk_download_file_from_url(
        format!(
            "http://files.tmdb.org/p/exports/movie_ids_{}.json.gz",
            fetch_date
        ),
        &"/movie.gz".to_string(),
    )
    .await;
    let json_result = mk_lib_compression::mk_lib_compression::mk_decompress_gz_data("/movie.gz")
        .await
        .unwrap();
    // Please note that the data is NOT in id order
    for json_item in json_result.split('\n') {
        if !json_item.trim().is_empty() {
            let metadata_struct: MetadataMovie = serde_json::from_str(json_item.trim())?;
            let result =
                mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_exists_movie(
                    &sqlx_pool,
                    metadata_struct.id.unwrap_or(0),
                )
                .await
                .unwrap();
            if result == false {
                let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
                                                                                                                              "themoviedb".to_string(),
                                                                                                                              mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                              metadata_struct.id.unwrap_or(0)).await.unwrap();
                if download_result == false {
                    let result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                            "themoviedb".to_string(),
                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                            uuid::Uuid::new_v4(),
                                                                                                            metadata_struct.id,
                                                                                                            "Fetch".to_string()).await.unwrap();
                }
            }
        }
    }

    // grab the TV id's
    let _fetch_result_tv = mk_lib_network::mk_lib_network::mk_download_file_from_url(
        format!(
            "http://files.tmdb.org/p/exports/tv_series_ids_{}.json.gz",
            fetch_date
        ),
        &"/tv.gz".to_string(),
    )
    .await;
    let json_result = mk_lib_compression::mk_lib_compression::mk_decompress_gz_data("/tv.gz")
        .await
        .unwrap();
    for json_item in json_result.split('\n') {
        if !json_item.trim().is_empty() {
            let metadata_struct: MetadataTV = serde_json::from_str(json_item.trim())?;
            let result =
                mk_lib_database::database_metadata::mk_lib_database_metadata_tv::mk_lib_database_metadata_exists_tv(
                    &sqlx_pool,
                    metadata_struct.id.unwrap_or(0),
                )
                .await
                .unwrap();
            if result == false {
                let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
                                                                                                                              "themoviedb".to_string(),
                                                                                                                              mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                              metadata_struct.id.unwrap_or(0)).await.unwrap();
                if download_result == false {
                    let result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                            "themoviedb".to_string(),
                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                            uuid::Uuid::new_v4(),
                                                                                                            metadata_struct.id,
                                                                                                            "Fetch".to_string()).await.unwrap();
                }
            }
        }
    }
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk("info", json!({"STOP": "STOP"})).await;
    }
    Ok(())
}
