use amqprs::channel::{BasicQosArguments, Channel};
use amqprs::{
    BasicProperties,
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicAckArguments, BasicConsumeArguments, BasicPublishArguments, ConsumerMessage,
        QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;

const RABBITMQ_HOST: &str = "mkstack-rabbitmq-production.rabbitmq-system";
const RABBITMQ_PORT: u16 = 5672;
const RABBITMQ_USER: &str = "guest";
const RABBITMQ_PASSWORD: &str = "guest";
const DEFAULT_EXCHANGE: &str = "amq.direct";
const CONSUMER_TAG: &str = "mkrabbitconsume";
const CONTENT_TYPE_JSON: &str = "application/json";
const PUBLISH_RETRY_DELAY: Duration = Duration::from_secs(5);
const PUBLISH_MAX_ATTEMPTS: u32 = 3;

pub async fn rabbitmq_ack(
    rabbit_channel: &Channel,
    rabbit_msg_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    rabbit_channel
        .basic_ack(BasicAckArguments::new(rabbit_msg_id, false))
        .await?;
    Ok(())
}

pub async fn rabbitmq_qos(
    rabbit_channel: &Channel,
    rabbit_prefetch_count: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    rabbit_channel
        .basic_qos(BasicQosArguments::new(0, rabbit_prefetch_count, false))
        .await?;
    Ok(())
}

pub async fn rabbitmq_connect(
    rabbit_queue: &str,
) -> Result<(Connection, Channel), Box<dyn std::error::Error>> {
    let rabbit_connection = Connection::open(&OpenConnectionArguments::new(
        RABBITMQ_HOST,
        RABBITMQ_PORT,
        RABBITMQ_USER,
        RABBITMQ_PASSWORD,
    ))
    .await?;
    rabbit_connection
        .register_callback(DefaultConnectionCallback)
        .await?;

    let rabbit_channel = rabbit_connection.open_channel(None).await?;
    rabbit_channel
        .register_callback(DefaultChannelCallback)
        .await?;

    let (queue_name, _, _) = rabbit_channel
        .queue_declare(QueueDeclareArguments::durable_client_named(rabbit_queue))
        .await?
        .ok_or("failed to declare queue")?;

    rabbit_channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            DEFAULT_EXCHANGE,
            rabbit_queue,
        ))
        .await?;

    Ok((rabbit_connection, rabbit_channel))
}

pub async fn rabbitmq_consumer(
    rabbit_queue: &str,
    rabbit_channel: &Channel,
) -> Result<UnboundedReceiver<ConsumerMessage>, Box<dyn std::error::Error>> {
    let rabbit_args = BasicConsumeArguments::new(rabbit_queue, CONSUMER_TAG)
        .manual_ack(true)
        .finish();
    let (_consumer_tag, rabbit_rx) = rabbit_channel.basic_consume_rx(rabbit_args).await?;
    Ok(rabbit_rx)
}

/// Re-establish a full consumer session: open the connection, declare/bind the
/// durable queue, apply a per-consumer prefetch cap, and start consuming.
///
/// Intended for consumers that must survive broker disconnects: when the
/// consumer receiver closes (the connection dropped), call this again to come
/// back. amqprs' default connection callbacks do not reconnect on their own.
pub async fn rabbitmq_reestablish(
    rabbit_queue: &str,
    rabbit_prefetch_count: u16,
) -> Result<(Connection, Channel, UnboundedReceiver<ConsumerMessage>), Box<dyn std::error::Error>> {
    let (rabbit_connection, rabbit_channel) = rabbitmq_connect(rabbit_queue).await?;
    rabbitmq_qos(&rabbit_channel, rabbit_prefetch_count).await?;
    let rabbit_consumer = rabbitmq_consumer(rabbit_queue, &rabbit_channel).await?;
    Ok((rabbit_connection, rabbit_channel, rabbit_consumer))
}

pub async fn rabbitmq_publish(
    rabbit_channel: Channel,
    rabbit_queue: &str,
    rabbit_message: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut args = BasicPublishArguments::new(DEFAULT_EXCHANGE, rabbit_queue);
    args.mandatory(true);
    let properties = BasicProperties::default()
        .with_content_type(CONTENT_TYPE_JSON)
        .with_persistence(true)
        .finish();

    rabbit_channel
        .basic_publish(properties, rabbit_message.into_bytes(), args)
        .await?;

    Ok(())
}

/// Publish a message, waiting for the target queue to exist first.
///
/// `rabbitmq_connect` declares the caller's own queue, but this one publishes
/// to *another* service's queue (e.g. `"mktranscode"` from the web admin). If
/// that queue does not exist yet (consumer not running, or mid-restart), the
/// broker cannot route the message even when it is `mandatory`.
///
/// Each attempt opens a fresh channel and issues a passive `queue_declare`
/// as an existence check: it succeeds when the queue exists and fails
/// (404, channel closed by the broker) when it does not, without creating or
/// modifying anything. After `PUBLISH_MAX_ATTEMPTS` attempts spaced
/// `PUBLISH_RETRY_DELAY` apart, the last error is reported.
///
/// A new channel is required per attempt because a broker-initiated
/// `channel.close` (the 404 response) terminates that channel in amqprs.
pub async fn rabbitmq_publish_with_retry(
    rabbit_connection: &Connection,
    rabbit_queue: &str,
    rabbit_message: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_error = String::new();
    for attempt in 1..=PUBLISH_MAX_ATTEMPTS {
        let channel = rabbit_connection.open_channel(None).await?;
        channel.register_callback(DefaultChannelCallback).await?;

        match channel
            .queue_declare(
                QueueDeclareArguments::durable_client_named(rabbit_queue)
                    .passive(true)
                    .finish(),
            )
            .await
        {
            Ok(_) => {
                // Queue exists: publish on this channel. The channel handle is
                // moved into rabbitmq_publish and gracefully closed on drop.
                return rabbitmq_publish(channel, rabbit_queue, rabbit_message).await;
            }
            Err(error) => {
                last_error = error.to_string();
                if attempt < PUBLISH_MAX_ATTEMPTS {
                    tokio::time::sleep(PUBLISH_RETRY_DELAY).await;
                }
            }
        }
    }
    Err(format!(
        "queue '{rabbit_queue}' not available after {PUBLISH_MAX_ATTEMPTS} publish attempts: {last_error}"
    )
    .into())
}

pub async fn rabbitmq_close(
    rabbit_channel: Channel,
    rabbit_connection: Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    rabbit_channel.close().await?;
    rabbit_connection.close().await?;
    Ok(())
}
