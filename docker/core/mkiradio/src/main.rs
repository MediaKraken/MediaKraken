use chrono::prelude::*;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use mk_lib_common;
use mk_lib_database;
use mk_lib_file;
use mk_lib_rabbitmq;
use num_format::{Locale, ToFormattedString};
use radiobrowser::{RadioBrowserAPI, StationOrder};
use serde_json::{Value, json};
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

lazy_static! {
    static ref epoch: DateTime<Utc> = DateTime::<Utc>::from(UNIX_EPOCH);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
            .await
            .unwrap();
    let _result = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(
        &sqlx_pool_ro,
        false,
    )
    .await;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkiradio")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkiradio", &rabbit_channel)
            .await
            .unwrap();

    while let Some(msg) = rabbit_consumer.recv().await {
        if let Some(payload) = msg.content {
            let json_message: Value =
                serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
            println!(" [x] Received {:?}", json_message);
            if json_message["Type"] == "radiobrowser" {
                let mut api = RadioBrowserAPI::new().await?;

                // Fetch all stations instead of filtering by a specific genre/name.
                let stations = api
                    .get_stations()
                    .order(StationOrder::Clickcount)
                    .reverse(true)
                    .send()
                    .await?;

                if stations.is_empty() {
                    println!("No stations found.");
                    return Ok(());
                }

                let mut upsert_count: usize = 0;
                for (idx, station) in stations.iter().enumerate() {
                    println!("{}. {}", idx + 1, station.name);
                    println!("   Station UUID: {}", station.stationuuid);

                    if let Some(country) = &station.country {
                        println!("   Country: {}", country);
                    }

                    if let Some(language) = &station.language {
                        println!("   Language: {}", language);
                    }

                    if let Some(tags) = &station.tags {
                        println!("   Tags: {}", tags);
                    }

                    println!("   Stream URL: {}", station.url_resolved);
                    println!();

                    mk_lib_database::media::mk_lib_database_media_iradio::mk_lib_database_media_iradio_upsert(
                        &sqlx_pool_rw,
                        station.stationuuid.as_ref(),
                        station.name.as_ref(),
                        station.url.as_ref(),
                        station.country.as_deref(),
                        station.language.as_deref(),
                        station.tags.as_deref(),
                        station.url_resolved.as_ref(),
                    )
                    .await?;
                    upsert_count += 1;
                }
                println!("Upserted {} stations into mm_radio.", upsert_count);
            }
            let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                &rabbit_channel,
                msg.deliver.unwrap().delivery_tag(),
            )
            .await;
        }
    }
    Ok(())
}
