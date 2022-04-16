#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaMusicVideoList {
    mm_metadata_music_video_guid: uuid::Uuid,
}

pub async fn mk_lib_database_media_music_video_read(pool: &sqlx::PgPool,
                                                    search_value: String,
                                                    offset: i32, limit: i32)
                                                    -> Result<Vec<DBMediaMusicVideoList>, sqlx::Error> {
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMediaMusicVideoList> = select_query
        .map(|row: PgRow| DBMediaMusicVideoList {
            mm_metadata_music_video_guid: row.get("mm_metadata_music_video_guid"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_music_video_count(pool: &sqlx::PgPool,
                                                     search_value: String)
                                                     -> Result<i32, sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_music_video, mm_media \
            where mm_media_metadata_guid = mm_metadata_music_video_guid group \
            and mm_media_music_video_song % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_music_video, mm_media \
            where mm_media_metadata_guid = mm_metadata_music_video_guid")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}
