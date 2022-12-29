#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMUsageMovieList {
    mm_metadata_name: String,
    mm_metadata_times: i64,
}

pub async fn mk_lib_database_usage_top10_movie(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBMUsageMovieList>, sqlx::Error> {
    let select_query = sqlx::query(
        "select mm_metadata_name, \
        mm_metadata_user_json->'Watched'->'Times' as mm_metadata_times \
        from mm_metadata_movie order by mm_metadata_user_json->'Watched'->'Times' desc limit 10",
    );
    let table_rows: Vec<DBMUsageMovieList> = select_query
        .map(|row: PgRow| DBMUsageMovieList {
            mm_metadata_name: row.get("mm_metadata_name"),
            mm_metadata_times: row.get("mm_metadata_times"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBUsageTVList {
    mm_metadata_tvshow_name: String,
    mm_metadata_times: i64,
}

pub async fn mk_lib_database_usage_top10_tv(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBUsageTVList>, sqlx::Error> {
    let select_query = sqlx::query(
        "select mm_metadata_tvshow_name, \
        mm_metadata_tvshow_user_json->'Watched'->'Times' as mm_metadata_times \
        from mm_metadata_tvshow order by mm_metadata_tvshow_user_json->'Watched'->'Times' \
        desc limit 10",
    );
    let table_rows: Vec<DBUsageTVList> = select_query
        .map(|row: PgRow| DBUsageTVList {
            mm_metadata_tvshow_name: row.get("mm_metadata_tvshow_name"),
            mm_metadata_times: row.get("mm_metadata_times"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBUsageAllTimeList {
    mm_review_guid: uuid::Uuid,
    mm_review_json: serde_json::Value,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBUsageTVEpisodeList {
    mm_review_guid: uuid::Uuid,
    mm_review_json: serde_json::Value,
}

/*

// TODO port query
pub async fn db_usage_top10_alltime(self):
    """
    Top 10 of all time
    """
    return await db_conn.fetch('select 1 limit 10')


// TODO port query
pub async fn db_usage_top10_tv_episode(self):
    """
    Top 10 TV episode
    """
    return await db_conn.fetch('select 1 limit 10')

 */
