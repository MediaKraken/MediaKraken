#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::io;
use std::io::prelude::*;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

pub fn mk_read_file_data(file_to_read: &str) -> io::Result<String> {
    let buffer = std::fs::read_to_string(file_to_read).expect("Unable to read file");
    Ok(buffer)
}

pub fn mk_read_file_data_u8(file_to_read: &str) -> io::Result<Vec<u8>> {
    let buffer = std::fs::read(file_to_read).expect("Unable to read file");
    Ok(buffer)
}

pub fn mk_save_file_data(file_data: &str, file_to_save: &str) -> io::Result<()> {
    std::fs::write(file_to_save, file_data).expect("Unable to read file");
    Ok(())
}

pub fn mk_file_is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// "C:\\Users\\spoot\\Documents\\MediaKraken_Deployment\\source_rust\\bulk_themoviedb_netfetch"
// TODO allow ext filters and such
pub fn mk_directory_walk(dir_path: &str) -> Result<Vec, Box<dyn Error>> {
    let mut file_list = Vec::new();
    let walker = WalkDir::new(dir_path).into_iter();
    for entry in walker.filter_entry(|e| !mk_file_is_hidden(e)) {
        let entry = entry.unwrap();
        //println!("{}", entry.path().display());
        file_list.push(entry.path().display());
    }
    Ok(file_list)
}
