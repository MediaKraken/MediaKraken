#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;
use serde_json::json;

#[path = "mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaHomeMediaList {
    pub mm_metadata_home_guid: uuid::Uuid,
    pub mm_metadata_home_name: String,
}

pub async fn mk_lib_database_media_home_media_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i32,
    limit: i32,
) -> Result<Vec<DBMediaHomeMediaList>, sqlx::Error> {
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
        select_query = sqlx::query("").bind(search_value).bind(offset).bind(limit);
    } else {
        select_query = sqlx::query("").bind(offset).bind(limit);
    }
    let table_rows: Vec<DBMediaHomeMediaList> = select_query
        .map(|row: PgRow| DBMediaHomeMediaList {
            mm_metadata_home_guid: row.get("mm_metadata_home_guid"),
            mm_metadata_home_name: row.get("mm_metadata_home_name"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_home_media_count(
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
            "select count(*) from mm_media \
            where mmr_media_class_guid = $1
            and mm_media_path % $2",
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME)
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_media \
            where mmr_media_class_guid = $1",
        )
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}
