use crate::axum_custom_filters::filters;
use askama::Template;
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use crate::mk_lib_database;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_db_statistics.html")]
struct AdminDBStatsTemplate<'a> {
    template_data_db_version: &'a String,
    template_data_db_size: &'a Vec<mk_lib_database::mk_lib_database_postgresql::PGTableSize>,
    template_data_db_size_total: &'a i64,
    template_data_db_count: &'a Vec<mk_lib_database::mk_lib_database_postgresql::PGTableRows>,
    template_data_db_count_total: &'a f32,
    template_data_db_workers: &'a String,
    template_data_db_extension:
        &'a Vec<mk_lib_database::mk_lib_database_postgresql::PGExtensionActive>,
    template_data_db_extension_avail:
        &'a Vec<mk_lib_database::mk_lib_database_postgresql::PGExtensionAvailable>,
}

pub async fn admin_database(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError403Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let pg_version =
            mk_lib_database::mk_lib_database_version::mk_lib_database_postgresql_version(
                &sqlx_pool,
            )
            .await
            .unwrap();
        let pg_table_size =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_size(&sqlx_pool)
                .await
                .unwrap();
        let pg_table_size_total =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_size_total(
                &sqlx_pool,
            )
            .await
            .unwrap();
        let pg_table_row_count =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_rows(&sqlx_pool)
                .await
                .unwrap();
        let pg_table_row_count_total =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_row_count(
                &sqlx_pool,
            )
            .await
            .unwrap();
        let pg_worker_count =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_parallel_workers(
                &sqlx_pool,
            )
            .await
            .unwrap();
        let pg_extension =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_extension_active(
                &sqlx_pool,
            )
            .await
            .unwrap();
        let pg_extension_avail =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_extension_available(
                &sqlx_pool,
            )
            .await
            .unwrap();
        let template = AdminDBStatsTemplate {
            template_data_db_version: &pg_version,
            template_data_db_size: &pg_table_size,
            template_data_db_size_total: &pg_table_size_total,
            template_data_db_count: &pg_table_row_count,
            template_data_db_count_total: &pg_table_row_count_total,
            template_data_db_workers: &pg_worker_count,
            template_data_db_extension: &pg_extension,
            template_data_db_extension_avail: &pg_extension_avail,
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
