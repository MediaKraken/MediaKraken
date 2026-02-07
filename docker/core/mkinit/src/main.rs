use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create metadata paths, as before the db update will let it finish before
    // other containers can use them
    if Path::new(&"/tmp/meta.tar.gz").exists() {
        println!("Meta file exists")
    }
    if Path::new(&"/mediakraken/metadata").exists() {
        println!("Meta directory exists")
    }
    if !Path::new(&"/mediakraken/metadata/backdrop/aa").exists() {
        println!("Creating directories");
        // untar the tarball to /mediakraken/metadata
        let output = Command::new("tar")
            .args(["-xzf", "/tmp/meta.tar.gz", "-C", "/mediakraken/metadata"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        let stdout: String = String::from_utf8(output.stdout).unwrap();
        let stderr: String = String::from_utf8(output.stderr).unwrap();
        println!("tar output: {}", stdout);
        println!("tar erroutput: {}", stderr);
    }

    // connect to db and do a version check and upgrade if needed
    let (sqlx_pool_rw, sqlx_pool_ro) = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
        .await
        .unwrap();
    // see if db exists
    let db_exists = mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_exits(
        &sqlx_pool_ro,
        "mm_version",
    )
    .await
    .unwrap();
    if db_exists == false {
        let db_pass = env::var("POSTGRES_PASSWORD").unwrap();
        unsafe {
            env::set_var("PGPASSWORD", &db_pass);
        }
        let output = Command::new("psql")
            .args([
                "-h",
                "pgcluster-with-metrics-rw.cnpg-system",
                "-U",
                env::var("POSTGRES_USER").unwrap().as_str(),
                "-d",
                "mkdatabase",
                "-f",
                "/scripts/create_schema.sql",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        let stdout: String = String::from_utf8(output.stdout).unwrap();
        println!("stdout: {}", stdout);
        let stderr: String = String::from_utf8(output.stderr).unwrap();
        println!("stderr: {}", stderr);
    }
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, true)
        .await
        .unwrap();
    Ok(())
}
