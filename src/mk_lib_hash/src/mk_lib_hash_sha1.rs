#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sha1::{Sha1, Digest};
use std::{fs, io};

pub fn mk_file_hash_sha1(file_to_read: &str) -> io::Result<String> {
    let mut file = fs::File::open(&file_to_read)?;
    let mut hasher = Sha1::new();
    let n = io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}
