#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::{types::Uuid, types::Json};
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

#[path = "mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaHomeMediaList {
	mm_metadata_book_guid: uuid::Uuid,
	mm_metadata_book_name: String,
}

pub async fn mk_lib_database_media_home_media_count(pool: &sqlx::PgPool,
                                                    search_value: String)
                                                    -> Result<i32, sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_media \
            where mmr_media_class_guid = $1
            and mm_media_path % $2")
            .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME)
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_media \
            where mmr_media_class_guid = $1")
            .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}