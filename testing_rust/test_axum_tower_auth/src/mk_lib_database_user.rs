use async_trait::async_trait;
use axum_session_auth::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;
use std::collections::HashSet;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub email: String,
    // pub last_signin: DateTime<Utc>,
    // pub last_signoff: DateTime<Utc>,
    pub permissions: HashSet<String>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}

impl Default for User {
    fn default() -> Self {
        let mut permissions = HashSet::new();
        //permissions.insert("User::View".to_owned());
        permissions.insert("Category::View".to_owned());
        Self {
            id: 1,
            anonymous: true,
            username: "Guest".into(),
            email: "guest@fake.com".into(),
            // last_signin: Utc::now(),
            // last_signoff: Utc::now(),
            permissions: permissions,
        }
    }
}

#[async_trait]
impl Authentication<User, i64, SqlitePool> for User {
    async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
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
impl HasPermission<SqlitePool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&SqlitePool>) -> bool {
        self.permissions.contains(perm)
    }
}

impl User {
    pub async fn get_user(id: i64, pool: &SqlitePool) -> Option<Self> {
        let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()?;

        //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
        let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
            "SELECT token FROM user_permissions WHERE user_id = $1;",
        )
        .bind(id)
        .fetch_all(pool)
        .await
        .ok()?;

        Some(sqluser.into_user(Some(sql_user_perms)))
    }

    pub async fn create_user_tables(pool: &SqlitePool) {
        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS users (
                    "id" INTEGER PRIMARY KEY,
                    "anonymous" BOOLEAN NOT NULL,
                    "username" VARCHAR(256) NOT NULL,
                    "email" VARCHAR(256) NOT NULL
                )
            "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS user_permissions (
                    "user_id" INTEGER NOT NULL,
                    "token" VARCHAR(256) NOT NULL
                )
        "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                INSERT INTO users
                    (id, anonymous, username, email) SELECT 1, true, 'Guest', 'fake@email.com'
                ON CONFLICT(id) DO UPDATE SET
                    anonymous = EXCLUDED.anonymous,
                    username = EXCLUDED.username,
                    email = EXCLUDED.email
            "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                INSERT INTO users
                    (id, anonymous, username, email) SELECT 2, false, 'Test', 'fake@email.com'
                ON CONFLICT(id) DO UPDATE SET
                    anonymous = EXCLUDED.anonymous,
                    username = EXCLUDED.username,
                    email = EXCLUDED.email
            "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                INSERT INTO user_permissions
                    (user_id, token) SELECT 2, 'Category::View'
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlUser {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub email: String,
    // pub last_signin: DateTime<Utc>,
    // pub last_signoff: DateTime<Utc>,
}

impl SqlUser {
    pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
        User {
            id: self.id,
            anonymous: self.anonymous,
            username: self.username,
            email: self.email,
            // last_signin: self.last_signin,
            // last_signoff: self.last_signoff,
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
