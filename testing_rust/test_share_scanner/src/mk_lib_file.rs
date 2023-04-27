#![cfg_attr(debug_assertions, allow(dead_code))]

use serde_json::json;
use std::error::Error;
use std::io;
use stdext::function_name;

pub async fn mk_read_file_data(file_to_read: &str) -> io::Result<String> {
    let buffer = std::fs::read_to_string(file_to_read).expect("Unable to read file");
    Ok(buffer)
}
