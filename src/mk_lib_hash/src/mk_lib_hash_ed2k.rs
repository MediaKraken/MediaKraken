// https://github.com/runfalk/ed2k-rs

use ed2k::Ed2k;
use std::error::Error;

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
