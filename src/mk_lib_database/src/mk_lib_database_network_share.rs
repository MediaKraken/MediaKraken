#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::{FromRow, Row};
use std::num::NonZeroU8;
use stdext::function_name;
use serde_json::json;

pub async fn mk_lib_database_network_share_count(
    sqlx_pool: &sqlx::PgPool,
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
    let row: (i64,) = sqlx::query_as("select count(*) from mm_network_shares")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBShareList {
    pub mm_network_share_guid: uuid::Uuid,
    pub mm_network_share_xml: String,
}

pub async fn mk_lib_database_network_share_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBShareList>, sqlx::Error> {
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
        "select mm_network_share_guid, \
        mm_network_share_xml from mm_network_shares",
    );
    let table_rows: Vec<DBShareList> = select_query
        .map(|row: PgRow| DBShareList {
            mm_network_share_guid: row.get("mm_network_share_guid"),
            mm_network_share_xml: row.get("mm_network_share_xml"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_network_share_delete(
    sqlx_pool: &sqlx::PgPool,
    network_share_uuid: Uuid,
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
    sqlx::query("delete from mm_network_shares \
        where mm_network_share_guid = $1")
        .bind(network_share_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_network_share_insert(
    sqlx_pool: &sqlx::PgPool,
    network_share_xml: String,
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
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_network_shares \
        (mm_network_share_guid, mm_network_share_xml) \
        values ($1,$2)",
    )
    .bind(new_guid)
    .bind(network_share_xml)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}
