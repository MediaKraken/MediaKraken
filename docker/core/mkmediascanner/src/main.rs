#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use amiquip::{AmqpProperties, Connection, Exchange, Publish, Result};
use chrono::prelude::*;
use num_format::{Locale, ToFormattedString};
// use regex::Regex;
use fancy_regex::Regex;
use serde_json::{json, Value};
use sqlx::Row;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use stdext::function_name;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[path = "mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;
#[path = "mk_lib_common_media_extension.rs"]
mod mk_lib_common_media_extension;
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_library.rs"]
mod mk_lib_database_library;
#[path = "mk_lib_database_media.rs"]
mod mk_lib_database_media;
#[path = "mk_lib_database_media_movie.rs"]
mod mk_lib_database_media_movie;
#[path = "mk_lib_database_media_tv.rs"]
mod mk_lib_database_media_tv;
#[path = "mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;
#[path = "mk_lib_database_notification.rs"]
mod mk_lib_database_notification;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_file.rs"]
mod mk_lib_file;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"})).await;
    }

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
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool(1).await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false).await;

    // open rabbit connection
    let mut rabbit_connection =
        Connection::insecure_open("amqp://guest:guest@mkstack_rabbitmq:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let rabbit_channel = rabbit_connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let rabbit_exchange = Exchange::direct(&rabbit_channel);

    // determine directories to audit
    for row_data in mk_lib_database_library::mk_lib_database_library_path_audit_read(&sqlx_pool)
        .await
        .unwrap()
    {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({ "Audit Path": row_data }),
            )
            .await;
        }
        let mut media_path: PathBuf;
        // shouldn't need to care  let unc_slice = &row_data.get("mm_media_dir_path")[..1];
        // obviously this would mean we mount unc to below as well when defining libraries
        // make sure the path still exists
        let media_path: PathBuf = ["/mediakraken/mnt", &row_data.mm_media_dir_path]
            .iter()
            .collect();

        if !Path::new(&media_path).exists() {
            mk_lib_database_notification::mk_lib_database_notification_insert(
                &sqlx_pool,
                format!("Library path not found: {}", row_data.mm_media_dir_path),
                true,
            )
            .await
            .unwrap();
        } else {
            // verify the directory inodes has changed
            let metadata = fs::metadata(&media_path)?;
            let last_modified = metadata.modified()?.elapsed()?.as_secs();
            let diff = chrono::offset::Utc::now() - row_data.mm_media_dir_last_scanned;
            if last_modified > diff.num_seconds() as u64 {
                mk_lib_database_library::mk_lib_database_library_path_status_update(
                    &sqlx_pool,
                    row_data.mm_media_dir_guid,
                    json!({"Status": "Added to scan", "Pct": 100}),
                )
                .await
                .unwrap();
                #[cfg(debug_assertions)]
                {
                    mk_lib_logging::mk_logging_post_elk(
                        std::module_path!(),
                        json!({ "worker dir": media_path }),
                    )
                    .await;
                }

                let original_media_class = row_data.mm_media_dir_class_enum;
                // update the timestamp now so any other media added DURING this scan don"t get skipped
                mk_lib_database_library::mk_lib_database_library_path_timestamp_update(
                    &sqlx_pool,
                    row_data.mm_media_dir_guid,
                )
                .await
                .unwrap();
                mk_lib_database_library::mk_lib_database_library_path_status_update(
                    &sqlx_pool,
                    row_data.mm_media_dir_guid,
                    json!({"Status": "File search scan", "Pct": 0.0}),
                )
                .await
                .unwrap();
                let file_data = mk_lib_file::mk_directory_walk(media_path.display().to_string())
                    .await
                    .unwrap();
                let total_file_in_dir: u64 = file_data.len() as u64;
                let mut total_scanned: u64 = 0;
                let mut total_files: u64 = 0;
                for mut file_name in file_data.iter() {
                    if mk_lib_database_library::mk_lib_database_library_file_exists(
                        &sqlx_pool, file_name,
                    )
                    .await
                    .unwrap()
                        == false
                    {
                        // set lower here so I can remove a lot of .lower() in the code below
                        let file_lower = &file_name.to_lowercase();
                        let file_extension = Path::new(&file_lower)
                            .extension()
                            .and_then(OsStr::to_str)
                            .unwrap();
                        // checking subtitles for parts as need multiple files for multiple media files
                        if mk_lib_common_media_extension::MEDIA_EXTENSION.contains(&file_extension)
                            || mk_lib_common_media_extension::SUBTITLE_EXTENSION
                                .contains(&file_extension)
                            || mk_lib_common_media_extension::GAME_EXTENSION
                                .contains(&file_extension)
                        {
                            let mut ffprobe_bif_data = true;
                            let mut save_dl_record = true;
                            total_files += 1;
                            // set here which MIGHT be overrode later
                            let mut new_class_type_uuid = original_media_class;
                            // check for "stacked" media file
                            let base_file_name = Path::new(&file_name)
                                .file_name()
                                .and_then(OsStr::to_str)
                                .unwrap();
                            // check to see if it"s a "stacked" file
                            // including games since some are two or more discs
                            if stack_cd.is_match(&base_file_name).unwrap()
                                || stack_part.is_match(&base_file_name).unwrap()
                                || stack_dvd.is_match(&base_file_name).unwrap()
                                || stack_pt.is_match(&base_file_name).unwrap()
                                || stack_disk.is_match(&base_file_name).unwrap()
                                || stack_disc.is_match(&base_file_name).unwrap()
                            {
                                // check to see if it"s part one or not
                                if stack_cd1.is_match(&base_file_name).unwrap() == false
                                    && stack_part1.is_match(&base_file_name).unwrap() == false
                                    && stack_dvd1.is_match(&base_file_name).unwrap() == false
                                    && stack_pt1.is_match(&base_file_name).unwrap() == false
                                    && stack_disk1.is_match(&base_file_name).unwrap() == false
                                    && stack_disc1.is_match(&base_file_name).unwrap() == false
                                {
                                    // it's not a part one here so, no DL record needed
                                    save_dl_record = false;
                                }
                            }
                            // video game data
                            // TODO look for cue/bin data as well
                            if original_media_class
                                == mk_lib_common_enum_media_type::DLMediaType::GAME
                            {
                                if file_extension == "iso" {
                                    new_class_type_uuid =
                                        mk_lib_common_enum_media_type::DLMediaType::GAME_ISO;
                                } else {
                                    if file_extension == "chd" {
                                        new_class_type_uuid =
                                            mk_lib_common_enum_media_type::DLMediaType::GAME_CHD;
                                    } else {
                                        new_class_type_uuid =
                                            mk_lib_common_enum_media_type::DLMediaType::GAME_ROM;
                                    }
                                }
                                ffprobe_bif_data = false;
                            }
                            // set new media class for subtitles
                            else {
                                if mk_lib_common_media_extension::SUBTITLE_EXTENSION
                                    .contains(&file_extension)
                                {
                                    if original_media_class
                                        == mk_lib_common_enum_media_type::DLMediaType::MOVIE
                                    {
                                        new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::MOVIE_SUBTITLE;
                                    } else {
                                        if original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV
                                            || original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
                                            || original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV_SEASON {
                                            new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::TV_SUBTITLE;
                                        }
                                    }
                                    ffprobe_bif_data = false;
                                }
                                // set new media class for trailers or themes
                                else {
                                    if file_name.contains("/trailers/")
                                        || file_name.contains("\\trailers\\")
                                        || file_name.contains("/theme.mp3")
                                        || file_name.contains("\\theme.mp3")
                                        || file_name.contains("/theme.mp4")
                                        || file_name.contains("\\theme.mp4")
                                    {
                                        if original_media_class
                                            == mk_lib_common_enum_media_type::DLMediaType::MOVIE
                                        {
                                            if file_name.contains("/trailers/")
                                                || file_name.contains("\\trailers\\")
                                            {
                                                new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::MOVIE_TRAILER;
                                            } else {
                                                new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::MOVIE_THEME;
                                            }
                                        } else {
                                            if original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV
                                                || original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
                                                || original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV_SEASON {
                                                if file_name.contains("/trailers/") || file_name.contains("\\trailers\\") {
                                                    new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::TV_TRAILER;
                                                } else {
                                                    new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::TV_THEME;
                                                }
                                            }
                                        }
                                    }
                                    // set new media class for extras
                                    else {
                                        if file_name.contains("/extras/")
                                            || file_name.contains("\\extras\\")
                                        {
                                            if original_media_class
                                                == mk_lib_common_enum_media_type::DLMediaType::MOVIE
                                            {
                                                new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::MOVIE_EXTRAS;
                                            } else {
                                                if original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV
                                                    || original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
                                                    || original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV_SEASON {
                                                    new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::TV_EXTRAS;
                                                }
                                            }
                                        }
                                        // set new media class for backdrops (usually themes)
                                        else {
                                            if file_name.contains("/backdrops/")
                                                || file_name.contains("\\backdrops\\")
                                            {
                                                if file_name.contains("/theme.mp3")
                                                    || file_name.contains("\\theme.mp3")
                                                    || file_name.contains("/theme.mp4")
                                                    || file_name.contains("\\theme.mp4")
                                                {
                                                    if original_media_class == mk_lib_common_enum_media_type::DLMediaType::MOVIE {
                                                        new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::MOVIE_THEME;
                                                    } else {
                                                        if original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV
                                                            || original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
                                                            || original_media_class == mk_lib_common_enum_media_type::DLMediaType::TV_SEASON {
                                                            new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::TV_THEME;
                                                        }
                                                    }
                                                }
                                            }
                                            // flip around slashes for smb paths
                                            // if file_name == "\\" {
                                            //     file_name = &file_name
                                            //         .replace("\\\\", "smb://guest:\"\"@")
                                            //         .replace("\\", "/");
                                            // }
                                            // create media_json data
                                            let media_json =
                                                json!({ "Added": Utc::now().to_string() });
                                            let media_id = Uuid::new_v4();
                                            mk_lib_database_media::mk_lib_database_media_insert(
                                                &sqlx_pool,
                                                media_id,
                                                new_class_type_uuid as i16,
                                                file_name,
                                                None,
                                                json!({}),
                                                media_json,
                                            )
                                            .await
                                            .unwrap();
                                            // verify ffprobe and bif should run on the data
                                            if mk_lib_common_media_extension::MEDIA_EXTENSION_SKIP_FFMPEG.contains(&file_extension) == false
                                                && mk_lib_common_media_extension::MEDIA_EXTENSION.contains(&file_extension) {
                                                // Send a message so ffprobe runs
                                                rabbit_exchange.publish(Publish::with_properties(json!({"Type": "FFProbe", "Media UUID": media_id, "Media Path": file_name}).to_string().as_bytes(),
                                                                                                 "mk_ffmpeg".to_string(),
                                                                                                 AmqpProperties::default().with_delivery_mode(2).with_content_type("text/plain".to_string())))?;
                                                if ffprobe_bif_data == true && original_media_class != mk_lib_common_enum_media_type::DLMediaType::MUSIC {
                                                    // Send a message so roku thumbnail is generated
                                                    rabbit_exchange.publish(Publish::with_properties(json!({"Type": "Roku", "Media UUID": media_id, "Media Path": file_name}).to_string().as_bytes(),
                                                                                                     "mk_ffmpeg".to_string(),
                                                                                                     AmqpProperties::default().with_delivery_mode(2).with_content_type("text/plain".to_string())))?;
                                                }
                                            }
                                            // verify it should save a dl "Z" record for search/lookup/etc
                                            if save_dl_record == true {
                                                // media id begin and download que insert
                                                mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(&sqlx_pool,
                                                                                                                                        "Z".to_string(),
                                                                                                                                        new_class_type_uuid,
                                                                                                                                        Uuid::new_v4(),
                                                                                                                                        None,
                                                                                                                                        String::new()).await.unwrap();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        total_scanned += 1;
                        mk_lib_database_library::mk_lib_database_library_path_status_update(&sqlx_pool,
                                                                                            row_data.mm_media_dir_guid,
                                                                                            json!({"Status": format!("File scan: {:?}/{:?}",
                                                                                                total_scanned.to_formatted_string(&Locale::en),
                                                                                                     total_file_in_dir.to_formatted_string(&Locale::en)),
                                                                                           "Pct": (total_scanned / total_file_in_dir) * 100})).await.unwrap();
                    }
                }
                // end of for loop for each file in library
                // set to none so it doesn't show up anymore in admin status page
                mk_lib_database_library::mk_lib_database_library_path_status_update(
                    &sqlx_pool,
                    row_data.mm_media_dir_guid,
                    json!({"Status": "File scan complete", "Pct": 100}),
                )
                .await
                .unwrap();
                if total_files > 0 {
                    // add notification to admin status page
                    mk_lib_database_notification::mk_lib_database_notification_insert(
                        &sqlx_pool,
                        format!(
                            "{} file(s) added from {}",
                            total_files.to_formatted_string(&Locale::en),
                            row_data.mm_media_dir_path
                        ),
                        true,
                    )
                    .await
                    .unwrap();
                }
            }
        }
    }

    // close rabbitmq
    rabbit_connection.close().unwrap();

    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk("info", json!({"STOP": "STOP"})).await;
    }
    Ok(())
}
