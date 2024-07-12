use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use urlencoding::encode;

pub async fn mk_lib_database_open_pool(
    pool_connections: u32,
    connection_timeout: u64,
) -> Result<sqlx::PgPool, sqlx::Error> {
    let db_pass = env::var("POSTGRES_PASSWORD").unwrap();
    let connection_string = format!(
        "postgresql://postgres:{}@mkstack-database-service/postgres?sslmode=disable",
        encode(&db_pass)
    );
    let sqlx_pool = PgPoolOptions::new()
        .max_connections(pool_connections)
        .idle_timeout(Duration::new(connection_timeout, 0))
        .connect(&connection_string)
        .await?;
    Ok(sqlx_pool)
}
