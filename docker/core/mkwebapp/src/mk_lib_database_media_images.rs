use sqlx::postgres::PgRow;
use uuid::Uuid;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_metadata_image_count(pool: &sqlx::PgPool,
                                                  class_id: i32)
                                                  -> Result<i32, sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select count(*) from mm_media \
        where mm_media_class_guid = $1")
        .bind(class_id)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_image_read(pool: &sqlx::PgPool,
                                                 class_id: i32, offset: i32, limit: i32)
                                                 -> Result<i32, sqlx::Error> {
    let rows: (i32, ) = sqlx::query_as("select mm_media_path from mm_media \
        where mm_media_class_guid = $1 offset $2 limit $3")
        .bind(class_id)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
