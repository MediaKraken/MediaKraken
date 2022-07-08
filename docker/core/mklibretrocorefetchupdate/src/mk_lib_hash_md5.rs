#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use md5::{Digest, Md5};
use std::error::Error;
use std::fs;

#[path = "mk_lib_file.rs"]
mod mk_lib_file;

pub fn mk_file_hash_md5(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Md5::new();
    let mut file_data = mk_lib_file::mk_read_file_data_u8(&file_to_read)?;
    hasher.update(&mut file_data);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
