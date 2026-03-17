use mk_lib_hash;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use tokio::sync::Notify;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn parse_version_components(version: &str) -> Option<Vec<u32>> {
    let parsed = version
        .split('.')
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    if parsed.is_empty() {
        return None;
    }
    Some(parsed)
}

fn latest_stable_version(index_html: &str) -> Option<String> {
    index_html
        .split("href=\"")
        .skip(1)
        .filter_map(|segment| segment.split('\"').next())
        .filter_map(|href| href.strip_suffix('/'))
        .filter(|version| version.chars().all(|c| c.is_ascii_digit() || c == '.'))
        .filter_map(|version| parse_version_components(version).map(|parts| (parts, version)))
        .max_by(|(left, _), (right, _)| left.cmp(right))
        .map(|(_, version)| version.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mklibretrocorefetchupdate")
            .await
            .unwrap();

    let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
        "mklibretrocorefetchupdate",
        &rabbit_channel,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
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
                        mk_lib_hash::mk_lib_hash_crc32::mk_file_hash_crc32(
                            &entry.path().display().to_string()
                        )
                        .await
                    );
                    let file_name = entry.path().display().to_string();
                    emulation_cores.insert(
                        file_name,
                        mk_lib_hash::mk_lib_hash_crc32::mk_file_hash_crc32(
                            &entry.path().display().to_string(),
                        )
                        .await
                        .unwrap(),
                    );
                }
                println!("hash: {:?}", emulation_cores);

                // date crc32 core_filename.zip
                let stable_root = "http://buildbot.libretro.com/stable/";
                let stable_index =
                    mk_lib_network::mk_lib_network::mk_data_from_url(stable_root.to_string())
                        .await
                        .unwrap();
                let latest_version = latest_stable_version(&stable_index).unwrap();
                let libtro_url = format!("{}{}/linux/x86_64/", stable_root, latest_version);
                let fetch_result = mk_lib_network::mk_lib_network::mk_data_from_url(format!(
                    "{}{}",
                    &libtro_url, ".index-extended"
                ))
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
                            mk_lib_network::mk_lib_network::mk_download_file_from_url(
                                format!("{}{}", &libtro_url, core_name),
                                &format!("/mediakraken/emulation/cores/{}", core_name),
                            )
                            .await
                            .unwrap();
                        }
                    }
                }
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.unwrap().delivery_tag(),
                )
                .await;
            }
        }
    });
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
