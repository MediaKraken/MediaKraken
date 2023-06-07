use amqprs::channel::Channel;
use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicAckArguments, BasicConsumeArguments, BasicPublishArguments, ConsumerMessage,
        QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use tokio::sync::mpsc::UnboundedReceiver;

pub async fn rabbitmq_ack(rabbit_channel: &Channel, rabbit_msg_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    rabbit_channel
        .basic_ack(BasicAckArguments::new(
            rabbit_msg_id,
            false,
        ))
        .await
        .unwrap();
    Ok(())
}

pub async fn rabbitmq_connect(
    rabbit_queue: &str,
) -> Result<(Connection, Channel), Box<dyn std::error::Error>> {
    // open a connection to RabbitMQ server
    let rabbit_connection = Connection::open(&OpenConnectionArguments::new(
        "mkstack_rabbitmq",
        5672,
        "guest",
        "guest",
    ))
    .await
    .unwrap();
    rabbit_connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();
    let rabbit_channel = rabbit_connection.open_channel(None).await.unwrap();
    rabbit_channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();
    let (queue_name, _, _) = rabbit_channel
        .queue_declare(QueueDeclareArguments::durable_client_named(rabbit_queue))
        .await
        .unwrap()
        .unwrap();
    let rounting_key = rabbit_queue;
    let exchange_name = "mediakraken";
    rabbit_channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            rounting_key,
        ))
        .await
        .unwrap();
    Ok((rabbit_connection, rabbit_channel))
}

pub async fn rabbitmq_consumer(
    rabbit_queue: &str,
    rabbit_channel: &Channel,
) -> Result<UnboundedReceiver<ConsumerMessage>, Box<dyn std::error::Error>> {
    let rabbit_args = BasicConsumeArguments::new(rabbit_queue, "mkstack_consumer")
        .manual_ack(true)
        .finish();
    let (_ctag, mut rabbit_rx) = rabbit_channel.basic_consume_rx(rabbit_args).await.unwrap();
    Ok(rabbit_rx)
}

pub async fn rabbitmq_publish(
    rabbit_channel: Channel,
    rabbit_queue: &str,
    rabbit_message: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let args = BasicPublishArguments::new("mediakraken", rabbit_queue);
    rabbit_channel
        .basic_publish(
            BasicProperties::default()
                .with_content_type("application/json")
                .with_persistence(true)
                .finish(),
            rabbit_message.into_bytes(),
            args,
        )
        .await
        .unwrap();
    Ok(())
}

pub async fn rabbitmq_close(
    rabbit_channel: Channel,
    rabbit_connection: Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    rabbit_channel.close().await.unwrap();
    rabbit_connection.close().await.unwrap();
    Ok(())
}
