use crate::hash_file_reader::read_file_chunks;
use sha1::{Digest, Sha1};
use std::error::Error;

/// Compute the SHA-1 digest of `file_to_read` and return it as a lowercase
/// hex string. Streams the file through a fixed-size buffer so the full
/// contents never sit in memory.
pub async fn mk_file_hash_sha1(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Sha1::new();
    read_file_chunks(file_to_read, |chunk| hasher.update(chunk)).await?;
    let hash = hasher.finalize();
    Ok(hex::encode(hash.as_slice()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_file_hash_sha1() {
        assert_eq!(
            "b2dfeef48e0ad8b260674dcf2a8fb92f1456afba",
            mk_file_hash_sha1("testing_data/HashCalc.txt")
                .await
                .unwrap()
        );
    }
}
