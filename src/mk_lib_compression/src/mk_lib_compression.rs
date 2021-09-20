use std::io::Read;
use std::fs::File;

//#![allow(unused)]
pub fn mk_decompress_tar_gz_file(archive_file: &str) -> Result<(), std::io::Error> {
    let tar_gz = std::fs::File::open(archive_file)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}

pub fn mk_decompress_gz_data(archive_file: &str) -> Result<String, std::io::Error> {
    let file_handle = std::fs::File::open(archive_file)?;
    let mut gz = flate2::read::GzDecoder::new(file_handle);
    let mut gz_data = String::new();
    gz.read_to_string(&mut gz_data)?;
    Ok(gz_data)
}

pub fn mk_decompress_zip(archive_file: &str, write_to_file: bool,
                         remove_zip: bool) -> Result<String, std::io::Error> {
    println!("gz 1 {}", archive_file);
    let mut f = std::fs::File::open(archive_file)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    //let buffer = std::fs::File::read_to_end(archive_file).expect("Unable to read file");
    println!("gz2");
    let mut gz = flate2::read::ZlibDecoder::new(buffer.as_bytes());
    println!("gz3");
    let mut gz_data = String::new();
    println!("gz4");
    gz.read_to_string(&mut gz_data)?;
    println!("gz5");
    if write_to_file {
        std::fs::write("/tmp/foo", &gz_data).expect("Unable to write file");
    }
    if remove_zip {
        std::fs::remove_file(archive_file)?;
    }
    println!("gz {}", gz_data);
    Ok(gz_data)
}

// cargo test -- --show-output
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