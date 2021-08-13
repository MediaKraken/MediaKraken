use std::io;
use std::io::prelude::*;
use walkdir::{DirEntry, WalkDir};

pub fn mk_read_file_data(file_to_read: &str) -> io::Result<String> {
    let buffer = std::fs::read_to_string(file_to_read).expect("Unable to read file");
    Ok(buffer)
}

pub fn mk_read_file_data_u8(file_to_read: &str) -> io::Result<String> {
    let buffer = std::fs::read(file_to_read).expect("Unable to read file");
    Ok(buffer)
}

pub fn mk_save_file_data(file_data: &str, file_to_save: &str) -> io::Result<String> {
    let buffer = std::fs::write(file_to_save, file_data).expect("Unable to read file");
    Ok(buffer)
}

pub fn mk_file_is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// "C:\\Users\\spoot\\Documents\\MediaKraken_Deployment\\source_rust\\bulk_themoviedb_netfetch"
pub fn mk_directory_walk(dir_path: &str) {
    let walker = WalkDir::new(dir_path).into_iter();
    for entry in walker.filter_entry(|e| !mk_file_is_hidden(e)) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
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