use mk_lib_common::mk_lib_common_ffmpeg;
use mk_lib_database;
use mk_lib_rabbitmq;
use serde_json::{json, Value};
use std::error::Error;
use std::process::{Command, Stdio};
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1, 120)
        .await
        .unwrap();
    let _results = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await;

    // pull options for metadata/chapters/images location
    let option_json: serde_json::Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mktranscode")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mktranscode", &rabbit_channel)
            .await
            .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                if json_message["Type"] == "Roku" {
                    if json_message["Subtype"] == "Thumbnail" {
                        //common_hardware_roku_bif.com_roku_create_bif(&json_message["Media Path"].to_string());
                    }
                } else if json_message["Type"] == "HDHomeRun" {
                } else if json_message["Type"] == "FFMPEG" {
                    if json_message["Subtype"] == "Probe" {
                        // scan media file via ffprobeS
                        let ffprobe_data: serde_json::Value =
                            mk_lib_common_ffmpeg::mk_common_ffmpeg_get_info(
                                &json_message["Media Path"].to_string(),
                            )
                            .await
                            .unwrap();
                        let tmp_uuid =
                            uuid::Uuid::parse_str(&json_message["Media UUID"].to_string()).unwrap();
                        let _result = mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_ffmpeg_update_by_uuid(
                        &sqlx_pool,
                        tmp_uuid,
                        ffprobe_data,
                    )
                    .await;
                    } else if json_message["Subtype"] == "Cast" {
                        if json_message["Command"] == "Chapter Back" {
                        } else if json_message["Command"] == "Chapter Forward" {
                        } else if json_message["Command"] == "Fast Forward" {
                        } else if json_message["Command"] == "Mute" {
                            let output = Command::new("python3")
                                .args([
                                    "/mediakraken/stream2chromecast/stream2chromecast.py",
                                    "-devicename",
                                    &json_message["Device"].to_string(),
                                    "-mute",
                                ])
                                .stdout(Stdio::piped())
                                .output()
                                .unwrap();
                            let stdout = String::from_utf8(output.stdout).unwrap();
                        } else if json_message["Command"] == "Pause" {
                            let output = Command::new("python3")
                                .args([
                                    "/mediakraken/stream2chromecast/stream2chromecast.py",
                                    "-devicename",
                                    &json_message["Device"].to_string(),
                                    "-pause",
                                ])
                                .stdout(Stdio::piped())
                                .output()
                                .unwrap();
                            let stdout = String::from_utf8(output.stdout).unwrap();
                        } else if json_message["Command"] == "Rewind" {
                        } else if json_message["Command"] == "Stop" {
                            let output = Command::new("python3")
                                .args([
                                    "/mediakraken/stream2chromecast/stream2chromecast.py",
                                    "-devicename",
                                    &json_message["Device"].to_string(),
                                    "-stop",
                                ])
                                .stdout(Stdio::piped())
                                .output()
                                .unwrap();
                            let stdout = String::from_utf8(output.stdout).unwrap();
                        } else if json_message["Command"] == "Volume Down" {
                            let output = Command::new("python3")
                                .args([
                                    "/mediakraken/stream2chromecast/stream2chromecast.py",
                                    "-devicename",
                                    &json_message["Device"].to_string(),
                                    "-voldown",
                                ])
                                .stdout(Stdio::piped())
                                .output()
                                .unwrap();
                            let stdout = String::from_utf8(output.stdout).unwrap();
                        } else if json_message["Command"] == "Volume Set" {
                            let output = Command::new("python3")
                                .args([
                                    "/mediakraken/stream2chromecast/stream2chromecast.py",
                                    "-devicename",
                                    &json_message["Device"].to_string(),
                                    "-setvol",
                                    &json_message["Data"].to_string(),
                                ])
                                .stdout(Stdio::piped())
                                .output()
                                .unwrap();
                            let stdout = String::from_utf8(output.stdout).unwrap();
                        } else if json_message["Command"] == "Volume Up" {
                            let output = Command::new("python3")
                                .args([
                                    "/mediakraken/stream2chromecast/stream2chromecast.py",
                                    "-devicename",
                                    &json_message["Device"].to_string(),
                                    "-volup",
                                ])
                                .stdout(Stdio::piped())
                                .output()
                                .unwrap();
                            let stdout = String::from_utf8(output.stdout).unwrap();
                        }
                    } else if json_message["Subtype"] == "ChapterImage" {
                        // begin image generation
                        let mut chapter_image_list = json!({});
                        let mut chapter_count: i16 = 0;
                        let mut first_image: bool = true;
                        let mut image_file_path: String = String::new();
                        // do this check as not all media has chapters....like LD rips
                        if json_message["Data"].get("chapters").is_some() {
                            // for chapter_data in json_message["Data"]["chapters"].iter() {
                            //     chapter_count += 1;
                                // file path, time, output name
                                // check image save option whether to
                                // save this in media folder or metadata folder
                            //     if option_json["MetadataImageLocal"] == false {
                            //         image_file_path = os.path.join(
                            //             common_metadata.com_meta_image_file_path(
                            //                 json_message["Media Path"],
                            //                 "chapter",
                            //             ),
                            //             json_message["Media UUID"]
                            //                 + "_"
                            //                 + str(chapter_count)
                            //                 + ".png",
                            //         );
                            //     } else {
                            //         image_file_path = os.path.join(
                            //             os.path.dirname(json_message["Media Path"]),
                            //             "chapters",
                            //         );
                            //         // have this bool so I don't hit the os looking for path each time
                            //         if first_image == true
                            //             && !Path::new("/mediakraken/certs/image_file_path")
                            //                 .exists()
                            //         {
                            //             os.makedirs(image_file_path);
                            //         }
                            //         image_file_path = os
                            //             .path
                            //             .join(image_file_path, chapter_count.as_str() + ".png");
                            //     }
                                 // format the seconds to what ffmpeg is looking for
                            //     (minutes, seconds) =
                            //         divmod(float(chapter_data["start_time"]), 60);
                            //     (hours, minutes) = divmod(minutes, 60);
                            //     // if ss is before the input it seeks
                            //     // and doesn't convert every frame like after input
                            //     let output = Command::new("ffmpeg")
                            //         .args([
                            //             "-ss",
                            //             command_list.append(
                            //                 "%02d:%02d:%02f" % (hours, minutes, seconds),
                            //             ),
                            //             "-i",
                            //             "\"" + json_message["Media Path"] + "\"",
                            //             "-vframes",
                            //             "1",
                            //             "\"" + image_file_path + "\"",
                            //         ])
                            //         .stdout(Stdio::piped())
                            //         .output()
                            //         .unwrap();
                            //     let stdout = String::from_utf8(output.stdout).unwrap();
                                 // as the worker might see it as finished if allowed to continue
                            //     chapter_image_list[chapter_data["tags"]["title"]] =
                            //         image_file_path;
                            //     first_image = false;
                            // }
                        }
                        // db_connection.db_update_media_json(json_message["Media UUID"], {
                        //     "ChapterImages": chapter_image_list
                        //});
                    }
                    // } else if json_message["Subtype"] == "Sync" {
                    //     ffmpeg_params = [
                    //         "ffmpeg",
                    //         "-i",
                    //         db_connection.db_media_path_by_uuid(
                    //             json_message["mm_sync_options_json"]["Media GUID"],
                    //         )[0],
                    //     ];
                    //     if json_message["mm_sync_options_json"]["Options"]["Size"] != "Clone" {
                    //         ffmpeg_params.extend((
                    //             "-fs",
                    //             json_message["mm_sync_options_json"]["Options"]["Size"],
                    //         ));
                    //     }
                    //     if json_message["mm_sync_options_json"]["Options"]["VCodec"] != "Copy" {
                    //         ffmpeg_params.extend((
                    //             "-vcodec",
                    //             json_message["mm_sync_options_json"]["Options"]["VCodec"],
                    //         ));
                    //     }
                    //     if json_message["mm_sync_options_json"]["Options"]["AudioChannels"]
                    //         != "Copy"
                    //     {
                    //         ffmpeg_params.extend((
                    //             "-ac",
                    //             json_message["mm_sync_options_json"]["Options"]
                    //                 ["AudioChannels"],
                    //         ));
                    //     }
                    //     if json_message["mm_sync_options_json"]["Options"]["ACodec"] != "Copy" {
                    //         ffmpeg_params.extend((
                    //             "-acodec",
                    //             json_message["mm_sync_options_json"]["Options"]["ACodec"],
                    //         ));
                    //     }
                    //     if json_message["mm_sync_options_json"]["Options"]["ASRate"]
                    //         != "Default"
                    //     {
                    //         ffmpeg_params.extend((
                    //             "-ar",
                    //             json_message["mm_sync_options_json"]["Options"]["ASRate"],
                    //         ));
                    //     }
                    //     ffmpeg_params.append(
                    //         json_message["mm_sync_path_to"]
                    //             + "."
                    //             + json_message["mm_sync_options_json"]["Options"]["VContainer"],
                    //     );

                    //     let ffmpeg_pid = subprocess.Popen(shlex.split(ffmpeg_params));
                    //     // output after it gets started
                    //     //  Duration: 01:31:10.10, start: 0.000000, bitrate: 4647 kb/s
                    //     // frame= 1091 fps= 78 q=-1.0 Lsize=    3199kB time=00:00:36.48
                    //     // bitrate= 718.4kbits/s dup=197 drop=0 speed= 2.6x
                    //     let mut media_duration = None;
                    //     loop {
                    //         line = ffmpeg_pid.stdout.readline();
                    //         if line != "" {
                    //             if line.find("Duration:") != -1 {
                    //                 media_duration = timedelta(float(
                    //                     line.split(": ", 1)[1].split(",", 1)[0],
                    //                 ));
                    //             } else if line[0..5] == "frame" {
                    //                 time_string = timedelta(float(
                    //                     line.split("=", 5)[5].split(" ", 1)[0],
                    //                 ));
                    //                 time_percent = time_string.total_seconds()
                    //                     / media_duration.total_seconds();
                    //                 db_connection.db_sync_progress_update(
                    //                     row_data["mm_sync_guid"],
                    //                     time_percent,
                    //                 );
                    //                 db_connection.db_commit();
                    //             }
                    //         } else {
                    //             break;
                    //         }
                    //     }
                    //     ffmpeg_pid.wait();
                    //     // deal with converted file
                    //     if json_message["mm_sync_options_json"]["Type"] == "Local File System" {
                    //         // just go along merry way as ffmpeg shoulda output to mm_sync_path_to
                    //     } else if json_message["mm_sync_options_json"]["Type"]
                    //         == "Remote Client"
                    //     {
                    //         XFER_THREAD = common_xfer.FileSenderThread(
                    //             json_message["mm_sync_options_json"]["TargetIP"],
                    //             json_message["mm_sync_options_json"]["TargetPort"],
                    //             json_message["mm_sync_path_to"]
                    //                 + "."
                    //                 + json_message["mm_sync_options_json"]["Options"]
                    //                     ["VContainer"],
                    //             json_message["mm_sync_path_to"],
                    //         );
                    //     } else {
                    //         // cloud item
                    //         CLOUD_HANDLE = common_cloud.CommonCloud(option_config_json);
                    //         CLOUD_HANDLE.com_cloud_file_store(
                    //             json_message["mm_sync_options_json"]["Type"],
                    //             json_message["mm_sync_path_to"],
                    //             json_message["mm_sync_path_to"]
                    //                 + "."
                    //                 + json_message["mm_sync_options_json"]["Options"]
                    //                     ["VContainer"]
                    //                     .split("/", 1)[1],
                    //             false,
                    //         );
                    //     }
                    //     db_connection.db_sync_delete(json_message[0]); // guid of sync record
                    // }
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
