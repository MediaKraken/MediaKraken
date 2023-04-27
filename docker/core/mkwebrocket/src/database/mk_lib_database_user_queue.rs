#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::mk_lib_logging;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

pub async fn mk_lib_database_meta_queue_count(
    sqlx_pool: &sqlx::PgPool,
    user_uuid: Uuid,
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
            "select count(*) from mm_user_queue \
            where mm_user_queue_name % $1 and mm_user_queue_user_id = $2",
        )
        .bind(search_value)
        .bind(user_uuid)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_user_queue \
            where mm_user_queue_user_id = $1",
        )
        .bind(user_uuid)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}