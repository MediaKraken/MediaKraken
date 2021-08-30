use sqlx::postgres::PgRow;
use sqlx::types::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::num::NonZeroU8;

pub async fn mk_lib_database_cron_service_read(pool: &sqlx::PgPool)
                                               -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_cron_guid, \
        mm_cron_schedule_type, mm_cron_schedule_time, \
        mm_cron_last_run, mm_cron_json from mm_cron_jobs \
        where mm_cron_enabled = true")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_cron_time_update(pool: &sqlx::PgPool,
                                              cron_uuid: uuid::Uuid)
                                              -> Result<(), sqlx::Error> {
    sqlx::query("update mm_cron_jobs set mm_cron_last_run = NOW() \
        where mm_cron_guid = $1")
        .bind(cron_uuid)
        .execute(pool)
        .await?;
    Ok(())
}

// // cargo test -- --show-output
// #[cfg(test)]
// mod test_mk_lib_common {
//     use super::*;
//
//     macro_rules! aw {
//     ($e:expr) => {
//         tokio_test::block_on($e)
//     };
//   }
// }