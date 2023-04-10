use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use axum_extra::routing::RouterExt;
use axum_session::{Key, SessionConfig, SessionLayer, SessionPgPool, SessionStore};
use axum_session_auth::*;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::signal;

#[path = "mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[path = "bp_login.rs"]
mod bp_login;

#[tokio::main]
async fn main() {
    let pool = connect_to_database().await.unwrap();

    //This Defaults as normal Cookies.
    //To enable Private cookies for integrity, and authenticity please check the next Example.
    let session_config = SessionConfig::default()
        .with_table_name("mm_session")
        .with_key(Key::generate());
    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config);

    //Create the Database table for storing our Session Data.
    session_store.initiate().await.unwrap();
    //User::create_user_tables(&pool).await;  // will run without this just fine (after first run)

    // build our application with some routes
    let app = Router::new()
        .route("/gr", get(greet))
        // .route_with_tsr("/greet", get(bp_login::greet))
        .route_with_tsr("/login", get(login))
        // .route_with_tsr(
        //     "/public/login",
        //     get(bp_login::public_login).post(bp_login::public_login_post),
        // )
        .route_with_tsr("/perm", get(perm))
        .route_with_tsr("/perm2", get(bp_login::perm))
        .nest("/static", axum_static::static_router("static"))
        .layer(
            AuthSessionLayer::<mk_lib_database_user::User, i64, SessionPgPool, PgPool>::new(Some(
                pool.clone().into(),
            ))
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

async fn greet(
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> String {
    format!(
        "Hello {}, Try logging in via /login or testing permissions via /perm",
        auth.current_user.unwrap().username
    )
}

async fn login(
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> String {
    auth.login_user(2);
    "You are logged in as a User please try /perm to check permissions".to_owned()
}

async fn perm(
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> String {
    let current_user = auth.current_user.clone().unwrap_or_default();
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

async fn connect_to_database() -> Result<sqlx::PgPool, sqlx::Error> {
    let connection_string = "postgresql://postgres:metaman@mkstage/postgres".to_string();
    let sqlx_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;
    Ok(sqlx_pool)
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
