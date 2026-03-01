use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaTVShowList {
    pub mm_metadata_tvshow_guid: uuid::Uuid,
    pub mm_metadata_tvshow_name: String,
    mm_count: i32,
    mm_poster: serde_json::Value,
}

pub async fn mk_lib_database_media_tv_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaTVShowList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(
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
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
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
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_media_tv_count(
    sqlx_pool: &sqlx::PgPool,
    search_string: String,
) -> Result<i64, sqlx::Error> {
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
