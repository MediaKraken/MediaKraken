// crc32fast = "1.2.1"
use crc32fast::Hasher;
use std::fs;
use std::error::Error;

#[path = "mk_lib_file.rs"]
mod mk_lib_file;

pub fn mk_file_hash_crc32(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Hasher::new();
    let mut file_data = mk_lib_file::mk_read_file_data_u8(&file_to_read)?;
    hasher.update(&mut file_data);
    let checksum = hasher.finalize();
    Ok(format!("{:x}", checksum))
}
