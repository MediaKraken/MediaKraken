#![cfg_attr(debug_assertions, allow(dead_code))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde_json::json;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use stdext::function_name;
use walkdir::{DirEntry, WalkDir};

pub fn mk_read_file_data(file_to_read: &str) -> io::Result<String> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        );
    }
    let buffer = std::fs::read_to_string(file_to_read).expect("Unable to read file");
    Ok(buffer)
}

pub fn mk_read_file_data_u8(file_to_read: &str) -> io::Result<Vec<u8>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        );
    }
    let buffer = std::fs::read(file_to_read).expect("Unable to read file");
    Ok(buffer)
}

pub fn mk_save_file_data(file_data: &str, file_to_save: &str) -> io::Result<()> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        );
    }
    std::fs::write(file_to_save, file_data).expect("Unable to read file");
    Ok(())
}

pub fn mk_file_is_hidden(entry: &DirEntry) -> bool {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        );
    }
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// "C:\\Users\\spoot\\Documents\\MediaKraken_Deployment\\source_rust\\bulk_themoviedb_netfetch"
// TODO allow ext filters and such
//  .filter_entry(|e| !is_hidden(e))
//  .filter_map(Result::ok)
//  .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"zip")))
//  .filter(|e| !e.file_type().is_dir())
pub async fn mk_directory_walk(dir_path: String) -> Result<Vec<String>, Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut file_list: Vec<String> = Vec::new();
    let walker = WalkDir::new(dir_path).into_iter();
    for entry in walker.filter_entry(|e| !mk_file_is_hidden(e)) {
        let entry = entry.unwrap();
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({ "walk file_name": entry.path().display().to_string() }),
            )
            .await
            .unwrap();
        }
        file_list.push(entry.path().display().to_string());
    }
    Ok(file_list)
}
