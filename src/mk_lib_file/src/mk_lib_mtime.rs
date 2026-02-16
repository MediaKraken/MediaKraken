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
