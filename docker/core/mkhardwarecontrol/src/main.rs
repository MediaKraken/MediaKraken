use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, Exchange, QueueDeclareOptions, Result,
};
use mk_lib_hardware;
use mk_lib_logging::mk_lib_logging;
use mk_lib_network;
use serde_json::{json, Value};
use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;
use stdext::function_name;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}))
            .await
            .unwrap();
    }

    // open rabbit connection
    let mut rabbit_connection =
        Connection::insecure_open("amqp://guest:guest@mkstack_rabbitmq:5672")?;
    // Open a channel - None says let the library choose the channel ID.
    let rabbit_channel = rabbit_connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let _rabbit_exchange = Exchange::direct(&rabbit_channel);

    // Declare the queue.
    let queue = rabbit_channel.queue_declare("mk_hardware", QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;

    loop {
        for (i, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let json_message: Value =
                        serde_json::from_str(&String::from_utf8_lossy(&delivery.body))?;
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
                    println!("({:>3}) Received [{}]", i, json_message);
                    consumer.ack(delivery)?;
                }
                other => {
                    eprintln!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
    }
}
