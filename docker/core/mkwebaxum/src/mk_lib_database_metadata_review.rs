#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::mk_lib_logging;

use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row};
use stdext::function_name;
use serde_json::json;

pub async fn mk_lib_database_metadata_review_insert(sqlx_pool: &sqlx::PgPool,
                                                    metadata_uuid: Uuid,
                                                    review_json: serde_json::Value)
                                                    -> Result<uuid::Uuid, sqlx::Error> {
                                                        #[cfg(debug_assertions)]
                                                        {
                                                            mk_lib_logging::mk_logging_post_elk(
                                                                std::module_path!(),
                                                                json!({ "Function": function_name!() }),
                                                            )
                                                            .await
                                                            .unwrap();
                                                        }
                                                        new_guid = Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
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

pub async fn mk_lib_database_metadata_review_count(sqlx_pool: &sqlx::PgPool,
                                                   metadata_uuid: Uuid)
                                                   -> Result<i32, sqlx::Error> {
                                                    #[cfg(debug_assertions)]
                                                    {
                                                        mk_lib_logging::mk_logging_post_elk(
                                                            std::module_path!(),
                                                            json!({ "Function": function_name!() }),
                                                        )
                                                        .await
                                                        .unwrap();
                                                    }
                                                    let row: (i32, ) = sqlx::query("select count(*) from mm_review \
        where mm_review_metadata_guid = $1")
        .bind(metadata_uuid)
        .execute(sqlx_pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaReviewList {
    pub mm_review_guid: uuid::Uuid,
    pub mm_review_json: serde_json::Value,
}

pub async fn mk_lib_database_metadata_review_list_metadata(sqlx_pool: &sqlx::PgPool,
                                                           metadata_uuid: Uuid)
                                                           -> Result<Vec<DBMetaReviewList>, sqlx::Error> {
                                                            #[cfg(debug_assertions)]
                                                            {
                                                                mk_lib_logging::mk_logging_post_elk(
                                                                    std::module_path!(),
                                                                    json!({ "Function": function_name!() }),
                                                                )
                                                                .await
                                                                .unwrap();
                                                            }
                                                            // TODO order by date
    // TODO order by rating? (optional?)
    let select_query = sqlx::query sqlx::query("select mm_review_guid, mm_review_json \
        from mm_review where mm_review_metadata_guid = $1")
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
