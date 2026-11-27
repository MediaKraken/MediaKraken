use crate::AppState;
use crate::axum_custom_filters::filters;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::State;
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {
    is_admin: bool,
}

/// One database schema and its tables, for the per-schema record count and
/// size breakdown on the admin database page.
#[derive(Debug)]
struct AdminDBStatsSchemaGroup {
    schema_name: String,
    row_count_tables: Vec<mk_lib_database::mk_lib_database_postgresql::PGTableRows>,
    size_tables: Vec<mk_lib_database::mk_lib_database_postgresql::PGTableSizeSchema>,
    total_rows: f64,
    total_size: i64,
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_db_statistics.html")]
struct AdminDBStatsTemplate<'a> {
    template_data_db_version: &'a String,
    template_data_db_schemas: &'a Vec<AdminDBStatsSchemaGroup>,
    template_data_db_size_total: &'a i64,
    template_data_db_count_total: &'a f64,
    template_data_db_workers: &'a String,
    template_data_number_format_language: &'a String,
    template_data_db_extension:
        &'a Vec<mk_lib_database::mk_lib_database_postgresql::PGExtensionActive>,
    template_data_db_extension_avail:
        &'a Vec<mk_lib_database::mk_lib_database_postgresql::PGExtensionAvailable>,
    page_title: Option<String>,
}

pub async fn admin_database(
    State(state): State<AppState>,
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
        let template = TemplateError403Context { is_admin: true };
        let reply_html = template.render().unwrap();
        (StatusCode::FORBIDDEN, Html(reply_html).into_response())
    } else {
        let pg_version =
            mk_lib_database::mk_lib_database_version::mk_lib_database_postgresql_version(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let pg_table_size_by_schema =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_size_by_schema(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let pg_table_size_total =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_size_total(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let pg_table_row_count =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_rows(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let pg_table_row_count_total =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_row_count(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let pg_worker_count =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_parallel_workers(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let pg_extension =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_extension_active(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let pg_extension_avail =
            mk_lib_database::mk_lib_database_postgresql::mk_lib_database_extension_available(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let number_format_language = user_preferences::load_user_number_format_language(
            &state.sqlx_pool_ro,
            current_user.id,
        )
        .await
        .unwrap_or_else(|_| user_preferences::DEFAULT_NUMBER_FORMAT_LANGUAGE.to_string());

        // Group the record count and table size rows by schema so each schema
        // (mediakraken, musicbrainz, covert_art_archive, ...) gets its own
        // section; data-driven, so new schemas appear automatically.
        let mut schema_groups: Vec<AdminDBStatsSchemaGroup> = Vec::new();
        for row in pg_table_row_count {
            let schema = row.table_schema_name.clone();
            let count = row.table_rows;
            if let Some(group) = schema_groups
                .iter_mut()
                .find(|group| group.schema_name == schema)
            {
                group.row_count_tables.push(row);
                group.total_rows += count;
            } else {
                schema_groups.push(AdminDBStatsSchemaGroup {
                    schema_name: schema,
                    row_count_tables: vec![row],
                    size_tables: Vec::new(),
                    total_rows: count,
                    total_size: 0,
                });
            }
        }
        for row in pg_table_size_by_schema {
            let schema = row.table_schema_name.clone();
            let size = row.table_size;
            if let Some(group) = schema_groups
                .iter_mut()
                .find(|group| group.schema_name == schema)
            {
                group.size_tables.push(row);
                group.total_size += size;
            } else {
                schema_groups.push(AdminDBStatsSchemaGroup {
                    schema_name: schema,
                    row_count_tables: Vec::new(),
                    size_tables: vec![row],
                    total_rows: 0.0,
                    total_size: size,
                });
            }
        }
        schema_groups.sort_by(|a, b| a.schema_name.cmp(&b.schema_name));

        let template = AdminDBStatsTemplate {
            template_data_db_version: &pg_version,
            template_data_db_schemas: &schema_groups,
            template_data_db_size_total: &pg_table_size_total,
            template_data_db_count_total: &pg_table_row_count_total,
            template_data_db_workers: &pg_worker_count,
            template_data_number_format_language: &number_format_language,
            template_data_db_extension: &pg_extension,
            template_data_db_extension_avail: &pg_extension_avail,
            page_title: Some("MediaKraken Admin Database".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
