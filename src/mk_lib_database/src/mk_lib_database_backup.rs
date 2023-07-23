use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use stdext::function_name;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBBackupList {
    pub mm_backup_guid: uuid::Uuid,
    pub mm_backup_description: String,
    pub mm_backup_location_type: i16,
    pub mm_backup_location: String,
    pub mm_backup_created: DateTime<Utc>,
}

pub async fn mk_lib_database_backup_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBBackupList>, sqlx::Error> {
    let select_query = sqlx::query(
        "select mm_backup_guid, mm_backup_description, \
            mm_backup_location_type, mm_backup_location, mm_backup_created \
            from mm_backup \
            order by mm_backup_created desc \
            offset $1 limit $2",
    )
    .bind(offset)
    .bind(limit);
    let table_rows: Vec<DBBackupList> = select_query
        .map(|row: PgRow| DBBackupList {
            mm_backup_guid: row.get("mm_backup_guid"),
            mm_backup_description: row.get("mm_backup_description"),
            mm_backup_location_type: row.get("mm_backup_location_type"),
            mm_backup_location: row.get("mm_backup_location"),
            mm_backup_created: row.get("mm_backup_created"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_backup_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("select count(*) from mm_backup")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}
