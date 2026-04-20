use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;

pub async fn mk_lib_database_metadata_exists_collection(
    sqlx_pool: &sqlx::PgPool,
    metadata_id: i32,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        r#"select exists(
            select 1 from mm_metadata_collection
            where (mm_metadata_collection_json->>'id')::int = $1
            limit 1
        ) as found_record limit 1"#,
    )
    .bind(metadata_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_collection_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_collection
            where mm_metadata_collection_name &@ $1"#,
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_collection"#,
        )
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaCollectionList {
    pub mm_metadata_collection_guid: uuid::Uuid,
    pub mm_metadata_collection_name: String,
    pub mm_metadata_collection_imagelocal_json: serde_json::Value,
}

pub async fn mk_lib_database_metadata_collection_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaCollectionList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_metadata_collection_guid,
            mm_metadata_collection_name,
            mm_metadata_collection_imagelocal_json from mm_metadata_collection
            where mm_metadata_collection_guid in (
                select mm_metadata_collection_guid
                from mm_metadata_collection
                where mm_metadata_collection_name &@ $1
                offset $2 limit $3
            )"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select mm_metadata_collection_guid,
            mm_metadata_collection_name,
            mm_metadata_collection_imagelocal_json from mm_metadata_collection
            where mm_metadata_collection_guid in (
                select mm_metadata_collection_guid
                from mm_metadata_collection
                order by mm_metadata_collection_name
                order by LOWER(mm_metadata_collection_name)
                offset $1 limit $2
            )"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_meta_collection_detail(
    sqlx_pool: &sqlx::PgPool,
    collection_uuid: String,
) -> Result<PgRow, sqlx::Error> {
    let row: PgRow = sqlx::query(
        r#"select mm_metadata_collection_json,
        mm_metadata_collection_imagelocal_json from mm_metadata_collection
        where mm_metadata_collection_guid = $1"#,
    )
    .bind(collection_uuid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaCollectionByNameList {
    mm_metadata_guid: uuid::Uuid,
    mm_metadata_json: serde_json::Value,
}

pub async fn mk_lib_database_meta_collection_by_name(
    sqlx_pool: &sqlx::PgPool,
    collection_name: String,
) -> Result<Vec<DBMetaCollectionByNameList>, sqlx::Error> {
    let table_rows: Vec<DBMetaCollectionByNameList> = sqlx::query_as(
        r#"select mm_metadata_guid, mm_metadata_json
         from mm_metadata_movie
         where mm_metadata_json->'belongs_to_collection'::text
         <> '{}'::text
         order by mm_metadata_json->'belongs_to_collection'"#,
    )
    .bind(collection_name)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_collection_guid_by_name(
    sqlx_pool: &sqlx::PgPool,
    collection_name: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        r#"select mm_metadata_collection_guid
        from mm_metadata_collection
        where mm_metadata_collection_name->>'name' = $1"#,
    )
    .bind(collection_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_collection_guid_by_tmdb(
    sqlx_pool: &sqlx::PgPool,
    tmdb_id: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        r#"select mm_metadata_collection_guid from mm_metadata_collection
        where mm_metadata_collection_json @> '{"id":$1}'"#,
    )
    .bind(tmdb_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_meta_collection_insert(
    sqlx_pool: &sqlx::PgPool,
    collection_name: String,
    guid_json: serde_json::Value,
    metadata_json: serde_json::Value,
    local_image_json: serde_json::Value,
) -> Result<Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_collection (
        mm_metadata_collection_guid,
        mm_metadata_collection_name,
        mm_metadata_collection_media_ids,
        mm_metadata_collection_json,
        mm_metadata_collection_imagelocal_json
        ) values ($1,$2,$3,$4,$5)"#,
    )
    .bind(new_guid)
    .bind(collection_name)
    .bind(guid_json)
    .bind(metadata_json)
    .bind(local_image_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}