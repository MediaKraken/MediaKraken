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
            if row_data.mm_cron_schedule_type == "Week(s)" {
                time_delta = chrono::Duration::weeks(i64::from(row_data.mm_cron_schedule_time));
            } else if row_data.mm_cron_schedule_type == "Day(s)" {
                time_delta = chrono::Duration::days(i64::from(row_data.mm_cron_schedule_time));
            } else if row_data.mm_cron_schedule_type == "Hour(s)" {
                time_delta = chrono::Duration::hours(i64::from(row_data.mm_cron_schedule_time));
            } else if row_data.mm_cron_schedule_type == "Minute(s)" {
                time_delta = chrono::Duration::minutes(i64::from(row_data.mm_cron_schedule_time));
            } else {
                time_delta = chrono::Duration::seconds(i64::from(row_data.mm_cron_schedule_time));
            }
            let date_check: DateTime<Utc> = Utc::now() - time_delta;
            if row_data.mm_cron_last_run < date_check {
                rabbit_exchange.publish(Publish::with_properties("hello there".as_bytes(),
                                                                 row_data.mm_cron_json["route_key"].to_string(),
                                                                 AmqpProperties::default().with_delivery_mode(2).with_content_type("text/plain".to_string())))?;
                mk_lib_database_cron::mk_lib_database_cron_time_update(&sqlx_pool,
                                                                       row_data.mm_cron_guid).await?;
            }
        }
        sleep(Duration::from_secs(60)).await;
    }
}