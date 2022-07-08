#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};

pub async fn mk_lib_database_user_profile_insert(
    pool: &sqlx::PgPool,
    profile_name: String,
    profile_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = pool.begin().await?;
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
    pool: &sqlx::PgPool,
    group_name: String,
    group_desc: String,
    group_rights_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::new_v4();
    let mut transaction = pool.begin().await?;
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
