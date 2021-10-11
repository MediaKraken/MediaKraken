use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
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
    let mut transaction = pool.begin().await?;
    sqlx::query("update mm_cron_jobs set mm_cron_last_run = NOW() \
        where mm_cron_guid = $1")
        .bind(cron_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_cron_delete(pool: &sqlx::PgPool,
                                         cron_uuid: uuid::Uuid)
                                         -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("delete from mm_cron where mm_cron_guid = $1")
        .bind(cron_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
/*

async def db_cron_insert(self, cron_name, cron_desc, cron_enabled,
                         cron_schedule, cron_last_run, cron_json, db_connection=None):
    """
    insert cron job
    """
    new_cron_id = uuid.uuid4()
    await db_conn.execute('insert into mm_cron (mm_cron_guid,'
                          ' mm_cron_name,'
                          ' mm_cron_description,'
                          ' mm_cron_enabled,'
                          ' mm_cron_schedule,'
                          ' mm_cron_last_run, mm_cron_json)'
                          ' values ($1,$2,$3,$4,$5,$6,$7)',
                          new_cron_id, cron_name, cron_desc,
                          cron_enabled, cron_schedule,
                          cron_last_run, cron_json)
    return new_cron_id

*/

pub async fn mk_lib_database_cron_count(pool: &sqlx::PgPool,
                                        cron_enabled: bool)
                                        -> Result<(i32), sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select count(*) from mm_cron where mm_cron_enabled = $1")
        .bind(cron_enabled)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}
