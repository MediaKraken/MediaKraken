use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use sqlx::types::Uuid;

pub async fn mk_lib_database_metadata_exists_upc(
    sqlx_pool: &sqlx::PgPool,
    upc_code: &i32,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        "select exists(select 1 from mm_bar_codes \
        where mm_bar_code_data = $1 limit 1) as found_record limit 1",
    )
    .bind(upc_code)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_exists_upc_own(
    sqlx_pool: &sqlx::PgPool,
    upc_code: &i32,
    user_id: i64,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        "select exists(select 1 from mm_bar_codes, mm_bar_code_own \
        where mm_bar_code_uuid = mm_bar_code_own_uuid \
        and mm_bar_code_data = $1 and mm_bar_code_own_user = $2 limit 1) as found_record limit 1",
    )
    .bind(upc_code)
    .bind(user_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_upc_insert(
    sqlx_pool: &sqlx::PgPool,
    upc_code: &i32,
    upc_code_type: &i8,
    data_json: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_bar_codes (mm_bar_code_uuid, \
        mm_bar_code_code, \
        mm_bar_code_type \
        mm_bar_code_json) \
        values ($1, $2, $3, $4)",
    )
    .bind(new_guid)
    .bind(upc_code)
    .bind(upc_code_type)
    .bind(data_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_upc_own_insert(
    sqlx_pool: &sqlx::PgPool,
    upc_uuid: Uuid,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_bar_codes (mm_bar_code_own_uuid, \
        mm_bar_code_own_user) \
        values ($1, $2)",
    )
    .bind(upc_uuid)
    .bind(user_id)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}