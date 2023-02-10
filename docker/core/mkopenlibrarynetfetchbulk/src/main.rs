#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;
use sqlx::{types::Json, types::Uuid};
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use stdext::function_name;

#[path = "mk_lib_compression.rs"]
mod mk_lib_compression;
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_network.rs"]
mod mk_lib_network;

#[derive(Serialize, Deserialize)]
struct MetadataBook {
    adult: bool,
    id: Option<i32>,
    original_title: String,
    popularity: f32,
    video: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"})).await;
    }

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool(1).await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();

    let _fetch_result_movie = mk_lib_network::mk_download_file_from_url(
        "https://openlibrary.org/data/ol_cdump_latest.txt.gz".to_string(),
        &"/ol_cdump_latest.txt.gz".to_string(),
    )
    .await;

    mk_lib_compression::mk_decompress_tar_gz_file("/ol_cdump_latest.txt.gz")
        .await
        .unwrap();

    let file = File::open("foo.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }

    #[cfg(debug_assertions)]
    {
        // stop logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"STOP": "STOP"})).await;
    }
    Ok(())
}
