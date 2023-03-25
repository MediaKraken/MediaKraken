#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;
use serde_json::json;

pub async fn mk_lib_database_media_game_clone_read(
    sqlx_pool: &sqlx::PgPool,
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
    let rows: Vec<PgRow> = sqlx::query(
        "select gi_id, gi_cloneof from mm_metadata_game_software_info \
        where gi_system_id is null and gi_cloneof IS NOT NULL and gi_gc_category is null",
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

pub async fn mk_lib_database_media_category_by_name(
    sqlx_pool: &sqlx::PgPool,
    category_name: String,
) -> Result<PgRow, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: PgRow = sqlx::query(
        "select gi_gc_category from mm_metadata_game_software_info \
        where gi_short_name = $1",
    )
    .bind(category_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row)
}

pub async fn mk_lib_database_media_game_category_update(
    sqlx_pool: &sqlx::PgPool,
    category: String,
    game_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query("update mm_metadata_game_software_info set gi_gc_category = $1 where gi_id = $2")
        .bind(category)
        .bind(game_id)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaGameList {
    pub mm_metadata_game_guid: uuid::Uuid,
}

pub async fn mk_lib_database_media_game_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i32,
    limit: i32,
) -> Result<Vec<DBMediaGameList>, sqlx::Error> {
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
        select_query = sqlx::query("").bind(search_value).bind(offset).bind(limit);
    } else {
        select_query = sqlx::query("").bind(offset).bind(limit);
    }
    let table_rows: Vec<DBMediaGameList> = select_query
        .map(|row: PgRow| DBMediaGameList {
            mm_metadata_game_guid: row.get("mm_metadata_game_guid"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_game_count(
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
        let row: (i64,) = sqlx::query_as("")
            .bind(search_value)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_metadata_game_software_info")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}
