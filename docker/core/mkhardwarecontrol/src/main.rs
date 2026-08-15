use serde_json::Value;
use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;
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
    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkhardwarecontrol").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkhardwarecontrol", &rabbit_channel)
            .await?;

    // Spawn the message-processing task and keep its handle so we can stop it on shutdown.
    let handle = tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    match serde_json::from_str(&String::from_utf8_lossy(&payload)) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                            continue;
                        }
                    };

                if json_message["Type"] == "Hardware"
                    && json_message["Subtype"] == "Lights"
                    && json_message["Hardware"] == "Hue"
                {
                    // Parse the target once. Reject (and log) a missing/invalid address instead of
                    // silently defaulting to localhost, which would mis-route commands nowhere useful.
                    match json_message["Target"]
                        .as_str()
                        .map(|t| t.trim())
                        .filter(|t| !t.is_empty())
                        .and_then(|t| t.parse::<IpAddr>().ok())
                    {
                        Some(target_ip) => {
                            let client_key = json_message["ClientKey"].to_string();

                            if json_message["Action"] == "OnOff" {
                                if let Err(e) = mk_lib_hardware::mk_lib_hardware_phue::
                                    mk_hardware_phue_bridge_set_light_onoff(
                                        target_ip,
                                        client_key.clone(),
                                        json_message["LightList"].to_string(),
                                        bool::from_str(json_message["Setting"]
                                            .as_str()
                                            .unwrap_or(""))
                                        .unwrap_or(false),
                                    )
                                    .await
                                {
                                    eprintln!("Phue OnOff failed: {}", e);
                                }
                            } else if json_message["Action"] == "Color" {
                                if let Err(e) = mk_lib_hardware::mk_lib_hardware_phue::
                                    mk_hardware_phue_bridge_set_color(
                                        target_ip,
                                        client_key.clone(),
                                        json_message["LightList"].to_string(),
                                        json_message["Color"].to_string(),
                                    )
                                    .await
                                {
                                    eprintln!("Phue Color failed: {}", e);
                                }
                            } else if json_message["Action"] == "Bright" {
                                // Previously this future was created but never awaited, so the command silently no-oped.
                                if let Err(e) = mk_lib_hardware::mk_lib_hardware_phue::
                                    mk_hardware_phue_bridge_set_light(
                                        target_ip,
                                        client_key,
                                        json_message["LightList"].to_string(),
                                        json_message["Saturation"].as_u64(),
                                        json_message["Brightness"].as_u64(),
                                    )
                                    .await
                                {
                                    eprintln!("Phue Bright failed: {}", e);
                                }
                            }
                        }
                        None => eprintln!(
                            "Hue command has missing or invalid Target '{}', skipping",
                            json_message["Target"].as_str().unwrap_or("<absent>")
                        ),
                    }
                }

                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.map(|d| d.delivery_tag()).unwrap_or(0),
                )
                .await;
            }
        }
    });

    // Wait for a shutdown signal, then stop the consumer task.
    shutdown_signal().await;
    handle.abort();
    eprintln!("mkhardwarecontrol: shutdown signal received");
    Ok(())
}
