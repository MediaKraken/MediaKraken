use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn join_error(err: tokio::task::JoinError) -> io::Error {
    io::Error::other(format!("blocking task join error: {err}"))
}

pub async fn mk_decompress_tar_gz_file(archive_file: &str) -> io::Result<()> {
    let path = archive_file.to_owned();
    tokio::task::spawn_blocking(move || {
        let tar_gz = std::fs::File::open(&path)?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(".")
    })
    .await
    .map_err(join_error)?
}

pub async fn mk_decompress_tar_gz_file_gunzip(archive_file: &str) -> io::Result<()> {
    if archive_file.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "archive_file must not be empty",
        ));
    }
    if archive_file.starts_with('-') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "archive_file must not start with '-' (potential option injection)",
        ));
    }

    let archive_file = archive_file.to_owned();
    tokio::task::spawn_blocking(move || {
        let status = Command::new("gunzip")
            .arg("--")
            .arg(&archive_file)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "gunzip exited with status: {status}"
            )));
        }
        Ok(())
    })
    .await
    .map_err(join_error)?
}

pub async fn mk_decompress_gz_file(archive_file: &str) -> io::Result<String> {
    let path = archive_file.to_owned();
    tokio::task::spawn_blocking(move || {
        let file_handle = std::fs::File::open(&path)?;
        let mut gz = flate2::read::GzDecoder::new(file_handle);
        let mut gz_data = String::new();
        gz.read_to_string(&mut gz_data)?;
        Ok::<_, io::Error>(gz_data)
    })
    .await
    .map_err(join_error)?
}

pub async fn mk_decompress_gz_bytes(bytes: Vec<u8>) -> io::Result<String> {
    tokio::task::spawn_blocking(move || mk_decompress_gz_slice(&bytes))
        .await
        .map_err(join_error)?
}

pub fn mk_decompress_gz_slice(bytes: &[u8]) -> io::Result<String> {
    let mut gz = flate2::read::GzDecoder::new(bytes);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}

pub async fn mk_decompress_zip(
    archive_file: &str,
    remove_zip: bool,
    output_path: &str,
) -> io::Result<()> {
    let archive_file = archive_file.to_owned();
    let output_path = output_path.to_owned();
    tokio::task::spawn_blocking(move || extract_zip(&archive_file, remove_zip, &output_path))
        .await
        .map_err(join_error)?
}

fn extract_zip(archive_file: &str, remove_zip: bool, output_path: &str) -> io::Result<()> {
    let file = std::fs::File::open(Path::new(archive_file))?;
    let mut archive = zip::ZipArchive::new(file)?;

    let out_dir = Path::new(output_path);
    std::fs::create_dir_all(out_dir)?;
    let out_dir_canon = std::fs::canonicalize(out_dir)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let inside = match entry.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let outpath: PathBuf = out_dir_canon.join(&inside);
        if !outpath.starts_with(&out_dir_canon) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "zip entry escapes output directory: {}",
                    inside.display()
                ),
            ));
        }

        if entry.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let outfile = std::fs::File::create(&outpath)?;
            let mut writer = BufWriter::new(outfile);
            std::io::copy(&mut entry, &mut writer)?;
            writer.flush()?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = entry.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }
    if remove_zip {
        std::fs::remove_file(archive_file)?;
    }
    Ok(())
}
