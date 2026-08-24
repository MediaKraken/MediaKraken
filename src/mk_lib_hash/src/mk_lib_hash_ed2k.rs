// https://github.com/runfalk/ed2k-rs

use crate::hash_file_reader::read_file_chunks;
use ed2k::Ed2k;
use ed2k::digest::Digest;
use std::error::Error;

/// Compute the eD2k (blue) hash of `file_to_read` and return it as a hex
/// string. Streams the file through a fixed-size buffer.
pub async fn mk_file_hash_ed2k(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Ed2k::new();
    read_file_chunks(file_to_read, |chunk| hasher.update(chunk)).await?;
    let result = hasher.finalize();
    Ok(hex::encode(result.as_slice()))
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
