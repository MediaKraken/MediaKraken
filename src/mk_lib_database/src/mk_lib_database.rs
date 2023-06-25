use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs;
use std::path::Path;
use stdext::function_name;
use urlencoding::encode;

pub async fn mk_lib_database_open_pool(pool_connections: u32) -> Result<sqlx::PgPool, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // trim is get rid of the \r returned in hostname
    let hostname: String = sys_info::hostname().unwrap().trim().to_string();
    let connection_string: String;
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "database open hostname": hostname }),
        )
        .await
        .unwrap();
    }
    if hostname == "wsripper2"
        || hostname == "th-hplaptop-1"
        || hostname == "th-hplap-1"
        || hostname == "th-linuxgui-1"
        || hostname == "mkcode"
    {
        connection_string = "postgresql://postgres:metaman@mkstage/postgres".to_string();
    } else if Path::new("/run/secrets/db_password").exists() {
        let db_pass = fs::read_to_string("/run/secrets/db_password").unwrap();
        connection_string = format!(
            "postgresql://postgres:{}@mkstack_database/postgres",
            encode(&db_pass)
        );
    } else {
        let db_pass = env::var("POSTGRES_PASSWORD").unwrap();
        connection_string = format!(
            "postgresql://postgres:{}@mkdatabase/postgres",
            encode(&db_pass)
        );
    }
    let sqlx_pool = PgPoolOptions::new()
        .max_connections(pool_connections)
        .connect(&connection_string)
        .await?;
    Ok(sqlx_pool)
}
