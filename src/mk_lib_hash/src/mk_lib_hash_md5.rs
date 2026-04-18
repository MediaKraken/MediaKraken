use crate::hash_file_reader::read_file_chunks;
use md5::{Digest, Md5};
use std::error::Error;

/// Compute the MD5 digest of `file_to_read` and return it as a lowercase
/// hex string. Streams the file through a fixed-size buffer.
///
/// MD5 is not cryptographically secure; use `mk_file_hash_blake3` or
/// `mk_file_hash_sha1` for anything that needs collision resistance.
/// MD5 remains appropriate for compatibility with external tooling that
/// expects MD5 (e.g. file-identification databases).
pub async fn mk_file_hash_md5(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Md5::new();
    read_file_chunks(file_to_read, |chunk| hasher.update(chunk)).await?;
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_file_hash_md5() {
        assert_eq!(
            "4efd2e93b6b8525d93c310ef232639eb",
            mk_file_hash_md5("testing_data/HashCalc.txt").await.unwrap()
        );
    }
}
