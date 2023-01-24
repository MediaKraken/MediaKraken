#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

// crc32fast = "1.2.1"
use crc32fast::Hasher;
use std::error::Error;
use std::fs;
use stdext::function_name;
use serde_json::json;

#[path = "mk_lib_file.rs"]
mod mk_lib_file;

pub fn mk_file_hash_crc32(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        );
    }
    let mut hasher = Hasher::new();
    let mut file_data = mk_lib_file::mk_read_file_data_u8(&file_to_read)?;
    hasher.update(&mut file_data);
    let checksum = hasher.finalize();
    Ok(format!("{:x}", checksum))
}
