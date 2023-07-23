use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;

use sqlx::{FromRow, Row};
use stdext::function_name;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaAnimeList {
    pub mm_metadata_anime_guid: uuid::Uuid,
    pub mm_metadata_anime_name: String,
}

pub async fn mk_lib_database_media_anime_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaAnimeList>, sqlx::Error> {
    let select_query;
    if search_value != "" {
        select_query = sqlx::query("").bind(search_value).bind(offset).bind(limit);
    } else {
        select_query = sqlx::query("").bind(offset).bind(limit);
    }
    let table_rows: Vec<DBMediaAnimeList> = select_query
        .map(|row: PgRow| DBMediaAnimeList {
            mm_metadata_anime_guid: row.get("mm_metadata_anime_guid"),
            mm_metadata_anime_name: row.get("mm_metadata_anime_name"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_anime_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if search_value != "" {
        let row: (i64,) = sqlx::query_as("")
            .bind(search_value)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("").fetch_one(sqlx_pool).await?;
        Ok(row.0)
    }
}
