#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, QueueDeclareOptions, Result};
use serde_json::{json, Value};
use std::error::Error;
use std::path::Path;
use std::process::Command;
//use rustube::{Id, VideoFetcher};

#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_option_status.rs"]
mod mk_lib_database_option_status;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_network.rs"]
mod mk_lib_network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkdownload";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           false).await.unwrap();
    let option_config_json: Value = mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool).await.unwrap();

    // open rabbit connection
    let mut rabbit_connection = Connection::insecure_open(
        "amqp://guest:guest@mkstack_rabbitmq:5672")?;
    // Open a channel - None says let the library choose the channel ID.
    let rabbit_channel = rabbit_connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let _rabbit_exchange = Exchange::direct(&rabbit_channel);

    // Declare the queue.
    let queue = rabbit_channel.queue_declare("mk_download", QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;

    loop {
        for (i, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let json_message: Value = serde_json::from_str(
                        &String::from_utf8_lossy(&delivery.body))?;
                    mk_lib_logging::mk_logging_post_elk("info",
                                                        json!({ "msg body": json_message }),
                                                        LOGGING_INDEX_NAME).await;
                    if json_message["Type"].to_string() == "File" {
                        // do NOT remove the header.....this is the SAVE location
                        mk_lib_network::mk_download_file_from_url(json_message["URL"].to_string(),
                                                                  &json_message["Local Save Path"].to_string()).await.unwrap();
                    } else if json_message["Type"].to_string() == "Youtube" {
                        if validator::validate_url(json_message["URL"].to_string()) {
                            continue;
                            //println!("downloaded video to {:?}", rustube::download_best_quality(&json_message["URL"].to_string()).await.unwrap());
                        } else {
                            // TODO log error by user requested
                            continue;
                        }
                    } else if json_message["Type"].to_string() == "HDTrailers" {
                        // try to grab the RSS feed itself
                        let data: serde_json::Value = serde_json::from_str(&mk_lib_network::mk_data_from_url(
                            "http://feeds.hd-trailers.net/hd-trailers".to_string()).await.unwrap()).unwrap();
                        mk_lib_logging::mk_logging_post_elk("info",
                                                            json!({ "download": { "hdtrailer_json": data } }),
                                                            LOGGING_INDEX_NAME).await;
                        let an_array = data["rss"]["channel"]["item"].as_array().unwrap();
                        for item in an_array.iter() {
                            mk_lib_logging::mk_logging_post_elk("info",
                                                                json!({ "item": item }),
                                                                LOGGING_INDEX_NAME).await;
                            let mut download_link = String::new();
                            if (item["title"].to_string().contains("(Trailer") &&
                                option_config_json["Metadata"]["Trailer"]["Trailer"] == true)
                                || (item["title"].to_string().contains("(Behind")
                                && option_config_json["Metadata"]["Trailer"]["Behind"] == true)
                                || (item["title"].to_string().contains("(Clip")
                                && option_config_json["Metadata"]["Trailer"]["Clip"] == true)
                                || (item["title"].to_string().contains("(Featurette")
                                && option_config_json["Metadata"]["Trailer"]["Featurette"] == true)
                                || (item["title"].to_string().contains("(Carpool")
                                && option_config_json["Metadata"]["Trailer"]["Carpool"] == true)
                            {
                                download_link = item["enclosure"]["@url"].to_string();
                            }
                            if download_link != String::new() {
                                // do NOT remove the header.....this is the SAVE location
                                let file_save_name = format!("/mediakraken/web_app/static/meta/trailer/{:?}",
                                                             download_link.rsplitn(1, "/"));
                                // verify it doesn't exist in meta folder
                                if !Path::new(&file_save_name).exists() {
                                    mk_lib_network::mk_download_file_from_url(download_link.to_string(),
                                                                              &file_save_name.to_string()).await.unwrap();
                                }
                            }
                        }
                    }
                    println!("({:>3}) Received [{}]", i, json_message);
                    consumer.ack(delivery)?;
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
    }
}