use crate::hash_file_reader::read_file_chunks;
use sha1::{Digest, Sha1};
use std::io;

pub async fn mk_file_hash_sha1(file_to_read: &str) -> io::Result<String> {
    let mut hasher = Sha1::new();
    read_file_chunks(file_to_read, |chunk| hasher.update(chunk)).await?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
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
