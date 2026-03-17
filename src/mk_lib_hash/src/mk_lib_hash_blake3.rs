// https://docs.rs/blake3/1.0.0/blake3/

use crate::hash_file_reader::read_file_chunks;
use std::error::Error;

pub async fn mk_file_hash_blake3(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = blake3::Hasher::new();
    read_file_chunks(file_to_read, |chunk| {
        hasher.update(chunk);
    })
    .await?;
    let checksum = hasher.finalize();
    Ok(format!("{}", checksum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_file_hash_blake3() {
        assert_eq!(
            "8ded73d934fbe4d9cf796dd562d8fbc64f00089b049e66dab39c57b6d9a1c5b2",
            mk_file_hash_blake3("testing_data/HashCalc.txt")
                .await
                .unwrap()
        );
    }
}
