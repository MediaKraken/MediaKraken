use chrono::Local;
use mk_lib_common;
use mk_lib_compression;
use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use tokio::sync::Notify;

async fn process_rabbit_message(
    json_message: Value,
    sqlx_pool_rw: &sqlx::PgPool,
    sqlx_pool_ro: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    if json_message["Type"] == "Bulk" {
        let mut record_limit = i64::MAX;
        let skip_rows = json_message["Skip"].as_u64().unwrap_or(0) as usize;
        if json_message["Limit"].is_number() {
            record_limit = json_message["Limit"].as_i64().unwrap_or(i64::MAX);
        }
        let date_to_use =
            find_date_to_use("http://files.tmdb.org/p/exports/movie_ids_{}.json.gz").await?;
        let fetch_result_movie = mk_lib_network::mk_lib_network::mk_network_download_file_to_vec(
            format!(
                "http://files.tmdb.org/p/exports/movie_ids_{}.json.gz",
                date_to_use
            )
            .replace("\"", ""),
        )
        .await?;
        let json_result =
            mk_lib_compression::mk_lib_compression::mk_decompress_gz_bytes(fetch_result_movie)
                .await?;
        let mut record_count = 0;
        let mut skipped_rows = 0usize;
        for json_item in json_result.lines() {
            if !json_item.trim().is_empty() {
                if skipped_rows < skip_rows {
                    skipped_rows += 1;
                    continue;
                }
                let metadata_struct: MetadataMovie = serde_json::from_str(json_item.trim())?;
                let Some(metadata_id) = metadata_struct.id else {
                    continue;
                };
                let result =
                    mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_exists_movie(
                        &sqlx_pool_rw,
                        metadata_id,
                    )
                    .await
                    ?;
                if result == false {
                    let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool_rw,
                                                                                                                            "themoviedb".to_string(),
                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                            metadata_id).await?;
                    if download_result == false {
                        record_count += 1;
                        if record_count > record_limit {
                            break;
                        }
                        let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                        Some(metadata_id),
                                                                                                                        "Fetch".to_string(), None).await?;
                    }
                }
            }
        }

        let date_to_use =
            find_date_to_use("http://files.tmdb.org/p/exports/tv_series_ids_{}.json.gz").await?;
        let fetch_result_tv = mk_lib_network::mk_lib_network::mk_network_download_file_to_vec(
            format!(
                "http://files.tmdb.org/p/exports/tv_series_ids_{}.json.gz",
                date_to_use
            )
            .replace("\"", ""),
        )
        .await?;
        let json_result =
            mk_lib_compression::mk_lib_compression::mk_decompress_gz_bytes(fetch_result_tv).await?;
        let mut record_count = 0;
        let mut skipped_rows = 0usize;
        for json_item in json_result.lines() {
            if !json_item.trim().is_empty() {
                if skipped_rows < skip_rows {
                    skipped_rows += 1;
                    continue;
                }
                let metadata_struct: MetadataTV = serde_json::from_str(json_item.trim())?;
                let Some(metadata_id) = metadata_struct.id else {
                    continue;
                };
                let result =
                    mk_lib_database::database_metadata::mk_lib_database_metadata_tv::mk_lib_database_metadata_exists_tv(
                        &sqlx_pool_rw,
                        metadata_id,
                    )
                    .await
                    ?;
                if result == false {
                    let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool_rw,
                                                                                                                            "themoviedb".to_string(),
                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                            metadata_id).await?;
                    if download_result == false {
                        record_count += 1;
                        if record_count > record_limit {
                            break;
                        }
                        let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                        Some(metadata_id),
                                                                                                                        "Fetch".to_string(), None).await?;
                    }
                }
            }
        }

        let date_to_use =
            find_date_to_use("http://files.tmdb.org/p/exports/person_ids_{}.json.gz").await?;
        let fetch_result_person = mk_lib_network::mk_lib_network::mk_network_download_file_to_vec(
            format!(
                "http://files.tmdb.org/p/exports/person_ids_{}.json.gz",
                date_to_use
            )
            .replace("\"", ""),
        )
        .await?;
        let json_result =
            mk_lib_compression::mk_lib_compression::mk_decompress_gz_bytes(fetch_result_person)
                .await?;
        let mut record_count = 0;
        let mut skipped_rows = 0usize;
        for json_item in json_result.lines() {
            if !json_item.trim().is_empty() {
                if skipped_rows < skip_rows {
                    skipped_rows += 1;
                    continue;
                }
                let metadata_struct: MetadataPerson = serde_json::from_str(json_item.trim())?;
                let Some(metadata_id) = metadata_struct.id else {
                    continue;
                };
                let result =
                    mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_exists_person(
                        &sqlx_pool_rw,
                        metadata_id,
                    )
                    .await
                    ?;
                if result == false {
                    let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool_rw,
                                                                                                                            "themoviedb".to_string(),
                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                            metadata_id).await?;
                    if download_result == false {
                        record_count += 1;
                        if record_count > record_limit {
                            break;
                        }
                        let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                        Some(metadata_id),
                                                                                                                        "Fetch".to_string(), None).await?;
                    }
                }
            }
        }

        let date_to_use =
            find_date_to_use("http://files.tmdb.org/p/exports/collection_ids_{}.json.gz").await?;
        let fetch_result_collection =
            mk_lib_network::mk_lib_network::mk_network_download_file_to_vec(
                format!(
                    "http://files.tmdb.org/p/exports/collection_ids_{}.json.gz",
                    date_to_use
                )
                .replace("\"", ""),
            )
            .await?;
        let json_result =
            mk_lib_compression::mk_lib_compression::mk_decompress_gz_bytes(fetch_result_collection)
                .await?;
        let mut record_count = 0;
        let mut skipped_rows = 0usize;
        for json_item in json_result.lines() {
            if !json_item.trim().is_empty() {
                if skipped_rows < skip_rows {
                    skipped_rows += 1;
                    continue;
                }
                let metadata_struct: MetadataCollection = serde_json::from_str(json_item.trim())?;
                let Some(metadata_id) = metadata_struct.id else {
                    continue;
                };
                let result =
                    mk_lib_database::database_metadata::mk_lib_database_metadata_collection::mk_lib_database_metadata_exists_collection(
                        &sqlx_pool_rw,
                        metadata_id,
                    )
                    .await
                    ?;
                if result == false {
                    let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool_rw,
                                                                                                                            "themoviedb".to_string(),
                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::COLLECTION,
                                                                                                                            metadata_id).await?;
                    if download_result == false {
                        record_count += 1;
                        if record_count > record_limit {
                            break;
                        }
                        let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::COLLECTION,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                        Some(metadata_id),
                                                                                                                        "Fetch".to_string(), None).await?;
                    }
                }
            }
        }
    } else {
        let option_config_json: serde_json::Value =
            mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(
                &sqlx_pool_ro,
            )
            .await?;
        let url_result = mk_lib_network::mk_lib_network::mk_data_from_url(
            format!(
                "https://api.themoviedb.org/3/movie/changes?api_key={}",
                option_config_json["API"]["themoviedb"]
            )
            .replace("\"", ""),
        )
        .await?;
        println!("one {:?}", url_result);
        let resp: ResponseMetadata = serde_json::from_str(&url_result.trim())?;
        for json_item in resp.results {
            println!("movie item {}", json_item.id);
            let result =
                mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_exists_movie(
                    &sqlx_pool_rw,
                    json_item.id,
                )
                .await
                ?;
            if result == false {
                let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool_rw,
                                                                                                                                        "themoviedb".to_string(),
                                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                                        json_item.id).await?;
                if download_result == false {
                    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                       Some(json_item.id),
                                                                                                                        "Fetch".to_string(), None).await;
                } else {
                    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                        "themoviedb".to_string(),
                                                                                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE,
                                                                                                                        uuid::Uuid::now_v7(),
                                                                                                                        Some(json_item.id),
                                                                                                                        "Update".to_string(), None).await;
                }
            }
        }

        let url_result = mk_lib_network::mk_lib_network::mk_data_from_url(
            format!(
                "https://api.themoviedb.org/3/tv/changes?api_key={}",
                option_config_json["API"]["themoviedb"]
            )
            .replace("\"", ""),
        )
        .await?;
        let resp: ResponseMetadata = serde_json::from_str(&url_result.trim())?;
        for json_item in resp.results {
            println!("tv item {}", json_item.id);
            let result =
                mk_lib_database::database_metadata::mk_lib_database_metadata_tv::mk_lib_database_metadata_exists_tv(
                    &sqlx_pool_rw,
                    json_item.id,
                )
                .await
                ?;
            if result == false {
                let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool_rw,
                                                                                                                                            "themoviedb".to_string(),
                                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                                            json_item.id).await?;
                if download_result == false {
                    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                            "themoviedb".to_string(),
                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                            uuid::Uuid::now_v7(),
                                                                                                                            Some(json_item.id),
                                                                                                                            "Fetch".to_string(), None).await;
                } else {
                    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                            "themoviedb".to_string(),
                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV,
                                                                                                                            uuid::Uuid::now_v7(),
                                                                                                                            Some(json_item.id),
                                                                                                                            "Update".to_string(), None).await;
                }
            }
        }

        let url_result = mk_lib_network::mk_lib_network::mk_data_from_url(
            format!(
                "https://api.themoviedb.org/3/person/changes?api_key={}",
                option_config_json["API"]["themoviedb"]
            )
            .replace("\"", ""),
        )
        .await?;
        let resp: ResponseMetadata = serde_json::from_str(&url_result)?;
        for json_item in resp.results {
            println!("person item {}", json_item.id);
            let result =
                mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_exists_person(
                    &sqlx_pool_rw,
                    json_item.id,
                )
                .await
                ?;
            if result == false {
                let download_result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool_rw,
                                                                                                                                            "themoviedb".to_string(),
                                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                                            json_item.id).await?;
                if download_result == false {
                    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                            "themoviedb".to_string(),
                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                            uuid::Uuid::now_v7(),
                                                                                                                            Some(json_item.id),
                                                                                                                            "Fetch".to_string(), None).await;
                } else {
                    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                            "themoviedb".to_string(),
                                                                                                                            mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PERSON,
                                                                                                                            uuid::Uuid::now_v7(),
                                                                                                                            Some(json_item.id),
                                                                                                                            "Update".to_string(), None).await;
                }
            }
        }
    }
    Ok(())
}

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

