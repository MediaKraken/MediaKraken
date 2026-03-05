use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaMusicVideoList {
    pub mm_metadata_music_video_guid: uuid::Uuid,
}

pub async fn mk_lib_database_media_music_video_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaMusicVideoList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as("")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    } else {
        sqlx::query_as("")
            .bind(offset)
            .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    }
}

pub async fn mk_lib_database_media_music_video_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_music_video, mm_media
            where mm_media_metadata_guid = mm_metadata_music_video_guid group
            and mm_media_music_video_song % $1"#,
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_music_video, mm_media
            where mm_media_metadata_guid = mm_metadata_music_video_guid"#,
        )
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}
