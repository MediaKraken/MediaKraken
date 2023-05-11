// https://github.com/runfalk/ed2k-rs


use digest::Digest;
use ed2k::Ed2k;
use std::error::Error;
use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;

pub async fn mk_file_hash_ed2k(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
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
