#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

pub fn mk_decompress_tar_gz_file(archive_file: &str) -> Result<(), std::io::Error> {
    let tar_gz = std::fs::File::open(archive_file)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}

pub fn mk_decompress_gz_data(archive_file: &str) -> Result<String, std::io::Error> {
    let file_handle = std::fs::File::open(archive_file)?;
    let mut gz = flate2::read::GzDecoder::new(file_handle);
    let mut gz_data = String::new();
    gz.read_to_string(&mut gz_data)?;
    Ok(gz_data)
}

pub fn mk_decompress_zip(archive_file: &str, write_to_file: bool,
                         remove_zip: bool, output_path: &str)
                         -> Result<String, std::io::Error> {
    let fname = std::path::Path::new(archive_file);
    let file = std::fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    if write_to_file == true {
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let mut outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };
            let mut override_path = PathBuf::from(output_path);
            override_path.push(outpath);
            outpath = override_path;
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
            if (&*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.display());
                std::fs::create_dir_all(&outpath).unwrap();
            } else {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = std::fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }

            // Get and Set permissions
            #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;

                    if let Some(mode) = file.unix_mode() {
                        std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).unwrap();
                    }
                }
        }
    } else {}
    // println!("gz 1 {}", archive_file);
    // let mut f = std::fs::File::open(archive_file)?;
    // let mut buffer = Vec::new();
    // f.read_to_end(&mut buffer)?;
    //
    // //let buffer = std::fs::File::read_to_end(archive_file).expect("Unable to read file");
    // println!("gz2");
    // let mut gz = flate2::read::ZlibDecoder::new(buffer.as_bytes());
    // println!("gz3");
    // let mut gz_data = String::new();
    // println!("gz4");
    // gz.read_to_string(&mut gz_data)?;
    // println!("gz5");
    // if write_to_file {
    //     std::fs::write("/tmp/foo", &gz_data).expect("Unable to write file");
    // }
    if remove_zip {
        std::fs::remove_file(archive_file)?;
    }
    // println!("gz {}", gz_data);
    // Ok(gz_data)
    Ok("OK".to_string())
}
