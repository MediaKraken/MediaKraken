use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, AdminUser};
use sqlx::postgres::PgRow;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../mk_lib_database_version.rs"]
mod mk_lib_database_version;

#[path = "../mk_lib_database_postgresql.rs"]
mod mk_lib_database_postgresql;

#[get("/admin_database")]
pub async fn admin_database(sqlx_pool: &rocket::State<sqlx::PgPool>) -> Template {
    let pg_version = mk_lib_database_version::mk_lib_database_postgresql_version(&sqlx_pool).await.unwrap();
    let pg_table_size = mk_lib_database_postgresql::mk_lib_database_table_size(&sqlx_pool).await.unwrap();
    let pg_row_count = mk_lib_database_postgresql::mk_lib_database_table_rows(&sqlx_pool).await.unwrap();
    let pg_worker_count = mk_lib_database_postgresql::mk_lib_database_parallel_workers(&sqlx_pool).await.unwrap();
    Template::render("bss_admin/bss_admin_db_statistics", context! {
        data_db_version: pg_version,
        data_db_size: pg_table_size,
        data_db_count: pg_row_count,
        data_workers: pg_worker_count,
    })
}

/*
    db_stats_count = []
    db_stats_total = 0
    db_connection = await request.app.db_pool.acquire()
    db_version = await request.app.db_functions.db_pgsql_version(db_connection=db_connection)
    for row_data in await request.app.db_functions.db_pgsql_row_count(db_connection=db_connection):
        db_stats_total += row_data[2]
        db_stats_count.append((row_data[1],
                               common_internationalization.com_inter_number_format(row_data[2])))
    db_stats_count.append(
        ('Total records:', common_internationalization.com_inter_number_format(db_stats_total)))
    db_size_data = []
    db_size_total = 0
    for row_data in await request.app.db_functions.db_pgsql_table_sizes(
            db_connection=db_connection):
        db_size_total += row_data['total_size']
        db_size_data.append(
            (row_data['relation'], common_string.com_string_bytes2human(row_data['total_size'])))
    db_size_data.append(('Total Size:', common_string.com_string_bytes2human(db_size_total)))
    data_workers = await request.app.db_functions.db_pgsql_parallel_workers(
        db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'data_db_version': db_version,
        'data_db_size': db_size_data,
        'data_db_count': db_stats_count,
        'data_workers': data_workers,
    }

 */