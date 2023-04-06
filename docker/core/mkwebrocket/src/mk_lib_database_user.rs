#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use async_trait::async_trait;
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use std::{collections::HashSet, str::FromStr};
use stdext::function_name;

/*
Adult::View
Admin::View
Admin::Edit
User::View
Category::View   ??
*/

#[async_trait]
impl Authentication<User, i64, PgPool> for User {
    async fn load_user(userid: i64, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();
        User::get_user(userid, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

#[async_trait]
impl HasPermission<PgPool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        self.permissions.contains(perm)
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub email: String,
    pub last_signin: DateTime<Utc>,
    pub last_signoff: DateTime<Utc>,
    pub permissions: HashSet<String>,
}

impl Default for User {
    fn default() -> Self {
        let mut permissions = HashSet::new();
        permissions.insert("User::View".to_owned());
        Self {
            id: 1,
            anonymous: true,
            username: "Guest".into(),
            email: "guest@fake.com".into(),
            last_signin: Utc::now(),
            last_signoff: Utc::now(),
            permissions: permissions,
            }
    }
}

impl User {
    pub async fn get_user(id: i64, pool: &PgPool) -> Option<Self> {
        let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM axum_users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()?;
        // lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
        let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
            "SELECT token FROM axum_user_permissions WHERE user_id = $1",
        )
        .bind(id)
        .fetch_all(pool)
        .await
        .ok()?;
        Some(sqluser.into_user(Some(sql_user_perms)))
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlUser {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub email: String,
    pub last_signin: DateTime<Utc>,
    pub last_signoff: DateTime<Utc>,
}

impl SqlUser {
    pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
        User {
            id: self.id,
            anonymous: self.anonymous,
            username: self.username,
            email: self.email,
            last_signin: self.last_signin,
            last_signoff: self.last_signoff,
            permissions: if let Some(user_perms) = sql_user_perms {
                user_perms
                    .into_iter()
                    .map(|x| x.token)
                    .collect::<HashSet<String>>()
            } else {
                HashSet::<String>::new()
            },
        }
    }
}

pub async fn mk_lib_database_user_exists(
    sqlx_pool: &sqlx::PgPool,
    user_name: &String,
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
    id: i64,
    pub email: String,
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
        "select id, email from axum_users order by LOWER(email) offset $1 limit $2",
    )
    .bind(offset)
    .bind(limit);
    let table_rows: Vec<DBUserList> = select_query
        .map(|row: PgRow| DBUserList {
            id: row.get("id"),
            email: row.get("email"),
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

pub async fn mk_lib_database_user_set_admin(sqlx_pool: &sqlx::PgPool, user_id: i64) -> Result<(), sqlx::Error> {
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
    sqlx::query("insert into axum_user_permissions (user_id, token) values ($1, $2)")
        .bind(user_id)
        .bind("Admin::Edit")
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_user_insert(sqlx_pool: &sqlx::PgPool, 
    email: &String, password: &String) -> Result<i64, sqlx::Error> {
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
    let row: (i64,) = sqlx::query_as("insert into axum_users (id, email, password) values (NULL, $1, $2) RETURNING id")
        .bind(email)
        .bind(password)
        .fetch_one(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(row.0)
}
