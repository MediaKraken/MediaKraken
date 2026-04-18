use std::error::Error;
use std::io;
use tokio::fs;
use tokio::task;
use walkdir::{DirEntry, WalkDir};

pub async fn mk_read_file_data(file_to_read: &str) -> io::Result<String> {
    fs::read_to_string(file_to_read).await
}

pub async fn mk_read_file_data_u8(file_to_read: &str) -> io::Result<Vec<u8>> {
    fs::read(file_to_read).await
}

pub async fn mk_save_file_data(file_data: &str, file_to_save: &str) -> io::Result<()> {
    fs::write(file_to_save, file_data).await
}

pub fn mk_file_is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// "C:\\Users\\spoot\\Documents\\MediaKraken_Deployment\\source_rust\\bulk_themoviedb_netfetch"
// TODO allow ext filters and such
//  .filter_entry(|e| !is_hidden(e))
//  .filter_map(Result::ok)
//  .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"zip")))
//  .filter(|e| !e.file_type().is_dir())
pub async fn mk_directory_walk(dir_path: String) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    // WalkDir performs blocking syscalls per entry, so run it off the async
    // runtime to avoid stalling other tasks on the current worker thread.
    task::spawn_blocking(move || -> Result<Vec<String>, walkdir::Error> {
        let mut file_list: Vec<String> = Vec::new();
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_entry(|e| !mk_file_is_hidden(e))
        {
            let entry = entry?;
            file_list.push(entry.path().display().to_string());
        }
        Ok(file_list)
    })
    .await?
    .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_read_file_data() {
        assert_eq!(
            "thisisafileforhashcalctests",
            mk_read_file_data("testing_data/HashCalc.txt")
                .await
                .unwrap()
        );
    }
}
