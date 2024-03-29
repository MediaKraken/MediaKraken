use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs;
use std::path::Path;
use urlencoding::encode;

pub async fn mk_lib_database_open_pool(pool_connections: u32, connection_timeout: u64) -> Result<sqlx::PgPool, sqlx::Error> {
    // trim is get rid of the \r returned in hostname
    let hostname: String = sys_info::hostname().unwrap().trim().to_string();
    let connection_string: String;
    if hostname == "wsripper2"
        || hostname == "th-hplaptop-1"
        || hostname == "th-hplap-1"
        || hostname == "th-linuxgui-1"
        || hostname == "spoot-Inspiron-15-3525"
    {
        connection_string =
            "postgresql://postgres:metaman@mkstage/postgres?sslmode=disable".to_string();
    } else if hostname == "mkcode" {
        let db_pass = fs::read_to_string("/run/secrets/db_password").unwrap();
        connection_string = format!(
            "postgresql://postgres:{}@mkprod/postgres?sslmode=disable",
            encode(&db_pass)
        );
    } else if Path::new("/run/secrets/db_password").exists() {
        let db_pass = fs::read_to_string("/run/secrets/db_password").unwrap();
        connection_string = format!(
            "postgresql://postgres:{}@mkstack_database/postgres?sslmode=disable",
            encode(&db_pass)
        );
    } else {
        let db_pass = env::var("POSTGRES_PASSWORD").unwrap();
        connection_string = format!(
            "postgresql://postgres:{}@mkdatabase/postgres?sslmode=disable",
            encode(&db_pass)
        );
    }
    let sqlx_pool = PgPoolOptions::new()
        .max_connections(pool_connections)
        .idle_timeout(Duration::new(connection_timeout, 0))
        .connect(&connection_string)
        .await?;
    Ok(sqlx_pool)
}
