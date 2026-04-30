use mk_lib_common::mk_lib_common_ffmpeg;
use serde_json::{Value, json};
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use tokio::process::Command;
use tokio::signal;
use tokio::time::timeout;

const STREAM2CHROMECAST_PATH: &str = "/mediakraken/stream2chromecast/stream2chromecast.py";
const COMMAND_TIMEOUT: Duration = Duration::from_secs(60 * 60);

async fn run_cast_command(device_name: &str, command_flag: &str, extra: Option<&str>) {
    if device_name.is_empty() || device_name.starts_with('-') {
        eprintln!("mktranscode: cast skipped, invalid device_name '{device_name}'");
        return;
    }
    if let Some(extra_arg) = extra
        && extra_arg.starts_with('-')
    {
        eprintln!("mktranscode: cast skipped, argument '{extra_arg}' looks like a flag");
        return;
    }

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

    match timeout(COMMAND_TIMEOUT, process.status()).await {
        Ok(Ok(status)) if status.success() => {}
        Ok(Ok(status)) => {
            eprintln!("mktranscode: stream2chromecast exited with status {status}");
        }
        Ok(Err(error)) => {
            eprintln!("mktranscode: failed to spawn stream2chromecast ({error})");
        }
        Err(_) => {
            eprintln!("mktranscode: stream2chromecast timed out after {COMMAND_TIMEOUT:?}");
        }
    }
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

async fn run_conversion(binary: &str, args: &[&str]) -> Result<(), String> {
    let output = match timeout(COMMAND_TIMEOUT, Command::new(binary).args(args).output()).await {
        Ok(Ok(output)) => output,
        Ok(Err(error)) => return Err(format!("{binary} unavailable ({error})")),
        Err(_) => return Err(format!("{binary} timed out after {COMMAND_TIMEOUT:?}")),
    };

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(format!(
        "{binary} failed with status {}: {}",
        output.status,
        stderr.trim()
    ))
}

async fn convert_ebook(json_message: &Value) {
    let Some((input_path, output_path)) = ebook_conversion_targets(json_message) else {
        eprintln!("mktranscode: ebook conversion skipped, missing input/output parameters");
        return;
    };

    match run_conversion("ebook-convert", &[&input_path, &output_path]).await {
        Ok(()) => return,
        Err(reason) => {
            eprintln!("mktranscode: {reason}, falling back to pandoc");
        }
    }

    if let Err(reason) = run_conversion("pandoc", &[&input_path, "-o", &output_path]).await {
        eprintln!("mktranscode: pandoc ebook conversion failed: {reason}");
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let _option_json =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool_ro)
            .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mktranscode").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mktranscode", &rabbit_channel).await?;

    loop {
        tokio::select! {
            _ = shutdown_signal() => {
                eprintln!("mktranscode: shutdown signal received");
                return Ok(());
            }
            msg = rabbit_consumer.recv() => {
                let Some(msg) = msg else {
                    eprintln!("mktranscode: rabbit consumer closed");
                    return Ok(());
                };

                let Some(payload) = msg.content else {
                    if let Some(deliver) = msg.deliver {
                        let _ = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                            &rabbit_channel,
                            deliver.delivery_tag(),
                        )
                        .await;
                    }
                    continue;
                };

                let json_message: Value = match serde_json::from_slice(&payload) {
                    Ok(value) => value,
                    Err(error) => {
                        eprintln!("mktranscode: malformed payload ({error})");
                        if let Some(deliver) = msg.deliver {
                            let _ = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                                &rabbit_channel,
                                deliver.delivery_tag(),
                            )
                            .await;
                        }
                        continue;
                    }
                };

                if json_message["Type"] == "Roku" {
                    if json_message["Subtype"] == "Thumbnail" {
                        //common_hardware_roku_bif.com_roku_create_bif(&json_message["Media Path"].to_string());
                    }
                } else if json_message["Type"] == "HDHomeRun" {
                } else if json_message["Type"] == "FFMPEG" {
                    if json_message["Subtype"] == "Probe" {
                        let media_path =
                            json_message["Media Path"].as_str().unwrap_or_default();
                        if media_path.is_empty() {
                            eprintln!("mktranscode: ffprobe skipped, missing 'Media Path'");
                        } else {
                            let media_uuid_str =
                                json_message["Media UUID"].as_str().unwrap_or_default();
                            match uuid::Uuid::parse_str(media_uuid_str) {
                                Ok(tmp_uuid) => {
                                    match mk_lib_common_ffmpeg::mk_common_ffmpeg_get_info(media_path).await {
                                        Ok(ffprobe_data) => {
                                            if let Err(error) = mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_ffmpeg_update_by_uuid(
                                                &sqlx_pool_rw,
                                                tmp_uuid,
                                                ffprobe_data,
                                            )
                                            .await
                                            {
                                                eprintln!(
                                                    "mktranscode: ffprobe db update failed for {tmp_uuid} ({error})"
                                                );
                                            }
                                        }
                                        Err(error) => {
                                            eprintln!(
                                                "mktranscode: ffprobe failed for '{media_path}' ({error})"
                                            );
                                        }
                                    }
                                }
                                Err(error) => {
                                    eprintln!(
                                        "mktranscode: ffprobe skipped, invalid Media UUID '{media_uuid_str}' ({error})"
                                    );
                                }
                            }
                        }
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
                            if let Some(volume) = json_value_to_string(&json_message["Data"]) {
                                run_cast_command(device_name, "-setvol", Some(&volume)).await;
                            } else {
                                eprintln!("mktranscode: Volume Set skipped, missing 'Data'");
                            }
                        } else if json_message["Command"] == "Volume Up" {
                            run_cast_command(device_name, "-volup", None).await;
                        }
                    } else if json_message["Subtype"] == "ChapterImage" {
                        // begin image generation
                        let chapter_image_list = json!({});
                        let chapter_count: i16 = 0;
                        let first_image: bool = true;
                        let image_file_path: String = String::new();
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
                        let _ = (&chapter_image_list, &chapter_count, &first_image, &image_file_path);
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

                if let Some(deliver) = msg.deliver {
                    if let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                        &rabbit_channel,
                        deliver.delivery_tag(),
                    )
                    .await
                    {
                        eprintln!("mktranscode: rabbit ack failed ({error})");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_value_to_string_handles_scalars() {
        assert_eq!(json_value_to_string(&json!("hi")).as_deref(), Some("hi"));
        assert_eq!(json_value_to_string(&json!(42)).as_deref(), Some("42"));
        assert_eq!(json_value_to_string(&json!(-7)).as_deref(), Some("-7"));
        assert_eq!(json_value_to_string(&json!(1.5)).as_deref(), Some("1.5"));
        assert_eq!(json_value_to_string(&json!(true)).as_deref(), Some("true"));
        assert!(json_value_to_string(&json!(null)).is_none());
        assert!(json_value_to_string(&json!({"a": 1})).is_none());
    }

    #[test]
    fn cast_stream_argument_detects_url_vs_file() {
        let message = json!({"Data": {"URL": "https://example.com/stream.m3u8"}});
        assert_eq!(
            cast_stream_argument(&message),
            Some(("-playurl", "https://example.com/stream.m3u8".to_string()))
        );

        let message = json!({"Data": {"Path": "/media/movie.mkv"}});
        assert_eq!(
            cast_stream_argument(&message),
            Some(("-playfile", "/media/movie.mkv".to_string()))
        );
    }

    #[test]
    fn cast_stream_argument_falls_back_to_scalar_data() {
        let message = json!({"Data": "http://example.com/a.mp4"});
        assert_eq!(
            cast_stream_argument(&message),
            Some(("-playurl", "http://example.com/a.mp4".to_string()))
        );

        let message = json!({"Data": null, "Media Path": "/tv/show.mkv"});
        assert_eq!(
            cast_stream_argument(&message),
            Some(("-playfile", "/tv/show.mkv".to_string()))
        );
    }

    #[test]
    fn cast_stream_argument_missing_returns_none() {
        let message = json!({"Data": {}});
        assert!(cast_stream_argument(&message).is_none());
    }

    #[test]
    fn ebook_conversion_targets_uses_explicit_output() {
        let message = json!({
            "Data": {"Input": "/books/a.epub", "Output": "/books/a.mobi"}
        });
        assert_eq!(
            ebook_conversion_targets(&message),
            Some(("/books/a.epub".to_string(), "/books/a.mobi".to_string()))
        );
    }

    #[test]
    fn ebook_conversion_targets_derives_output_from_format() {
        let message = json!({
            "Data": {"Source Path": "/books/a.epub", "Format": ".MOBI"}
        });
        let (input, output) = ebook_conversion_targets(&message).unwrap();
        assert_eq!(input, "/books/a.epub");
        assert_eq!(output, "/books/a.mobi");
    }

    #[test]
    fn ebook_conversion_targets_requires_input() {
        let message = json!({"Data": {"Format": "mobi"}});
        assert!(ebook_conversion_targets(&message).is_none());
    }

    #[test]
    fn ebook_conversion_targets_requires_output_info() {
        let message = json!({"Data": {"Input": "/books/a.epub"}});
        assert!(ebook_conversion_targets(&message).is_none());
    }
}
