#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use serde::{Serialize, Deserialize};

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

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct MediaImageList {
	image_path: String,
}

pub async fn mk_lib_database_metadata_image_read(pool: &sqlx::PgPool,
                                                 class_id: i32, offset: i32, limit: i32)
                                                 -> Result<Vec<MediaImageList>, sqlx::Error> {
    let select_query = sqlx::query("select mm_media_path from mm_media \
        where mm_media_class_guid = $1 offset $2 limit $3")
        .bind(class_id)
        .bind(offset)
        .bind(limit);
    let table_rows: Vec<MediaImageList> = select_query
		.map(|row: PgRow| MediaImageList {
			image_path: row.get("mm_media_path"),
		})
		.fetch_all(pool)
		.await?;
    Ok(table_rows)
}
