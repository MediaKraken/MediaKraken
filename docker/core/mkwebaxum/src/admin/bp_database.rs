use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use bytesize::ByteSize;
use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgRow;
use stdext::function_name;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_db_statistics.html")]
struct AdminDBStatsTemplate<'a> {
    template_data_db_version: &'a String,
    template_data_db_size: &'a Vec<mk_lib_database::mk_lib_database_postgresql::PGTableSize>,
    template_data_db_count: &'a Vec<mk_lib_database::mk_lib_database_postgresql::PGTableRows>,
    template_data_db_workers: &'a String,
}

pub async fn admin_database(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let pg_version =
        mk_lib_database::mk_lib_database_version::mk_lib_database_postgresql_version(&sqlx_pool)
            .await
            .unwrap();
    let pg_table_size =
        mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_size(&sqlx_pool)
            .await
            .unwrap();
    let pg_table_row_count =
        mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_rows(&sqlx_pool)
            .await
            .unwrap();
    let pg_worker_count =
        mk_lib_database::mk_lib_database_postgresql::mk_lib_database_parallel_workers(&sqlx_pool)
            .await
            .unwrap();
    let template = AdminDBStatsTemplate {
        template_data_db_version: &pg_version,
        template_data_db_size: &pg_table_size,
        template_data_db_count: &pg_table_row_count,
        template_data_db_workers: &pg_worker_count,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
