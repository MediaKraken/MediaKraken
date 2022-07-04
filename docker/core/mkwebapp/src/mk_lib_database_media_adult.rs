#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::{types::Uuid, types::Json};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaAdultList {
	mm_metadata_adult_guid: uuid::Uuid,
	mm_metadata_adult_name: String,
}

pub async fn mk_lib_database_media_adult_read(pool: &sqlx::PgPool,
                                              search_value: String,
                                              offset: i32, limit: i32)
                                              -> Result<Vec<DBMediaAdultList>, sqlx::Error> {
    if search_value != "" {
        let rows = sqlx::query("")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    } else {
        let rows = sqlx::query("")
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

pub async fn mk_lib_database_media_adult_count(pool: &sqlx::PgPool,
                                               search_value: String)
                                               -> Result<i64, sqlx::Error> {
    if search_value != "" {
        let row: (i64, ) = sqlx::query("")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i64, ) = sqlx::query("")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}