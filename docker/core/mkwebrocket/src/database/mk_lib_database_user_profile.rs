#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::mk_lib_logging;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

pub async fn mk_lib_database_user_profile_insert(
    sqlx_pool: &sqlx::PgPool,
    profile_name: String,
    profile_json: serde_json::Value,
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
        "insert into mm_user_profile(mm_user_profile_guid, \
        mm_user_profile_name, mm_user_profile_json) values($1, $2, $3)",
    )
    .bind(new_guid)
    .bind(profile_name)
    .bind(profile_json)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

pub async fn mk_lib_database_user_group_insert(
    sqlx_pool: &sqlx::PgPool,
    group_name: String,
    group_desc: String,
    group_rights_json: serde_json::Value,
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
        "insert into mm_user_group(mm_user_group_guid, \
        mm_user_group_name, mm_user_group_description, \
        mm_user_group_rights_json) values($1, $2, $3, $4)",
    )
    .bind(new_guid)
    .bind(group_name)
    .bind(group_desc)
    .bind(group_rights_json)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}
