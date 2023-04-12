#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::mk_lib_logging;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

pub async fn mk_lib_database_metadata_image_count(
    sqlx_pool: &sqlx::PgPool,
    class_id: i32,
) -> Result<i32, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (i32,) = sqlx::query_as(
        "select count(*) from mm_media \
        where mm_media_class_guid = $1",
    )
    .bind(class_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct MediaImageList {
    image_path: String,
}

pub async fn mk_lib_database_metadata_image_read(
    sqlx_pool: &sqlx::PgPool,
    class_id: i32,
    offset: i64,
    limit: i64,
) -> Result<Vec<MediaImageList>, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let select_query = sqlx::query(
        "select mm_media_path from mm_media \
        where mm_media_class_guid = $1 offset $2 limit $3",
    )
    .bind(class_id)
    .bind(offset)
    .bind(limit);
    let table_rows: Vec<MediaImageList> = select_query
        .map(|row: PgRow| MediaImageList {
            image_path: row.get("mm_media_path"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}
