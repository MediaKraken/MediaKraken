use crate::hash_file_reader::read_file_chunks;
use crc32fast::Hasher;
use std::error::Error;

/// Compute the CRC32 checksum of `file_to_read` and return it as a
/// lowercase hex string. Streams the file through a fixed-size buffer.
pub async fn mk_file_hash_crc32(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Hasher::new();
    read_file_chunks(file_to_read, |chunk| hasher.update(chunk)).await?;
    let checksum = hasher.finalize();
    Ok(format!("{:x}", checksum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_file_hash_crc32() {
        assert_eq!(
            "ba0d5184",
            mk_file_hash_crc32("testing_data/HashCalc.txt")
                .await
                .unwrap()
        );
    }
}
