use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaBookList {
    pub mm_metadata_book_guid: uuid::Uuid,
    pub mm_metadata_book_name: String,
    pub mm_metadata_book_cover: String,
}

pub async fn mk_lib_database_media_book_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaBookList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_metadata_book_guid, mm_metadata_book_name
            from mm_metadata_book, mm_media
            where mm_media_metadata_guid = mm_metadata_book_guid
            and mm_metadata_book_name % $1
            order by LOWER(mm_metadata_book_name)
            offset $2 limit $3"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select mm_metadata_book_guid, mm_metadata_book_name
            from mm_metadata_book, mm_media
            where mm_media_metadata_guid = mm_metadata_book_guid
            order by LOWER(mm_metadata_book_name)
            offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_media_book_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_book,
            mm_media
            where mm_media_metadata_guid = mm_metadata_book_guid
            and mm_metadata_book_name % $1"#,
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_book,
            mm_media
            where mm_media_metadata_guid = mm_metadata_book_guid"#,
        )
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}
