use md5::{Md5, Digest};
use std::fs;
use std::error::Error;

pub fn mk_file_hash_md5(file_to_read: &str) -> Result<String, Error> {
    let mut hasher = Md5::new();

    let mut file = fs::File::open(&file_to_read)?;
    //let hash = md5::digest_reader(&mut file)?;

    hasher.update(&mut file);

    let result = hasher.finalize();
    Ok(result)
}

// pub fn mk_file_hash_md5(file_to_read: &str) -> io::Result<()> {
//     let mut file = fs::File::open(&file_to_read)?;
//     let hash = md5::digest_reader(&mut file)?;
//     Ok(hash)
// }

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