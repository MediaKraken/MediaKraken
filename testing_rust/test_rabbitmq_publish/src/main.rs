use mk_lib_rabbitmq;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (rabbit_connection, rabbit_channel) =
        "mkrabbitmq", "mktest")
            .await
            .unwrap();

    for _ndx in 1..100_000_001 {
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
            rabbit_channel.clone(),
            "mktest",
            json!({}).to_string(),
        )
        .await
        .unwrap();
    }

    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(rabbit_channel, rabbit_connection)
        .await
        .unwrap();
    Ok(())
}
