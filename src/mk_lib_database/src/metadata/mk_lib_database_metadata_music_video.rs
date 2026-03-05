use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicVideoList {
    pub mm_metadata_music_video_guid: uuid::Uuid,
    pub mm_metadata_music_video_band: String,
    pub mm_metadata_music_video_song: String,
    pub mm_metadata_music_video_localimage_json: String,
}

pub async fn mk_lib_database_metadata_music_video_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaMusicVideoList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_metadata_music_video_guid, mm_metadata_music_video_band, mm_metadata_music_video_song, mm_metadata_music_video_localimage_json from mm_metadata_music_video where mm_metadata_music_video_song &@ $1 offset $2 limit $3"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select mm_metadata_music_video_guid, mm_metadata_music_video_band, mm_metadata_music_video_song, mm_metadata_music_video_localimage_json from mm_metadata_music_video order by LOWER(mm_metadata_music_video_band), LOWER(mm_metadata_music_video_song) offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_metadata_music_video_lookup(
    sqlx_pool: &sqlx::PgPool,
    artist_name: String,
    song_title: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (Uuid,) = sqlx::query_as(
        r#"select mm_metadata_music_video_guid from mm_metadata_music_video where lower(mm_media_music_video_band) = $1 and lower(mm_media_music_video_song) = $2"#,
    )
    .bind(artist_name.to_lowercase())
    .bind(song_title.to_lowercase())
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_music_video_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    imvdb_id: i32,
) -> Result<i64, sqlx::Error> {
    if imvdb_id == 0 {
        if !search_value.is_empty() {
            let row: (i64,) = sqlx::query_as(
                r#"select count(*) from mm_metadata_music_video where mm_media_music_video_song &@ $1"#,
            )
            .bind(search_value)
            .fetch_one(sqlx_pool)
            .await?;
            Ok(row.0)
        } else {
            let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_metadata_music_video"#)
                .fetch_one(sqlx_pool)
                .await?;
            Ok(row.0)
        }
    } else {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_music_video where mm_metadata_music_video_media_id->'imvdb' ? $1"#,
        )
        .bind(imvdb_id)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_music_video_detail(
    sqlx_pool: &sqlx::PgPool,
    music_video_uuid: String,
) -> Result<PgRow, sqlx::Error> {
    let row: PgRow = sqlx::query(
        r#"select mm_media_music_video_band, mm_media_music_video_song, mm_metadata_music_video_json, mm_metadata_music_video_localimage_json from mm_metadata_music_video where mm_metadata_music_video_guid = $1"#,
    )
    .bind(music_video_uuid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row)
}

pub async fn mk_lib_database_metadata_music_video_insert(
    sqlx_pool: &sqlx::PgPool,
    artist_name: String,
    artist_song: String,
    id_json: serde_json::Value,
    data_json: serde_json::Value,
    image_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_music_video (mm_metadata_music_video_guid, mm_metadata_music_video_media_id, mm_media_music_video_band, mm_media_music_video_song, mm_metadata_music_video_json, mm_metadata_music_video_localimage_json) values ($1,$2,$3,$4,$5,$6)"#,
    )
    .bind(new_guid)
    .bind(id_json)
    .bind(artist_name)
    .bind(artist_song)
    .bind(data_json)
    .bind(image_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}
