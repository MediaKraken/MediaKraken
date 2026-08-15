use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub async fn mk_lib_database_table_row_count(sqlx_pool: &sqlx::PgPool) -> Result<f64, sqlx::Error> {
    let row: (f64,) = sqlx::query_as(
        r#"
        SELECT COALESCE(sum(reltuples), 0)::float8
        FROM pg_class c
        LEFT JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname NOT IN ('pg_catalog', 'information_schema')
          AND c.relkind = 'r'
        "#,
    )
    .fetch_one(sqlx_pool)
    .await?;

    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PGTableRows {
    pub table_schema_name: String,
    pub table_name: String,
    pub table_rows: f64,
}

pub async fn mk_lib_database_table_rows(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<PGTableRows>, sqlx::Error> {
    let table_rows: Vec<PGTableRows> = sqlx::query_as(
        r#"
        SELECT
            n.nspname AS table_schema_name,
            c.relname AS table_name,
            c.reltuples::float8 AS table_rows
        FROM pg_class c
        LEFT JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname NOT IN ('pg_catalog', 'information_schema')
          AND c.relkind = 'r'
        ORDER BY c.reltuples DESC
        "#,
    )
    .fetch_all(sqlx_pool)
    .await?;

    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PGTable {
    pub table_name: String,
}

pub async fn mk_lib_database_tables(sqlx_pool: &sqlx::PgPool) -> Result<Vec<PGTable>, sqlx::Error> {
    let table_rows: Vec<PGTable> = sqlx::query_as(
        r#"
        SELECT tablename AS table_name
        FROM pg_catalog.pg_tables
        WHERE schemaname != 'pg_catalog'
          AND schemaname != 'information_schema'
        ORDER BY tablename
        "#,
    )
    .fetch_all(sqlx_pool)
    .await?;

    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PGTableSize {
    pub table_name: String,
    pub table_size: i64,
}

pub async fn mk_lib_database_table_size(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<PGTableSize>, sqlx::Error> {
    let table_rows: Vec<PGTableSize> = sqlx::query_as(
        r#"
        SELECT
            c.relname AS table_name,
            pg_total_relation_size(c.oid)::bigint AS table_size
        FROM pg_class c
        LEFT JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname NOT IN ('pg_catalog', 'information_schema')
          AND c.relkind <> 'i'
          AND n.nspname !~ '^pg_toast'
        ORDER BY pg_total_relation_size(c.oid) DESC
        "#,
    )
    .fetch_all(sqlx_pool)
    .await?;

    Ok(table_rows)
}

pub async fn mk_lib_database_table_size_total(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT COALESCE(sum(pg_total_relation_size(c.oid)), 0)::bigint AS total_size
        FROM pg_class c
        LEFT JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname NOT IN ('pg_catalog', 'information_schema')
          AND c.relkind <> 'i'
          AND n.nspname !~ '^pg_toast'
        "#,
    )
    .fetch_one(sqlx_pool)
    .await?;

    Ok(row.0)
}

pub async fn mk_lib_database_parallel_workers(
    sqlx_pool: &sqlx::PgPool,
) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("SHOW max_parallel_workers_per_gather")
        .fetch_one(sqlx_pool)
        .await?;

    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PGExtensionActive {
    pub extname: String,
    pub extversion: String,
}

pub async fn mk_lib_database_extension_active(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<PGExtensionActive>, sqlx::Error> {
    let table_rows: Vec<PGExtensionActive> =
        sqlx::query_as("SELECT extname, extversion FROM pg_extension ORDER BY extname")
            .fetch_all(sqlx_pool)
            .await?;

    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PGExtensionAvailable {
    pub name: String,
    pub default_version: String,
}

pub async fn mk_lib_database_extension_available(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<PGExtensionAvailable>, sqlx::Error> {
    let table_rows: Vec<PGExtensionAvailable> = sqlx::query_as(
        r#"
        SELECT name, default_version
        FROM pg_available_extensions
        ORDER BY name
        "#,
    )
    .fetch_all(sqlx_pool)
    .await?;

    Ok(table_rows)
}

pub async fn mk_lib_database_table_exists(
    sqlx_pool: &sqlx::PgPool,
    table_name: &str,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public'
              AND tablename = $1
        )
        "#,
    )
    .bind(table_name)
    .fetch_one(sqlx_pool)
    .await?;

    Ok(row.0)
}

/*
// TODO port query
pub async fn db_pgsql_vacuum_stat_by_day(self, days=1):
    """
    # vacuum stats by day list
    """
    if days == 0:
        return await db_conn.fetch('SELECT relname FROM pg_stat_all_tables'
                               ' WHERE schemaname = 'public'')
    else:
        return await db_conn.fetch('SELECT relname FROM pg_stat_all_tables'
                               ' WHERE schemaname = 'public' AND ((last_analyze is NULL'
                               ' AND last_autoanalyze is NULL)'
                               ' OR ((last_analyze < last_autoanalyze'
                               ' OR last_analyze is null)'
                               ' AND last_autoanalyze < now() - interval $1)'
                               ' OR ((last_autoanalyze < last_analyze'
                               ' OR last_autoanalyze is null)'
                               ' AND last_analyze < now() - interval $2));',
                               str(days) + ' day', str(days) + ' day')




// TODO port query
def db_pgsql_vacuum_table(self, table_name):
    """
    # vacuum table
    """
    if self.db_pgsql_table_exits(table_name) != None:
        # self.db_pgsql_set_iso_level(ISOLATION_LEVEL_AUTOCOMMIT)
        self.db_cursor.execute('VACUUM ANALYZE ' + table_name)
        # self.db_pgsql_set_iso_level(ISOLATION_LEVEL_READ_COMMITTED)
    else:
        common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text={
            'Vacuum table missing': table_name})

// TODO - see last analynze, etc
# SELECT schemaname, relname, last_analyze FROM pg_stat_all_tables WHERE relname = 'city';
 */
