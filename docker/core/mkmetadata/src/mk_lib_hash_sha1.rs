#![cfg_attr(debug_assertions, allow(dead_code))]

use serde_json::json;
use sha1::{Digest, Sha1};
use std::{fs, io};
use stdext::function_name;

use crate::mk_lib_logging;

pub async fn mk_file_hash_sha1(file_to_read: &str) -> io::Result<String> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut file = fs::File::open(&file_to_read)?;
    let mut hasher = Sha1::new();
    let n = io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}
