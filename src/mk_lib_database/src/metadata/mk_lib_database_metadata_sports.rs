use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub async fn mk_lib_database_metadata_sports_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_sports where mm_metadata_sports_name &@ $1"#,
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_metadata_sports"#)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaSportsList {
    pub mm_metadata_sports_guid: uuid::Uuid,
    pub mm_metadata_sports_name: String,
}

pub async fn mk_lib_database_metadata_sports_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaSportsList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select
                mm_metadata_sports_guid,
                coalesce(mm_metadata_sports_name, 'Unknown Event') as mm_metadata_sports_name
            from mm_metadata_sports
            where mm_metadata_sports_name &@ $1
            order by lower(coalesce(mm_metadata_sports_name, 'Unknown Event'))
            offset $2 limit $3"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select
                mm_metadata_sports_guid,
                coalesce(mm_metadata_sports_name, 'Unknown Event') as mm_metadata_sports_name
            from mm_metadata_sports
            order by lower(coalesce(mm_metadata_sports_name, 'Unknown Event'))
            offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

/// Metadata guid by TheSportsDB id.
pub async fn mk_lib_database_metadata_sports_guid_by_thesportsdb(
    sqlx_pool: &sqlx::PgPool,
    thesports_uuid: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let row: Option<(uuid::Uuid,)> = sqlx::query_as(
        r#"select mm_metadata_sports_guid from mm_metadata_sports
        where mm_metadata_media_sports_id->'thesportsdb' ? $1"#,
    )
    .bind(thesports_uuid)
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row.map(|row| row.0))
}

/// Fetch guid by event name.
pub async fn mk_lib_database_metadata_sports_guid_by_event_name(
    sqlx_pool: &sqlx::PgPool,
    event_name: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let row: Option<(uuid::Uuid,)> = sqlx::query_as(
        r#"select mm_metadata_sports_guid from mm_metadata_sports
        where mm_metadata_sports_name = $1"#,
    )
    .bind(event_name)
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row.map(|row| row.0))
}

/// Sports metadata json by guid.
pub async fn mk_lib_database_metadata_sports_json_by_guid(
    sqlx_pool: &sqlx::PgPool,
    guid: uuid::Uuid,
) -> Result<Option<serde_json::Value>, sqlx::Error> {
    let row: Option<(serde_json::Value,)> = sqlx::query_as(
        r#"select mm_metadata_sports_json from mm_metadata_sports
        where mm_metadata_sports_guid = $1"#,
    )
    .bind(guid)
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row.map(|row| row.0))
}

/// Insert a new sports metadata record; returns the new guid.
pub async fn mk_lib_database_metadata_sports_insert(
    sqlx_pool: &sqlx::PgPool,
    series_id_json: &serde_json::Value,
    event_name: &str,
    show_detail: &serde_json::Value,
    image_json: &serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_sports (mm_metadata_sports_guid,
        mm_metadata_media_sports_id, mm_metadata_sports_name,
        mm_metadata_sports_json, mm_metadata_sports_image_json)
        values ($1,$2,$3,$4,$5)"#,
    )
    .bind(new_guid)
    .bind(series_id_json)
    .bind(event_name)
    .bind(show_detail)
    .bind(image_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/// Update a sports metadata record found by TheSportsDB id.
pub async fn mk_lib_database_metadata_sports_update(
    sqlx_pool: &sqlx::PgPool,
    series_id_json: &serde_json::Value,
    event_name: &str,
    show_detail: &serde_json::Value,
    sportsdb_id: &str,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_metadata_sports
        set mm_metadata_media_sports_id = $1,
            mm_metadata_sports_name = $2,
            mm_metadata_sports_json = $3
        where mm_metadata_media_sports_id->'thesportsdb' ? $4"#,
    )
    .bind(series_id_json)
    .bind(event_name)
    .bind(show_detail)
    .bind(sportsdb_id)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}
