#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaBookList {
    pub mm_metadata_book_guid: uuid::Uuid,
    pub mm_metadata_book_name: String,
}

pub async fn mk_lib_database_media_book_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaBookList>, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book, mm_media \
            where mm_media_metadata_guid = mm_metadata_book_guid \
            and mm_metadata_book_name % $1 \
            order by LOWER(mm_metadata_book_name) \
            offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book, mm_media \
            where mm_media_metadata_guid = mm_metadata_book_guid \
            order by LOWER(mm_metadata_book_name) \
            offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBMediaBookList> = select_query
        .map(|row: PgRow| DBMediaBookList {
            mm_metadata_book_guid: row.get("mm_metadata_book_guid"),
            mm_metadata_book_name: row.get("mm_metadata_book_name"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_book_count(
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
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_book, \
            mm_media where mm_media_metadata_guid = mm_metadata_book_guid \
            and mm_metadata_book_name % $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_book, \
            mm_media where mm_media_metadata_guid = mm_metadata_book_guid",
        )
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}
