use sqlx::postgres::PgRow;

pub async fn mk_lib_database_table_rows(pool: &sqlx::PgPool)
                                        -> Result<Vec<PgRow>, sqlx::Error> {
    // query provided by postgresql wiki
    let rows = sqlx::query("SELECT nspname AS schemaname,relname,reltuples \
        FROM pg_class C LEFT JOIN pg_namespace N ON (N.oid = C.relnamespace) \
        WHERE nspname NOT IN (\'pg_catalog\', \'information_schema\') \
        AND relkind=\'r\' ORDER BY reltuples DESC")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_table_size(pool: &sqlx::PgPool)
                                        -> Result<Vec<PgRow>, sqlx::Error> {
    // query provided by postgresql wiki
    let rows = sqlx::query("SELECT nspname | | \'.\'  | | relname AS \'relation\', \
        pg_total_relation_size(C.oid) AS 'total_size' FROM pg_class C \
        LEFT JOIN pg_namespace N ON (N.oid = C.relnamespace) \
        WHERE nspname NOT IN (\'pg_catalog\', \'information_schema\') \
        AND C.relkind < > \'i\' AND nspname!~ \'^pg_toast\' \
        ORDER BY pg_total_relation_size(C.oid) DESC")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_parallel_workers(pool: &sqlx::PgPool)
                                              -> Result<Vec<PgRow>, sqlx::Error> {
    let rows = sqlx::query("show max_parallel_workers_per_gather")
        .fetch_one(pool)
        .await?;
    Ok(rows)
}