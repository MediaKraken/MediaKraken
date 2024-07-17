use rcgen::generate_simple_self_signed;
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
    if !Path::new(&"/mediakraken/metadata/meta").exists() {
        // untar the tarball to /mediakraken/static
        let output = Command::new("tar")
            .args(["-xzf", "/tmp/meta.tar.gz"])
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
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1, 120)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, true)
        .await
        .unwrap();

    Ok(())
}
