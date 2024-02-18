use mk_lib_common;
use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde::Deserialize;
use std::error::Error;
use tokio::sync::Notify;
use uuid::Uuid;

#[derive(Deserialize)]
struct ResponseMetadata {
    results: Vec<MetadataGeneral>,
}

#[derive(Deserialize)]
struct MetadataGeneral {
    id: i32,
    _adult: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1, 120)
        .await
        .unwrap();
    let _result =
        mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
            .await;
    let option_config_json =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mktmdbnetfetchupdate")
            .await
            .unwrap();

    let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
        "mktmdbnetfetchupdate",
        &rabbit_channel,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(_payload) = msg.content {
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
                let resp: ResponseMetadata = serde_json::from_str(&url_result.trim()).unwrap();
                for json_item in resp.results {
                    //for json_item in vec_result.iter() {
                    //println!("item {}", json_item);
                    println!("item {}", json_item.id);
                    //println!("key {:?} item {:?}", json_key, json_item);
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
                                                                                                                    Uuid::new_v4(),
                                                                                                                   Some(json_item.id),
                                                                                                                    "Fetch".to_string(), None).await;
                        } else {
                            // it's on the database, so must update the record with latest information
                            let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                    "themoviedb".to_string(),
                                                                                                                    mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                    Uuid::new_v4(),
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
                                                                                                                        Uuid::new_v4(),
                                                                                                                        Some(json_item.id),
                                                                                                                        "Fetch".to_string(), None).await;
                        } else {
                            // it's on the database, so must update the record with latest information
                            let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                        Uuid::new_v4(),
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
                                                                                                                        Uuid::new_v4(),
                                                                                                                        Some(json_item.id),
                                                                                                                        "Fetch".to_string(), None).await;
                        } else {
                            // it's on the database, so must update the record with latest information
                            let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                        Uuid::new_v4(),
                                                                                                                        Some(json_item.id),
                                                                                                                        "Update".to_string(), None).await;
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
