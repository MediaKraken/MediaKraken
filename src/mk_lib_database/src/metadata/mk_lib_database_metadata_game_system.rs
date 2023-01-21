#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_metadata_game_system_detail(
    sqlx_pool: &sqlx::PgPool,
    game_sys_uuid: Uuid,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) = sqlx::query_as(
        "select gs_game_system_json from mm_metadata_game_systems_info \
        where gs_id = $1",
    )
    .bind(game_sys_uuid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_game_system_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if search_value != "" {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_game_systems_info \
            where gs_game_system_name % $1",
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaGameSystemList {
    gs_game_system_id: uuid::Uuid,
    gs_game_system_name: String,
    gs_description: Option<String>,
    gs_year: String,
    gs_game_system_alias: String,
}

pub async fn mk_lib_database_metadata_game_system_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i32,
    limit: i32,
) -> Result<Vec<DBMetaGameSystemList>, sqlx::Error> {
    // TODO might need to sort by release year as well for machines with multiple releases
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select gs_game_system_id, gs_game_system_name, \
            gs_game_system_json->>'description' as gs_description, \
            gs_game_system_json->>'year' as gs_year, \
            gs_game_system_alias from mm_metadata_game_systems_info \
            where gs_game_system_name % $1 \
            order by gs_game_system_json->'description' \
            offset $2 limit $2",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select gs_game_system_id, gs_game_system_name, \
            gs_game_system_json->>'description' as gs_description, \
            gs_game_system_json->>'year' as gs_year, \
            gs_game_system_alias from mm_metadata_game_systems_info \
            order by gs_game_system_json->'description' offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBMetaGameSystemList> = select_query
        .map(|row: PgRow| DBMetaGameSystemList {
            gs_game_system_id: row.get("gs_game_system_id"),
            gs_game_system_name: row.get("gs_game_system_name"),
            gs_description: row.get("gs_description"),
            gs_year: row.get("gs_year"),
            gs_game_system_alias: row.get("gs_game_system_alias"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_game_system_upsert(
    sqlx_pool: &sqlx::PgPool,
    system_name: String,
    system_alias: String,
    system_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "INSERT INTO mm_metadata_game_systems_info \
        (gs_game_system_id, \
        gs_game_system_name, \
        gs_game_system_alias, \
        gs_game_system_json) \
        VALUES ($1, $2, $3, $4) \
        ON CONFLICT (gs_game_system_name) \
        DO UPDATE SET gs_game_system_alias = $5, gs_game_system_json = $6",
    )
    .bind(new_guid)
    .bind(system_name)
    .bind(system_alias)
    .bind(system_json)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_metadata_game_system_guid_by_short_name(
    sqlx_pool: &sqlx::PgPool,
    game_system_short_name: String,
) -> Result<Uuid, sqlx::Error> {
    let row: (Uuid,) = sqlx::query_as(
        "select gs_game_system_id \
        from mm_metadata_game_systems_info \
        where gs_game_system_name = $1",
    )
    .bind(game_system_short_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_game_system_game_count_by_short_name(
    sqlx_pool: &sqlx::PgPool,
    game_system_short_name: String,
) -> Result<i32, sqlx::Error> {
    // TODO this query doesn't return game count.......
    let row: (i32,) = sqlx::query_as(
        "select count(*) \
        from mm_metadata_game_systems_info \
        where gs_game_system_name = $1",
    )
    .bind(game_system_short_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}
