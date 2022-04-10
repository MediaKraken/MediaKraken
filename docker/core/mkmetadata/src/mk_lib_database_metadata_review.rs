#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use rocket_dyn_templates::serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_metadata_review_insert(pool: &sqlx::PgPool,
                                                    metadata_uuid: Uuid,
                                                    review_json: serde_json::Value)
                                                    -> Result<uuid::Uuid, sqlx::Error> {
    new_guid = Uuid::new_v4();
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_review(mm_review_guid, mm_review_metadata_guid, \
        mm_review_json) values($1, $2, $3)")
        .bind(new_guid)
        .bind(metadata_uuid)
        .bind(review_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_metadata_review_count(pool: &sqlx::PgPool,
                                                   metadata_uuid: Uuid)
                                                   -> Result<i32, sqlx::Error> {
    let row: (i32, ) = sqlx::query("select count(*) from mm_review \
        where mm_review_metadata_guid = $1")
        .bind(metadata_uuid)
        .execute(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_review_list_metadata(pool: &sqlx::PgPool,
                                                           metadata_uuid: Uuid)
                                                           -> Result<Vec<PgRow>, sqlx::Error> {
    // TODO order by release date
    // TODO order by rating? (optional?)
    let rows: Vec<PgRow> = sqlx::query("select mm_review_guid, mm_review_json \
        from mm_review where mm_review_metadata_guid = $1")
        .bind(metadata_uuid)
        .execute(pool)
        .await?;
    Ok(rows)
}
