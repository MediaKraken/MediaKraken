#![cfg_attr(debug_assertions, allow(dead_code))]

// https://docs.rs/blake3/1.0.0/blake3/

use crate::mk_lib_logging;

use blake3;
use serde_json::json;
use std::error::Error;
use std::fs;
use stdext::function_name;

#[path = "mk_lib_file.rs"]
mod mk_lib_file;

pub fn mk_file_hash_blake3(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        );
    }
    let mut hasher = blake3::Hasher::new();
    let mut file_data = mk_lib_file::mk_read_file_data_u8(&file_to_read)?;
    hasher.update(&mut file_data);
    let checksum = hasher.finalize();
    Ok(format!("{:x}", checksum))
}
