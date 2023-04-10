#![cfg_attr(debug_assertions, allow(dead_code))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

pub async fn mk_lib_database_metadata_collection_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    if search_value != "" {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_collection \
            where mm_metadata_collection_name = $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_metadata_collection")
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
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select mm_metadata_collection_guid, \
            mm_metadata_collection_name, \
            mm_metadata_collection_imagelocal_json from mm_metadata_collection \
            where mm_metadata_collection_guid in (select mm_metadata_collection_guid \
            from mm_metadata_collection where mm_metadata_collection_name % $1 \
            order by mm_metadata_collection_name \
            offset $2 limit $3) order by mm_metadata_collection_name",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_metadata_collection_guid, \
            mm_metadata_collection_name, \
            mm_metadata_collection_imagelocal_json from mm_metadata_collection \
            where mm_metadata_collection_guid in (select mm_metadata_collection_guid \
            from mm_metadata_collection order by mm_metadata_collection_name \
            offset $1 limit $2) order by mm_metadata_collection_name",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBMetaCollectionList> = select_query
        .map(|row: PgRow| DBMetaCollectionList {
            mm_metadata_collection_guid: row.get("mm_metadata_collection_guid"),
            mm_metadata_collection_name: row.get("mm_metadata_collection_name"),
            mm_metadata_collection_imagelocal_json: row
                .get("mm_metadata_collection_imagelocal_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_meta_collection_detail(
    sqlx_pool: &sqlx::PgPool,
    collection_uuid: String,
) -> Result<PgRow, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: PgRow = sqlx::query(
        "select mm_metadata_collection_json, \
        mm_metadata_collection_imagelocal_json from mm_metadata_collection \
        where mm_metadata_collection_guid = $1",
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
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let select_query = sqlx::query(
        "select mm_metadata_guid, mm_metadata_json \
         from mm_metadata_movie where mm_metadata_json->'belongs_to_collection'::text \
         <> '{}'::text order by mm_metadata_json->'belongs_to_collection'",
    )
    .bind(collection_name);
    let table_rows: Vec<DBMetaCollectionByNameList> = select_query
        .map(|row: PgRow| DBMetaCollectionByNameList {
            mm_metadata_guid: row.get("mm_metadata_guid"),
            mm_metadata_json: row.get("mm_metadata_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_collection_guid_by_name(
    sqlx_pool: &sqlx::PgPool,
    collection_name: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (uuid::Uuid,) = sqlx::query_as(
        "select mm_metadata_collection_guid \
        from mm_metadata_collection \
        where mm_metadata_collection_name->>'name' = $1",
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
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (uuid::Uuid,) = sqlx::query_as(
        "sselect mm_metadata_collection_guid from mm_metadata_collection
        where mm_metadata_collection_json @> '{\"id\":$1}'",
    )
    .bind(tmdb_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/*

// TODO port query
pub async fn db_collection_update(self, collection_guid, guid_json):
    """
    Update the ids listed within a collection
    """
    await db_conn.execute('update mm_metadata_collection'
                          ' set mm_metadata_collection_media_ids = $1,'
                          ' mm_metadata_collection_json = $2'
                          ' where mm_metadata_collection_guid = $3',
                          TODOfield, json.dumps(guid_json), collection_guid)

 */

pub async fn mk_lib_database_meta_collection_insert(
    sqlx_pool: &sqlx::PgPool,
    collection_name: String,
    guid_json: serde_json::Value,
    metadata_json: serde_json::Value,
    local_image_json: serde_json::Value,
) -> Result<Uuid, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_metadata_collection (mm_metadata_collection_guid, \
        mm_metadata_collection_name, mm_metadata_collection_media_ids, \
        mm_metadata_collection_json, mm_metadata_collection_imagelocal_json) \
        values ($1,$2,$3,$4,$5)",
    )
    .bind(new_guid)
    .bind(collection_name)
    .bind(guid_json)
    .bind(metadata_json)
    .bind(local_image_json)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}
