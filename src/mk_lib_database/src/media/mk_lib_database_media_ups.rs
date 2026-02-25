use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

pub async fn mk_lib_database_media_upc_count(sqlx_pool: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("select count(*) from mm_bar_codes")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_media_upc_owned_count(
    sqlx_pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) =
        sqlx::query_as("select count(*) from mm_bar_code_own where mm_bar_code_own_user = $1")
            .bind(user_id)
            .fetch_one(sqlx_pool)
            .await?;
    Ok(row.0)
}
