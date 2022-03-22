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

/*

# TODO port query
async def db_pgsql_vacuum_stat_by_day(self, days=1, db_connection=None):
    """
    # vacuum stats by day list
    """
    if days == 0:
        return await db_conn.fetch('SELECT relname FROM pg_stat_all_tables'
                               ' WHERE schemaname = \'public\'')
    else:
        return await db_conn.fetch('SELECT relname FROM pg_stat_all_tables'
                               ' WHERE schemaname = \'public\' AND ((last_analyze is NULL'
                               ' AND last_autoanalyze is NULL)'
                               ' OR ((last_analyze < last_autoanalyze'
                               ' OR last_analyze is null)'
                               ' AND last_autoanalyze < now() - interval $1)'
                               ' OR ((last_autoanalyze < last_analyze'
                               ' OR last_autoanalyze is null)'
                               ' AND last_analyze < now() - interval $2));',
                               str(days) + ' day', str(days) + ' day')




# TODO port query
def db_pgsql_vacuum_table(self, table_name):
    """
    # vacuum table
    """
    if self.db_pgsql_table_exits(table_name) is not None:
        # self.db_pgsql_set_iso_level(ISOLATION_LEVEL_AUTOCOMMIT)
        self.db_cursor.execute('VACUUM ANALYZE ' + table_name)
        # self.db_pgsql_set_iso_level(ISOLATION_LEVEL_READ_COMMITTED)
    else:
        common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text={
            'Vacuum table missing': table_name})


# TODO port query
def db_pgsql_set_iso_level(self, isolation_level):
    """
    # set isolation level
    """
    self.sql3_conn.set_isolation_level(isolation_level)


# TODO port query
def db_pgsql_table_exits(self, table_name):
    """
    Check to see if table exits. Will return NULL if not.
    """
    self.db_cursor.execute('SELECT to_regclass(%s)::text', (table_name,))
    return self.db_cursor.fetchone()[0]

# TODO - see last analynze, etc
# SELECT schemaname, relname, last_analyze FROM pg_stat_all_tables WHERE relname = 'city';

# TODO port query
async def db_table_index_check(self, resource_name, db_connection=None):
    """
    # check for table or index
    """
    # TODO little bobby tables
    await self.db_cursor.execute('SELECT to_regclass(\'public.$1\')', resource_name)
    return await self.db_cursor.fetchval()
 */