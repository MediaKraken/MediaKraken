use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use urlencoding::encode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStatusUpdatePayload {
    pub guid: uuid::Uuid,
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
    let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_pass = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let create_pool = |host: &str| {
        let connection_string = format!(
            "postgresql://{}:{}@{}:5432/mkdatabase?sslmode=prefer",
            db_user, encode(&db_pass), host
        );
        PgPoolOptions::new()
            .max_connections(pool_connections)
            .acquire_timeout(Duration::from_secs(connection_timeout))
            .max_lifetime(Duration::from_secs(1800)) 
            .connect_lazy(&connection_string)
    };
    let sqlx_pool_rw = create_pool("pgcluster-with-metrics-pgbouncer-rw.cnpg-system")?;
    let sqlx_pool_ro = create_pool("pgcluster-with-metrics-pgbouncer-ro.cnpg-system")?;
    Ok((sqlx_pool_rw, sqlx_pool_ro))
}
