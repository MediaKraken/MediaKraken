use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde_json::{json, Value};
use std::error::Error;
use stdext::function_name;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}))
            .await
            .unwrap();
    }

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await;
    let option_config_json =
        &mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkrabbitconsume")
            .await
            .unwrap();

    // Declare the queue.
    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkrabbitconsume", &rabbit_channel)
            .await
            .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                #[cfg(debug_assertions)]
                {
                    mk_lib_logging::mk_logging_post_elk(
                        std::module_path!(),
                        json!({ "msg body": json_message }),
                    )
                    .await
                    .unwrap();
                }
                /*
                Do I actually launch a docker swarm container that checks for cuda
                and then that launches the slave container with ffmpeg

                # this is for the debian one
                docker run -it --rm $(ls /dev/nvidia* | xargs -I{} echo "--device={}") $(ls /usr/lib/x86_64-linux-gnu/{libcuda,libnvidia}* | xargs -I{} echo "-v {}:{}:ro") mediakraken/mkslavenvidiadebian

                --device /dev/nvidia0:/dev/nvidia0 \
                --device /dev/nvidiactl:/dev/nvidiactl \

                The minimum required Nvidia driver for nvenc is 378.13 or newer from ffmpeg error
                """
                if body != None:
                    json_message = json.loads(body)
                    if json_message["Type"] == "Playback":
                        if json_message["Subtype"] == "Play":
                            # to address the 30 char name limit for container
                            name_container = (json_message["User"] + "_"
                                              + str(uuid.uuid4()).replace("-", ""))[:30]
                            // TODO only for now until I get the device for websessions (cookie perhaps?)
                            if "Device" in json_message:
                                define_new_container = (name_container, json_message["Device"])
                            else:
                                define_new_container = (name_container, None)
                            if json_message["User"] in mk_containers:
                                user_activity_list = mk_containers[json_message["User"]]
                                user_activity_list.append(define_new_container)
                                mk_containers[json_message["User"]] = user_activity_list
                            else:
                                # "double list" so each one is it"s own instance
                                mk_containers[json_message["User"]] = (define_new_container)
                            container_command = None
                            if json_message["Device"] == "Cast":
                                # should only need to check for subs on initial play command
                                if "Subtitle" in json_message:
                                    subtitle_command = " -subtitles " + json_message["Subtitle"]
                                else:
                                    subtitle_command = ""
                                return_video_container, return_video_codec, return_audio_codec, return_audio_channels \
                                    = common_device_capability.com_device_compat_best_fit(
                                    device_type="Chromecast",
                                    device_model=None,
                                    video_container=None,
                                    video_codec=None,
                                    audio_codec=None,
                                    audio_channels=None)
                                // TODO take number of channels into account
                                // TODO take the video codec into account
                                container_command = "castnow --tomp4 --ffmpeg-acodec ac3 --ffmpeg-movflags " \
                                                    "frag_keyframe+empty_moov+faststart --address " \
                                                    + json_message["Target"] + " --myip " \
                                                    + docker_inst.com_docker_info()["Swarm"]["NodeAddr"] \
                                                    + subtitle_command + " \"" + json_message["Data"] + "\""
                                hwaccel = False
                                docker_inst.com_docker_run_cast(hwaccel=hwaccel,
                                                                name_container=name_container,
                                                                container_command=container_command)
                            else if json_message["Device"] == "HDHomerun":
                                # stream from hdhomerun
                                container_command = "ffmpeg -i http://" + json_message["IP"] \
                                                    + ":5004/auto/v" + json_message["Channel"] \
                                                    + "?transcode=" + json_message[
                                                        "Quality"] + "-vcodec copy" \
                                                    + "./static/streams/" + \
                                                    json_message["Channel"] + ".m3u8"
                            else if json_message["Device"] == "HLS":
                                # stream to hls
                                // TODO take the video codec into account
                                container_command = "ffmpeg -i \"" + json_message["Input File"] \
                                                    + "\" -vcodec libx264 -preset veryfast" \
                                                    + " -acodec aac -ac:a:0 2 -vbr 5 " \
                                                    + json_message["Audio Track"] \
                                                    + "-vf " + json_message["Subtitle Track"] \
                                                    + " yadif=0:0:0 " \
                                                    + json_message["Target UUID"]
                            else if json_message["Device"] == "Roku":
                                pass
                            else if json_message["Device"] == "Web":
                                # stream to web
                                container_command = "ffmpeg -v fatal {ss_string}" \
                                                    + " -i ".format(**locals()) \
                                                    + json_message["Data"] \
                                                    + "-c:a aac -strict experimental -ac 2 -b:a 64k" \
                                                      " -c:v libx264 -pix_fmt yuv420p" \
                                                      " -profile:v high -level 4.0" \
                                                      " -preset ultrafast -trellis 0" \
                                                      " -crf 31 -vf scale=w=trunc(oh*a/2)*2:h=480" \
                                                      " -shortest -f mpegts" \
                                                      " -output_ts_offset {output_ts_offset:.6f}" \
                                                      " -t {t:.6f} pipe:%d.ts".format(**locals())
                            else:
                                common_logging_elasticsearch_httpx.com_es_httpx_post(
                                    message_type="critical", message_text=
                                    {"stuff": "unknown subtype"})
                            if container_command != None:
                                hwaccel = False
                                docker_inst.com_docker_run_slave(hwaccel=hwaccel,
                                                                 port_mapping=None,
                                                                 name_container=name_container,
                                                                 container_command=container_command)
                        else if json_message["Subtype"] == "Stop":
                            # this will force stop the container and then delete it
                            common_logging_elasticsearch_httpx.com_es_httpx_post(message_type="info",
                                                                                 message_text={
                                                                                     "user stop":
                                                                                         mk_containers[
                                                                                             json_message[
                                                                                                 "User"]]})
                            docker_inst.com_docker_delete_container(
                                container_image_name=mk_containers[json_message["User"]])
                        else if json_message["Subtype"] == "Pause":
                            if json_message["Device"] == "Cast":
                                pass

                            # if json_message["Device Type"] == "Slave":
                            #     if json_message["Command"] == "Chapter Back":
                            #         pass
                            #     else if json_message["Command"] == "Chapter Forward":
                            #         pass
                            #     else if json_message["Command"] == "Fast Forward":
                            #         pass
                            #     else if json_message["Command"] == "Pause":
                            #         pass
                            #     else if json_message["Command"] == "Play":
                            #         pass
                            #     else if json_message["Command"] == "Rewind":
                            #         pass
                            #     else if json_message["Command"] == "Stop":
                            #         os.killpg(self.proc_ffmpeg_stream.pid, signal.SIGTERM)
                             */
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
