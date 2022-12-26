#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_meta_queue_count(
    sqlx_pool: &sqlx::PgPool,
    user_uuid: Uuid,
    search_value: String,
) -> Result<i32, sqlx::Error> {
    if search_value != "" {
        let row: (i32,) = sqlx::query_as(
            "select count(*) from mm_user_queue \
            where mm_user_queue_name % $1 and mm_user_queue_user_id = $2",
        )
        .bind(search_value)
        .bind(user_uuid)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i32,) = sqlx::query_as(
            "select count(*) from mm_user_queue \
            where mm_user_queue_user_id = $1",
        )
        .bind(user_uuid)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}
