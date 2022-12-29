#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use std::num::NonZeroU8;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBCronList {
    pub mm_cron_guid: uuid::Uuid,
    mm_cron_name: String,
    mm_cron_description: String,
    mm_cron_enabled: bool,
    pub mm_cron_schedule_type: String,
    pub mm_cron_schedule_time: i16,
    pub mm_cron_last_run: DateTime<Utc>,
    pub mm_cron_json: serde_json::Value,
}

pub async fn mk_lib_database_cron_service_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBCronList>, sqlx::Error> {
    let select_query = sqlx::query(
        "select mm_cron_guid, \
        mm_cron_name, mm_cron_description, mm_cron_enabled, \
        mm_cron_schedule_type, mm_cron_schedule_time, \
        mm_cron_last_run, mm_cron_json from mm_cron_jobs \
        order by mm_cron_name",
    );
    let table_rows: Vec<DBCronList> = select_query
        .map(|row: PgRow| DBCronList {
            mm_cron_guid: row.get("mm_cron_guid"),
            mm_cron_name: row.get("mm_cron_name"),
            mm_cron_description: row.get("mm_cron_description"),
            mm_cron_enabled: row.get("mm_cron_enabled"),
            mm_cron_schedule_type: row.get("mm_cron_schedule_type"),
            mm_cron_schedule_time: row.get("mm_cron_schedule_time"),
            mm_cron_last_run: row.get("mm_cron_last_run"),
            mm_cron_json: row.get("mm_cron_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_cron_time_update(
    sqlx_pool: &sqlx::PgPool,
    cron_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "update mm_cron_jobs set mm_cron_last_run = NOW() \
        where mm_cron_guid = $1",
    )
    .bind(cron_uuid)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_cron_delete(
    sqlx_pool: &sqlx::PgPool,
    cron_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query("delete from mm_cron where mm_cron_guid = $1")
        .bind(cron_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_cron_insert(
    sqlx_pool: &sqlx::PgPool,
    cron_name: String,
    cron_desc: String,
    cron_enabled: bool,
    cron_schedule: String,
    cron_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_cron (mm_cron_guid, mm_cron_name, mm_cron_description, \
        mm_cron_enabled, mm_cron_schedule, mm_cron_last_run, mm_cron_json) \
        values ($1,$2,$3,$4,$5,Null,$6)",
    )
    .bind(new_guid)
    .bind(cron_name)
    .bind(cron_desc)
    .bind(cron_enabled)
    .bind(cron_schedule)
    .bind(cron_json)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_cron_count(
    sqlx_pool: &sqlx::PgPool,
    cron_enabled: bool,
) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as("select count(*) from mm_cron where mm_cron_enabled = $1")
        .bind(cron_enabled)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}
