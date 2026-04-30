use chrono::prelude::*;
use serde_json::json;
use std::error::Error;
use tokio::signal;
use tokio::time::{Duration, MissedTickBehavior, interval};

fn cron_schedule_to_duration(schedule_type: &str, schedule_time: i16) -> Option<chrono::Duration> {
    if schedule_time < 0 {
        return None;
    }
    let n = i64::from(schedule_time);
    match schedule_type {
        "Week(s)" => Some(chrono::Duration::weeks(n)),
        "Day(s)" => Some(chrono::Duration::days(n)),
        "Hour(s)" => Some(chrono::Duration::hours(n)),
        "Minute(s)" => Some(chrono::Duration::minutes(n)),
        "Second(s)" => Some(chrono::Duration::seconds(n)),
        _ => None,
    }
}

async fn log_event(payload: serde_json::Value) {
    if let Err(err) =
        mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(payload).await
    {
        eprintln!("mkcron: loki push failed: {err}");
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkcron").await?;

    let mut ticker = interval(Duration::from_secs(60));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = shutdown_signal() => {
                log_event(json!({
                    "Module": std::module_path!(),
                    "Event": "shutdown",
                }))
                .await;
                return Ok(());
            }
            _ = ticker.tick() => {
                let cron_rows = match mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_service_read(&sqlx_pool_ro).await {
                    Ok(rows) => rows,
                    Err(err) => {
                        log_event(json!({
                            "Module": std::module_path!(),
                            "Event": "cron_read_failed",
                            "Error": err.to_string(),
                        }))
                        .await;
                        continue;
                    }
                };

                let now = Utc::now();

                for row_data in cron_rows {
                    if !row_data.mm_cron_enabled {
                        continue;
                    }

                    let Some(time_delta) = cron_schedule_to_duration(
                        row_data.mm_cron_schedule_type.as_str(),
                        row_data.mm_cron_schedule_time,
                    ) else {
                        log_event(json!({
                            "Module": std::module_path!(),
                            "Event": "invalid_schedule",
                            "Guid": row_data.mm_cron_guid,
                            "ScheduleType": row_data.mm_cron_schedule_type,
                            "ScheduleTime": row_data.mm_cron_schedule_time,
                        }))
                        .await;
                        continue;
                    };

                    let date_check: DateTime<Utc> = now - time_delta;

                    if let Some(last_run) = row_data.mm_cron_last_run
                        && last_run >= date_check
                    {
                        continue;
                    }

                    let Some(route_key) = row_data.mm_cron_json["route_key"].as_str() else {
                        log_event(json!({
                            "Module": std::module_path!(),
                            "Event": "missing_route_key",
                            "Guid": row_data.mm_cron_guid,
                        }))
                        .await;
                        continue;
                    };

                    if let Err(err) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                        rabbit_channel.clone(),
                        route_key,
                        row_data.mm_cron_json.to_string(),
                    )
                    .await
                    {
                        log_event(json!({
                            "Module": std::module_path!(),
                            "Event": "publish_failed",
                            "Guid": row_data.mm_cron_guid,
                            "RouteKey": route_key,
                            "Error": err.to_string(),
                        }))
                        .await;
                        continue;
                    }

                    if let Err(err) = mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_time_update(
                        &sqlx_pool_rw,
                        row_data.mm_cron_guid,
                    )
                    .await
                    {
                        log_event(json!({
                            "Module": std::module_path!(),
                            "Event": "last_run_update_failed",
                            "Guid": row_data.mm_cron_guid,
                            "Error": err.to_string(),
                        }))
                        .await;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::cron_schedule_to_duration;

    #[test]
    fn converts_known_units() {
        assert_eq!(
            cron_schedule_to_duration("Week(s)", 2),
            Some(chrono::Duration::weeks(2))
        );
        assert_eq!(
            cron_schedule_to_duration("Day(s)", 3),
            Some(chrono::Duration::days(3))
        );
        assert_eq!(
            cron_schedule_to_duration("Hour(s)", 5),
            Some(chrono::Duration::hours(5))
        );
        assert_eq!(
            cron_schedule_to_duration("Minute(s)", 15),
            Some(chrono::Duration::minutes(15))
        );
        assert_eq!(
            cron_schedule_to_duration("Second(s)", 30),
            Some(chrono::Duration::seconds(30))
        );
    }

    #[test]
    fn rejects_unknown_unit() {
        assert_eq!(cron_schedule_to_duration("Fortnight(s)", 1), None);
        assert_eq!(cron_schedule_to_duration("", 1), None);
    }

    #[test]
    fn rejects_negative_time() {
        assert_eq!(cron_schedule_to_duration("Minute(s)", -1), None);
        assert_eq!(cron_schedule_to_duration("Week(s)", i16::MIN), None);
    }

    #[test]
    fn zero_is_allowed() {
        assert_eq!(
            cron_schedule_to_duration("Hour(s)", 0),
            Some(chrono::Duration::zero())
        );
    }
}
