use mk_lib_common::mk_lib_common_ffmpeg;
use mk_lib_database;
use mk_lib_rabbitmq;
use serde_json::{Value, json};
use std::error::Error;
use std::path::Path;
use tokio::process::Command;
use tokio::sync::Notify;

const STREAM2CHROMECAST_PATH: &str = "/mediakraken/stream2chromecast/stream2chromecast.py";

async fn run_cast_command(device_name: &str, command_flag: &str, extra: Option<&str>) {
    let mut process = Command::new("python3");
    process.args([
        STREAM2CHROMECAST_PATH,
        "-devicename",
        device_name,
        command_flag,
    ]);
    if let Some(extra_arg) = extra {
        process.arg(extra_arg);
    }
    let _result = process.status().await;
}

fn json_value_to_string(value: &Value) -> Option<String> {
    value
        .as_str()
        .map(ToString::to_string)
        .or_else(|| value.as_i64().map(|inner| inner.to_string()))
        .or_else(|| value.as_u64().map(|inner| inner.to_string()))
        .or_else(|| value.as_f64().map(|inner| inner.to_string()))
        .or_else(|| value.as_bool().map(|inner| inner.to_string()))
}

fn cast_stream_argument(json_message: &Value) -> Option<(&'static str, String)> {
    let data = &json_message["Data"];
    let stream_target = data
        .as_object()
        .and_then(|data_object| {
            data_object
                .get("URL")
                .or_else(|| data_object.get("Url"))
                .or_else(|| data_object.get("url"))
                .or_else(|| data_object.get("Media Path"))
                .or_else(|| data_object.get("Path"))
        })
        .and_then(json_value_to_string)
        .or_else(|| json_value_to_string(data))
        .or_else(|| json_value_to_string(&json_message["Media Path"]));

    stream_target.map(|target| {
        if target.starts_with("http://") || target.starts_with("https://") {
            ("-playurl", target)
        } else {
            ("-playfile", target)
        }
    })
}

fn json_string_from_keys<'a>(json_message: &'a Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .find_map(|key| json_message.get(*key))
        .and_then(Value::as_str)
}

fn ebook_conversion_targets(json_message: &Value) -> Option<(String, String)> {
    let data = &json_message["Data"];
    let source_path = json_string_from_keys(
        data,
        &[
            "Input",
            "Input Path",
            "Source",
            "Source Path",
            "Path",
            "Media Path",
        ],
    )
    .or_else(|| json_string_from_keys(json_message, &["Input", "Input Path", "Media Path"]))?;

    let output_path = json_string_from_keys(data, &["Output", "Output Path"]).map(String::from);
    if let Some(explicit_path) = output_path {
        return Some((source_path.to_string(), explicit_path));
    }

    let output_format = json_string_from_keys(data, &["Format", "Output Format", "To", "Target"])
        .or_else(|| json_string_from_keys(json_message, &["Format", "Output Format", "To"]))?
        .trim()
        .trim_start_matches('.')
        .to_lowercase();

    if output_format.is_empty() {
        return None;
    }

    let source = Path::new(source_path);
    let source_parent = source.parent().unwrap_or_else(|| Path::new(""));
    let source_stem = source.file_stem()?.to_str()?;
    let output_file_name = format!("{source_stem}.{output_format}");
    let output_target = source_parent.join(output_file_name);

    Some((
        source_path.to_string(),
        output_target.to_string_lossy().to_string(),
    ))
}

async fn convert_ebook(json_message: &Value) {
    let Some((input_path, output_path)) = ebook_conversion_targets(json_message) else {
        eprintln!("mktranscode: ebook conversion skipped, missing input/output parameters");
        return;
    };

    let calibre_result = Command::new("ebook-convert")
        .args([&input_path, &output_path])
        .status()
        .await;

    match calibre_result {
        Ok(status) if status.success() => return,
        Ok(status) => {
            eprintln!(
                "mktranscode: ebook-convert failed with status {status}, falling back to pandoc"
            );
        }
        Err(error) => {
            eprintln!("mktranscode: ebook-convert unavailable ({error}), falling back to pandoc");
        }
    }

    let pandoc_result = Command::new("pandoc")
        .args([&input_path, "-o", &output_path])
        .status()
        .await;

    match pandoc_result {
        Ok(status) if status.success() => {}
        Ok(status) => {
            eprintln!("mktranscode: pandoc ebook conversion failed with status {status}");
        }
        Err(error) => {
            eprintln!("mktranscode: pandoc unavailable for ebook conversion ({error})");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1, 120)
            .await
            .unwrap();
    let _results = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(
        &sqlx_pool_ro,
        false,
    )
    .await;

    // pull options for metadata/chapters/images location
    let option_json: serde_json::Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool_ro)
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
                        &sqlx_pool_rw,
                        tmp_uuid,
                        ffprobe_data,
                    )
                    .await;
                    } else if json_message["Subtype"] == "Cast" {
                        let device_name = json_message["Device"].as_str().unwrap_or_default();
                        if json_message["Command"] == "Chapter Back" {
                        } else if json_message["Command"] == "Chapter Forward" {
                        } else if json_message["Command"] == "Fast Forward" {
                        } else if json_message["Command"] == "Mute" {
                            run_cast_command(device_name, "-mute", None).await;
                        } else if json_message["Command"] == "Pause" {
                            run_cast_command(device_name, "-pause", None).await;
                        } else if json_message["Command"] == "Play" {
                            if let Some((command_flag, stream_target)) =
                                cast_stream_argument(&json_message)
                            {
                                run_cast_command(device_name, command_flag, Some(&stream_target))
                                    .await;
                            }
                        } else if json_message["Command"] == "Rewind" {
                        } else if json_message["Command"] == "Stop" {
                            run_cast_command(device_name, "-stop", None).await;
                        } else if json_message["Command"] == "Volume Down" {
                            run_cast_command(device_name, "-voldown", None).await;
                        } else if json_message["Command"] == "Volume Set" {
                            let volume = json_message["Data"]
                                .as_str()
                                .map(String::from)
                                .unwrap_or_else(|| json_message["Data"].to_string());
                            run_cast_command(device_name, "-setvol", Some(&volume)).await;
                        } else if json_message["Command"] == "Volume Up" {
                            run_cast_command(device_name, "-volup", None).await;
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
                    } else if json_message["Subtype"] == "EbookConvert" {
                        convert_ebook(&json_message).await;
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
