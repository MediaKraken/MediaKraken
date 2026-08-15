use serde_json::Value;
use std::collections::HashSet;
use std::error::Error;
use tokio::signal;

// Data-version marker stored on newly discovered shares when inserting them.
const NETWORK_SHARE_VERSION: i16 = 1;

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

async fn process_message(
    payload: &[u8],
    sqlx_pool_rw: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    let json_message: Value = serde_json::from_slice(payload)?;
    let Some(scan_target) = json_message.get("Data").and_then(Value::as_str) else {
        return Ok(());
    };

    // Find and store all network shares.
    let share_vec = mk_lib_network::mk_lib_network_share::mk_network_share_scan_port_rustscan(
        scan_target.to_string(),
    )
    .await?;

    let mut seen_shares: HashSet<(std::net::IpAddr, String)> = HashSet::new();

    for share_info in &share_vec {
        let Some(share_path) = share_info.mm_share_path.as_str() else {
            continue;
        };
        let share_comment = share_info.mm_share_comment.as_str().unwrap_or("");

        let normalized_share_path = share_path.to_owned();
        if !seen_shares.insert((share_info.mm_share_ip, normalized_share_path.clone())) {
            continue;
        }

        // Insert newly discovered shares (skip ones already present).
        if !mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_exists(
            sqlx_pool_rw,
            share_info.mm_share_ip,
            share_path,
        )
        .await?
        {
            let _guid = mk_lib_database::mk_lib_database_network_share::
                mk_lib_database_network_share_insert(
                    sqlx_pool_rw,
                    share_info.mm_share_ip,
                    share_path,
                    share_comment,
                    NETWORK_SHARE_VERSION,
                )
                .await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mksharescanner").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mksharescanner", &rabbit_channel)
            .await?;

    // Spawn the message processing task
    let handle = tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content.as_deref()
                && let Err(err) = process_message(payload, &sqlx_pool_rw).await
            {
                eprintln!("failed to process mksharescanner message: {err}");
                // Do not ack the message on error to allow for retry
                continue;
            }

            // Acknowledge message on successful processing
            if let Some(deliver) = msg.deliver
                && let Err(err) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    deliver.delivery_tag(),
                )
                .await
            {
                eprintln!("failed to acknowledge message: {err}");
            }
        }
    });

    // Wait for shutdown signal
    shutdown_signal().await;

    // Cancel the processing task
    handle.abort();

    eprintln!("mksharescanner: shutdown signal received");
    Ok(())
}
