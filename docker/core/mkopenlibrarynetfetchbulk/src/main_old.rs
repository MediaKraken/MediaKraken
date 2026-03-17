use mk_lib_compression;
use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use tokio::sync::Notify;

/* tab delimited
type - type of record (/type/edition, /type/work etc.)  0
key - unique key of the record. (/books/OL1M etc.       1
revision - revision number of the record                2
last_modified - last modified timestamp                 3
JSON - the complete record in JSON format               4
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120)
            .await
            .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
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

    struct OLTask {
        msg_type: &'static str,
        url: &'static str,
        gz_path: &'static str,
        txt_path: &'static str,
    }

    // Define the tasks
    const TASKS: [OLTask; 3] = [
        OLTask {
            msg_type: "authors",
            url: "https://openlibrary.org/data/ol_dump_authors_latest.txt.gz",
            gz_path: "/mediakraken/ol_dump_authors_latest.txt.gz",
            txt_path: "/mediakraken/ol_dump_authors_latest.txt",
        },
        OLTask {
            msg_type: "editions",
            url: "https://openlibrary.org/data/ol_dump_editions_latest.txt.gz",
            gz_path: "/mediakraken/ol_dump_editions_latest.txt.gz",
            txt_path: "/mediakraken/ol_dump_editions_latest.txt",
        },
        OLTask {
            msg_type: "works",
            url: "https://openlibrary.org/data/ol_dump_works_latest.txt.gz",
            gz_path: "/mediakraken/ol_dump_works_latest.txt.gz",
            txt_path: "/mediakraken/ol_dump_works_latest.txt",
        },
    ];

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            let Some(payload) = msg.content else { continue };

            // Use a Result-based approach for the JSON
            let json_message: Value = match serde_json::from_slice(&payload) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                    continue;
                }
            };

            let msg_type = json_message["Type"].as_str().unwrap_or("");

            for task in TASKS.iter() {
                if msg_type == task.msg_type || msg_type == "all" {
                    // 1. Download if missing
                    if !Path::new(task.gz_path).exists() && !Path::new(task.txt_path).exists() {
                        let _ = mk_lib_network::mk_lib_network::mk_download_file_from_url_tokio(
                            task.url.to_string(),
                            &task.gz_path.to_string(),
                        )
                        .await;
                    }

                    // 2. Decompress
                    if !Path::new(task.txt_path).exists() {
                        let _ = mk_lib_compression::mk_lib_compression::mk_decompress_tar_gz_file_gunzip(
                        task.gz_path,
                    ).await;
                    }

                    // 3. Database Sync (Using a match or specific dispatch)
                    match task.msg_type {
                        "authors" => {
                            let _ = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib_copy::mk_lib_database_copy(&sqlx_pool_rw, task.txt_path).await;
                            let _ = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib_copy::mk_lib_database_copy_author_upsert(&sqlx_pool_rw).await;
                        }
                        "editions" => {
                            let _ = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib_copy::mk_lib_database_copy(&sqlx_pool_rw, task.txt_path).await;
                            let _ = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib_copy::mk_lib_database_copy_edition_upsert(&sqlx_pool_rw).await;
                        }
                        "works" => {
                            let _ = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib_copy::mk_lib_database_copy(&sqlx_pool_rw, task.txt_path).await;
                            let _ = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib_copy::mk_lib_database_copy_work_upsert(&sqlx_pool_rw).await;
                        }
                        _ => {}
                    }

                    // 4. Cleanup both files to keep disk clean
                    let _ = fs::remove_file(task.txt_path);
                    let _ = fs::remove_file(task.gz_path);
                }
            }

            // Always Ack at the end of successful processing
            if let Some(deliver) = msg.deliver {
                let _ = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    deliver.delivery_tag(),
                )
                .await;
            }
        }
    });
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
