use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{AdminUser, Auth, Error, Login, Signup, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use sqlx::postgres::PgRow;

#[path = "../mk_lib_database_version.rs"]
mod mk_lib_database_version;

#[path = "../mk_lib_database_postgresql.rs"]
mod mk_lib_database_postgresql;

#[derive(Serialize)]
struct TemplateDatabaseContext {
    template_data_db_version: String,
    template_data_db_size: Vec<mk_lib_database_postgresql::PGTableSize>,
    template_data_db_count: Vec<mk_lib_database_postgresql::PGTableRows>,
    template_data_db_workers: String,
}

#[get("/database")]
pub async fn admin_database(sqlx_pool: &rocket::State<sqlx::PgPool>, user: AdminUser) -> Template {
    let pg_version = mk_lib_database_version::mk_lib_database_postgresql_version(&sqlx_pool)
        .await
        .unwrap();
    let pg_table_size = mk_lib_database_postgresql::mk_lib_database_table_size(&sqlx_pool)
        .await
        .unwrap();
    let pg_table_row_count = mk_lib_database_postgresql::mk_lib_database_table_rows(&sqlx_pool)
        .await
        .unwrap();
    let pg_worker_count = mk_lib_database_postgresql::mk_lib_database_parallel_workers(&sqlx_pool)
        .await
        .unwrap();
    Template::render(
        "bss_admin/bss_admin_db_statistics",
        &TemplateDatabaseContext {
            template_data_db_version: pg_version,
            template_data_db_size: pg_table_size,
            template_data_db_count: pg_table_row_count,
            template_data_db_workers: pg_worker_count,
        },
    )
}
