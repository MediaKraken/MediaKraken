use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde_json::{Value, json};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::{Notify, Semaphore};

const DEFAULT_MAX_CONCURRENT_DOWNLOADS: usize = 4;

async fn process_message(json_message: Value, option_config_json: &Value) {
    if json_message["Type"].to_string() == "File" {
        // do NOT remove the header.....this is the SAVE location
        mk_lib_network::mk_lib_network::mk_download_file_from_url(
            json_message["URL"].to_string(),
            &json_message["Local Save Path"].to_string(),
        )
        .await
        .unwrap();
    } else if json_message["Type"].to_string() == "Youtube" {
        if validator::ValidateUrl::validate_url(&json_message["URL"].to_string()) {
            //println!("downloaded video to {:?}", rustube::download_best_quality(&json_message["URL"].to_string()).await.unwrap());
            return;
        }
        // TODO log error by user requested
    } else if json_message["Type"].to_string() == "Subtitle" {
        let _output = Command::new("subliminal")
            .args(["-l", "en", &json_message["Data"].as_str().unwrap()])
            .stdout(Stdio::piped())
            .output()
            .unwrap();
    } else if json_message["Type"].to_string() == "Twitch" {
        if validator::ValidateUrl::validate_url(&json_message["URL"].to_string()) {
            let _res = mk_lib_network::mk_lib_network::mk_download_file_from_url_tokio(
                json_message["URL"].to_string(),
                &json_message["Local Save Path"].to_string(),
            )
            .await;
        }
        // TODO log error by user requested
        // } else if json_message["Type"].to_string() == "DigitalUPCNet" {
        //     let sheet_data = mk_lib_metadata::mk_lib_metadata_provider_google_sheets::provider_google_sheets_fetch(
        //         "1po70GCN9JUwrWgycMueNfxpEvBjLd7DQkiMRUQFFsL8".to_string(),
        //         "tsv".to_string(),
        //     )
        //     .await
        //     .unwrap();
        //     // process sheet data TODO, this might be worthless d2d only
        //     let mut rdr = csv::Reader::from_reader(sheet_dataq.as_bytes());
        //     for result in rdr.deserialize() {
        //         let record: DigitalUPCNetRecord = result?;
        //         println!("{:?}", record);
        //         // TODO "one-time" load.....do this BEFORE upc master list
        //     }
        // } else if json_message["Type"].to_string() == "UPCMasterList" {
        //     let sheet_data = mk_lib_metadata::mk_lib_metadata_provider_google_sheets::provider_google_sheets_fetch(
        //         "1IgK7tIEKngP59PUOs_lsF4P1hbSRIG71tgxncpu1Mws".to_string(),
        //         "tsv".to_string(),
        //     )
        //     .await
        //     .unwrap();
        //     // process sheet data, this might be worthless d2d only
        //     let mut rdr = csv::Reader::from_reader(sheet_dataq.as_bytes());
        //     for result in rdr.deserialize() {
        //         let record: UPCMasterNetRecord = result?;
        //         println!("{:?}", record);
        //         // TODO "one-time" load
        //     }
    } else if json_message["Type"].to_string() == "Dosage" {
        // This saves to ./Comics
        let output = Command::new("dosage")
            .args(["--adult", &json_message["Data"].as_str().unwrap()])
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        let _stdout = String::from_utf8(output.stdout).unwrap();
        if json_message["Data"].as_str().unwrap() == "--list" {
            // TODO parse list and store the strips, see notes in data example
        }
    } else if json_message["Type"].to_string() == "HDTrailers" {
        // try to grab the RSS feed itself
        let data: serde_json::Value = serde_json::from_str(
            &mk_lib_network::mk_lib_network::mk_data_from_url(
                "http://feeds.hd-trailers.net/hd-trailers".to_string(),
            )
            .await
            .unwrap(),
        )
        .unwrap();
        let an_array = data["rss"]["channel"]["item"].as_array().unwrap();
        for item in an_array.iter() {
            if (item["title"].to_string().contains("(Trailer")
                && option_config_json["Metadata"]["Trailer"]["Trailer"] == true)
                || (item["title"].to_string().contains("(Behind")
                    && option_config_json["Metadata"]["Trailer"]["Behind"] == true)
                || (item["title"].to_string().contains("(Clip")
                    && option_config_json["Metadata"]["Trailer"]["Clip"] == true)
                || (item["title"].to_string().contains("(Featurette")
                    && option_config_json["Metadata"]["Trailer"]["Featurette"] == true)
                || (item["title"].to_string().contains("(Carpool")
                    && option_config_json["Metadata"]["Trailer"]["Carpool"] == true)
            {
                let download_link = item["enclosure"]["@url"].to_string();
                // do NOT remove the header.....this is the SAVE location
                // TODO use image directory format
                let file_save_name = format!(
                    "/mediakraken/metadata/meta/trailer/{:?}",
                    download_link.rsplitn(1, "/")
                );
                // verify it doesn't exist in meta folder before downloading
                if !Path::new(&file_save_name).exists() {
                    mk_lib_network::mk_lib_network::mk_download_file_from_url(
                        download_link.to_string(),
                        &file_save_name.to_string(),
                    )
                    .await
                    .unwrap();
                }
            }
        }
    }
}

// #[derive(Debug, serde::Deserialize)]
// struct DigitalUPCNetRecord {
//     title: String,
//     year: String,
//     quality: String,
//     upc: String,
//     notes: Option<String>,
//     fah_id: Option<String>,
// }

// #[derive(Debug, serde::Deserialize)]
// struct UPCMasterNetRecord {
//     upc: String,
//     title: String,
//     description: Option<String>,
//     link: String,
//     notes: Option<String>,
//     upc_type: String,
//     year: String,
//     genres: String,
//     rated: String,
//     length: String,
//     added: String,
//     alt_upc: Option<String>,
//     nw_bluray_upc: Option<String>,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
            .await
            .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await
        .unwrap();
    let option_config_json: Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool_ro)
            .await
            .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkdownload")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkdownload", &rabbit_channel)
            .await
            .unwrap();

    let max_concurrent_downloads = env::var("MKDOWNLOAD_CONCURRENT_WORKERS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_MAX_CONCURRENT_DOWNLOADS);

    let worker_limit = Arc::new(Semaphore::new(max_concurrent_downloads));
    let option_config_json = Arc::new(option_config_json);
    let rabbit_channel = Arc::new(rabbit_channel);

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                let delivery_tag = msg.deliver.unwrap().delivery_tag();
                let worker_limit = Arc::clone(&worker_limit);
                let option_config_json = Arc::clone(&option_config_json);
                let rabbit_channel = Arc::clone(&rabbit_channel);

                tokio::spawn(async move {
                    if let Ok(_permit) = worker_limit.acquire_owned().await {
                        process_message(json_message, option_config_json.as_ref()).await;
                        let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                            &rabbit_channel,
                            delivery_tag,
                        )
                        .await;
                    }
                });
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
