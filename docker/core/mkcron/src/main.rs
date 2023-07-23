use chrono::prelude::*;
use mk_lib_database;
use mk_lib_rabbitmq;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    let _db_check =
        mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
            .await
            .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkstack_rabbitmq", "mkcron")
            .await
            .unwrap();

    // start loop for cron checks
    loop {
        let cron_row =
            mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_service_read(&sqlx_pool)
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
                mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                    rabbit_channel.clone(),
                    row_data.mm_cron_json["route_key"].as_str().unwrap(),
                    row_data.mm_cron_json.to_string(),
                )
                .await
                .unwrap();
                mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_time_update(
                    &sqlx_pool,
                    row_data.mm_cron_guid,
                )
                .await?;
            }
        }
        sleep(Duration::from_secs(60)).await;
    }
}
