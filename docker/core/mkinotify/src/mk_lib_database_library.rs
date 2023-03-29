#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Map, Value};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLibraryList {
    pub mm_media_dir_guid: uuid::Uuid,
    pub mm_media_dir_path: String,
}

pub async fn mk_lib_database_library_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBLibraryList>, sqlx::Error> {
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
        "select mm_media_dir_guid, mm_media_dir_path \
        from mm_library_dir offset $1 limit $2",
    )
    .bind(offset)
    .bind(limit);
    let table_rows: Vec<DBLibraryList> = select_query
        .map(|row: PgRow| DBLibraryList {
            mm_media_dir_guid: row.get("mm_media_dir_guid"),
            mm_media_dir_path: row.get("mm_media_dir_path"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLibraryAuditList {
    pub mm_media_dir_guid: uuid::Uuid,
    pub mm_media_dir_path: String,
    pub mm_media_dir_class_enum: i16,
    pub mm_media_dir_last_scanned: DateTime<Utc>,
}

pub async fn mk_lib_database_library_path_audit_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBLibraryAuditList>, sqlx::Error> {
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
        "select mm_media_dir_guid, mm_media_dir_path, \
        mm_media_dir_class_enum, \
        mm_media_dir_last_scanned \
        from mm_library_dir",
    );
    let table_rows: Vec<DBLibraryAuditList> = select_query
        .map(|row: PgRow| DBLibraryAuditList {
            mm_media_dir_guid: row.get("mm_media_dir_guid"),
            mm_media_dir_path: row.get("mm_media_dir_path"),
            mm_media_dir_class_enum: row.get("mm_media_dir_class_enum"),
            mm_media_dir_last_scanned: row.get("mm_media_dir_last_scanned"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLibraryPathStatus {
    mm_media_dir_path: String,
    mm_media_dir_status: String,
}

pub async fn mk_lib_database_library_path_status(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBLibraryPathStatus>, sqlx::Error> {
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
        "select mm_media_dir_path, mm_media_dir_status \
        from mm_library_dir where mm_media_dir_status IS NOT NULL \
        order by mm_media_dir_path",
    );
    let table_rows: Vec<DBLibraryPathStatus> = select_query
        .map(|row: PgRow| DBLibraryPathStatus {
            mm_media_dir_path: row.get("mm_media_dir_path"),
            mm_media_dir_status: row.get("mm_media_dir_status"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_library_path_status_update(
    sqlx_pool: &sqlx::PgPool,
    library_uuid: uuid::Uuid,
    library_status_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "update mm_library_dir set mm_media_dir_status = $1 \
        where mm_media_dir_guid = $2",
    )
    .bind(library_status_json)
    .bind(library_uuid)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_library_path_timestamp_update(
    sqlx_pool: &sqlx::PgPool,
    library_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "update mm_library_dir set mm_media_dir_last_scanned = NOW() \
        where mm_media_dir_guid = $1",
    )
    .bind(library_uuid)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_library_file_exists(
    sqlx_pool: &sqlx::PgPool,
    file_name: &String,
) -> Result<bool, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (bool,) = sqlx::query_as(
        "select exists(select 1 from mm_media \
        where mm_media_path = $1 limit 1) \
        as found_record limit 1",
    )
    .bind(file_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_library_count(sqlx_pool: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (i64,) = sqlx::query_as("select count(*) from mm_library_dir")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}
