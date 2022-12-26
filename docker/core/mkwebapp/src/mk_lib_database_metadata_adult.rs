#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetadataAdultList {
    mm_metadata_adult_guid: uuid::Uuid,
    mm_metadata_adult_name: String,
}

pub async fn mk_lib_database_metadata_adult_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i32,
    limit: i32,
) -> Result<Vec<DBMetadataAdultList>, sqlx::Error> {
    if search_value != "" {
        let rows = sqlx::query("")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await?;
        Ok(rows)
    } else {
        let rows = sqlx::query("")
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await?;
        Ok(rows)
    }
}

pub async fn mk_lib_database_metadata_adult_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if search_value != "" {
        let row: (i64,) = sqlx::query("").bind(search_value).fetch_one(sqlx_pool).await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query("").fetch_one(sqlx_pool).await?;
        Ok(row.0)
    }
}
