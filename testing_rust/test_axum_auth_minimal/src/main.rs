use async_trait::async_trait;
use axum::{Extension, http::Method, routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use axum_session::{SessionConfig, SessionLayer, SessionSqlitePool, SessionStore};
use axum_session_auth::*;
use rcgen::generate_simple_self_signed;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{collections::HashSet, str::FromStr};
use std::{net::SocketAddr, path::PathBuf};
use tokio::net::TcpListener;
use mk_lib_database;

#[path = "mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[path = "bp_login.rs"]
mod bp_login;

#[tokio::main]
async fn main() {
    // check for and create ssl certs if needed
    if Path::new("/mediakraken/certs/cacert.pem").exists() == false {
        // generate certs/keys
        let subject_alt_names = vec!["www.mediakraken.org".to_string(), "localhost".to_string()];
        let cert = generate_simple_self_signed(subject_alt_names).unwrap();
        let mut file_pem = File::create("/mediakraken/certs/cacert.pem").unwrap();
        file_pem
            .write_all(cert.serialize_pem().unwrap().as_bytes())
            .unwrap();
        let mut file_key_pem = File::create("/mediakraken/certs/privkey.pem").unwrap();
        file_key_pem
            .write_all(cert.serialize_private_key_pem().as_bytes())
            .unwrap();
    }

    let pool = connect_to_database().await;

    //This Defaults as normal Cookies.
    //To enable Private cookies for integrity, and authenticity please check the next Example.
    let session_config = SessionConfig::default().with_table_name("test_table");
    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));

    // create SessionStore and initiate the database tables
    let session_store =
        SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config)
            .await
            .unwrap();

    mk_lib_database_user::User::create_user_tables(&pool).await;

    // build our application with some routes
    let app = Router::new()
        .route("/", get(bp_login::greet))
        .route("/greet", get(bp_login::greet))
        .route("/login", get(bp_login::login))
        .route("/perm", get(bp_login::perm))
        .layer(
            AuthSessionLayer::<mk_lib_database_user::User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone().into()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
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
