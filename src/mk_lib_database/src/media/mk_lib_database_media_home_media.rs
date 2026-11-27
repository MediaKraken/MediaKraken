use mk_lib_common::mk_lib_common_enum_media_type;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaHomeMediaList {
    pub mm_metadata_home_guid: uuid::Uuid,
    pub mm_metadata_home_name: String,
}

pub async fn mk_lib_database_media_home_media_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaHomeMediaList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_media_guid as mm_metadata_home_guid,
            coalesce(mm_media_path, '') as mm_metadata_home_name
            from mm_media
            where mm_media_class_enum = $1
            and mm_media_path % $2
            order by mm_media_path
            offset $3 limit $4"#,
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME)
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select mm_media_guid as mm_metadata_home_guid,
            coalesce(mm_media_path, '') as mm_metadata_home_name
            from mm_media
            where mm_media_class_enum = $1
            order by mm_media_path
            offset $2 limit $3"#,
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_media_home_media_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_media
            where mm_media_class_enum = $1
            and mm_media_path % $2"#,
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME)
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_media
            where mm_media_class_enum = $1"#,
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}
