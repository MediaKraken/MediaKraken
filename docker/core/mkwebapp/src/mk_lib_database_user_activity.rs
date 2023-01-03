#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_activity_insert(
    sqlx_pool: &sqlx::PgPool,
    activity_name: String,
    activity_overview: String,
    activity_short_overview: String,
    activity_type: String,
    activity_itemid: Uuis,
    activity_userid: Uuid,
    activity_severity: String,
) -> Result<Uuid, sqlx::Error> {
    new_guid = Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_user_activity (mm_activity_guid, mm_activity_name, \
        mm_activity_overview, mm_activity_short_overview, \
        mm_activity_type, mm_activity_itemid, \
        mm_activity_userid, mm_activity_datecreated, \
        mm_activity_log_severity) \
        values ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
    )
    .bind(new_guid)
    .bind(activity_name)
    .bind(activity_overview)
    .bind(activity_short_overview)
    .bind(activity_type)
    .bind(activity_itemid)
    .bind(activity_userid)
    .bind(Utc::now())
    .bind(activity_log_severity)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_activity_delete(
    sqlx_pool: &sqlx::PgPool,
    day_range: i32,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "delete from mm_user_activity \
        where mm_activity_datecreated < now() - interval $1 day;",
    )
    .bind(day_range)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}
