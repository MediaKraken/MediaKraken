use chrono::prelude::*;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use mk_lib_common;
use mk_lib_database;
use mk_lib_file;
use mk_lib_rabbitmq;
use num_format::{Locale, ToFormattedString};
use serde_json::{json, Value};
use std::error::Error;
use std::ffi::OsStr;g
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use radiobrowser::{RadioBrowserAPI, StationOrder};

lazy_static! {
    static ref epoch: DateTime<Utc> = DateTime::<Utc>::from(UNIX_EPOCH);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
        .await
        .unwrap();
    let _result =
        mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
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

    // Search for stations by name.
    // Change "jazz" to whatever you want.
    let stations = api
        .get_stations()
        .name("jazz")
        .order(StationOrder::Clickcount)
        .reverse(true)
        .limit(10)
        .send()
        .await?;

    if stations.is_empty() {
        println!("No stations found.");
        return Ok(());
    }

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
    }
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
