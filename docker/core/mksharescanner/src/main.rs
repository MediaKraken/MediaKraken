#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, Exchange, QueueDeclareOptions, Result,
};
use serde_json::{json, Value};
use std::error::Error;
use std::path::Path;
use std::process::Command;
//use rustube::{Id, VideoFetcher};
use stdext::function_name;

#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[path = "mk_lib_database_option_status.rs"]
mod mk_lib_database_option_status;
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;
#[path = "mk_lib_network_nmap.rs"]
mod mk_lib_network_nmap;

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
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool(1).await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();
    let option_config_json: Value =
        mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();

    // open rabbit connection
    let mut rabbit_connection =
        Connection::insecure_open("amqp://guest:guest@mkstack_rabbitmq:5672")?;
    // Open a channel - None says let the library choose the channel ID.
    let rabbit_channel = rabbit_connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let _rabbit_exchange = Exchange::direct(&rabbit_channel);

    // Declare the queue.
    let queue = rabbit_channel.queue_declare("mk_sharescanner", QueueDeclareOptions::default())?;

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
                    // find and store all network shares
                    let share_vec = mk_lib_network_nmap::mk_network_share_scan("192.168.1.1".to_string()).await.unwrap();
                    for share_info in share_vec.iter() {}
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
