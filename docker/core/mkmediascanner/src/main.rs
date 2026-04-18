use chrono::prelude::*;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use mk_lib_common;
use mk_lib_database;
use mk_lib_file;
use mk_lib_rabbitmq;
use num_format::{Locale, ToFormattedString};
use serde_json::{Value, json};
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

lazy_static! {
    static ref epoch: DateTime<Utc> = DateTime::<Utc>::from(UNIX_EPOCH);
}

fn mk_nfs_uri(
    share_info: &mk_lib_database::mk_lib_database_network_share::DBShareList,
    uri: &str,
) -> String {
    let share_path = share_info.mm_network_share_path.trim_start_matches('/');
    let path_part = uri.trim_start_matches('/');
    if path_part.is_empty() {
        format!("nfs://{}/{}", share_info.mm_network_share_ip, share_path)
    } else {
        format!(
            "nfs://{}/{}/{}",
            share_info.mm_network_share_ip, share_path, path_part
        )
    }
}

fn mk_nfs_tree(
    share_info: &mk_lib_database::mk_lib_database_network_share::DBShareList,
    uri: &str,
) -> Result<Vec<mk_lib_file::mk_lib_smb::File_Metadata>, Box<dyn Error>> {
    let output = Command::new("nfs-ls")
        .arg("-R")
        .arg(mk_nfs_uri(share_info, uri))
        .output()?;
    if output.status.success() == false {
        return Err(format!("nfs-ls failed with status {:?}", output.status.code()).into());
    }
    let stdout_data = String::from_utf8(output.stdout)?;
    let mut file_list = Vec::new();
    for line in stdout_data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "." || trimmed == ".." {
            continue;
        }
        let is_dir = trimmed.ends_with('/');
        let normalized = trimmed.trim_end_matches('/');
        file_list.push(mk_lib_file::mk_lib_smb::File_Metadata {
            name: format!("/{}", normalized.trim_start_matches('/')),
            directory: is_dir,
        });
    }
    if file_list.is_empty() {
        return Err("nfs-ls returned no parsable entries".into());
    }
    Ok(file_list)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup regex for finding media parts
    let stack_cd = Regex::new(r"(?i)-cd\d").unwrap();
    let stack_cd1 = Regex::new(r"(?i)-cd1(?!\d)").unwrap();
    let stack_part = Regex::new(r"(?i)-part\d").unwrap();
    let stack_part1 = Regex::new(r"(?i)-part1(?!\d)").unwrap();
    let stack_dvd = Regex::new(r"(?i)-dvd\d").unwrap();
    let stack_dvd1 = Regex::new(r"(?i)-dvd1(?!\d)").unwrap();
    let stack_pt = Regex::new(r"(?i)-pt\d").unwrap();
    let stack_pt1 = Regex::new(r"(?i)-pt1(?!\d)").unwrap();
    let stack_disk = Regex::new(r"(?i)-disk\d").unwrap();
    let stack_disk1 = Regex::new(r"(?i)-disk1(?!\d)").unwrap();
    let stack_disc = Regex::new(r"(?i)-disc\d").unwrap();
    let stack_disc1 = Regex::new(r"(?i)-disc1(?!\d)").unwrap();

    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
            .await
            .unwrap();
    let _result = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(
        &sqlx_pool_ro,
        false,
    )
    .await;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkmediascanner")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkmediascanner", &rabbit_channel)
            .await
            .unwrap();

    while let Some(msg) = rabbit_consumer.recv().await {
        if let Some(payload) = msg.content {
            // determine directories to audit
            for row_data in
                mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_audit_read(
                    &sqlx_pool_ro,
                )
                .await
                .unwrap()
            {
                let share_info: mk_lib_database::mk_lib_database_network_share::DBShareList =
                    mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_detail(
                    &sqlx_pool_ro,
                    row_data.mm_media_dir_share_guid,
                )
                .await
                .unwrap();
                // SMB first, then direct NFS listing (no host mount/PVC).
                let smb_client =
                    mk_lib_file::mk_lib_smb::mk_file_smb_client_connect(share_info.clone()).ok();
                if smb_client.is_some() || mk_nfs_tree(&share_info, "/").is_ok() {
                    // make sure the path still exists
                    let data_stat = if let Some(smb_client) = smb_client.as_ref() {
                        smb_client
                            .stat(format!("/{}", row_data.mm_media_dir_path))
                            .map(|file_stat| file_stat.modified)
                            .map_err(|_| ())
                    } else {
                        Ok(SystemTime::now())
                    };
                    match data_stat {
                        Ok(modified_time) => {
                            let last_modified =
                                mk_lib_common::mk_lib_common_date::system_time_to_date_time(
                                    modified_time,
                                );
                            if last_modified > row_data.mm_media_dir_last_scanned {
                                let _result = mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_status_update(
                                            &sqlx_pool_rw,
                                            row_data.mm_media_dir_guid,
                                            json!({"Status": "Added to scan", "Pct": 100}),
                                        )
                                        .await;
                                let original_media_class = row_data.mm_media_dir_class_enum;
                                // update the timestamp now so any other media added DURING this scan don't get skipped
                                let _result = mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_timestamp_update(
                                            &sqlx_pool_rw,
                                            row_data.mm_media_dir_guid,
                                        )
                                        .await;
                                let _result = mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_status_update(
                                            &sqlx_pool_rw,
                                            row_data.mm_media_dir_guid,
                                            json!({"Status": "File search scan", "Pct": 0.0}),
                                        )
                                        .await;
                                let mut file_data = if let Some(smb_client) = smb_client.as_ref() {
                                    mk_lib_file::mk_lib_smb::mk_file_smb_client_tree_smbclient(
                                        &share_info,
                                        format!("/{}", row_data.mm_media_dir_path).as_str(),
                                    )
                                    .or_else(|_| {
                                        mk_lib_file::mk_lib_smb::mk_file_smb_client_tree(
                                            smb_client,
                                            format!("/{}", row_data.mm_media_dir_path).as_str(),
                                        )
                                    })
                                    .unwrap_or_default()
                                } else {
                                    mk_nfs_tree(
                                        &share_info,
                                        format!("/{}", row_data.mm_media_dir_path).as_str(),
                                    )
                                    .unwrap_or_default()
                                };
                                let mut total_scanned: u64 = 0;
                                let mut total_files: u64 = 0;
                                while file_data.len() > 0 {
                                    let file_metadata = file_data[0].clone();
                                    println!("meta: {:?}", file_metadata);
                                    if file_metadata.directory == true {
                                        let mut child_files = if let Some(smb_client) =
                                            smb_client.as_ref()
                                        {
                                            mk_lib_file::mk_lib_smb::mk_file_smb_client_tree_smbclient(
                                                        &share_info,
                                                        format!("/{}", file_metadata.name).as_str(),
                                                    )
                                                    .or_else(|_| {
                                                        mk_lib_file::mk_lib_smb::mk_file_smb_client_tree(
                                                            smb_client,
                                                            format!("/{}", file_metadata.name).as_str(),
                                                        )
                                                    })
                                                    .unwrap_or_default()
                                        } else {
                                            mk_nfs_tree(
                                                &share_info,
                                                format!("/{}", file_metadata.name).as_str(),
                                            )
                                            .unwrap_or_default()
                                        };
                                        file_data.append(&mut child_files);
                                    } else {
                                        if mk_lib_database::mk_lib_database_library::mk_lib_database_library_file_exists(&sqlx_pool_ro, &file_metadata.name).await.unwrap() == false {
                                        // set lower here so I can remove a lot of .lower() in the code below
                                        let file_lower = &file_metadata.name.to_lowercase();
                                        let file_extension = Path::new(&file_lower)
                                            .extension()
                                            .and_then(OsStr::to_str)
                                            .unwrap();
                                        println!("filelower: {} {}", file_lower, file_extension);
                                        // checking subtitles for parts as need multiple files for multiple media files
                                        if mk_lib_common::mk_lib_common_media_extension::MEDIA_EXTENSION.contains(&file_extension)
                                            || mk_lib_common::mk_lib_common_media_extension::SUBTITLE_EXTENSION
                                                .contains(&file_extension)
                                            || mk_lib_common::mk_lib_common_media_extension::GAME_EXTENSION
                                                .contains(&file_extension)
                                            {
                                            println!("Matched Extension");
                                            let mut ffprobe_bif_data = true;
                                            let mut save_dl_record = true;
                                            total_files += 1;
                                            // set here which MIGHT be overrode later
                                            let mut new_class_type_uuid = original_media_class;
                                            // check for "stacked" media file
                                            let base_file_name = Path::new(&file_metadata.name)
                                                .file_name()
                                                .and_then(OsStr::to_str)
                                                .unwrap();
                                            // check to see if it's a "stacked" file
                                            // including games since some are two or more discs
                                            if stack_cd.is_match(&base_file_name).unwrap()
                                                || stack_part.is_match(&base_file_name).unwrap()
                                                || stack_dvd.is_match(&base_file_name).unwrap()
                                                || stack_pt.is_match(&base_file_name).unwrap()
                                                || stack_disk.is_match(&base_file_name).unwrap()
                                                || stack_disc.is_match(&base_file_name).unwrap()
                                            {
                                                println!("WHAT!");
                                                // check to see if it's part one or not
                                                if stack_cd1.is_match(&base_file_name).unwrap() == false
                                                    && stack_part1.is_match(&base_file_name).unwrap() == false
                                                    && stack_dvd1.is_match(&base_file_name).unwrap() == false
                                                    && stack_pt1.is_match(&base_file_name).unwrap() == false
                                                    && stack_disk1.is_match(&base_file_name).unwrap() == false
                                                    && stack_disc1.is_match(&base_file_name).unwrap() == false
                                                {
                                                    println!("WHAT2!");
                                                    // it's not a part one here so, no DL record needed
                                                    save_dl_record = false;
                                                }
                                            }
                                            // video game data
                                            // TODO look for cue/bin data as well
                                            if original_media_class
                                                == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME
                                            {
                                                if file_extension == "iso" {
                                                    new_class_type_uuid =
                                                    mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_ISO;
                                                } else {
                                                    if file_extension == "chd" {
                                                        new_class_type_uuid =
                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_CHD;
                                                    } else {
                                                        new_class_type_uuid =
                                                        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_ROM;
                                                    }
                                                }
                                                ffprobe_bif_data = false;
                                            }
                                            // set new media class for subtitles
                                            else {
                                                if mk_lib_common::mk_lib_common_media_extension::SUBTITLE_EXTENSION
                                                    .contains(&file_extension)
                                                {
                                                    if original_media_class
                                                        == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE
                                                    {
                                                        new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_SUBTITLE;
                                                    } else {
                                                        if original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV
                                                            || original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
                                                            || original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_SEASON {
                                                            new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_SUBTITLE;
                                                        }
                                                    }
                                                    ffprobe_bif_data = false;
                                                }
                                                // set new media class for trailers or themes
                                                else {
                                                    if file_metadata.name.contains("/trailers/")
                                                        || file_metadata.name.contains("\\trailers\\")
                                                        || file_metadata.name.contains("/theme.")
                                                        || file_metadata.name.contains("\\theme.")
                                                    {
                                                        if original_media_class
                                                            == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE
                                                        {
                                                            if file_metadata.name.contains("/trailers/")
                                                                || file_metadata.name.contains("\\trailers\\")
                                                            {
                                                                new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_TRAILER;
                                                            } else {
                                                                new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_THEME;
                                                            }
                                                        } else {
                                                            if original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV
                                                                || original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
                                                                || original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_SEASON {
                                                                if file_metadata.name.contains("/trailers/") || file_metadata.name.contains("\\trailers\\") {
                                                                    new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_TRAILER;
                                                                } else {
                                                                    new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_THEME;
                                                                }
                                                            }
                                                            // set new media class for extras
                                                            else {
                                                                if file_metadata.name.contains("/extras/")
                                                                    || file_metadata.name.contains("\\extras\\")
                                                                {
                                                                    if original_media_class
                                                                        == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE
                                                                    {
                                                                        new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_EXTRAS;
                                                                    } else {
                                                                        if original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV
                                                                            || original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
                                                                            || original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_SEASON {
                                                                            new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_EXTRAS;
                                                                        }
                                                                    }
                                                                }
                                                                // set new media class for backdrops (usually themes)
                                                                else {
                                                                    if file_metadata.name.contains("/backdrops/")
                                                                        || file_metadata.name.contains("\\backdrops\\")
                                                                    {
                                                                        if file_metadata.name.contains("/theme.")
                                                                            || file_metadata.name.contains("\\theme.")
                                                                        {
                                                                            if original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE {
                                                                                new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_THEME;
                                                                            } else {
                                                                                if original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV
                                                                                    || original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
                                                                                    || original_media_class == mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_SEASON {
                                                                                    new_class_type_uuid = mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_THEME;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            // create media_json data
                                            let media_json =
                                                json!({ "Added": Utc::now().to_string() });
                                            let media_id = Uuid::now_v7();
                                            let _result = mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_insert(
                                                &sqlx_pool_rw,
                                                media_id,
                                                new_class_type_uuid as i16,
                                                &file_metadata.name,
                                                None,
                                                json!({}),
                                                media_json,
                                            )
                                            .await;
                                            // verify ffprobe and bif should run on the data
                                            if mk_lib_common::mk_lib_common_media_extension::MEDIA_EXTENSION_SKIP_FFMPEG.contains(&file_extension) == false
                                                && mk_lib_common::mk_lib_common_media_extension::MEDIA_EXTENSION.contains(&file_extension) {
                                                // Send a message so ffprobe runs
                                                mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                                                    rabbit_channel.clone(),
                                                    "mktranscode",
                                                    json!({"Type": "FFMPEG", "Subtype": "Probe", "Media UUID": media_id, "Media Path": file_metadata.name}).to_string(),
                                                )
                                                .await.unwrap();
                                                if ffprobe_bif_data == true && original_media_class != mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MUSIC {
                                                    // Send a message so roku thumbnail is generated
                                                    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                                                        rabbit_channel.clone(),
                                                        "mktranscode",
                                                        json!({"Type": "Roku", "Media UUID": media_id, "Media Path": file_metadata.name}).to_string(),
                                                    )
                                                    .await.unwrap();
                                                }
                                            }
                                            // it should save a dl "Z" record for search/lookup/etc
                                            if save_dl_record == true {
                                                // media id begin and download que insert
                                                let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool_rw,
                                                                                                                                        "Z".to_string(),
                                                                                                                                        new_class_type_uuid,
                                                                                                                                        media_id,
                                                                                                                                        None,
                                                                                                                                        "Search".to_string(),
                                                                                                                                        Some(&file_metadata.name)).await;
                                            }
                                        }
                                    }
                                    }
                                    file_data.remove(0);
                                }
                                total_scanned += 1;
                                // end of for loop for each file in library
                                // set to none so it doesn't show up anymore in admin status page
                                mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_status_update(
                                            &sqlx_pool_rw,
                                            row_data.mm_media_dir_guid,
                                            json!({"Status": "File scan complete", "Pct": 100}),
                                        )
                                        .await
                                        .unwrap();
                                if total_files > 0 {
                                    // add notification to admin status page
                                    let _result = mk_lib_database::mk_lib_database_notification::mk_lib_database_notification_insert(
                                                &sqlx_pool_rw,
                                                format!(
                                                    "{} file(s) added from {}",
                                                    total_files.to_formatted_string(&Locale::en),
                                                    row_data.mm_media_dir_path
                                                ),
                                                true,
                                            )
                                            .await;
                                }
                            }
                        }
                        Err(_) => {
                            // Lib path is not found on share
                            let _result = mk_lib_database::mk_lib_database_notification::mk_lib_database_notification_insert(
                                        &sqlx_pool_rw,
                                        format!("Library path not found: {}", row_data.mm_media_dir_path),
                                        true,
                                    )
                                    .await;
                        }
                    };
                    if let Some(smb_client) = smb_client {
                        mk_lib_file::mk_lib_smb::mk_file_smb_client_disconnect(smb_client);
                    }
                } else {
                    // Fail share login
                    let _result = mk_lib_database::mk_lib_database_notification::mk_lib_database_notification_insert(
                                &sqlx_pool_rw,
                                format!("Unable to connect to share: {}", row_data.mm_media_dir_path),
                                true,
                            )
                            .await;
                }
            }
            let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                &rabbit_channel,
                msg.deliver.unwrap().delivery_tag(),
            )
            .await;
        }
    }
    Ok(())
}
