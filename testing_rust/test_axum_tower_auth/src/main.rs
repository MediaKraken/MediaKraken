use async_trait::async_trait;
use axum::{http::Method, routing::get, Extension, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{collections::HashSet, str::FromStr};
use std::{net::SocketAddr, path::PathBuf};
use tokio::net::TcpListener;
//use mk_lib_database;
use time::Duration;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

// #[path = "mk_lib_database_user.rs"]
// mod mk_lib_database_user;

// #[path = "bp_login.rs"]
// mod bp_login;

#[tokio::main]
async fn main() {
    let pool = connect_to_database().await;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

    //mk_lib_database_user::User::create_user_tables(&pool).await;
    // Auth service.
    let backend = Backend::default();
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    // build our application with some routes
    let app = Router::new()
        // .route("/", get(bp_login::greet))
        // .route("/greet", get(bp_login::greet))
        // .route("/login", get(bp_login::login))
        // .route("/perm", get(bp_login::perm))
        .layer(session_layer)
        .layer(Extension(pool));

    // run it
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn connect_to_database() -> SqlitePool {
    let connect_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .unwrap()
}
