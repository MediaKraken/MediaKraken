#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;
use serde_json::json;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaTVShowList {
    mm_metadata_tvshow_guid: uuid::Uuid,
    mm_metadata_tvshow_name: String,
    mm_count: i32,
    mm_poster: serde_json::Value,
}

pub async fn mk_lib_database_media_tv_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i32,
    limit: i32,
) -> Result<Vec<DBMediaTVShowList>, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "elect mm_metadata_tvshow_guid, \
            mm_metadata_tvshow_name, \
            count(*) as mm_count, \
            mm_metadata_tvshow_localimage_json->'Images'->>'Poster' as mm_poster \
            from mm_metadata_tvshow, \
            mm_media where mm_media_metadata_guid = mm_metadata_tvshow_guid \
            and mm_metadata_tvshow_name % $1 \
            group by mm_metadata_tvshow_guid \
            order by LOWER(mm_metadata_tvshow_name) \
            offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_metadata_tvshow_guid, \
            mm_metadata_tvshow_name, \
            count(*) as mm_count, \
            mm_metadata_tvshow_localimage_json->'Images'->>'Poster' as mm_poster \
            from mm_metadata_tvshow, \
            mm_media where mm_media_metadata_guid \
            = mm_metadata_tvshow_guid \
            group by mm_metadata_tvshow_guid \
            order by LOWER(mm_metadata_tvshow_name) \
            offset $1 limit $2",
        )
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
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_tv_count(
    sqlx_pool: &sqlx::PgPool,
    search_string: String,
) -> Result<i64, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    if search_string != "" {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_tvshow, \
        mm_media where mm_media_metadata_guid = mm_metadata_tvshow_guid \
        mm_metadata_tvshow_name = %1",
        )
        .bind(search_string)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_tvshow, \
        mm_media where mm_media_metadata_guid = mm_metadata_tvshow_guid",
        )
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}
