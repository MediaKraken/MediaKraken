#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs;
use std::path::Path;

pub async fn mk_lib_database_open_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    // trim is get rid of the \r returned in hostname
    let hostname: String = sys_info::hostname().unwrap().trim().to_string();
    let connection_string: String;
    #[cfg(debug_assertions)]
    {
        println!("hostname: {}", hostname);
    }
    if hostname == "wsripper2"
        || hostname == "th-hplaptop-1"
        || hostname == "th-hplap-1"
        || hostname == "th-linuxgui-1"
        || hostname == "mkstage"
        || hostname == "mkcode"
    {
        connection_string = "postgresql://postgres:metaman@mkstage/postgres".to_string();
    } else if hostname == "ip-172-31-90-110" {
        connection_string = "postgresql://postgres:metamanmetaman@database-1.cklzlsrpzbdf.us-east-1.rds.amazonaws.com/postgres".to_string();
    } else if Path::new("/run/secrets/db_password").exists() {
        let dp_pass = fs::read_to_string("/run/secrets/db_password").unwrap();
        connection_string = format!(
            "postgresql://postgres:{}@mkstack_mkdatabase/postgres",
            dp_pass
        );
    } else {
        let dp_pass = env::var("POSTGRES_PASSWORD").unwrap();
        connection_string = format!("postgresql://postgres:{}@mkdatabase/postgres", dp_pass);
    }
    let sqlx_pool = PgPoolOptions::new()
        .max_connections(25)
        .connect(&connection_string)
        .await?;
    Ok(sqlx_pool)
}
