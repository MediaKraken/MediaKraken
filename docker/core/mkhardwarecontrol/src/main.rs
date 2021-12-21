use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, QueueDeclareOptions, Result};
use serde_json::{json, Value};
use std::error::Error;
//use std::process::Command;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_logging/src/mk_lib_logging.rs"]
mod mk_lib_logging;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_network/src/mk_lib_network.rs"]
mod mk_lib_network;

#[cfg(not(debug_assertions))]
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_network.rs"]
mod mk_lib_network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkhardwarecontrol";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // open rabbit connection
    let mut rabbit_connection = Connection::insecure_open(
        "amqp://guest:guest@mkstack_rabbitmq:5672")?;
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
                    let json_message: Value = serde_json::from_str(
                        &String::from_utf8_lossy(&delivery.body))?;
                    mk_lib_logging::mk_logging_post_elk("info",
                                                        json!({ "msg body": json_message }),
                                                        LOGGING_INDEX_NAME).await;
              /*
                      if json_message['Type'] == 'Hardware':
                if json_message['Subtype'] == 'Lights':
                    if json_message['Hardware'] == 'Hue':
                        hardware_hue = common_hardware_hue.CommonHardwareHue(json_message['Target'])
                        if json_message['Action'] == 'OnOff':
                            hardware_hue.com_hardware_hue_light_set(json_message['LightList'], 'on',
                                                                    json_message['Setting'])
                        elif json_message['Action'] == 'Bright':
                            hardware_hue.com_hardware_hue_light_set(json_message['LightList'],
                                                                    'bri',
                                                                    json_message['Setting'])
            elif json_message['Type'] == 'Hardware Scan':
                hardware_proc = subprocess.Popen('/mediakraken/main_hardware_discover.py',
                                                 stdout=subprocess.PIPE,
                                                 shell=False)

               */
                    println!("({:>3}) Received [{}]", i, json_message);
                    consumer.ack(delivery)?;
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
    }
}