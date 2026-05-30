use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check and upgrade if needed
    let (sqlx_pool_rw, sqlx_pool_ro) = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120)
        .await
        ?;
    // see if db exists
    let db_exists = mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_exists(
        &sqlx_pool_ro,
        "mm_version",
    )
    .await
    ?;
    if db_exists == false {
        let db_pass = env::var("POSTGRES_PASSWORD")?;
        unsafe {
            env::set_var("PGPASSWORD", &db_pass);
        }
        let output = Command::new("psql")
            .args([
                "-h",
                "pgcluster-with-metrics-rw.cnpg-system",
                "-U",
                env::var("POSTGRES_USER").map_err(|e| format!("POSTGRES_USER not set: {e}"))?.as_str(),
                "-d",
                "mkdatabase",
                "-f",
                "/scripts/create_schema.sql",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            ?;
        let stdout: String = String::from_utf8(output.stdout)?;
        println!("stdout: {}", stdout);
        let stderr: String = String::from_utf8(output.stderr)?;
        println!("stderr: {}", stderr);
    }
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_rw, true)
        .await
        ?;
    Ok(())
}
