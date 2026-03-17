use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;

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
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select gs_game_system_id,
            gs_game_system_name,
            gs_game_system_alias,
            gs_game_system_localimage->>'Poster' as gs_game_system_poster
            from mm_metadata_game_systems_info
            where gs_game_system_name = $1
            offset $2 limit $3"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select gs_game_system_id,
            gs_game_system_name,
            gs_game_system_alias,
            gs_game_system_localimage->>'Poster' as gs_game_system_poster
            from mm_metadata_game_systems_info
            offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_media_game_system_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_game_systems_info where gs_game_system_name = $1"#,
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
