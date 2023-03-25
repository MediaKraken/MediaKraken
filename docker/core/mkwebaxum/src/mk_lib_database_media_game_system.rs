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
pub struct DBMediaBookList {
    pub mm_metadata_book_guid: uuid::Uuid,
    pub mm_metadata_book_name: String,
}

pub async fn mk_lib_database_media_game_system_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i32,
    limit: i32,
) -> Result<Vec<PgRow>, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // TODO this should only return systems where there are games for it since it's "media"
    if search_value != "" {
        let rows = sqlx::query(
            "select gs_game_system_id, \
            gs_game_system_name, \
            gs_game_system_alias, \
            gs_game_system_localimage->>'Poster' \
            from mm_metadata_game_systems_info \
            where gs_game_system_name = $1 \
            offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await?;
        Ok(rows)
    } else {
        let rows = sqlx::query(
            "select gs_game_system_id, \
            gs_game_system_name, \
            gs_game_system_alias, \
            gs_game_system_localimage->>'Poster' \
            from mm_metadata_game_systems_info \
            offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await?;
        Ok(rows)
    }
}

pub async fn mk_lib_database_media_game_system_count(
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
        let row: (i64,) = sqlx::query(
            "select count(*) from mm_metadata_game_systems_info where gs_game_system_name = $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query("select count(*) from mm_metadata_game_systems_info")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}
