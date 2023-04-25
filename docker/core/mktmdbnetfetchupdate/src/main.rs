#![cfg_attr(debug_assertions, allow(dead_code))]

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;
use std::error::Error;
use stdext::function_name;
use uuid::Uuid;

mod mk_lib_common;

mod mk_lib_common_enum_media_type;

mod mk_lib_compression;

#[path = "database"]
pub mod database {
    pub mod mk_lib_database;
    pub mod mk_lib_database_metadata_download_queue;
    pub mod mk_lib_database_metadata_movie;
    pub mod mk_lib_database_metadata_tv;
    pub mod mk_lib_database_option_status;
    pub mod mk_lib_database_version;
    pub mod mk_lib_database_version_schema;
}

mod mk_lib_logging;

mod mk_lib_network;

#[derive(Deserialize)]
struct Response {
    results: Vec<MetadataMovie>,
}

#[derive(Deserialize)]
struct MetadataMovie {
    id: i32,
    adult: bool,
}

#[derive(Serialize, Deserialize)]
struct MetadataTV {
    id: i32,
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
    let sqlx_pool = database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false).await;
    let option_config_json =
        database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();
    // println!("options: {:?}", option_config_json);
    // println!("api {:?}", option_config_json["API"]);
    // println!("tmdb{:?}", option_config_json["API"]["themoviedb"]);
    // println!("huh {:?}", format!("https://api.themoviedb.org/3/movie/changes?api_key={}",
    //                              option_config_json["API"]["themoviedb"]).replace("\"", ""));
    // process movie changes
    let url_result = mk_lib_network::mk_data_from_url(
        format!(
            "https://api.themoviedb.org/3/movie/changes?api_key={}",
            option_config_json["API"]["themoviedb"]
        )
        .replace("\"", ""),
    )
    .await?;
    //println!("result: {:?}", url_result);
    //let json_result: HashMap<String, Value> = serde_json::from_str(&url_result).unwrap();
    // let json_result: Value = serde_json::from_str(&url_result).unwrap();
    // println!("json_result: {:?}", json_result["results"]);
    // let vec_result: Vec<MetadataMovie> = json_result["results"];
    //println!("vec {:?}", vec_result);
    let resp: Response = serde_json::from_str(&url_result.trim()).unwrap();
    for json_item in resp.results {
        //for json_item in vec_result.iter() {
        //println!("item {}", json_item);
        println!("item {}", json_item.id);
        //println!("key {:?} item {:?}", json_key, json_item);
        // verify it's not already in the database
        // let result = database::mk_lib_database_metadata::mk_lib_database_metadata_exists_movie(&sqlx_pool,
        //                                                                              json_item["id"]).await.unwrap();
        // if result == false {
        //     let download_result = database::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
        //                                                                                                                   "themoviedb".to_string(),
        //                                                                                                                   mk_lib_common_enum_media_type::DLMediaType::MOVIE,
        //                                                                                                                   json_item["id"]).await.unwrap();
        //     if download_result == false {
        //         database::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
        //                                                                                                 "themoviedb".to_string(),
        //                                                                                                 mk_lib_common_enum_media_type::DLMediaType::MOVIE,
        //                                                                                                 Uuid::new_v4(),
        //                                                                                                json_item["id"],
        //                                                                                                 "Fetch".to_string()).await;
        //     } else {
        //         // it"s on the database, so must update the record with latest information
        //         database::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
        //                                                                                                 "themoviedb".to_string(),
        //                                                                                                 mk_lib_common_enum_media_type::DLMediaType::MOVIE,
        //                                                                                                 Uuid::new_v4(),
        //                                                                                                 json_item["id"],
        //                                                                                                 "Update".to_string()).await;
        //     }
        // }
    }

    // process tv changes
    // let url_result = mk_lib_network::mk_download_file_from_url(
    //     format!("https://api.themoviedb.org/3/tv/changes?api_key={}",
    //             option_config_json["API"]["themoviedb"]).replace("\"", "")).await?;
    // let json_result: Value = serde_json::from_str(&url_result).unwrap();
    // for json_item in json_result["results"].as_object().unwrap() {
    //         // verify it's not already in the database
    //         let result = database::mk_lib_database_metadata::mk_lib_database_metadata_exists_tv(&sqlx_pool,
    //                                                                                   metadata_struct.id).await.unwrap();
    //         if result == false {
    //             let download_result = database::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(&sqlx_pool,
    //                                                                                                                           "themoviedb".to_string(),
    //                                                                                                                           mk_lib_common_enum_media_type::DLMediaType::TV,
    //                                                                                                                           metadata_struct.id).await.unwrap();
    //             if download_result == false {
    //                 database::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
    //                                                                                                         "themoviedb".to_string(),
    //                                                                                                         mk_lib_common_enum_media_type::DLMediaType::TV,
    //                                                                                                         Uuid::new_v4(),
    //                                                                                                         metadata_struct.id,
    //                                                                                                         "Fetch".to_string()).await;
    //             } else {
    //                 // it's on the database, so must update the record with latest information
    //                 database::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
    //                                                                                                         "themoviedb".to_string(),
    //                                                                                                         mk_lib_common_enum_media_type::DLMediaType::TV,
    //                                                                                                         Uuid::new_v4(),
    //                                                                                                         metadata_struct.id,
    //                                                                                                         "Update".to_string()).await;
    //             }
    //         }
    // }

    // TODO need person changes in here as well

    #[cfg(debug_assertions)]
    {
        // stop logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"STOP": "STOP"})).await;
    }
    Ok(())
}
