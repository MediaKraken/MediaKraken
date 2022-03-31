use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use uuid::Uuid;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_media_album_count(pool: &sqlx::PgPool,
                                               search_value: String)
                                               -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query("elect count(*) from mm_metadata_album, mm_media \
            where mm_media_metadata_guid = mm_metadata_album_guid \
            and mm_metadata_album_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query("select count(*) from \
            (select distinct mm_metadata_album_guid from mm_metadata_album, mm_media \
            where mm_media_metadata_guid = mm_metadata_album_guid) as temp")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaMusicList {
	mm_metadata_album_guid: uuid::Uuid,
	mm_metadata_album_name: String,
	mm_metadata_album_json: Json,
}

pub async fn mk_lib_database_media_album_read(pool: &sqlx::PgPool,
                                              search_value: String,
                                              offset: i32, limit: i32)
                                              -> Result<Vec<DBMediaMusicList>, sqlx::Error> {
    // TODO only grab the image part of the json for list, might want runtime, etc as well
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("select mm_metadata_album_guid, mm_metadata_album_name, \
            mm_metadata_album_json from mm_metadata_album, mm_media \
            where mm_media_metadata_guid = mm_metadata_album_guid \
            and mm_metadata_album_name % $1 \
            group by mm_metadata_album_guid \
            order by LOWER(mm_metadata_album_name) \
            offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("select mm_metadata_album_guid, mm_metadata_album_name, \
            mm_metadata_album_json from mm_metadata_album, mm_media \
            where mm_media_metadata_guid = mm_metadata_album_guid \
            group by mm_metadata_album_guid \
            order by LOWER(mm_metadata_album_name) \
            offset $1 limit $2")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMediaMusicList> = select_query
        .map(|row: PgRow| DBMediaMusicList {
            mm_metadata_album_guid: row.get("mm_metadata_album_guid"),
            mm_metadata_album_name: row.get("mm_metadata_album_name"),
            mm_metadata_album_json: row.get("mm_metadata_album_json"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}
