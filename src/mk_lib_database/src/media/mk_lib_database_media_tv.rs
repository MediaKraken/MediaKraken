#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::{types::Uuid, types::Json};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use rocket_dyn_templates::serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaTVShowList {
    mm_metadata_tvshow_guid: uuid::Uuid,
    mm_metadata_tvshow_name: String,
    mm_count: i32,
    mm_poster: serde_json::Value,
}

pub async fn mk_lib_database_media_tv_read(pool: &sqlx::PgPool,
                                           search_value: String,
                                            offset: i32, limit: i32)
                                           -> Result<Vec<DBMediaTVShowList>, sqlx::Error> {
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("elect mm_metadata_tvshow_guid, \
            mm_metadata_tvshow_name, \
            count(*) as mm_count, \
            mm_metadata_tvshow_localimage_json->'Images'->>'Poster' as mm_poster, \
            from mm_metadata_tvshow, \
            mm_media where mm_media_metadata_guid = mm_metadata_tvshow_guid \
            and mm_metadata_tvshow_name % $1 \
            group by mm_metadata_tvshow_guid \
            order by LOWER(mm_metadata_tvshow_name) \
            offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("select mm_metadata_tvshow_guid, \
            mm_metadata_tvshow_name, \
            count(*) as mm_count, \
            mm_metadata_tvshow_localimage_json->'Images'->>'Poster' as mm_poster, \
            from mm_metadata_tvshow, \
            mm_media where mm_media_metadata_guid \
            = mm_metadata_tvshow_guid \
            group by mm_metadata_tvshow_guid \
            order by LOWER(mm_metadata_tvshow_name) \
            offset $1 limit $2")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMediaTVShowList> = select_query
        .map(|row: PgRow| DBMediaTVShowList {
            mm_metadata_tvshow_guid: row.get("mm_metadata_tvshow_guid"),
            mm_metadata_tvshow_name: row.get("mm_metadata_tvshow_name"),
            mm_count: row.get("mm_count"),
            mm_poster: row.get("mm_poster"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_tv_count(pool: &sqlx::PgPool,
                                            search_string: String)
                                            -> Result<i32, sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_tvshow, \
        mm_media where mm_media_metadata_guid = mm_metadata_tvshow_guid")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}
