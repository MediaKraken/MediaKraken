use mk_lib_hardware;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde_json::{Value, json};
use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;
use stdext::function_name;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkhardwarecontrol").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkhardwarecontrol", &rabbit_channel)
            .await?;

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value = match serde_json::from_str(&String::from_utf8_lossy(&payload)) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        continue;
                    }
                };
                if json_message["Type"] == "Hardware" {
                    if json_message["Subtype"] == "Lights" {
                        if json_message["Hardware"] == "Hue" {
                            if json_message["Action"] == "OnOff" {
                                if let Err(e) =
                                    mk_lib_hardware::mk_lib_hardware_phue::mk_hardware_phue_bridge_set_light_onoff(
                                        json_message["Target"].to_string().parse::<IpAddr>().unwrap_or_else(|_| "0.0.0.0".parse().unwrap()),
                                        json_message["ClientKey"].to_string(),
                                        json_message["LightList"].to_string(),
                                        bool::from_str(json_message["Setting"].as_str().unwrap_or("")).unwrap_or(false),
                                    )
                                    .await
                                {
                                    eprintln!("Phue OnOff failed: {}", e);
                                }
                            } else if json_message["Action"] == "Color" {
                                if let Err(e) = mk_lib_hardware::mk_lib_hardware_phue::mk_hardware_phue_bridge_set_color(
                                        json_message["Target"].to_string().parse::<IpAddr>().unwrap_or_else(|_| "0.0.0.0".parse().unwrap()),
                                        json_message["ClientKey"].to_string(),
                                        json_message["LightList"].to_string(),
                                        json_message["Color"].to_string(),
                                    ).await
                                {
                                    eprintln!("Phue Color failed: {}", e);
                                }
                            } else if json_message["Action"] == "Bright" {
                                let _hardware_hue = mk_lib_hardware::mk_lib_hardware_phue::mk_hardware_phue_bridge_set_light(
                                    json_message["Target"].to_string().parse::<IpAddr>().unwrap_or_else(|_| "0.0.0.0".parse().unwrap()),
                                    json_message["ClientKey"].to_string(),
                                    json_message["LightList"].to_string(),
                                    json_message["Saturation"].as_u64(),
                                    json_message["Brightness"].as_u64(),
                                );
                            }
                        }
                    }
                }
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.map(|d| d.delivery_tag()).unwrap_or(0),
                )
                .await;
            }
        }
    });
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
