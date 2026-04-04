use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBCronList {
    pub mm_cron_guid: uuid::Uuid,
    pub mm_cron_name: String,
    pub mm_cron_description: String,
    pub mm_cron_enabled: bool,
    pub mm_cron_schedule_type: String,
    pub mm_cron_schedule_time: i16,
    pub mm_cron_last_run: DateTime<Utc>,
    pub mm_cron_json: serde_json::Value,
}

pub async fn mk_lib_database_cron_service_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBCronList>, sqlx::Error> {
    let table_rows: Vec<DBCronList> = sqlx::query_as(
        r#"select mm_cron_guid, mm_cron_name, mm_cron_description, mm_cron_enabled, 
        mm_cron_schedule_type, mm_cron_schedule_time, mm_cron_last_run, 
        mm_cron_json from mm_cron_jobs order by mm_cron_name"#,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_cron_service_json(
    sqlx_pool: &sqlx::PgPool,
    cron_uuid: Uuid,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) =
        sqlx::query_as(r#"select mm_cron_json from mm_cron_jobs where mm_cron_guid = $1"#)
            .bind(cron_uuid)
            .fetch_one(sqlx_pool)
            .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_cron_time_update(
    sqlx_pool: &sqlx::PgPool,
    cron_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(r#"update mm_cron_jobs set mm_cron_last_run = NOW() where mm_cron_guid = $1"#)
        .bind(cron_uuid)
        .execute(sqlx_pool)
        .await?;
    Ok(())
}

pub async fn mk_lib_database_cron_delete(
    sqlx_pool: &sqlx::PgPool,
    cron_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(r#"delete from mm_cron where mm_cron_guid = $1"#)
        .bind(cron_uuid)
        .execute(sqlx_pool)
        .await?;
    Ok(())
}

pub async fn mk_lib_database_cron_insert(
    sqlx_pool: &sqlx::PgPool,
    cron_name: String,
    cron_desc: String,
    cron_enabled: bool,
    cron_schedule_type: String,
    cron_json: serde_json::Value,
    cron_scedule_time: i16,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    sqlx::query(
        r#"insert into mm_cron (mm_cron_guid, mm_cron_name, mm_cron_description, 
        mm_cron_enabled, mm_cron_schedule_type, mm_cron_last_run, mm_cron_json, 
        mm_cron_schedule_time) 
        values ($1,$2,$3,$4,$5,Null,$6,$7)"#,
    )
    .bind(new_guid)
    .bind(cron_name)
    .bind(cron_desc)
    .bind(cron_enabled)
    .bind(cron_schedule_type)
    .bind(cron_json)
    .bind(cron_scedule_time)
    .execute(sqlx_pool)
    .await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_cron_count(
    sqlx_pool: &sqlx::PgPool,
    cron_enabled: bool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_cron where mm_cron_enabled = $1"#)
        .bind(cron_enabled)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}
