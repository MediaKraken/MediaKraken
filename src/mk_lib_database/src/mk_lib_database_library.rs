use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBLibraryList {
    pub mm_media_dir_guid: uuid::Uuid,
    pub mm_media_dir_path: String,
    pub mm_media_dir_share_guid: uuid::Uuid,
}

pub async fn mk_lib_database_library_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBLibraryList>, sqlx::Error> {
    let table_rows: Vec<DBLibraryList> = sqlx::query_as(
        r#"select mm_media_dir_guid, mm_media_dir_path, mm_media_dir_share_guid
        from mm_library_dir"#,
    )
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
    pub mm_media_dir_share_guid: uuid::Uuid,
}

pub async fn mk_lib_database_library_path_audit_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBLibraryAuditList>, sqlx::Error> {
    let table_rows: Vec<DBLibraryAuditList> = sqlx::query_as(
        r#"select mm_media_dir_guid,
        mm_media_dir_path,
        mm_media_dir_class_enum,
        mm_media_dir_last_scanned,
        mm_media_dir_share_guid
        from mm_library_dir"#,
    )
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
    let table_rows: Vec<DBLibraryPathStatus> = sqlx::query_as(
        r#"select mm_media_dir_path, mm_media_dir_status
        from mm_library_dir where mm_media_dir_status IS NOT NULL
        order by mm_media_dir_path"#,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_library_path_status_update(
    sqlx_pool: &sqlx::PgPool,
    library_uuid: uuid::Uuid,
    library_status_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"update mm_library_dir set mm_media_dir_status = $1
        where mm_media_dir_guid = $2"#,
    )
    .bind(library_status_json)
    .bind(library_uuid)
    .execute(sqlx_pool)
    .await?;
    Ok(())
}

pub async fn mk_lib_database_library_path_timestamp_update(
    sqlx_pool: &sqlx::PgPool,
    library_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"update mm_library_dir set mm_media_dir_last_scanned = NOW()
        where mm_media_dir_guid = $1"#,
    )
    .bind(library_uuid)
    .execute(sqlx_pool)
    .await?;
    Ok(())
}

pub async fn mk_lib_database_library_file_exists(
    sqlx_pool: &sqlx::PgPool,
    file_name: &str,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        r#"select exists(select 1 from mm_media
        where mm_media_path = $1)
        as found_record"#,
    )
    .bind(file_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_library_count(sqlx_pool: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_library_dir"#)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_library_path_exists(
    sqlx_pool: &sqlx::PgPool,
    library_path: &str,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        r#"select exists(select 1 from mm_library_dir
        where mm_media_dir_path = $1) as found_record"#,
    )
    .bind(library_path)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_library_path_insert(
    sqlx_pool: &sqlx::PgPool,
    library_path: &str,
    media_class: i16,
    share_guid: Uuid,
) -> Result<Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_library_dir (
        mm_media_dir_guid,
        mm_media_dir_path,
        mm_media_dir_class_enum,
        mm_media_dir_last_scanned,
        mm_media_dir_share_guid
        ) values ($1, $2, $3, NOW(), $4)"#,
    )
    .bind(new_guid)
    .bind(library_path)
    .bind(media_class)
    .bind(share_guid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}
