#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[path = "mk_lib_compression.rs"]
mod mk_lib_compression;
#[path = "mk_lib_hash_crc32.rs"]
mod mk_lib_hash_crc32;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_network.rs"]
mod mk_lib_network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mklibretrocorenetfetchupdate";
    mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}), LOGGING_INDEX_NAME)
        .await;

    // populate current zipped cores into hashmap
    let mut emulation_cores = HashMap::new();
    let walker = WalkDir::new("/mediakraken/emulation/cores").into_iter();
    for entry in walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"zip")))
        .filter(|e| !e.file_type().is_dir())
    {
        println!("str: {}", &entry.path().display().to_string());
        println!(
            "crc: {:?}",
            mk_lib_hash_crc32::mk_file_hash_crc32(&entry.path().display().to_string())
        );
        let file_name = entry.path().display().to_string();
        emulation_cores.insert(
            file_name,
            mk_lib_hash_crc32::mk_file_hash_crc32(&entry.path().display().to_string()).unwrap(),
        );
    }
    println!("hash: {:?}", emulation_cores);

    // date crc32 core_filename.zip
    let libtro_url = "http://buildbot.libretro.com/nightly/linux/x86_64/latest/";
    let fetch_result =
        mk_lib_network::mk_data_from_url(format!("{}{}", libtro_url, ".index-extended"))
            .await
            .unwrap();
    for libretro_core in fetch_result.split('\n') {
        if libretro_core.len() > 0 {
            let mut download_core = false;
            let mut iter = libretro_core.splitn(3, " ");
            let core_date = iter.next().unwrap();
            let core_crc32 = iter.next().unwrap();
            let core_name = iter.next().unwrap();
            println!("line: {} {} {}", core_date, core_crc32, core_name);
            let path_core_name = format!(
                "/mediakraken/emulation/cores/{}",
                core_name.replace(".zip", "")
            );
            println!("path: {}", path_core_name);
            if emulation_cores.contains_key(&path_core_name) {
                // we have the core, check to see if crc32 changed\
                println!("emu: {}", emulation_cores[&path_core_name]);
                println!("md5: {}", core_crc32);
                if emulation_cores[&path_core_name] != core_crc32 {
                    download_core = true;
                }
            } else {
                download_core = true;
            }
            if download_core {
                // download the missing or newer core
                mk_lib_network::mk_download_file_from_url(
                    format!("{}{}", libtro_url, core_name),
                    &format!("/mediakraken/emulation/cores/{}", core_name),
                )
                .await
                .unwrap();
                // unzip the core for use
                mk_lib_compression::mk_decompress_zip(
                    &format!("/mediakraken/emulation/cores/{}", core_name),
                    true,
                    false,
                    "/mediakraken/emulation/cores/",
                )
                .unwrap();
            }
        }
    }

    // stop logging
    mk_lib_logging::mk_logging_post_elk("info", json!({"STOP": "STOP"}), LOGGING_INDEX_NAME).await;
    Ok(())
}
