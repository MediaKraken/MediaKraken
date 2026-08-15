use serde_json::Value;
use std::error::Error;
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        if signal::ctrl_c().await.is_err() {
            eprintln!("Failed to install Ctrl+C handler");
        }
    };

    let terminate = async {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut s) => s.recv().await,
            Err(e) => {
                eprintln!("Failed to install SIGTERM handler: {}", e);
                None
            }
        };
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;
    let _option_config_json =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool_ro)
            .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkrabbitconsume").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkrabbitconsume", &rabbit_channel)
            .await?;

    loop {
        tokio::select! {
            _ = shutdown_signal() => {
                eprintln!("mkrabbitconsume: shutdown signal received");
                return Ok(());
            }
            msg = rabbit_consumer.recv() => {
                let Some(msg) = msg else {
                    eprintln!("mkrabbitconsume: rabbit consumer closed");
                    return Ok(());
                };

                if let Some(payload) = msg.content {
                    match serde_json::from_slice::<Value>(&payload) {
                        Ok(json_message) => {
                            // Process playback dispatch based on message type
                            if let Some(playback_type) = json_message.get("playback_type") {
                                match playback_type.as_str() {
                                    Some("cast") => {
                                        eprintln!("mkrabbitconsume: dispatching to cast playback");
                                        // TODO: implement cast playback handling
                                    }
                                    Some("hdhomerun") => {
                                        eprintln!("mkrabbitconsume: dispatching to hdhomerun playback");
                                        // TODO: implement hdhomerun playback handling
                                    }
                                    Some("hls") => {
                                        eprintln!("mkrabbitconsume: dispatching to hls playback");
                                        // TODO: implement hls playback handling
                                    }
                                    Some("web") => {
                                        eprintln!("mkrabbitconsume: dispatching to web playback");
                                        // TODO: implement web playback handling
                                    }
                                    Some("roku") => {
                                        eprintln!("mkrabbitconsume: dispatching to roku playback");
                                        // TODO: implement roku playback handling
                                    }
                                    _ => {
                                        eprintln!("mkrabbitconsume: unknown playback type: {}", playback_type);
                                    }
                                }
                            } else {
                                eprintln!("mkrabbitconsume: no playback type in message");
                            }
                        }
                        Err(error) => {
                            eprintln!("mkrabbitconsume: malformed payload ({error})");
                        }
                    }
                }

                if let Some(deliver) = msg.deliver
                    && let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                        &rabbit_channel,
                        deliver.delivery_tag(),
                    )
                    .await
                    {
                        eprintln!("mkrabbitconsume: rabbit ack failed ({error})");
                    }
            }
        }
    }
}