#[derive(Serialize, Deserialize)]
struct MetadataCollection {
    id: Option<i32>,
    name: String,
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
    for _ in 0..7 {
        let test_url = url_template.replace("{}", &date_to_use);
        if mk_lib_network::mk_lib_network::is_url_available(test_url.as_str()).await {
            return Ok(date_to_use);
        }
        let date = chrono::NaiveDate::parse_from_str(&date_to_use, "%m_%d_%Y")?;
        let previous_date = date - chrono::Duration::days(1);
        date_to_use = previous_date.format("%m_%d_%Y").to_string();
    }
    Err("No valid date found within the last 7 days".into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mktmdbnetfetchbulk").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mktmdbnetfetchbulk", &rabbit_channel)
            .await?;

    let spawn_pool_rw = sqlx_pool_rw.clone();
    let spawn_pool_ro = sqlx_pool_ro.clone();
    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let parsed: Value = match serde_json::from_slice(&payload) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        continue;
                    }
                };
                println!(" [x] Received {:?}", parsed);
                if let Err(e) = process_rabbit_message(parsed, &spawn_pool_rw, &spawn_pool_ro).await
                {
                    eprintln!("Processing failed: {}", e);
                }
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.map(|d| d.delivery_tag()).unwrap_or(0),
                )
                .await;
            }
        }
    });
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_movie_deserialize_valid() {
        let json = r#"{"adult":false,"id":123,"original_title":"Test Movie","popularity":5.0,"video":false}"#;
        let movie: MetadataMovie = serde_json::from_str(json).unwrap();
        assert!(!movie.adult);
        assert_eq!(movie.id, Some(123));
        assert_eq!(movie.original_title, "Test Movie");
        assert_eq!(movie.popularity, 5.0);
        assert!(!movie.video);
    }

    #[test]
    fn test_metadata_movie_deserialize_null_id() {
        let json =
            r#"{"adult":false,"id":null,"original_title":"No ID","popularity":1.0,"video":false}"#;
        let movie: MetadataMovie = serde_json::from_str(json).unwrap();
        assert!(movie.id.is_none());
    }

    #[test]
    fn test_metadata_movie_serialize_roundtrip() {
        let movie = MetadataMovie {
            adult: false,
            id: Some(456),
            original_title: "RoundTrip".to_string(),
            popularity: 10.5,
            video: true,
        };
        let serialized = serde_json::to_string(&movie).unwrap();
        let deserialized: MetadataMovie = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.id, Some(456));
        assert_eq!(deserialized.original_title, "RoundTrip");
    }

    #[test]
    fn test_metadata_tv_deserialize_valid() {
        let json = r#"{"id":789,"original_name":"Test Series","popularity":8.5}"#;
        let tv: MetadataTV = serde_json::from_str(json).unwrap();
        assert_eq!(tv.id, Some(789));
        assert_eq!(tv.original_name, "Test Series");
        assert_eq!(tv.popularity, 8.5);
    }

    #[test]
    fn test_metadata_tv_deserialize_null_id() {
        let json = r#"{"id":null,"original_name":"No ID","popularity":1.0}"#;
        let tv: MetadataTV = serde_json::from_str(json).unwrap();
        assert!(tv.id.is_none());
    }

    #[test]
    fn test_metadata_person_deserialize_valid() {
        let json = r#"{"adult":false,"id":321,"name":"Actor Name","popularity":7.0}"#;
        let person: MetadataPerson = serde_json::from_str(json).unwrap();
        assert!(!person.adult);
        assert_eq!(person.id, Some(321));
        assert_eq!(person.name, "Actor Name");
        assert_eq!(person.popularity, 7.0);
    }

    #[test]
    fn test_metadata_person_deserialize_null_id() {
        let json = r#"{"adult":true,"id":null,"name":"No ID","popularity":1.0}"#;
        let person: MetadataPerson = serde_json::from_str(json).unwrap();
        assert!(person.id.is_none());
    }

    #[test]
    fn test_metadata_collection_deserialize_valid() {
        let json = r#"{"id":654,"name":"Test Collection"}"#;
        let collection: MetadataCollection = serde_json::from_str(json).unwrap();
        assert_eq!(collection.id, Some(654));
        assert_eq!(collection.name, "Test Collection");
    }

    #[test]
    fn test_metadata_collection_deserialize_null_id() {
        let json = r#"{"id":null,"name":"No ID"}"#;
        let collection: MetadataCollection = serde_json::from_str(json).unwrap();
        assert!(collection.id.is_none());
    }

    #[test]
    fn test_response_metadata_deserialize_valid() {
        let json = r#"{"results":[{"id":1,"adult":false},{"id":2,"adult":true}]}"#;
        let response: ResponseMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(response.results.len(), 2);
        assert_eq!(response.results[0].id, 1);
        assert_eq!(response.results[1].id, 2);
    }

    #[test]
    fn test_response_metadata_deserialize_empty_results() {
        let json = r#"{"results":[]}"#;
        let response: ResponseMetadata = serde_json::from_str(json).unwrap();
        assert!(response.results.is_empty());
    }

    #[test]
    fn test_metadata_general_deserialize_null_adult() {
        let json = r#"{"id":999,"adult":null}"#;
        let general: MetadataGeneral = serde_json::from_str(json).unwrap();
        assert_eq!(general.id, 999);
        assert!(general.adult.is_none());
    }

    #[test]
    fn test_metadata_general_deserialize_bool_adult() {
        let json = r#"{"id":888,"adult":true}"#;
        let general: MetadataGeneral = serde_json::from_str(json).unwrap();
        assert_eq!(general.id, 888);
        assert_eq!(general.adult, Some(true));
    }

    #[test]
    fn test_metadata_movie_clone() {
        let movie = MetadataMovie {
            adult: false,
            id: Some(111),
            original_title: "Clone Test".to_string(),
            popularity: 3.0,
            video: false,
        };
        let cloned = movie.clone();
        assert_eq!(cloned.id, movie.id);
        assert_eq!(cloned.original_title, movie.original_title);
    }

    #[test]
    fn test_metadata_tv_clone() {
        let tv = MetadataTV {
            id: Some(222),
            original_name: "Clone TV".to_string(),
            popularity: 4.0,
        };
        let cloned = tv.clone();
        assert_eq!(cloned.id, tv.id);
    }

    #[test]
    fn test_metadata_person_clone() {
        let person = MetadataPerson {
            adult: false,
            id: Some(333),
            name: "Clone Person".to_string(),
            popularity: 5.0,
        };
        let cloned = person.clone();
        assert_eq!(cloned.id, person.id);
        assert_eq!(cloned.name, person.name);
    }

    #[test]
    fn test_metadata_collection_clone() {
        let collection = MetadataCollection {
            id: Some(444),
            name: "Clone Collection".to_string(),
        };
        let cloned = collection.clone();
        assert_eq!(cloned.id, collection.id);
        assert_eq!(cloned.name, collection.name);
    }
}
