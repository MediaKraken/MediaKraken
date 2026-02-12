use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use urlencoding::encode;

#[derive(Deserialize)]
pub struct MediaStatusUpdatePayload {
    pub guid: Uuid,
    pub favorite: bool,
    pub watched: bool,
    pub good: bool,
    pub bad: bool,
    pub trash: bool,
}

pub async fn mk_lib_database_open_pool(
    pool_connections: u32,
    connection_timeout: u64,
) -> Result<(sqlx::PgPool, sqlx::PgPool), sqlx::Error> {
    let db_user = env::var("POSTGRES_USER").unwrap();
    let db_pass = env::var("POSTGRES_PASSWORD").unwrap();
    // make rw pool
    let connection_string = format!(
        "postgresql://{}:{}@pgcluster-with-metrics-pgbouncer-rw.cnpg-system:5432/mkdatabase?sslmode=prefer",
        db_user,
        encode(&db_pass)
    );
    let sqlx_pool_rw = PgPoolOptions::new()
        .max_connections(pool_connections)
        .idle_timeout(Duration::new(connection_timeout, 0))
        .connect(&connection_string)
        .await?;
    // make ro pool
    let connection_string = format!(
        "postgresql://{}:{}@pgcluster-with-metrics-pgbouncer-ro.cnpg-system:5432/mkdatabase?sslmode=prefer",
        db_user,
        encode(&db_pass)
    );
    let sqlx_pool_ro = PgPoolOptions::new()
        .max_connections(pool_connections)
        .idle_timeout(Duration::new(connection_timeout, 0))
        .connect(&connection_string)
        .await?;
    Ok((sqlx_pool_rw, sqlx_pool_ro))
}
