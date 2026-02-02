use mk_lib_common;
use mk_lib_compression;
use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use tokio::sync::Notify;
use chrono::Local;

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

#[derive(Deserialize)]
struct ResponseMetadata {
    results: Vec<MetadataGeneral>,
}

#[derive(Deserialize)]
struct MetadataGeneral {
    id: i32,
    adult: Option<bool>,
}

pub async fn find_date_to_use(url_template: &str) -> Result<String, Box<dyn Error>> {
    let mut date_to_use = Local::now().format("%m_%d_%Y").to_string();
    let mut found = false;
    for _ in 0..7 {
        let test_url = url_template.replace("{}", &date_to_use.clone());
        if mk_lib_network::mk_lib_network::is_url_available(&test_url.clone().as_str()).await {
            found = true;
            break;
        } else {
            let date = chrono::NaiveDate::parse_from_str(&date_to_use, "%m_%d_%Y")?;
            let previous_date = date - chrono::Duration::days(1);
            date_to_use = previous_date.format("%m_%d_%Y").to_string();
        }
    }
    if found {
        Ok(date_to_use)
    } else {
        Err("No valid date found within the last 7 days".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool_write(1, 120)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mktmdbnetfetchbulk")
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
                println!(" [x] Received {:?}", json_message);
                if json_message["Type"] == "Bulk" {
                     let mut record_limit = 0;
                    if json_message["Limit"].is_number() {
                        record_limit = json_message["Limit"].as_i64().unwrap_or(i64::MAX);
                    }
                    let date_to_use = find_date_to_use("http://files.tmdb.org/p/exports/movie_ids_{}.json.gz").await.unwrap();
                    // grab the movie id's
                    let fetch_result_movie =
                        mk_lib_network::mk_lib_network::mk_network_download_file_to_vec(
                            format!(
                                "http://files.tmdb.org/p/exports/movie_ids_{}.json.gz",
                                date_to_use
                            )
                            .replace("\"", "")
                        )
                        .await
                        .unwrap();
                    let json_result =
                        mk_lib_compression::mk_lib_compression::mk_decompress_gz_bytes(fetch_result_movie)
                            .await
                            .unwrap();
                    // Please note that the data is NOT in id order
                    let mut record_count = 0;
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
                                    record_count += 1;
                                    if record_count > record_limit {
                                        break;
                                    }
                                    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                            "themoviedb".to_string(),
                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                            uuid::Uuid::now_v7(),
                                                                                                            metadata_struct.id,
                                                                                                            "Fetch".to_string(), None).await.unwrap();
                                }
                            }
                        }
                    }

                    let date_to_use = find_date_to_use("http://files.tmdb.org/p/exports/tv_series_ids_{}.json.gz").await.unwrap();
                    // grab the TV id's
                    let fetch_result_tv =
                        mk_lib_network::mk_lib_network::mk_network_download_file_to_vec(
                            format!(
                                "http://files.tmdb.org/p/exports/tv_series_ids_{}.json.gz",
                                date_to_use
                            )
                            .replace("\"", "")
                        )
                        .await
                        .unwrap();
                    let json_result =
                        mk_lib_compression::mk_lib_compression::mk_decompress_gz_bytes(fetch_result_tv)
                            .await
                            .unwrap();
                    let mut record_count = 0;
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
                                    record_count += 1;
                                    if record_count > record_limit {
                                        break;
                                    }
                                    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                            "themoviedb".to_string(),
                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                            uuid::Uuid::now_v7(),
                                                                                                            metadata_struct.id,
                                                                                                            "Fetch".to_string(), None).await.unwrap();
                                }
                            }
                        }
                    }

                    let date_to_use = find_date_to_use("http://files.tmdb.org/p/exports/person_ids_{}.json.gz").await.unwrap();
                    // grab the Person id's
                    let fetch_result_person =
                        mk_lib_network::mk_lib_network::mk_network_download_file_to_vec(
                            format!(
                                "http://files.tmdb.org/p/exports/person_ids_{}.json.gz",
                                date_to_use
                            )
                            .replace("\"", "")
                        )
                        .await
                        .unwrap();
                    let json_result =
                        mk_lib_compression::mk_lib_compression::mk_decompress_gz_bytes(fetch_result_person)
                            .await
                            .unwrap();
                    let mut record_count = 0;
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
                                    record_count += 1;
                                    if record_count > record_limit {
                                        break;
                                    }
                                    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                            "themoviedb".to_string(),
                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                            uuid::Uuid::now_v7(),
                                                                                                            metadata_struct.id,
                                                                                                            "Fetch".to_string(), None).await.unwrap();
                                }
                            }
                        }
                    }
                } else {
                    let option_config_json: serde_json::Value =
                    mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
                        .await
                        .unwrap();
                    // process movie changes
                    let url_result = mk_lib_network::mk_lib_network::mk_data_from_url(
                        format!(
                            "https://api.themoviedb.org/3/movie/changes?api_key={}",
                            option_config_json["API"]["themoviedb"]
                        )
                        .replace("\"", ""),
                    )
                    .await
                    .unwrap();
                    println!("one {:?}", url_result);
                    let resp: ResponseMetadata = serde_json::from_str(&url_result.trim()).unwrap();
                    for json_item in resp.results {
                        println!("movie item {}", json_item.id);
                        // verify it's not already in the database
                        let result =
                            mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_exists_movie(
                                &sqlx_pool,
                                json_item.id,
                            )
                            .await
                            .unwrap();
                        if result == false {
                            let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
                                                                                                                                      "themoviedb".to_string(),
                                                                                                                                      mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                                      json_item.id).await.unwrap();
                            if download_result == false {
                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                    "themoviedb".to_string(),
                                                                                                                    mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                    uuid::Uuid::now_v7(),
                                                                                                                   Some(json_item.id),
                                                                                                                    "Fetch".to_string(), None).await;
                            } else {
                                // it's on the database, so must update the record with latest information
                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                    "themoviedb".to_string(),
                                                                                                                    mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                    uuid::Uuid::now_v7(),
                                                                                                                    Some(json_item.id),
                                                                                                                    "Update".to_string(), None).await;
                            }
                        }
                    }

                    // process tv changes
                    let url_result = mk_lib_network::mk_lib_network::mk_data_from_url(
                        format!(
                            "https://api.themoviedb.org/3/tv/changes?api_key={}",
                            option_config_json["API"]["themoviedb"]
                        )
                        .replace("\"", ""),
                    )
                    .await
                    .unwrap();
                    let resp: ResponseMetadata = serde_json::from_str(&url_result.trim()).unwrap();
                    for json_item in resp.results {
                        println!("tv item {}", json_item.id);
                        // verify it's not already in the database
                        let result =
                            mk_lib_database::database_metadata::mk_lib_database_metadata_tv::mk_lib_database_metadata_exists_tv(
                                &sqlx_pool,
                                json_item.id,
                            )
                            .await
                            .unwrap();
                        if result == false {
                            let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
                                                                                                                                          "themoviedb".to_string(),
                                                                                                                                          mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                                          json_item.id).await.unwrap();
                            if download_result == false {
                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                        Some(json_item.id),
                                                                                                                        "Fetch".to_string(), None).await;
                            } else {
                                // it's on the database, so must update the record with latest information
                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                        Some(json_item.id),
                                                                                                                        "Update".to_string(), None).await;
                            }
                        }
                    }

                    // process person changes
                    let url_result = mk_lib_network::mk_lib_network::mk_data_from_url(
                        format!(
                            "https://api.themoviedb.org/3/person/changes?api_key={}",
                            option_config_json["API"]["themoviedb"]
                        )
                        .replace("\"", ""),
                    )
                    .await
                    .unwrap();
                    let resp: ResponseMetadata = serde_json::from_str(&url_result).unwrap();
                    for json_item in resp.results {
                        println!("person item {}", json_item.id);
                        // verify it's not already in the database
                        let result =
                            mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_exists_person(
                                &sqlx_pool,
                                json_item.id,
                            )
                            .await
                            .unwrap();
                        if result == false {
                            let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
                                                                                                                                          "themoviedb".to_string(),
                                                                                                                                          mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                                          json_item.id).await.unwrap();
                            if download_result == false {
                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                        Some(json_item.id),
                                                                                                                        "Fetch".to_string(), None).await;
                            } else {
                                // it's on the database, so must update the record with latest information
                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                        Some(json_item.id),
                                                                                                                        "Update".to_string(), None).await;
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
