#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket_dyn_templates::serde::{Serialize, Deserialize};
use sqlx::{types::Uuid, types::Json};
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_usage_top10_movie(pool: &sqlx::PgPool)
                                               -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_metadata_user_json->'Watched'->'Times' \
        from mm_metadata_movie order by mm_metadata_user_json->'Watched'->'Times' desc limit 10")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_usage_top10_tv(pool: &sqlx::PgPool)
                                            -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_metadata_tvshow_user_json->'Watched'->'Times' \
        from mm_metadata_tvshow order by mm_metadata_tvshow_user_json->'Watched'->'Times' \
        desc limit 10")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/*

// TODO port query
async def db_usage_top10_alltime(self):
    """
    Top 10 of all time
    """
    return await db_conn.fetch('select 1 limit 10')


// TODO port query
async def db_usage_top10_tv_episode(self):
    """
    Top 10 TV episode
    """
    return await db_conn.fetch('select 1 limit 10')

 */