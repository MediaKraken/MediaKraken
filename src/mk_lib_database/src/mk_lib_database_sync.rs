use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_sync_delete(
    sqlx_pool: &sqlx::PgPool,
    sync_guid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query("delete from mm_media_sync where mm_sync_guid = $1")
        .bind(sync_guid)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_sync_process_update(
    sqlx_pool: &sqlx::PgPool,
    sync_guid: Uuid,
    sync_percent: f32,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "update mm_media_sync set mm_sync_options_json->'Progress' = $1
        where mm_sync_guid = $2",
    )
    .bind(sync_percent)
    .bind(sync_guid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_sync_count(sqlx_pool: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("select count(*) from mm_media_sync")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_sync_insert(
    sqlx_pool: &sqlx::PgPool,
    sync_path: String,
    sync_path_to: String,
    sync_json: serde_json::Value,
) -> Result<Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_media_sync (mm_sync_guid, mm_sync_path, \
        mm_sync_path_to, mm_sync_options_json) \
        values ($1, $2, $3, $4)",
    )
    .bind(new_guid)
    .bind(sync_path)
    .bind(sync_path_to)
    .bind(sync_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBSyncList {
    pub mm_sync_guid: uuid::Uuid,
    pub mm_sync_path: String,
    pub mm_sync_path_to: String,
    pub mm_sync_options_json: serde_json::Value,
}

pub async fn mk_lib_database_sync_list(
    sqlx_pool: &sqlx::PgPool,
    user_id: Uuid,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBSyncList>, sqlx::Error> {
    let select_query;
    if user_id == uuid::Uuid::nil() {
        select_query = sqlx::query(
            "select mm_sync_guid, mm_sync_path, \
            mm_sync_path_to, mm_sync_options_json \
            from mm_media_sync where mm_sync_guid in (select mm_sync_guid \
            from mm_media_sync order by mm_sync_options_json->'Priority' desc, \
            mm_sync_path offset $1 limit $2) \
            order by mm_sync_options_json->'Priority' desc, mm_sync_path",
        )
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_sync_guid, mm_sync_path, \
            mm_sync_path_to, mm_sync_options_json \
            from mm_media_sync where mm_sync_guid in (select mm_sync_guid \
            from mm_media_sync where mm_sync_options_json->'User'::text = $1 \
            order by mm_sync_options_json->'Priority' desc, \
            mm_sync_path offset $2 limit $3) \
            order by mm_sync_options_json->'Priority' desc, mm_sync_path",
        )
        .bind(user_id)
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBSyncList> = select_query
        .map(|row: PgRow| DBSyncList {
            mm_sync_guid: row.get("mm_sync_guid"),
            mm_sync_path: row.get("mm_sync_path"),
            mm_sync_path_to: row.get("mm_sync_path_to"),
            mm_sync_options_json: row.get("mm_sync_options_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}
