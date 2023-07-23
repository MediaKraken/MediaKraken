use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;

use sqlx::{FromRow, Row};
use stdext::function_name;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaGameSystemList {
    pub gs_game_system_id: uuid::Uuid,
    pub gs_game_system_name: String,
    pub gs_game_system_alias: String,
    pub gs_game_system_poster: String,
}

pub async fn mk_lib_database_media_game_system_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMediaGameSystemList>, sqlx::Error> {
    // TODO this should only return systems where there are games for it since it's "media"
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select gs_game_system_id, \
            gs_game_system_name, \
            gs_game_system_alias, \
            gs_game_system_localimage->>'Poster' as gs_game_system_poster \
            from mm_metadata_game_systems_info \
            where gs_game_system_name = $1 \
            offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select gs_game_system_id, \
            gs_game_system_name, \
            gs_game_system_alias, \
            gs_game_system_localimage->>'Poster' as gs_game_system_poster \
            from mm_metadata_game_systems_info \
            offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBMediaGameSystemList> = select_query
        .map(|row: PgRow| DBMediaGameSystemList {
            gs_game_system_id: row.get("gs_game_system_id"),
            gs_game_system_name: row.get("gs_game_system_name"),
            gs_game_system_alias: row.get("gs_game_system_alias"),
            gs_game_system_poster: row.get("gs_game_system_poster"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_game_system_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if search_value != "" {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_game_systems_info where gs_game_system_name = $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_metadata_game_systems_info")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}
