use crc32fast::Hasher;
use mk_lib_file::mk_lib_file;
use std::error::Error;

pub async fn mk_file_hash_crc32(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Hasher::new();
    let mut file_data = mk_lib_file::mk_read_file_data_u8(&file_to_read)
        .await
        .unwrap();
    hasher.update(&mut file_data);
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
