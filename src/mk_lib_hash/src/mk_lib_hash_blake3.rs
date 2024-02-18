// https://docs.rs/blake3/1.0.0/blake3/

use blake3;
use mk_lib_file::mk_lib_file;
use std::error::Error;

pub async fn mk_file_hash_blake3(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = blake3::Hasher::new();
    let mut file_data = mk_lib_file::mk_read_file_data_u8(&file_to_read)
        .await
        .unwrap();
    hasher.update(&mut file_data);
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
