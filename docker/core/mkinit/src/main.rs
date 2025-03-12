use rcgen::generate_simple_self_signed;
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
    if !Path::new(&"/mediakraken/metadata").exists() {
        let output = Command::new("gunzip")
            .args(["/tmp/meta.tar.gz"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        // untar the tarball to /mediakraken/metadata
        let output = Command::new("tar")
            .args(["-xf", "/tmp/meta.tar -C /"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        let stdout: String = String::from_utf8(output.stdout).unwrap();
        println!("output: {}", stdout);
    }

    // check for and create ssl certs if needed
    if Path::new("/mediakraken/certs/cacert.pem").exists() == false {
        // generate certs/keys
        let subject_alt_names = vec!["www.mediakraken.org".to_string(), "localhost".to_string()];
        let cert = generate_simple_self_signed(subject_alt_names).unwrap();
        let mut file_pem = File::create("/mediakraken/certs/cacert.pem").unwrap();
        file_pem
            .write_all(cert.serialize_pem().unwrap().as_bytes())
            .unwrap();
        let mut file_key_pem = File::create("/mediakraken/certs/privkey.pem").unwrap();
        file_key_pem
            .write_all(cert.serialize_private_key_pem().as_bytes())
            .unwrap();
    }

    // connect to db and do a version check and upgrade if needed
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool_write(1, 120)
        .await
        .unwrap();
    // see if db exists
    let db_exists = mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_exits(
        &sqlx_pool,
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
                "mkdbinstance.stackgres",
                "-U",
                "postgres",
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
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, true)
        .await
        .unwrap();
    Ok(())
}
