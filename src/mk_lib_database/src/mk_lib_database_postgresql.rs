
pub async fn mk_lib_database_table_rows(pool: &sqlx::PgPool)
                                        -> Result<Vec<Row>, sqlx::Error> {
    // query provided by postgresql wiki
    let rows = client
        .query("SELECT nspname AS schemaname,relname,reltuples \
        FROM pg_class C LEFT JOIN pg_namespace N ON (N.oid = C.relnamespace) \
        WHERE nspname NOT IN (\'pg_catalog\', \'information_schema\') \
        AND relkind=\'r\' ORDER BY reltuples DESC", &[])
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_table_size(pool: &sqlx::PgPool)
                                        -> Result<Vec<Row>, sqlx::Error> {
    // query provided by postgresql wiki
    let rows = client
        .query("SELECT nspname | | \'.\'  | | relname AS \'relation\', \
        pg_total_relation_size(C.oid) AS 'total_size' FROM pg_class C \
        LEFT JOIN pg_namespace N ON (N.oid = C.relnamespace) \
        WHERE nspname NOT IN (\'pg_catalog\', \'information_schema\') \
        AND C.relkind < > \'i\' AND nspname!~ \'^pg_toast\' \
        ORDER BY pg_total_relation_size(C.oid) DESC", &[])
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_parallel_workers(pool: &sqlx::PgPool)
                                              -> Result<Vec<Row>, sqlx::Error> {
    let rows = client
        .query_one("show max_parallel_workers_per_gather", &[])
        .await?;
    Ok(rows)
}