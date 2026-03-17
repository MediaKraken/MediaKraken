use chrono::prelude::*;
use mk_lib_database;
use mk_lib_rabbitmq;
use std::error::Error;
use tokio::time::{Duration, MissedTickBehavior, interval};

fn cron_schedule_to_duration(schedule_type: &str, schedule_time: i16) -> chrono::Duration {
    match schedule_type {
        "Week(s)" => chrono::Duration::weeks(schedule_time.into()),
        "Day(s)" => chrono::Duration::days(schedule_time.into()),
        "Hour(s)" => chrono::Duration::hours(schedule_time.into()),
        "Minute(s)" => chrono::Duration::minutes(schedule_time.into()),
        _ => chrono::Duration::seconds(schedule_time.into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkcron").await?;

    // start loop for cron checks
    let mut ticker = interval(Duration::from_secs(60));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        ticker.tick().await;
        let cron_rows =
            mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_service_read(&sqlx_pool_ro)
                .await?;

        let now = Utc::now();

        for row_data in cron_rows {
            let time_delta = cron_schedule_to_duration(
                row_data.mm_cron_schedule_type.as_str(),
                row_data.mm_cron_schedule_time,
            );
            let date_check: DateTime<Utc> = now - time_delta;

            if row_data.mm_cron_last_run >= date_check {
                continue;
            }

            let Some(route_key) = row_data.mm_cron_json["route_key"].as_str() else {
                eprintln!(
                    "Skipping cron job {} due to missing route_key",
                    row_data.mm_cron_guid
                );
                continue;
            };

            mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                rabbit_channel.clone(),
                route_key,
                row_data.mm_cron_json.to_string(),
            )
            .await?;

            mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_time_update(
                &sqlx_pool_rw,
                row_data.mm_cron_guid,
            )
            .await?;
        }
    }
}
