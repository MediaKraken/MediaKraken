use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

pub async fn mk_lib_database_link_delete(
    sqlx_pool: &sqlx::PgPool,
    link_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query("delete from mm_link where mm_link_guid = $1")
        .bind(link_uuid)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLinkList {
    pub mm_link_guid: uuid::Uuid,
    pub mm_link_name: String,
    pub mm_link_json: serde_json::Value,
}

pub async fn mk_lib_database_link_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    records: i64,
) -> Result<Vec<DBLinkList>, sqlx::Error> {
    let table_rows: Vec<DBLinkList> = sqlx::query_as(
        "select mm_link_guid, mm_link_name, \
        mm_link_json from mm_link \
        order by mm_link_name \
        offset $1 limit $2",
    )
    .bind(offset)
    .bind(records)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_link_insert(
    sqlx_pool: &sqlx::PgPool,
    link_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_link (mm_link_guid, mm_link_json) \
        values ($1, $2)",
    )
    .bind(new_guid)
    .bind(link_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_link_list_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_library_link \
            where mm_link_name % $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_library_link")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}
