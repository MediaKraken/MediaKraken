use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub async fn mk_lib_database_metadata_image_count(
    sqlx_pool: &sqlx::PgPool,
    class_id: i32,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_media
        where mm_media_class_guid = $1"#,
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
    let table_rows: Vec<MediaImageList> = sqlx::query_as(
        r#"select mm_media_path from mm_media
        where mm_media_class_guid = $1
        offset $2 limit $3"#,
    )
    .bind(class_id)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}
