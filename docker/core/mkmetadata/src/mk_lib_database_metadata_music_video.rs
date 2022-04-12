#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use sqlx::{types::Uuid, types::Json};
use rocket_dyn_templates::serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicVideoList {
    mm_metadata_music_video_guid: uuid::Uuid,
    mm_media_music_video_band: String,
    mm_media_music_video_song: String,
    mm_metadata_music_video_localimage_json: String,
}

pub async fn mk_lib_database_metadata_music_video_read(pool: &sqlx::PgPool,
                                                       search_value: String,
                                                       offset: i32, limit: i32)
                                                       -> Result<Vec<DBMetaMusicVideoList>, sqlx::Error> {
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("select mm_metadata_music_video_guid, mm_media_music_video_band, \
            mm_media_music_video_song, mm_metadata_music_video_localimage_json \
            from mm_metadata_music_video where mm_media_music_video_song % $1 \
            order by LOWER(mm_media_music_video_band), LOWER(mm_media_music_video_song) \
            offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("select mm_metadata_music_video_guid, mm_media_music_video_band, \
            mm_media_music_video_song, mm_metadata_music_video_localimage_json \
            from mm_metadata_music_video order by LOWER(mm_media_music_video_band), \
            LOWER(mm_media_music_video_song) offset $1 limit $2")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMetaMusicVideoList> = select_query
        .map(|row: PgRow| DBMetaMusicVideoList {
            mm_metadata_music_video_guid: row.get("mm_metadata_music_video_guid"),
            mm_media_music_video_band: row.get("mm_media_music_video_band"),
            mm_media_music_video_song: row.get("mm_media_music_video_song"),
            mm_metadata_music_video_localimage_json: row.get("mm_metadata_music_video_localimage_json"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_music_video_lookup(pool: &sqlx::PgPool,
                                                         artist_name: String,
                                                         song_title: String)
                                                         -> Result<uuid::Uuid, sqlx::Error> {
    let row: (Uuid, ) = sqlx::query_as("select mm_metadata_music_video_guid \
        from mm_metadata_music_video \
        where lower(mm_media_music_video_band) = $1 \
        and lower(mm_media_music_video_song) = $2")
        .bind(artist_name.to_lowercase())
        .bind(song_title.to_lowercase())
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_music_video_count(pool: &sqlx::PgPool,
                                                        search_value: String,
                                                        imvdb_id: i32)
                                                        -> Result<i32, sqlx::Error> {
    if imvdb_id == 0 {
        if search_value != "" {
            let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_music_video \
                where mm_media_music_video_song % $1")
                .bind(search_value)
                .fetch_one(pool)
                .await?;
            Ok(row.0)
        } else {
            let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_music_video")
                .fetch_one(pool)
                .await?;
            Ok(row.0)
        }
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_music_video \
            where mm_metadata_music_video_media_id->'imvdb' ? $1")
            .bind(imvdb_id)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_music_video_detail(pool: &sqlx::PgPool,
                                                         music_video_uuid: String)
                                                         -> Result<PgRow, sqlx::Error> {
    let row: PgRow = sqlx::query("select mm_media_music_video_band, \
        mm_media_music_video_song, mm_metadata_music_video_json, \
        mm_metadata_music_video_localimage_json from mm_metadata_music_video \
        where mm_metadata_music_video_guid = $1")
        .bind(music_video_uuid)
        .fetch_one(pool)
        .await?;
    Ok(row)
}

pub async fn mk_lib_database_metadata_music_video_insert(pool: &sqlx::PgPool,
                                                         artist_name: String,
                                                         artist_song: String,
                                                         id_json: serde_json::Value,
                                                         data_json: serde_json::Value,
                                                         image_json: serde_json::Value)
                                                         -> Result<Uuid, sqlx::Error> {
    let new_guid = Uuid::new_v4();
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_metadata_music_video (mm_metadata_music_video_guid, \
        mm_metadata_music_video_media_id, \
        mm_media_music_video_band, \
        mm_media_music_video_song, \
        mm_metadata_music_video_json, \
        mm_metadata_music_video_localimage_json) \
        values ($1,$2,$3,$4,$5,$6)")
        .bind(new_guid)
        .bind(id_json)
        .bind(artist_name)
        .bind(artist_song)
        .bind(data_json)
        .bind(image_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(new_guid)
}