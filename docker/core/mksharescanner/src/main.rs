use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde_json::Value;
use std::collections::HashSet;
use std::error::Error;
use tokio::sync::Notify;

async fn process_message(
    payload: &[u8],
    sqlx_pool_rw: &sqlx::PgPool,
    sqlx_pool_ro: &sqlx::PgPool,
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

        let normalized_share_path = share_path.to_owned();
        if !seen_shares.insert((share_info.mm_share_ip, normalized_share_path.clone())) {
            continue;
        }

        // TODO make an upsert.
        if !mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_exists(
            sqlx_pool_ro,
            share_info.mm_share_ip,
            share_info.mm_share_path.clone(),
        )
        .await?
        {
            mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_insert(
                sqlx_pool_rw,
                share_info.mm_share_ip,
                &normalized_share_path,
                share_info.mm_share_comment.clone(),
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
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;
    let _option_config_json: Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool_ro)
            .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mksharescanner").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mksharescanner", &rabbit_channel)
            .await?;

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            let mut should_ack = true;

            if let Some(payload) = msg.content.as_deref() {
                if let Err(err) = process_message(payload, &sqlx_pool_rw, &sqlx_pool_ro).await {
                    should_ack = false;
                    eprintln!("failed to process mksharescanner message: {err}");
                }
            }

            if should_ack {
                if let Some(deliver) = msg.deliver {
                    let _ = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                        &rabbit_channel,
                        deliver.delivery_tag(),
                    )
                    .await;
                }
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
