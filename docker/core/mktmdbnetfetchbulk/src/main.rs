use mk_lib_common;
use mk_lib_compression;
use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use tokio::sync::Notify;

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

#[derive(Serialize, Deserialize)]
struct MetadataPerson {
    adult: bool,
    id: Option<i32>,
    name: String,
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

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();

    let (_rabbit_connection, rabbit_channel) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect(
        "mkstack_rabbitmq",
        "mktmdbnetfetchbulk",
    )
    .await
    .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mktmdbnetfetchbulk", &rabbit_channel)
            .await
            .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                #[cfg(debug_assertions)]
                {
                    mk_lib_logging::mk_logging_post_elk(
                        std::module_path!(),
                        json!({ "msg body": json_message }),
                    )
                    .await
                    .unwrap();
                }
                // let fetch_date: String = "05_30_2023".to_string();
                // grab the movie id's
                let _fetch_result_movie =
                    mk_lib_network::mk_lib_network::mk_download_file_from_url(
                        format!(
                            "http://files.tmdb.org/p/exports/movie_ids_{}.json.gz",
                            json_message["Data"].as_str().unwrap()
                        )
                        .replace("\"", ""),
                        &"/movie.gz".to_string(),
                    )
                    .await
                    .unwrap();
                let json_result =
                    mk_lib_compression::mk_lib_compression::mk_decompress_gz_data("/movie.gz")
                        .await
                        .unwrap();
                // Please note that the data is NOT in id order
                for json_item in json_result.split('\n') {
                    if !json_item.trim().is_empty() {
                        let metadata_struct: MetadataMovie =
                            serde_json::from_str(json_item.trim()).unwrap();
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
                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
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
                        json_message["Data"].as_str().unwrap()
                    )
                    .replace("\"", ""),
                    &"/tv.gz".to_string(),
                )
                .await
                .unwrap();
                let json_result =
                    mk_lib_compression::mk_lib_compression::mk_decompress_gz_data("/tv.gz")
                        .await
                        .unwrap();
                for json_item in json_result.split('\n') {
                    if !json_item.trim().is_empty() {
                        let metadata_struct: MetadataTV =
                            serde_json::from_str(json_item.trim()).unwrap();
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
                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                            "themoviedb".to_string(),
                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                            uuid::Uuid::new_v4(),
                                                                                                            metadata_struct.id,
                                                                                                            "Fetch".to_string()).await.unwrap();
                            }
                        }
                    }
                }

                // grab the Person id's
                let _fetch_result_tv = mk_lib_network::mk_lib_network::mk_download_file_from_url(
                    format!(
                        "http://files.tmdb.org/p/exports/person_ids{}.json.gz",
                        json_message["Data"].as_str().unwrap()
                    )
                    .replace("\"", ""),
                    &"/person.gz".to_string(),
                )
                .await
                .unwrap();
                let json_result =
                    mk_lib_compression::mk_lib_compression::mk_decompress_gz_data("/person.gz")
                        .await
                        .unwrap();
                for json_item in json_result.split('\n') {
                    if !json_item.trim().is_empty() {
                        let metadata_struct: MetadataPerson =
                            serde_json::from_str(json_item.trim()).unwrap();
                        let result =
                            mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_exists_person(
                                &sqlx_pool,
                                metadata_struct.id.unwrap_or(0),
                            )
                            .await
                            .unwrap();
                        if result == false {
                            let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
                                                                                                                              "themoviedb".to_string(),
                                                                                                                              mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                              metadata_struct.id.unwrap_or(0)).await.unwrap();
                            if download_result == false {
                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                            "themoviedb".to_string(),
                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                            uuid::Uuid::new_v4(),
                                                                                                            metadata_struct.id,
                                                                                                            "Fetch".to_string()).await.unwrap();
                            }
                        }
                    }
                }

                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.unwrap().delivery_tag(),
                )
                .await;
            }
        }
    });
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
