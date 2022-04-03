use amiquip::{AmqpProperties, Connection, Exchange, Publish, Result};
use std::path::PathBuf;
use chrono::prelude::*;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::path::Path;
use tokio::time::{Duration, sleep};
use uuid::Uuid;
use sqlx::Row;

#[path = "mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;
#[path = "mk_lib_common_media_extension.rs"]
mod mk_lib_common_media_extension;
#[path = "mk_lib_file.rs"]
mod mk_lib_file;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;
#[path = "mk_lib_database_library.rs"]
mod mk_lib_database_library;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_database_media_movie.rs"]
mod mk_lib_database_media_movie;
#[path = "mk_lib_database_media_tv.rs"]
mod mk_lib_database_media_tv;
#[path = "mk_lib_database_notification.rs"]
mod mk_lib_database_notification;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkmediascanner";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

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
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           false).await;

    // open rabbit connection
    let mut rabbit_connection = Connection::insecure_open(
        "amqp://guest:guest@mkstack_rabbitmq:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let rabbit_channel = rabbit_connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let rabbit_exchange = Exchange::direct(&rabbit_channel);

    // determine directories to audit
    for row_data in mk_lib_database_library::mk_lib_database_library_path_audit(&sqlx_pool).await.unwrap() {
        mk_lib_logging::mk_logging_post_elk("info",
                                            json!({ "Audit Path": row_data }),
                                            LOGGING_INDEX_NAME).await;
        let mut media_path: PathBuf;
        let mut scan_path: bool = true;
        // shouldn't need to care  let unc_slice = &row_data.get("mm_media_dir_path")[..1];
        // obviously this would mean we mount unc to below as well when defining libraries
        // make sure the path still exists
        let media_path: PathBuf = ["/mediakraken/mnt",
            row_data.get("mm_media_dir_path")].iter().collect();

        if !Path::new(&media_path).exists() {
            mk_lib_database_notification::mk_lib_database_notification_insert(&sqlx_pool,
                                                                              format!("Library path not found: {}",
                                                                                      row_data.get("mm_media_dir_path")),
                                                                              true).await.unwrap();
            scan_path = false;
        }

        if scan_path == true {
            // verify the directory inodes has changed
            let metadata = fs::metadata(&media_path)?;
            let last_modified = metadata.modified()?.elapsed()?.as_secs();
            if last_modified > row_data.get("mm_media_dir_last_scanned") {
                mk_lib_database_library::mk_lib_database_library_path_status_update(&sqlx_pool,
                                                                                    row_data.get("mm_media_dir_guid"),
                                                                                    json!({"Status": "Added to scan", "Pct": 100})).await.unwrap();

                mk_lib_logging::mk_logging_post_elk("info",
                                                    json!({"worker dir": media_path}),
                                                    LOGGING_INDEX_NAME).await;

                let original_media_class = row_data.get("mm_media_dir_class_enum");
                // update the timestamp now so any other media added DURING this scan don"t get skipped
                mk_lib_database_library::mk_lib_database_library_path_timestamp_update(&sqlx_pool,
                                                                                       row_data.get("mm_media_dir_guid")).await.unwrap();
                mk_lib_database_library::mk_lib_database_library_path_status_update(&sqlx_pool,
                                                                                    row_data.get("mm_media_dir_guid"),
                                                                                    json!({"Status": "File search scan", "Pct": 0.0})).await.unwrap();

                let mut file_data = mk_lib_file::mk_directory_walk(&media_path.to_str());

                let total_file_in_dir = file_data.len;
                let mut total_scanned: u64 = 0;
                let mut total_files: u64 = 0;
                for file_name in file_data {
                    if mk_lib_database_library::mk_lib_database_library_file_exists(&sqlx_pool,
                                                                                    file_name).await.unwrap() == false {
                        // set lower here so I can remove a lot of .lower() in the code below
                        let file_extension = Path::new(&file_name).extension().to_lowercase();
                        // checking subtitles for parts as need multiple files for multiple media files
                        if mk_lib_common_media_extension::MEDIA_EXTENSION.contains(&file_extension)
                            || mk_lib_common_media_extension::SUBTITLE_EXTENSION.contains(&file_extension)
                            || mk_lib_common_media_extension::GAME_EXTENSION.contains(&file_extension) {
                            let mut ffprobe_bif_data = true;
                            let mut save_dl_record = true;
                            total_files += 1;
                            // set here which MIGHT be overrode later
                            let mut new_class_type_uuid = media_class_type_uuid;
                            // check for "stacked" media file
                            let base_file_name = Path::new(&file_name).file_name().to_str().unwrap();
                            // check to see if it"s a "stacked" file
                            // including games since some are two or more discs
                            if stack_cd.is_match(&base_file_name)
                                || stack_part.is_match(&base_file_name)
                                || stack_dvd.is_match(&base_file_name)
                                || stack_pt.is_match(&base_file_name)
                                || stack_disk.is_match(&base_file_name)
                                || stack_disc.is_match(&base_file_name) {
                                // check to see if it"s part one or not
                                if stack_cd1.is_match(&base_file_name) == false
                                    && stack_part1.is_match(&base_file_name) == false
                                    && stack_dvd1.is_match(&base_file_name) == false
                                    && stack_pt1.is_match(&base_file_name) == false
                                    && stack_disk1.is_match(&base_file_name) == false
                                    && stack_disc1.is_match(&base_file_name) == false {
                                    // it's not a part one here so, no DL record needed
                                    save_dl_record = false;
                                }
                            }
                            // video game data
                            // TODO look for cue/bin data as well
                            if original_media_class == mk_lib_common_enum_media_type::DLMediaType::GAME {
                                if file_extension == "iso" {
                                    new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::GAME_ISO;
                                } else {
                                    if file_extension == "chd" {
                                        new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::GAME_CHD;
                                    } else {
                                        new_class_type_uuid = mk_lib_common_enum_media_type::DLMediaType::GAME_ROM;
                                    }
                                }
                                ffprobe_bif_data = false;
                            }
                            // set new media class for subtitles
                            else {
                                if mk_lib_common_media_extension::SUBTITLE_EXTENSION.contains(&file_extension) {
                                    if original_media_class == mk_lib_common_enum_media_type::DLMediaType::MOVIE {
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
                                        || file_name.contains("\\theme.mp4") {
                                        if original_media_class == mk_lib_common_enum_media_type::DLMediaType::MOVIE {
                                            if file_name.contains("/trailers/") || file_name.contains("\\trailers\\") {
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
                                        if file_name.contains("/extras/") || file_name.contains("\\extras\\") {
                                            if original_media_class == mk_lib_common_enum_media_type::DLMediaType::MOVIE {
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
                                                || file_name.contains("\\backdrops\\") {
                                                media_class_text = new_class_type_uuid;
                                                if file_name.contains("/theme.mp3")
                                                    || file_name.contains("\\theme.mp3")
                                                    || file_name.contains("/theme.mp4")
                                                    || file_name.contains("\\theme.mp4") {
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
                                            if file_name == "\\" {
                                                let file_name = file_name.replace("\\\\", "smb://guest:\"\"@").replace("\\", "/");
                                            }
                                            // create media_json data
                                            let media_json = json!({ "Added": Utc::now().to_string() });
                                            let media_id = Uuid::new_v4();
                                            mk_lib_database_media::mk_lib_database_media_insert(&sqlx_pool,
                                                                                                media_id,
                                                                                                new_class_type_uuid,
                                                                                                file_name,
                                                                                                None,
                                                                                                None,
                                                                                                media_json);
                                            // verify ffprobe and bif should run on the data
                                            if ffprobe_bif_data == true && mk_lib_common_media_extension::MEDIA_EXTENSION_SKIP_FFMPEG.contains(&file_extension) == false
                                                && mk_lib_common_media_extension::MEDIA_EXTENSION.contains(&file_extension) {
                                                // Send a message so ffprobe runs
                                                rabbit_exchange.publish(Publish::with_properties(json!({"Type": "FFProbe", "Media UUID": media_id, "Media Path": file_name}),
                                                                                                 "mk_ffmpeg".to_string(),
                                                                                                 AmqpProperties::default().with_delivery_mode(2).with_content_type("text/plain".to_string())))?;
                                                if original_media_class != mk_lib_common_enum_media_type::DLMediaType::MUSIC {
                                                    // Send a message so roku thumbnail is generated
                                                    rabbit_exchange.publish(Publish::with_properties(json!({"Type": "Roku", "Media UUID": media_id, "Media Path": file_name}),
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
                                                                                                                                        media_id,
                                                                                                                                        "".to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        total_scanned += 1;
                        mk_lib_database_library::mk_lib_database_library_path_status_update(&sqlx_pool,
                                                                                            row_data.get("mm_media_dir_guid"),
                                                                                            json!({format!("Status": "File scan: {:?}/{:?}",
                                                                                                total_scanned.to_formatted_string(&Locale::en),
                                                                                                     total_file_in_dir.to_formatted_string(&Locale::en)),
                                                                                           "Pct": (total_scanned / total_file_in_dir) * 100}));
                    }
                }
                // end of for loop for each file in library
                mk_lib_logging::mk_logging_post_elk("info",
                                                    json!({"worker dir done": media_path.to_str(),
                                                        "media class": media_class_type_uuid}),
                                                    LOGGING_INDEX_NAME).await;
                // set to none so it doesn"t show up anymore in admin status page
                mk_lib_database_library::mk_lib_database_library_path_status_update(&sqlx_pool,
                                                                                    row_data.get("mm_media_dir_guid"),
                                                                                    json!({"Status": "File scan complete", "Pct": 100}));
                if total_files > 0 {
                    // add notification to admin status page
                    mk_lib_database_notification::mk_lib_database_notification_insert(&sqlx_pool,
                                                                                      format!("{} file(s) added from {}",
                                                                                              total_files.to_formatted_string(&Locale::en),
                                                                                              row_data.get("mm_media_dir_path")),
                                                                                      true);
                }
            }
        }
    }

    // close rabbitmq
    rabbit_connection.close();

    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"STOP": "STOP"}),
                                        LOGGING_INDEX_NAME).await;
    Ok(())
}