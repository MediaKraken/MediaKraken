use mk_lib_database;
use mk_lib_rabbitmq;
use serde_json::Value;
use std::error::Error;
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
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
                        Ok(_json_message) => {
                            // TODO: implement playback dispatch (cast/hdhomerun/hls/web/roku).
                            // Previously a Python translation lived here; it has been removed
                            // while the port is pending.
                        }
                        Err(error) => {
                            eprintln!("mkrabbitconsume: malformed payload ({error})");
                        }
                    }
                }

                if let Some(deliver) = msg.deliver {
                    if let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
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
}
