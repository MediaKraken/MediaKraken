use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use serde_json::json;

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
#[path = "../../../../src/mk_lib_database/src/mk_lib_database_download.rs"]
mod mk_lib_database_download;
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
#[path = "mk_lib_database_download.rs"]
mod mk_lib_database_download;
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
    const LOGGING_INDEX_NAME: &str = "mktmdbnetfetchupdate";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // open the database
    let db_client = &mk_lib_database::mk_lib_database_open().await?;
    mk_lib_database_version::mk_lib_database_version_check(db_client,
                                                           false).await?;
    let option_config_json: Value = serde_json::from_str(
        &mk_lib_database::mk_lib_database_options(db_client).await.unwrap());

    // TODO this should go through the limiter
    // process movie changes
    let _fetch_result_movie = mk_lib_network::mk_download_file_from_url(
        format!("https://api.themoviedb.org/3/movie/changes?api_key={}",
                option_config_json["API"]["themoviedb"]),
        "/mediakraken/movie_update.gz".to_string()).await;
    let json_result = mk_lib_compression::mk_decompress_gzip(
        "/mediakraken/movie_update.gz").unwrap();
    for movie_change in json_result["results"] {
        // verify it's not already in the database
        let result = mk_lib_database_metadata::mk_lib_database_metadata_exists_movie(db_client,
                                                                                     metadata_struct.id).await.unwrap();
        if result == false {
            let download_result = mk_lib_database_download::mk_lib_database_download_exists(db_client,
                                                                                            "themoviedb".to_string(),
                                                                                            mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                            metadata_struct.id).await.unwrap();
            if download_result == false {
                mk_lib_database_download::mk_lib_database_download_insert(db_client,
                                                                          "themoviedb".to_string(),
                                                                          mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                          Uuid::new_v4(),
                                                                          metadata_struct.id,
                                                                          "Fetch".to_string()).await;
            } else {
                // it"s on the database, so must update the record with latest information
                mk_lib_database_download::mk_lib_database_download_insert(db_client,
                                                                          "themoviedb".to_string(),
                                                                          mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                          Uuid::new_v4(),
                                                                          metadata_struct.id,
                                                                          "Update".to_string()).await;
            }
        }
    }

    // TODO this should go through the limiter
    // process tv changes
    let _fetch_result_movie = mk_lib_network::mk_download_file_from_url(
        format!("https://api.themoviedb.org/3/tv/changes?api_key={}",
                option_config_json["API"]["themoviedb"]),
        "/mediakraken/tv_update.gz").await;
    let json_result = mk_lib_compression::mk_decompress_gzip(
        "/mediakraken/tv_update.gz").unwrap();
    for tv_change in json_result["results"] {
        // verify it's not already in the database
        let result = mk_lib_database_metadata::mk_lib_database_metadata_exists_tv(db_client,
                                                                                  metadata_struct.id).await.unwrap();
        if result == false {
            let download_result = mk_lib_database_download::mk_lib_database_download_exists(db_client,
                                                                                            "themoviedb".to_string(),
                                                                                            mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                            metadata_struct.id).await.unwrap();
            if download_result == false {
                mk_lib_database_download::mk_lib_database_download_insert(db_client,
                                                                          "themoviedb".to_string(),
                                                                          mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                          Uuid::new_v4(),
                                                                          metadata_struct.id,
                                                                          "Fetch".to_string()).await;
            } else {
                // it's on the database, so must update the record with latest information
                mk_lib_database_download::mk_lib_database_download_insert(db_client,
                                                                          "themoviedb".to_string(),
                                                                          mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                          Uuid::new_v4(),
                                                                          metadata_struct.id,
                                                                          "Update".to_string()).await;
            }
        }
    }

    // stop logging
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"STOP": "STOP"}),
                                        LOGGING_INDEX_NAME).await;
    Ok(())
}