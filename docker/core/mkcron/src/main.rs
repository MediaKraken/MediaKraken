#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use amiquip::{AmqpProperties, Connection, Exchange, Publish, Result};
use chrono::prelude::*;
use sqlx::Row;
use std::error::Error;
use stdext::function_name;
use serde_json::json;
use tokio::time::{sleep, Duration};

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
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}))
            .await
            .unwrap();
    }

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool(1).await.unwrap();
    let _db_check = mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();

    // open rabbit connection
    let mut rabbit_connection =
        Connection::insecure_open("amqp://guest:guest@mkstack_rabbitmq:5672")?;
    // Open a channel - None says let the library choose the channel ID.
    let rabbit_channel = rabbit_connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let rabbit_exchange = Exchange::direct(&rabbit_channel);

    // start loop for cron checks
    loop {
        let cron_row = mk_lib_database_cron::mk_lib_database_cron_service_read(&sqlx_pool)
            .await
            .unwrap();
        for row_data in cron_row {
            let time_delta: chrono::Duration;
            match row_data.mm_cron_schedule_type.as_str() {
                "Week(s)" => {
                    time_delta = chrono::Duration::weeks(i64::from(row_data.mm_cron_schedule_time))
                }
                "Day(s)" => {
                    time_delta = chrono::Duration::days(i64::from(row_data.mm_cron_schedule_time))
                }
                "Hour(s)" => {
                    time_delta = chrono::Duration::hours(i64::from(row_data.mm_cron_schedule_time))
                }
                "Minute(s)" => {
                    time_delta =
                        chrono::Duration::minutes(i64::from(row_data.mm_cron_schedule_time))
                }
                _ => {
                    time_delta =
                        chrono::Duration::seconds(i64::from(row_data.mm_cron_schedule_time))
                }
            }
            let date_check: DateTime<Utc> = Utc::now() - time_delta;
            if row_data.mm_cron_last_run < date_check {
                rabbit_exchange.publish(Publish::with_properties(
                    row_data.mm_cron_json.to_string().as_bytes(),
                    row_data.mm_cron_json["route_key"].to_string(),
                    AmqpProperties::default()
                        .with_delivery_mode(2)
                        .with_content_type("text/plain".to_string()),
                ))?;
                mk_lib_database_cron::mk_lib_database_cron_time_update(
                    &sqlx_pool,
                    row_data.mm_cron_guid,
                )
                .await?;
            }
        }
        sleep(Duration::from_secs(60)).await;
    }
}
