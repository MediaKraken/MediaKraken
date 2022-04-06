#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sha1::{Sha1, Digest};
use std::fs;
use std::io;

pub fn mk_file_hash_sha1(file_to_read: &str) -> io::Result<()> {
    let mut file = fs::File::open(&file_to_read)?;
    let hash = Sha1::digest_reader(&mut file)?;
    Ok(format!("{:x}", hash))
}
