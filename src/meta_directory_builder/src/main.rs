use std::error::Error;
use std::fs;
use std::path::Path;

// run and then:
//  chown -R 1000:1000 /mediakraken/metadata/meta
//  tar czf meta.tar.gz /mediakraken/metadata/meta

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create metadata paths, as before the db update will let it finish before
    // other containers can use them
    if !Path::new(&"/mediakraken/metadata/meta").exists() {
        fs::create_dir("/mediakraken/metadata/meta")?;
        let vec_of_metadata = vec!["poster", "backdrop", "trailer"];
        for metadata_type in vec_of_metadata.iter() {
            let file_name = format!("/mediakraken/metadata/meta/{}", metadata_type);
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
    Ok(())
}
