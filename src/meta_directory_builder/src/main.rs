use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create metadata paths, as before the db update will let it finish beforeball
    // other containers can use them
    if !Path::new(&"/mediakraken/metadata").exists() {
        println!("Creating folders");
        fs::create_dir("/mediakraken/metadata")?;
        let vec_of_metadata = vec!["poster", "backdrop", "trailer"];
        for metadata_type in vec_of_metadata.iter() {
            let file_name = format!("/mediakraken/metadata/{}", metadata_type);
            fs::create_dir(&file_name)?;
            for c in b'a'..=b'z' {
                for d in b'a'..=b'z' {
                    for e in b'a'..=b'z' {
                        for f in b'a'..=b'z' {
                            fs::create_dir_all(format!(
                                "{}/{}{}/{}{}",
                                file_name, c as char, d as char, e as char, f as char
                            ))?;
                        }
                    }
                }
            }
        }
    }
    println!("Set rights");
    let _output = Command::new("chown")
        .args([
            "-R",
            "1000:1000",
            "/mediakraken/metadata",
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    println!("Creating Tarball");
    //  tar czf meta.tar.gz -C /mediakraken/metadata .
    let _output = Command::new("tar")
        .args([
            "-cf",
            "meta.tar",
            "-C",
            "/mediakraken/metadata",
            ".",
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    println!("Compress Tarball");
    let _output = Command::new("gzip")
        .args([
            "meta.tar",
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    Ok(())
}
