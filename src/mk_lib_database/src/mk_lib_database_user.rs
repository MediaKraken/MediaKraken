use async_trait::async_trait;
use axum_session_auth::Authentication;
use axum_session_auth::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgPool;
use std::collections::HashSet;

/*
Adult::View
Admin::View
Admin::Edit
User::View
Category::View   ??
*/

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    // pub email: String,
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
            // email: "guest@fake.com".into(),
            // last_signin: Utc::now(),
            // last_signoff: Utc::now(),
            permissions,
        }
    }
}

#[async_trait]
impl Authentication<User, i64, PgPool> for User {
    async fn load_user(userid: i64, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
        let pool = pool.ok_or_else(|| anyhow::anyhow!("no PgPool provided to load_user"))?;
        let user_opt = User::get_user(userid, pool).await?;
        user_opt.ok_or_else(|| anyhow::anyhow!("Could not load user"))
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

impl User {
    pub async fn get_user(id: i64, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        let sqluser = sqlx::query_as::<_, SqlUser>(
            r#"SELECT id, anonymous, username FROM mm_axum_users WHERE id = $1"#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        // lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
        let sql_user_perms: Vec<SqlPermissionTokens> = sqlx::query_as::<_, SqlPermissionTokens>(
            r#"SELECT token FROM mm_axum_user_permissions WHERE user_id = $1"#,
        )
        .bind(id)
        .fetch_all(pool)
        .await
        .ok()
        .unwrap_or_default();
        Ok(Some(sqluser.into_user(Some(sql_user_perms))))
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlUser {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    // pub email: String,
    // pub last_signin: DateTime<Utc>,
    // pub last_signoff: DateTime<Utc>,
}

impl SqlUser {
    pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
        User {
            id: self.id,
            anonymous: self.anonymous,
            username: self.username,
            // email: self.email,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_default() {
        let user = User::default();
        assert_eq!(user.id, 1);
        assert!(user.anonymous);
        assert_eq!(user.username, "Guest");
        assert!(user.permissions.contains("Category::View"));
    }

    #[test]
    fn test_user_is_authenticated_default() {
        let user = User::default();
        assert!(!user.is_authenticated());
    }

    #[test]
    fn test_user_is_active_default() {
        let user = User::default();
        assert!(!user.is_active());
    }

    #[test]
    fn test_user_is_anonymous_default() {
        let user = User::default();
        assert!(user.is_anonymous());
    }

    #[test]
    fn test_user_non_anonymous() {
        let user = User {
            id: 2,
            anonymous: false,
            username: "testuser".to_string(),
            permissions: HashSet::from(["Admin::View".to_string(), "Category::View".to_string()]),
        };
        assert!(user.is_authenticated());
        assert!(user.is_active());
        assert!(!user.is_anonymous());
    }

    #[test]
    fn test_user_has_permission() {
        let user = User {
            id: 2,
            anonymous: false,
            username: "testuser".to_string(),
            permissions: HashSet::from(["Admin::View".to_string()]),
        };
        assert!(user.has("Admin::View", &None));
        assert!(!user.has("User::View", &None));
    }

    #[test]
    fn test_user_has_no_permissions() {
        let user = User {
            id: 3,
            anonymous: false,
            username: "noperms".to_string(),
            permissions: HashSet::new(),
        };
        assert!(!user.has("Admin::View", &None));
        assert!(!user.has("Category::View", &None));
    }

    #[test]
    fn test_sql_user_into_user_no_perms() {
        let sql_user = SqlUser {
            id: 10,
            anonymous: false,
            username: "sqluser".to_string(),
        };
        let user = sql_user.into_user(None);
        assert_eq!(user.id, 10);
        assert!(!user.anonymous);
        assert_eq!(user.username, "sqluser");
        assert!(user.permissions.is_empty());
    }

    #[test]
    fn test_sql_user_into_user_with_perms() {
        let sql_user = SqlUser {
            id: 10,
            anonymous: false,
            username: "sqluser".to_string(),
        };
        let perms = vec![
            SqlPermissionTokens {
                token: "Admin::View".to_string(),
            },
            SqlPermissionTokens {
                token: "User::View".to_string(),
            },
        ];
        let user = sql_user.into_user(Some(perms));
        assert!(user.permissions.contains("Admin::View"));
        assert!(user.permissions.contains("User::View"));
    }

    #[test]
    fn test_sql_user_into_user_duplicate_perms() {
        let sql_user = SqlUser {
            id: 10,
            anonymous: false,
            username: "sqluser".to_string(),
        };
        let perms = vec![
            SqlPermissionTokens {
                token: "Admin::View".to_string(),
            },
            SqlPermissionTokens {
                token: "Admin::View".to_string(),
            },
        ];
        let user = sql_user.into_user(Some(perms));
        assert_eq!(user.permissions.len(), 1);
        assert!(user.permissions.contains("Admin::View"));
    }

    #[test]
    fn test_user_clone() {
        let user = User {
            id: 1,
            anonymous: true,
            username: "Guest".to_string(),
            permissions: HashSet::from(["Category::View".to_string()]),
        };
        let cloned = user.clone();
        assert_eq!(user.id, cloned.id);
        assert_eq!(user.anonymous, cloned.anonymous);
        assert_eq!(user.username, cloned.username);
    }

    #[test]
    fn test_db_user_list_serialization() {
        let list = DBUserList {
            id: 1,
            anonymous: false,
            username: "test".to_string(),
            email: Some("test@example.com".to_string()),
            last_signin: None,
            last_signoff: None,
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"username\":\"test\""));
    }
}

pub async fn mk_lib_database_user_exists(
    sqlx_pool: &sqlx::PgPool,
    user_name: &str,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        r#"select exists(select 1 from mm_axum_users where username = $1 limit 1) limit 1"#,
    )
    .bind(user_name)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBUserList {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub email: Option<String>,
    pub last_signin: Option<DateTime<Utc>>,
    pub last_signoff: Option<DateTime<Utc>>,
}

pub async fn mk_lib_database_user_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBUserList>, sqlx::Error> {
    let table_rows: Vec<DBUserList> = sqlx::query_as(
        r#"select id, anonymous, username, email, last_signin, last_signoff from mm_axum_users order by LOWER(username) offset $1 limit $2"#,
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_user_count(
    sqlx_pool: &sqlx::PgPool,
    user_name: String,
) -> Result<i64, sqlx::Error> {
    if user_name.is_empty() {
        let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_axum_users"#)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i64,) =
            sqlx::query_as(r#"select count(*) from mm_axum_users where username = $1"#)
                .bind(user_name)
                .fetch_one(sqlx_pool)
                .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_user_delete(
    sqlx_pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(r#"delete from mm_axum_users where id = $1"#)
        .bind(user_id)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_user_set_admin(
    sqlx_pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(r#"insert into mm_axum_user_permissions (user_id, token) values ($1, $2)"#)
        .bind(user_id)
        .bind("Admin::View")
        .execute(&mut *transaction)
        .await?;
    sqlx::query(r#"insert into mm_axum_user_permissions (user_id, token) values ($1, $2)"#)
        .bind(user_id)
        .bind("User::View")
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_user_insert(
    sqlx_pool: &sqlx::PgPool,
    username: &str,
    password: &str,
) -> Result<i64, sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    let row: (i64,) = sqlx::query_as(
        r#"insert into mm_axum_users (username, password, anonymous) values ($1, crypt($2, gen_salt('bf', 10)), false) RETURNING id"#,
    )
    .bind(username)
    .bind(password)
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(row.0)
}

pub async fn mk_lib_database_user_login_verification(
    sqlx_pool: &sqlx::PgPool,
    username: &str,
    password: &str,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select coalesce((select id from mm_axum_users where username = $1 and password = crypt($2, password) limit 1), 0)"#,
    )
    .bind(username)
    .bind(password)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_user_login(
    sqlx_pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(r#"update mm_axum_users set last_signin = now() where id = $1"#)
        .bind(user_id)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_user_authy_id(
    sqlx_pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<Option<String>, sqlx::Error> {
    let has_authy_column: (bool,) = sqlx::query_as(
        r#"select exists(
            select 1
            from information_schema.columns
            where table_schema = 'public'
                and table_name = 'mm_axum_users'
                and column_name = 'authy_id'
        )"#,
    )
    .fetch_one(sqlx_pool)
    .await?;

    if !has_authy_column.0 {
        return Ok(None);
    }

    let authy_id: (Option<String>,) =
        sqlx::query_as(r#"select authy_id::text from mm_axum_users where id = $1 limit 1"#)
            .bind(user_id)
            .fetch_one(sqlx_pool)
            .await?;

    Ok(authy_id.0.and_then(|value| {
        if value.trim().is_empty() {
            None
        } else {
            Some(value)
        }
    }))
}

pub async fn mk_lib_database_user_logout(
    sqlx_pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(r#"update mm_axum_users set last_signoff = now() where id = $1"#)
        .bind(user_id)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
