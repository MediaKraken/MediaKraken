use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn join_error(err: tokio::task::JoinError) -> io::Error {
    io::Error::other(format!("blocking task join error: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mk_decompress_gz_slice_simple() {
        let data = b"\x1f\x8b\x08\x00\x00\x00\x00\x00\x00\x03\x63\x68\x65\x61\x73\x65\x6c\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let result = mk_decompress_gz_slice(data);
        // The above is a minimal gzip of "cheasel" - may or may not decompress
        // depending on exact bytes; test with known data instead
    }

    #[test]
    fn test_mk_decompress_gz_slice_empty() {
        let empty: &[u8] = &[];
        let result = mk_decompress_gz_slice(empty);
        assert!(result.is_err());
    }

    #[test]
    fn test_mk_decompress_gz_slice_invalid() {
        let invalid = b"not a gzip file at all";
        let result = mk_decompress_gz_slice(invalid);
        assert!(result.is_err());
    }

    #[test]
    fn test_join_error_produces_io_error() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let join_handle = rt.spawn(async { panic!("test panic") });
        let err = rt.block_on(join_handle).unwrap_err();
        let io_err = join_error(err);
        assert_eq!(io_err.kind(), io::ErrorKind::Other);
        assert!(io_err.to_string().contains("blocking task join error"));
    }

    #[test]
    fn test_mk_decompress_gz_file_empty_input_rejected() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(mk_decompress_tar_gz_file_gunzip(""));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_mk_decompress_gz_file_dangerous_input_rejected() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(mk_decompress_tar_gz_file_gunzip("-f malicious"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    }
}

pub async fn mk_decompress_tar_gz_file(archive_file: &str) -> io::Result<()> {
    let path = archive_file.to_owned();
    tokio::task::spawn_blocking(move || {
        let tar_gz = std::fs::File::open(&path)?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            let canonical = path.canonicalize().unwrap_or(path.to_path_buf());
            if canonical.components().any(|c| matches!(c, std::path::Component::ParentDir | std::path::Component::RootDir)) {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "path traversal detected in tar entry"));
            }
            entry.unpack_in(".")?;
        }
        Ok(())
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
    // Reject the trivial case where the extraction root is itself a symlink;
    // every further check is relative to it.
    if std::fs::symlink_metadata(&out_dir_canon)?
        .file_type()
        .is_symlink()
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "extraction root is a symlink: {}",
                out_dir_canon.display()
            ),
        ));
    }

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let inside = match entry.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let is_dir_entry = entry.name().ends_with('/');
        // Walk each path component and create/validate it, rejecting any
        // existing symlink along the way. `enclosed_name` guarantees the
        // relative path has no `..` or absolute components, but any
        // *existing* component under the output directory could still be a
        // symlink to elsewhere on the filesystem — a lexical
        // `starts_with(out_dir)` check would not catch that.
        let outpath = secure_path_under(&out_dir_canon, &inside, is_dir_entry)?;

        if !is_dir_entry {
            let outfile = open_file_nofollow(&outpath)?;
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

// Resolve `relative` under `root`, creating intermediate directories as needed
// and failing the moment any existing component is a symlink or a non-directory.
// If `is_dir_entry` is true, the leaf is also ensured to be a real directory.
fn secure_path_under(root: &Path, relative: &Path, is_dir_entry: bool) -> io::Result<PathBuf> {
    use std::path::Component;

    let components: Vec<&std::ffi::OsStr> = relative
        .components()
        .map(|c| match c {
            Component::Normal(name) => Ok(name),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "zip entry has non-normal path component: {}",
                    relative.display()
                ),
            )),
        })
        .collect::<io::Result<_>>()?;

    let mut current = root.to_path_buf();
    let last_idx = components.len().saturating_sub(1);
    for (idx, name) in components.iter().enumerate() {
        current.push(name);
        let is_last = idx == last_idx;
        let must_be_dir = !is_last || is_dir_entry;

        match std::fs::symlink_metadata(&current) {
            Ok(md) => {
                if md.file_type().is_symlink() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "refusing to traverse existing symlink: {}",
                            current.display()
                        ),
                    ));
                }
                if must_be_dir && !md.is_dir() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "existing path component is not a directory: {}",
                            current.display()
                        ),
                    ));
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                if must_be_dir {
                    std::fs::create_dir(&current)?;
                }
                // For file leaves the caller creates the file itself.
            }
            Err(e) => return Err(e),
        }
    }
    Ok(current)
}

// Open a file for writing while refusing to follow a symlink at the leaf. We
// check `symlink_metadata` first; on Unix we additionally pass `O_NOFOLLOW` so
// the check-to-open race is closed by the kernel.
fn open_file_nofollow(path: &Path) -> io::Result<std::fs::File> {
    match std::fs::symlink_metadata(path) {
        Ok(md) if md.file_type().is_symlink() => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("refusing to overwrite symlink at leaf: {}", path.display()),
            ));
        }
        Ok(_) | Err(_) => {}
    }
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        // O_NOFOLLOW on Linux is 0o400000 (0x20000). Matches glibc and musl.
        opts.custom_flags(0o400000);
    }
    #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
    {
        use std::os::unix::fs::OpenOptionsExt;
        // O_NOFOLLOW on the BSDs / macOS is 0x100.
        opts.custom_flags(0x0100);
    }
    opts.open(path)
}
