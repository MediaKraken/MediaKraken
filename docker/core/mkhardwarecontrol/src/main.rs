use mk_lib_hardware;
use mk_lib_logging::mk_lib_logging;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde_json::{json, Value};
use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;
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

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkstack_rabbitmq", "mkhardwarecontrol")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkhardwarecontrol", &rabbit_channel)
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
                if json_message["Type"] == "Hardware" {
                    if json_message["Subtype"] == "Lights" {
                        if json_message["Hardware"] == "Hue" {
                            if json_message["Action"] == "OnOff" {
                                let _hardware_hue =
                                    mk_lib_hardware::mk_lib_hardware_phue::mk_hardware_phue_bridge_set_light_onoff(
                                        json_message["Target"].to_string().parse::<IpAddr>().unwrap(),
                                        json_message["ClientKey"].to_string(),
                                        json_message["LightList"].to_string(),
                                        bool::from_str(json_message["Setting"].as_str().unwrap()).unwrap(),
                                    )
                                    .await
                                    .unwrap();
                            } else if json_message["Action"] == "Color" {
                                let _hardware_hue = mk_lib_hardware::mk_lib_hardware_phue::mk_hardware_phue_bridge_set_color(
                                        json_message["Target"].to_string().parse::<IpAddr>().unwrap(),
                                        json_message["ClientKey"].to_string(),
                                        json_message["LightList"].to_string(),
                                        json_message["Color"].to_string(),
                                    ).await.unwrap();
                            }
                            // else if json_message["Action"] == "Bright" {
                            //     mk_lib_hardware::mk_lib_hardware_phue::mk_hardware_phue_bridge_set_light(
                            //         json_message["Target"].to_string().parse::<IpAddr>().unwrap(),
                            //         json_message["ClientKey"].to_string(),
                            //         json_message["LightList"].to_string(),
                            //         json_message["Saturation"].into(),
                            //         json_message["Brightness"] as i64(),
                            //     );
                            // }
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
