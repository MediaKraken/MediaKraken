#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBNotificationList {
    mm_notification_guid: uuid::Uuid,
    mm_notification_text: String,
    mm_notification_time: String,
    mm_notification_dismissible: String,
}

pub async fn mk_lib_database_notification_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i32,
    limit: i32,
) -> Result<Vec<DBNotificationList>, sqlx::Error> {
    let select_query = sqlx::query(
        "select mm_notification_guid, mm_notification_text, \
        mm_notification_time, \
        mm_notification_dismissible from mm_notification \
        order by mm_notification_time desc offset $1 limit $2",
    )
    .bind(offset)
    .bind(limit);
    let table_rows: Vec<DBNotificationList> = select_query
        .map(|row: PgRow| DBNotificationList {
            mm_notification_guid: row.get("mm_notification_guid"),
            mm_notification_text: row.get("mm_notification_text"),
            mm_notification_time: row.get("mm_notification_time"),
            mm_notification_dismissible: row.get("mm_notification_dismissible"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_notification_insert(
    sqlx_pool: &sqlx::PgPool,
    mm_notification_text: String,
    mm_notification_dismissable: bool,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_notification (mm_notification_guid, \
        mm_notification_text, \
        mm_notification_time = NOW(), \
        mm_notification_dismissible) \
        values ($1, $2, $3)",
    )
    .bind(Uuid::new_v4())
    .bind(mm_notification_text)
    .bind(mm_notification_dismissable)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_notification_delete(
    sqlx_pool: &sqlx::PgPool,
    mk_notification_guid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query("delete from mm_notification where mm_notification_guid = $1")
        .bind(mk_notification_guid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
