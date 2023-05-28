use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, Exchange, QueueDeclareOptions, Result,
};
use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use mk_lib_network;
use serde_json::{json, Value};
use std::error::Error;
use std::net::IpAddr;

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
        .await
        .unwrap();
    let _option_config_json: Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
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
        for (_i, message) in consumer.receiver().iter().enumerate() {
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
                    let share_vec = mk_lib_network::mk_lib_network_nmap::mk_network_share_scan(
                        json_message["Data"].to_string(),
                    )
                    .await
                    .unwrap();
                    for share_info in share_vec.iter() {
                        mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_insert(
                            &sqlx_pool,
                            share_info.mm_share_ip.parse::<IpAddr>().unwrap(),
                            share_info.mm_share_path.clone(),
                            share_info.mm_share_comment.clone(),
                        )
                        .await;
                    }
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
