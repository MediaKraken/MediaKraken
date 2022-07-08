#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://docs.rs/blake3/1.0.0/blake3/

use blake3;
use std::error::Error;
use std::fs;

#[path = "mk_lib_file.rs"]
mod mk_lib_file;

pub fn mk_file_hash_blake3(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = blake3::Hasher::new();
    let mut file_data = mk_lib_file::mk_read_file_data_u8(&file_to_read)?;
    hasher.update(&mut file_data);
    let checksum = hasher.finalize();
    Ok(format!("{:x}", checksum))
}
