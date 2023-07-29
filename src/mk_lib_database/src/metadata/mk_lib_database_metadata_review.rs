use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use sqlx::types::Uuid;

pub async fn mk_lib_database_metadata_review_insert(
    sqlx_pool: &sqlx::PgPool,
    metadata_uuid: Uuid,
    review_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_review(mm_review_guid, mm_review_metadata_guid, \
        mm_review_json) values($1, $2, $3)",
    )
    .bind(new_guid)
    .bind(metadata_uuid)
    .bind(review_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_metadata_review_count(
    sqlx_pool: &sqlx::PgPool,
    metadata_uuid: Uuid,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "select count(*) from mm_review \
        where mm_review_metadata_guid = $1",
    )
    .bind(metadata_uuid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaReviewList {
    pub mm_review_guid: uuid::Uuid,
    pub mm_review_json: serde_json::Value,
}

pub async fn mk_lib_database_metadata_review_list_metadata(
    sqlx_pool: &sqlx::PgPool,
    metadata_uuid: Uuid,
) -> Result<Vec<DBMetaReviewList>, sqlx::Error> {
    // TODO order by date
    // TODO order by rating? (optional?)
    let select_query = sqlx::query(
        "select mm_review_guid, mm_review_json \
        from mm_review where mm_review_metadata_guid = $1",
    )
    .bind(metadata_uuid);
    let table_rows: Vec<DBMetaReviewList> = select_query
        .map(|row: PgRow| DBMetaReviewList {
            mm_review_guid: row.get("mm_review_guid"),
            mm_review_json: row.get("mm_review_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}
