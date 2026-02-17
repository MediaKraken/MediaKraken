use chrono::prelude::*;
use mk_lib_database;
use mk_lib_rabbitmq;
use std::env;
use std::error::Error;
use tokio::time::{sleep, Duration, interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
            .await
            .unwrap();
    let _db_check = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(
        &sqlx_pool_ro,
        false,
    )
    .await
    .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkcron")
            .await
            .unwrap();

    // start loop for cron checks
    let mut ticker = interval(Duration::from_secs(60));
    loop {
        ticker.tick().await;
        let cron_row =
            mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_service_read(&sqlx_pool_ro)
                .await
                .unwrap();
        for row_data in cron_row {
            let time_delta = match row_data.mm_cron_schedule_type.as_str() {
                "Week(s)" => chrono::Duration::weeks(row_data.mm_cron_schedule_time.into()),
                "Day(s)" => chrono::Duration::days(row_data.mm_cron_schedule_time.into()),
                "Hour(s)" => chrono::Duration::hours(row_data.mm_cron_schedule_time.into()),
                "Minute(s)" => chrono::Duration::minutes(row_data.mm_cron_schedule_time.into()),
                _ => chrono::Duration::seconds(row_data.mm_cron_schedule_time.into()),
            };
            let date_check: DateTime<Utc> = Utc::now() - time_delta;
            if row_data.mm_cron_last_run < date_check {
                mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                    rabbit_channel.clone(),
                    row_data.mm_cron_json["route_key"].as_str().unwrap(),
                    row_data.mm_cron_json.to_string(),
                )
                .await
                .unwrap();
                mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_time_update(
                    &sqlx_pool_rw,
                    row_data.mm_cron_guid,
                )
                .await?;
            }
        }
    }
}
