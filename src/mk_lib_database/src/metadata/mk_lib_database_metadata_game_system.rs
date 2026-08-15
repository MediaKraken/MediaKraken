use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

pub async fn mk_lib_database_metadata_game_system_detail(
    sqlx_pool: &sqlx::PgPool,
    game_sys_uuid: Uuid,
) -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value,) = sqlx::query_as(
        r#"select gs_game_system_json from mm_metadata_game_systems_info where gs_id = $1"#,
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
    if search_value != String::new() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_game_systems_info where gs_game_system_name &@ $1"#,
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_metadata_game_systems_info"#)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaGameSystemList {
    pub gs_game_system_id: uuid::Uuid,
    pub gs_game_system_name: String,
    pub gs_description: Option<String>,
    pub gs_year: Option<String>,
    pub gs_game_system_alias: String,
}

pub async fn mk_lib_database_metadata_game_system_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaGameSystemList>, sqlx::Error> {
    // TODO might need to sort by release year as well for machines with multiple releases
    if search_value != String::new() {
        sqlx::query_as(
            r#"select gs_game_system_id, gs_game_system_name, gs_game_system_json->>'description' as gs_description, gs_game_system_json->>'year' as gs_year, gs_game_system_alias from mm_metadata_game_systems_info where gs_game_system_name &@ $1 offset $2 limit $3"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select gs_game_system_id, gs_game_system_name, gs_game_system_json->>'description' as gs_description, gs_game_system_json->>'year' as gs_year, gs_game_system_alias from mm_metadata_game_systems_info order by gs_game_system_json->'description' offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_metadata_game_system_upsert(
    sqlx_pool: &sqlx::PgPool,
    system_name: String,
    system_alias: String,
    system_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO mm_metadata_game_systems_info (gs_game_system_id, gs_game_system_name, gs_game_system_alias, gs_game_system_json) VALUES ($1, $2, $3, $4) ON CONFLICT (gs_game_system_name) DO UPDATE SET gs_game_system_alias = $5, gs_game_system_json = $6"#,
    )
    .bind(new_guid)
    .bind(system_name)
    .bind(&system_alias)
    .bind(&system_json)
    .bind(&system_alias)
    .bind(&system_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_metadata_game_system_guid_by_short_name(
    sqlx_pool: &sqlx::PgPool,
    game_system_short_name: &str,
) -> Result<Uuid, sqlx::Error> {
    let row: (Uuid,) = sqlx::query_as(
        r#"select gs_game_system_id from mm_metadata_game_systems_info where gs_game_system_name = $1"#,
    )
    .bind(game_system_short_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_game_system_game_count_by_short_name(
    sqlx_pool: &sqlx::PgPool,
    game_system_short_name: &str,
) -> Result<i64, sqlx::Error> {
    // TODO this query doesn't return game count.......
    let row: (i64,) = sqlx::query_as(
        r#"select count(*) from mm_metadata_game_systems_info where gs_game_system_name = $1"#,
    )
    .bind(game_system_short_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}
