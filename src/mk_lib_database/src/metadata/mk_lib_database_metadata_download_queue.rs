use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use sqlx::types::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBDownloadQueueByProviderList {
    pub mm_download_guid: uuid::Uuid,
    pub mm_download_que_type: i16,
    pub mm_download_new_uuid: uuid::Uuid,
    pub mm_download_provider_id: Option<i32>,
    pub mm_download_status: String,
    pub mm_download_path: Option<String>,
}

pub async fn mk_lib_database_download_queue_by_provider(
    sqlx_pool: &sqlx::PgPool,
    provider_name: &str,
) -> Result<Vec<DBDownloadQueueByProviderList>, sqlx::Error> {
    let select_query = sqlx::query(
        "select mm_download_guid, \
        mm_download_que_type, \
        mm_download_new_uuid, \
        mm_download_provider_id, \
        mm_download_status, \
        mm_download_path \
        from mm_metadata_download_que \
        where mm_download_provider = $1 \
        order by mm_download_que_type limit 50",
    )
    .bind(provider_name);
    let table_rows: Vec<DBDownloadQueueByProviderList> = select_query
        .map(|row: PgRow| DBDownloadQueueByProviderList {
            mm_download_guid: row.get("mm_download_guid"),
            mm_download_que_type: row.get("mm_download_que_type"),
            mm_download_new_uuid: row.get("mm_download_new_uuid"),
            mm_download_provider_id: row.get("mm_download_provider_id"),
            mm_download_status: row.get("mm_download_status"),
            mm_download_path: row.get("mm_download_path"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_download_queue_delete(
    sqlx_pool: &sqlx::PgPool,
    download_guid: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("delete from mm_metadata_download_que where mm_download_guid = $1")
        .bind(download_guid)
        .execute(sqlx_pool)
        .await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_download_queue_exists(
    sqlx_pool: &sqlx::PgPool,
    metadata_provider: String,
    metadata_que_type: i16,
    metadata_provider_id: i32,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        "select exists(select 1 from mm_metadata_download_que \
        where mm_download_provider_id = $1 and mm_download_provider = $2 \
        and mm_download_que_type = $3 and mm_download_status != 'Search' limit 1) \
        as found_record limit 1",
    )
    .bind(metadata_provider_id)
    .bind(metadata_provider)
    .bind(metadata_que_type)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_download_queue_update_provider(
    sqlx_pool: &sqlx::PgPool,
    metadata_provider: String,
    metadata_queue_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "update mm_metadata_download_que \
        set mm_download_provider = $1 \
        where mm_download_guid = $2",
    )
    .bind(metadata_provider)
    .bind(metadata_queue_uuid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_download_queue_insert(
    sqlx_pool: &sqlx::PgPool,
    metadata_provider: String,
    metadata_que_type: i16,
    metadata_new_uuid: Uuid,
    metadata_provider_id: Option<i32>,
    metadata_status: String,
    metadata_path: Option<&String>,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_metadata_download_que (mm_download_guid, \
        mm_download_provider, \
        mm_download_que_type, \
        mm_download_new_uuid, \
        mm_download_provider_id, \
        mm_download_status, \
        mm_download_path) \
        values ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(metadata_provider)
    .bind(metadata_que_type)
    .bind(metadata_new_uuid)
    .bind(metadata_provider_id)
    .bind(metadata_status)
    .bind(metadata_path)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_download_status_update(
    sqlx_pool: &sqlx::PgPool,
    metadata_download_uuid: Uuid,
    metadata_status: String,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "update mm_metadata_download_que \
        set mm_download_status = $1 where mm_download_guid = $2",
    )
    .bind(metadata_status)
    .bind(metadata_download_uuid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_download_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("select count(*) from mm_metadata_download_que")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}
