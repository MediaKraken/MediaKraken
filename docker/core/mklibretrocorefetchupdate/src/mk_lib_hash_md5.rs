use md5::{Md5, Digest};
use std::fs;
use std::error::Error;

#[cfg(debug_assertions)]
#[path = "../../../src/mk_lib_file/src/mk_lib_file.rs"]
mod mk_lib_file;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_file.rs"]
mod mk_lib_file;

pub fn mk_file_hash_md5(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Md5::new();
    let mut file_data = mk_lib_file::mk_read_file_data_u8(&file_to_read)?;
    hasher.update(&mut file_data);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

// // cargo test -- --show-output
// #[cfg(test)]
// mod test_mk_lib_common {
//     use super::*;
//
//     macro_rules! aw {
//     ($e:expr) => {
//         tokio_test::block_on($e)
//     };
//   }
// }