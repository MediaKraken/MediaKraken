use mk_lib_compression;
use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use tokio::sync::Notify;

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
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkopenlibrarynetfetchbulk")
            .await
            .unwrap();

    let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
        "mkopenlibrarynetfetchbulk",
        &rabbit_channel,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();

                let _fetch_result = mk_lib_network::mk_lib_network::mk_download_file_from_url(
                    "https://openlibrary.org/data/ol_cdump_latest.txt.gz".to_string(),
                    &"/ol_cdump_latest.txt.gz".to_string(),
                )
                .await
                .unwrap();

                mk_lib_compression::mk_lib_compression::mk_decompress_tar_gz_file(
                    "/ol_cdump_latest.txt.gz",
                )
                .await
                .unwrap();

                let file = File::open("foo.txt").unwrap();
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    println!("{}", line.unwrap());
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
