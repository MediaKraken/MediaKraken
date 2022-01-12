use sha1::{Sha1, Digest};
use std::fs;

pub fn mk_file_hash_sha1(file_to_read: &str) -> io::Result<()> {
    let mut file = fs::File::open(&file_to_read)?;
    let hash = Sha1::digest_reader(&mut file)?;
    Ok(format!("{:x}", hash))
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