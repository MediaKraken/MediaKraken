use std::time::UNIX_EPOCH;
use tokio::task;

pub async fn file_version(path: String) -> u64 {
    task::spawn_blocking(move || {
        std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0)
    })
    .await
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_version_existing_file() {
        let version = file_version("testing_data/HashCalc.txt".to_string()).await;
        assert!(version > 0);
    }

    #[tokio::test]
    async fn test_file_version_nonexistent_file() {
        let version = file_version("/nonexistent_file_xyz.txt".to_string()).await;
        assert_eq!(version, 0);
    }

    #[tokio::test]
    async fn test_file_version_directory() {
        let version = file_version("testing_data".to_string()).await;
        assert!(version > 0);
    }
}
