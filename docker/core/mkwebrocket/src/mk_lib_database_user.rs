#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use std::{collections::HashSet, str::FromStr};
use stdext::function_name;

/*
Adult::View
Admin::View
Admin::Edit
User::View
*/

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub last_signin: DateTime<Utc>,
    pub last_signoff: DateTime<Utc>,
    pub permissions: HashSet<String>,
}

pub async fn mk_lib_database_user_exists(
    sqlx_pool: &sqlx::PgPool,
    user_name: String,
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
        "select exists(select 1 from axum_users \
        where email = $1 limit 1) limit 1",
    )
    .bind(user_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBUserList {
    id: i32,
    pub email: String,
    is_admin: bool,
}

pub async fn mk_lib_database_user_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBUserList>, sqlx::Error> {
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
        "select id, email, is_admin from axum_users order by LOWER(email) offset $1 limit $2",
    )
    .bind(offset)
    .bind(limit);
    let table_rows: Vec<DBUserList> = select_query
        .map(|row: PgRow| DBUserList {
            id: row.get("id"),
            email: row.get("email"),
            is_admin: row.get("is_admin"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_user_count(
    sqlx_pool: &sqlx::PgPool,
    user_name: String,
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
    if user_name == "" {
        let row: (i64,) = sqlx::query_as("select count(*) from axum_users")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from axum_users where email = $1")
            .bind(user_name)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_user_delete(
    sqlx_pool: &sqlx::PgPool,
    user_uuid: uuid::Uuid,
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
    sqlx::query("delete from axum_users where id = $1")
        .bind(user_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_user_set_admin(sqlx_pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
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
    sqlx::query("update axum_users set is_admin = true")
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
