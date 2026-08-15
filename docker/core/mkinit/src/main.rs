use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check and upgrade if needed
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    // see if db exists
    let db_exists = mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_exists(
        &sqlx_pool_ro,
        "mm_version",
    )
    .await?;
    if !db_exists {
        let db_pass = env::var("POSTGRES_PASSWORD")?;
        let postgres_user =
            env::var("POSTGRES_USER").map_err(|e| format!("POSTGRES_USER not set: {e}"))?;
        let output = Command::new("psql")
            .env("PGPASSWORD", &db_pass)
            .args([
                "-h",
                "pgcluster-with-metrics-rw.cnpg-system",
                "-U",
                postgres_user.as_str(),
                "-d",
                "mkdatabase",
                "-f",
                "/scripts/create_schema.sql",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;
        let stdout: String = String::from_utf8(output.stdout)?;
        if !stdout.is_empty() {
            println!("create_schema.sql stdout:\n{stdout}");
        }
        let stderr: String = String::from_utf8(output.stderr)?;

        // Surface a failed schema creation directly instead of relying on the version check below to catch it.
        if !output.status.success() {
            let message = format!(
                "create_schema.sql exited with status {}:\n{}",
                output.status,
                stderr.trim()
            );
            eprintln!("{message}");
            return Err(std::io::Error::other("database initialization script failed").into());
        }

        if !stderr.is_empty() {
            println!("create_schema.sql stderr:\n{stderr}");
        }
    }
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_rw, true)
        .await?;
    Ok(())
}
