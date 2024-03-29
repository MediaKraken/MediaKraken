use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_media_music_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if search_value != "" {
        let row: (i64,) = sqlx::query_as(
            "elect count(*) from mm_metadata_album, mm_media \
            where mm_media_metadata_guid = mm_metadata_album_guid \
            and mm_metadata_album_name % $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from \
            (select distinct mm_metadata_album_guid from mm_metadata_album, mm_media \
            where mm_media_metadata_guid = mm_metadata_album_guid) as temp",
        )
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaMusicList {
    pub mm_metadata_album_guid: uuid::Uuid,
    pub mm_metadata_album_name: String,
    pub mm_metadata_album_json: serde_json::Value,
}

pub async fn mk_lib_database_media_music_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaMusicList>, sqlx::Error> {
    // TODO only grab the image part of the json for list, might want runtime, etc as well
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select mm_metadata_album_guid, mm_metadata_album_name, \
            mm_metadata_album_json from mm_metadata_album, mm_media \
            where mm_media_metadata_guid = mm_metadata_album_guid \
            and mm_metadata_album_name % $1 \
            group by mm_metadata_album_guid \
            order by LOWER(mm_metadata_album_name) \
            offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_metadata_album_guid, mm_metadata_album_name, \
            mm_metadata_album_json from mm_metadata_album, mm_media \
            where mm_media_metadata_guid = mm_metadata_album_guid \
            group by mm_metadata_album_guid \
            order by LOWER(mm_metadata_album_name) \
            offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBMediaMusicList> = select_query
        .map(|row: PgRow| DBMediaMusicList {
            mm_metadata_album_guid: row.get("mm_metadata_album_guid"),
            mm_metadata_album_name: row.get("mm_metadata_album_name"),
            mm_metadata_album_json: row.get("mm_metadata_album_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}
