use md5::{Digest, Md5};
use std::error::Error;
use mk_lib_file::mk_lib_file;

pub async fn mk_file_hash_md5(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Md5::new();
    let mut file_data = mk_lib_file::mk_read_file_data_u8(&file_to_read).await.unwrap();
    hasher.update(&mut file_data);
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
            mk_file_hash_md5("testing_data/HashCalc.txt")
                .await
                .unwrap()
        );
    }
}
