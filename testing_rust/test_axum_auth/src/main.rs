use async_trait::async_trait;
use axum::{Extension, http::{Method, StatusCode}, routing::get, Router, response::{Html, IntoResponse}};
use axum_session::{Key, SessionPgPool, SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::*;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{collections::HashSet};
use tokio::signal;
use chrono::prelude::*;

#[path = "mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub email: String,
    pub last_signin: DateTime<Utc>,
    pub last_signoff: DateTime<Utc>,    
    pub permissions: HashSet<String>,
}

// #[derive(sqlx::FromRow, Clone)]
// pub struct SqlPermissionTokens {
//     pub token: String,
// }

// impl Default for User {
//     fn default() -> Self {
//         let mut permissions = HashSet::new();

//         permissions.insert("Category::View".to_owned());

//         Self {
//             id: 1,
//             anonymous: true,
//             username: "Guest".into(),
//             email: "fake@fake.com".into(),
//             last_signin: Utc::now(),
//             last_signoff: Utc::now(),            
//             permissions: permissions,
//         }
//     }
// }

// #[async_trait]
// impl Authentication<User, i64, PgPool> for User {
//     async fn load_user(userid: i64, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
//         let pool = pool.unwrap();

//         User::get_user(userid, pool)
//             .await
//             .ok_or_else(|| anyhow::anyhow!("Could not load user"))
//     }

//     fn is_authenticated(&self) -> bool {
//         !self.anonymous
//     }

//     fn is_active(&self) -> bool {
//         !self.anonymous
//     }

//     fn is_anonymous(&self) -> bool {
//         self.anonymous
//     }
// }

// #[async_trait]
// impl HasPermission<PgPool> for User {
//     async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
//         self.permissions.contains(perm)
//     }
// }

// impl User {
//     pub async fn get_user(id: i64, pool: &PgPool) -> Option<Self> {
//         let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = $1")
//             .bind(id)
//             .fetch_one(pool)
//             .await
//             .ok()?;

//         //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
//         let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
//             "SELECT token FROM user_permissions WHERE user_id = $1;",
//         )
//         .bind(id)
//         .fetch_all(pool)
//         .await
//         .ok()?;

//         Some(sqluser.into_user(Some(sql_user_perms)))
//     }

//     pub async fn create_user_tables(pool: &PgPool) {
//         sqlx::query(
//             r#"
//                 CREATE TABLE IF NOT EXISTS users (
//                     "id" bigint Primary Key Generated Always as Identity,
//                     "anonymous" BOOLEAN NOT NULL,
//                     "username" VARCHAR(256) NOT NULL,
//                     "email" TEXT
//                 )
//             "#,
//         )
//         .execute(pool)
//         .await
//         .unwrap();

//         sqlx::query(
//             r#"
//                 CREATE TABLE IF NOT EXISTS user_permissions (
//                     "user_id" INTEGER NOT NULL,
//                     "token" VARCHAR(256) NOT NULL
//                 )
//         "#,
//         )
//         .execute(pool)
//         .await
//         .unwrap();

//         sqlx::query(
//             r#"
//                 INSERT INTO users
//                     (anonymous, username) SELECT true, 'Guest'
//             "#,
//         )
//         .execute(pool)
//         .await
//         .unwrap();

//         sqlx::query(
//             r#"
//                 INSERT INTO users
//                     (anonymous, username) SELECT true, 'Test'
//             "#,
//         )
//         .execute(pool)
//         .await
//         .unwrap();

//         sqlx::query(
//             r#"
//                 INSERT INTO user_permissions
//                     (user_id, token) SELECT 2, 'Category::View'
//             "#,
//         )
//         .execute(pool)
//         .await
//         .unwrap();
//     }
// }

// #[derive(sqlx::FromRow, Clone)]
// pub struct SqlUser {
//     pub id: i64,
//     pub anonymous: bool,
//     pub username: String,
//     pub email: String,
//     pub last_signin: DateTime<Utc>,
//     pub last_signoff: DateTime<Utc>,    
// }

// impl SqlUser {
//     pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
//         User {
//             id: self.id,
//             anonymous: self.anonymous,
//             username: self.username,
//             email: self.email,
//             last_signin: self.last_signin,
//             last_signoff: self.last_signoff,            
//             permissions: if let Some(user_perms) = sql_user_perms {
//                 user_perms
//                     .into_iter()
//                     .map(|x| x.token)
//                     .collect::<HashSet<String>>()
//             } else {
//                 HashSet::<String>::new()
//             },
//         }
//     }
// }

#[tokio::main]
async fn main() {
    let pool = connect_to_database().await;

    //This Defaults as normal Cookies.
    //To enable Private cookies for integrity, and authenticity please check the next Example.
    let session_config = SessionConfig::default().with_table_name("test_table").with_key(Key::generate());
    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config);

    //Create the Database table for storing our Session Data.
    session_store.initiate().await.unwrap();
    //User::create_user_tables(&pool).await;  // will run without this just fine (after first run)

    // build our application with some routes
    let app = Router::new()
        .route("/", get(greet))
        .route("/greet", get(greet))
        .route("/login", get(login))
        .route("/perm", get(perm))
        .nest("/static", axum_static::static_router("static"))        
        .layer(
            AuthSessionLayer::<mk_lib_database_user::User, i64, SessionPgPool, PgPool>::new(Some(pool.clone().into()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .layer(Extension(pool));
    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn greet(auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>) -> String {
    format!(
        "Hello {}, Try logging in via /login or testing permissions via /perm",
        auth.current_user.unwrap().username
    )
}

async fn login(auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>) -> String {
    auth.login_user(2);
    "You are logged in as a User please try /perm to check permissions".to_owned()
}

async fn perm(
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> String {
    let current_user = auth.current_user.clone().unwrap_or_default();

    //lets check permissions only and not worry about if they are anon or not
    if !Auth::<mk_lib_database_user::User, i64, PgPool>::build([Method::GET], false)
        .requires(Rights::any([
            Rights::permission("Category::View"),
            Rights::permission("Admin::View"),
        ]))
        .validate(&current_user, &method, None)
        .await
    {
        return format!(
            "User {}, Does not have permissions needed to view this page please login",
            current_user.username
        );
    }

    format!(
        "User has Permissions needed. Here are the Users permissions: {:?}",
        current_user.permissions
    )
}

async fn connect_to_database() -> PgPool {
    let connection_string = "postgresql://postgres:metaman@mkstage/postgres".to_string();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .unwrap()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
