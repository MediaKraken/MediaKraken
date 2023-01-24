#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;
use serde_json::json;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaAnimeList {
    mm_metadata_anime_guid: uuid::Uuid,
    mm_metadata_anime_name: String,
}

pub async fn mk_lib_database_media_anime_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i32,
    limit: i32,
) -> Result<Vec<DBMediaAnimeList>, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
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

pub async fn mk_lib_database_media_anime_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    if search_value != "" {
        let row: (i64,) = sqlx::query("")
            .bind(search_value)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query("").fetch_one(sqlx_pool).await?;
        Ok(row.0)
    }
}
