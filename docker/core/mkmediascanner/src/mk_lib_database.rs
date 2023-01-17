#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde_json::json;
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
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "database open hostname": hostname }),
        )
        .await.unwrap();
    }
    if hostname == "wsripper2"
        || hostname == "th-hplaptop-1"
        || hostname == "th-hplap-1"
        || hostname == "th-linuxgui-1"
        || hostname == "mkcode"
    {
        connection_string = "postgresql://postgres:metaman@mkstage/postgres".to_string();
    } else if Path::new("/run/secrets/db_password").exists() {
        let dp_pass = fs::read_to_string("/run/secrets/db_password").unwrap();
        connection_string = format!(
            "postgresql://postgres:{}@mkstack_database/postgres",
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
