use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaAdultList {
    pub mm_metadata_adult_guid: uuid::Uuid,
    pub mm_metadata_adult_name: String,
}

pub async fn mk_lib_database_media_adult_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaAdultList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(r#""#)
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    } else {
        sqlx::query_as(r#""#)
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    }
}

pub async fn mk_lib_database_media_adult_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(r#""#)
            .bind(search_value)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(r#""#).fetch_one(sqlx_pool).await?;
        Ok(row.0)
    }
}
