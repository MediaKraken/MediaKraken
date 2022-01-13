use amiquip::{AmqpProperties, Connection, Exchange, Publish, Result};
use chrono::prelude::*;
use std::error::Error;
use tokio::time::{Duration, sleep};
use serde_json::json;
use sqlx::Row;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_cron.rs"]
mod mk_lib_database_cron;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkcron";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           false).await;

    // open rabbit connection
    let mut rabbit_connection = Connection::insecure_open(
        "amqp://guest:guest@mkstack_rabbitmq:5672")?;
    // Open a channel - None says let the library choose the channel ID.
    let rabbit_channel = rabbit_connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let rabbit_exchange = Exchange::direct(&rabbit_channel);

    // start loop for cron checks
    loop {
        let cron_row = mk_lib_database_cron::mk_lib_database_cron_service_read(&sqlx_pool).await.unwrap();
        for row_data in cron_row {
            let mut time_delta: chrono::Duration;
            //println!("row_data: {}", row_data);
            let cron_schedule: String = row_data.get("mm_cron_schedule_type");
            let cron_timespan: i64 = row_data.get("mm_cron_schedule_time");
            if cron_schedule == "Week(s)" {
                time_delta = chrono::Duration::weeks(cron_timespan);
            } else if cron_schedule == "Day(s)" {
                time_delta = chrono::Duration::days(cron_timespan);
            } else if cron_schedule == "Hour(s)" {
                time_delta = chrono::Duration::hours(cron_timespan);
            } else if cron_schedule == "Minute(s)" {
                time_delta = chrono::Duration::minutes(cron_timespan);
            } else {
                time_delta = chrono::Duration::seconds(cron_timespan);
            }
            let date_check: DateTime<Utc> = Utc::now() - time_delta;
            let last_run: DateTime<Utc> = row_data.get("mm_cron_last_run");
            println!("date_check: {:?}, {:?}", date_check, last_run);
            if last_run < date_check {
                let cron_json: serde_json::Value = row_data.try_get("mm_cron_json")?;
                rabbit_exchange.publish(Publish::with_properties("hello there".as_bytes(),
                                                                 cron_json["route_key"].to_string(),
                                                                 AmqpProperties::default().with_delivery_mode(2).with_content_type("text/plain".to_string())))?;
                mk_lib_database_cron::mk_lib_database_cron_time_update(&sqlx_pool,
                                                                       row_data.get("mm_cron_guid")).await?;
            }
            let uuid_cron: uuid::Uuid = row_data.get("mm_cron_guid");
            mk_lib_logging::mk_logging_post_elk("info",
                                                json!({"UUID": &uuid_cron.to_string()}),
                                                LOGGING_INDEX_NAME).await;
        }
        sleep(Duration::from_secs(60)).await;
    }
}