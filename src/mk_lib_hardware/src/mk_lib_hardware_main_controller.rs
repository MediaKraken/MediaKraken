pub async fn mk_hardware_main_command(machine_type: String, command_type: String, command_value: String
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match machine_type.as_str() {
        "alexa" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "ampro" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "appletv" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "arduino" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "chromecast" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "cestron" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "firetv" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "hdhomerun" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "lenbrook" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "lg" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "marantz" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "onkyo" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "phue" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "pioneer" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "roku" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "sansung" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "tivo" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        "yamaha" => {
            return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
        }
        _ => {
            #[cfg(debug_assertions)]
            {
                mk_lib_logging::mk_logging_post_elk(
                    std::module_path!(),
                    serde_json::json!({ "unknown machine_type": machine_type }),
                )
                .await
                .unwrap();
            }
            return Ok(serde_json::json!({}));
        }
    }
}