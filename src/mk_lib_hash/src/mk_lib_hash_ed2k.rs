// https://github.com/runfalk/ed2k-rs

use ed2k::Ed2k;
use std::error::Error;

/// Compute the eD2k hash of `file_to_read` and return it as a hex string.
///
/// Unlike the other hashes in this crate, eD2k is computed by the `ed2k`
/// crate using synchronous file I/O, so this function does not stream
/// through `read_file_chunks`. A future refactor could wrap it in
/// `tokio::task::spawn_blocking` for large files.
pub async fn mk_file_hash_ed2k(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let ed2k: Ed2k = Ed2k::from_path(file_to_read)?;
    Ok(format!("{}", ed2k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_file_hash_ed2k() {
        assert_eq!(
            "82711e358a7d031aedafdb01c1e986a4",
            mk_file_hash_ed2k("testing_data/HashCalc.txt")
                .await
                .unwrap()
        );
    }
}
