use mk_lib_rabbitmq;
use serde_json::Value;
use std::error::Error;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_rabbit_connection, rabbit_channel) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect(
        "mkrabbitmq",
        "mktest",
    )
    .await
    .unwrap();

    let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
        "mktest",
        &rabbit_channel,
    )
    .await
    .unwrap();

//    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                println!("Msg: {:?}", json_message);
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.unwrap().delivery_tag(),
                )
                .await;
            }
        };
//    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
