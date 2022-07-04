#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::{types::Uuid, types::Json};
use sqlx::postgres::PgRow;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaAnimeList {
	mm_metadata_anime_guid: uuid::Uuid,
	mm_metadata_anime_name: String,
}


pub async fn mk_lib_database_media_anime_read(pool: &sqlx::PgPool,
                                              search_value: String,
                                              offset: i32, limit: i32)
                                              -> Result<Vec<DBMediaAnimeList>, sqlx::Error> {
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

pub async fn mk_lib_database_media_anime_count(pool: &sqlx::PgPool,
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