use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub async fn mk_lib_database_table_row_count(sqlx_pool: &sqlx::PgPool) -> Result<f32, sqlx::Error> {
    // query provided by postgresql wiki
    let row: (f32,) = sqlx::query_as(
        r#"SELECT sum(reltuples) FROM pg_class C LEFT JOIN pg_namespace N ON (N.oid = C.relnamespace) WHERE nspname NOT IN ('pg_catalog', 'information_schema') AND relkind='r'"#,
    )
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PGTableRows {
    table_schema_name: String,
    pub table_name: String,
    pub table_rows: f32,
}

pub async fn mk_lib_database_table_rows(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<PGTableRows>, sqlx::Error> {
    // query provided by postgresql wiki
    let table_rows: Vec<PGTableRows> = sqlx::query_as(
        r#"SELECT nspname AS schemaname,relname,reltuples FROM pg_class C LEFT JOIN pg_namespace N ON (N.oid = C.relnamespace) WHERE nspname NOT IN ('pg_catalog', 'information_schema') AND relkind='r' ORDER BY reltuples DESC"#,
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
    // this does NOT return sequences like the tables size does
    let table_rows: Vec<PGTable> = sqlx::query_as(
        r#"SELECT tablename
        FROM pg_catalog.pg_tables
        WHERE schemaname != 'pg_catalog' AND 
        schemaname != 'information_schema'
        order by tablename;"#,
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
    // query provided by postgresql wiki
    let table_rows: Vec<PGTableSize> = sqlx::query_as(
        r##"SELECT relname AS "relation", pg_total_relation_size(C.oid) AS "total_size" FROM pg_class C LEFT JOIN pg_namespace N ON (N.oid = C.relnamespace) WHERE nspname NOT IN ('pg_catalog', 'information_schema') AND C.relkind <> 'i' AND nspname!~ '^pg_toast' ORDER BY pg_total_relation_size(C.oid) DESC"##,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_table_size_total(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    // query provided by postgresql wiki
    let row: (i64,) = sqlx::query_as(
        r##"SELECT sum(pg_total_relation_size(C.oid))::bigint AS "total_size" FROM pg_class C LEFT JOIN pg_namespace N ON (N.oid = C.relnamespace) WHERE nspname NOT IN ('pg_catalog', 'information_schema') AND C.relkind <> 'i' AND nspname!~ '^pg_toast'"##,
    )
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_parallel_workers(
    sqlx_pool: &sqlx::PgPool,
) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as(r#"show max_parallel_workers_per_gather"#)
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
        sqlx::query_as(r#"SELECT extname, extversion from pg_extension order by extname"#)
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
        r#"SELECT name, default_version FROM pg_available_extensions order by name"#,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_table_exits(
    sqlx_pool: &sqlx::PgPool,
    table_name: &str,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        format!(
            "SELECT EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'public' \
        AND tablename = '{}' limit 1) as found_record limit 1;",
            table_name
        )
        .as_str(),
    )
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
