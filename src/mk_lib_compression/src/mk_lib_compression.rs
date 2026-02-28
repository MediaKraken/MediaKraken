use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub async fn mk_decompress_tar_gz_file(archive_file: &str) -> Result<(), std::io::Error> {
    let tar_gz = std::fs::File::open(archive_file)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}

pub async fn mk_decompress_tar_gz_file_gunzip(archive_file: &str) -> Result<(), std::io::Error> {
    let status = Command::new("gunzip")
        .args([&archive_file])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        return Err(io::Error::other(format!(
            "gunzip exited with status: {status}"
        )));
    }
    Ok(())
}

pub async fn mk_decompress_gz_file(archive_file: &str) -> Result<String, std::io::Error> {
    let file_handle = std::fs::File::open(archive_file)?;
    let mut gz = flate2::read::GzDecoder::new(file_handle);
    let mut gz_data = String::new();
    gz.read_to_string(&mut gz_data)?;
    Ok(gz_data)
}

pub async fn mk_decompress_gz_bytes(bytes: Vec<u8>) -> io::Result<String> {
    mk_decompress_gz_slice(&bytes)
}

pub fn mk_decompress_gz_slice(bytes: &[u8]) -> io::Result<String> {
    let mut gz = flate2::read::GzDecoder::new(bytes);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}

/*
let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
     */

pub async fn mk_decompress_zip(
    archive_file: &str,
    remove_zip: bool,
    output_path: &str,
) -> Result<(), std::io::Error> {
    let fname = std::path::Path::new(archive_file);
    let file = std::fs::File::open(fname)?;
    let mut archive = zip::ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let mut override_path = PathBuf::from(output_path);
        override_path.push(outpath);
        outpath = override_path;
        // let comment = file.comment();
        // if !comment.is_empty() {
        //     #[cfg(debug_assertions)]
        //     {
        //         mk_lib_logging::mk_logging_post_elk(
        //             std::module_path!(),
        //             json!({ "File": i, "comment": comment }),
        //         )
        //         .await
        //         .unwrap();
        //     }
        // }
        if (&*file.name()).ends_with('/') {
            // #[cfg(debug_assertions)]
            // {
            //     mk_lib_logging::mk_logging_post_elk(
            //         std::module_path!(),
            //         json!({ "File": i, "extracted to": outpath.display().to_string() }),
            //     )
            //     .await
            //     .unwrap();
            // }
            std::fs::create_dir_all(&outpath)?;
        } else {
            // #[cfg(debug_assertions)]
            // {
            //     mk_lib_logging::mk_logging_post_elk(
            //         std::module_path!(),
            //         json!({ "File": i, "extracted to": outpath.display().to_string(), "bytes": file.size() }),
            //     )
            //     .await.unwrap();
            // }
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let outfile = std::fs::File::create(&outpath)?;
            let mut writer = BufWriter::new(outfile);
            std::io::copy(&mut file, &mut writer)?;
            writer.flush()?;
        }
        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }
    if remove_zip {
        std::fs::remove_file(archive_file)?;
    }
    Ok(())
}
