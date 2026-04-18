use amqprs::channel::{BasicAckArguments, BasicNackArguments};
use mk_lib_database::mk_lib_database_network_share;
use mk_lib_logging::mk_lib_logging_loki;
use mk_lib_network::mk_lib_network_share;
use mk_lib_rabbitmq::mk_lib_rabbitmq as rabbit;
use serde_json::{Value, json};
use std::collections::HashSet;
use std::error::Error;
use std::net::IpAddr;
use tokio::signal;

// Match the stripping logic in mk_lib_database_network_share so that the
// in-memory dedup set and the DB existence check agree on a single canonical
// share path.
fn normalize_share_path(raw: &str) -> Option<String> {
    let cleaned = raw.replace("\\\\", "/");
    let parts: Vec<&str> = cleaned.splitn(3, '/').collect();
    parts.get(1).filter(|s| !s.is_empty()).map(|s| (*s).to_string())
}

// scan_target arrives from RabbitMQ as an untrusted string and is spliced into
// the CIDR passed to rustscan/nmap. Only accept dotted triplets in the 0-255
// range so callers cannot widen the scan (e.g. "0.0.0/0") or inject extra args.
fn is_valid_subnet_prefix(input: &str) -> bool {
    let parts: Vec<&str> = input.split('.').collect();
    parts.len() == 3 && parts.iter().all(|p| p.parse::<u8>().is_ok())
}

async fn log(event: &str, fields: Value) {
    let mut payload = json!({
        "module": std::module_path!(),
        "event": event,
    });
    if let (Some(obj), Some(extra)) = (payload.as_object_mut(), fields.as_object()) {
        for (k, v) in extra {
            obj.insert(k.clone(), v.clone());
        }
    }
    let _ = mk_lib_logging_loki::mk_logging_loki_push(payload).await;
}

async fn process_message(
    payload: &[u8],
    sqlx_pool_rw: &sqlx::PgPool,
    sqlx_pool_ro: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    let json_message: Value = serde_json::from_slice(payload)?;
    let Some(scan_target) = json_message.get("Data").and_then(Value::as_str) else {
        log("drop_message", json!({ "reason": "missing Data field" })).await;
        return Ok(());
    };
    if !is_valid_subnet_prefix(scan_target) {
        log("reject_target", json!({ "target": scan_target })).await;
        return Ok(());
    }

    let share_vec = mk_lib_network_share::mk_network_share_scan_port_rustscan(
        scan_target.to_string(),
    )
    .await?;

    let mut seen: HashSet<(IpAddr, String)> = HashSet::new();

    for share in &share_vec {
        let Some(raw_path) = share.mm_share_path.as_str() else {
            continue;
        };
        let Some(norm_path) = normalize_share_path(raw_path) else {
            continue;
        };
        if !seen.insert((share.mm_share_ip, norm_path)) {
            continue;
        }

        let comment = share.mm_share_comment.as_str().unwrap_or("");

        if !mk_lib_database_network_share::mk_lib_database_network_share_exists(
            sqlx_pool_ro,
            share.mm_share_ip,
            raw_path,
        )
        .await?
        {
            mk_lib_database_network_share::mk_lib_database_network_share_insert(
                sqlx_pool_rw,
                share.mm_share_ip,
                raw_path,
                comment,
                share.mm_share_type,
            )
            .await?;
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
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
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (rabbit_connection, rabbit_channel) = rabbit::rabbitmq_connect("mksharescanner").await?;
    let mut rabbit_consumer =
        rabbit::rabbitmq_consumer("mksharescanner", &rabbit_channel).await?;

    let consumer_loop = async {
        while let Some(msg) = rabbit_consumer.recv().await {
            let Some(deliver) = msg.deliver else {
                continue;
            };
            let delivery_tag = deliver.delivery_tag();
            let payload = msg.content.as_deref().unwrap_or(&[]);

            match process_message(payload, &sqlx_pool_rw, &sqlx_pool_ro).await {
                Ok(()) => {
                    if let Err(err) = rabbit_channel
                        .basic_ack(BasicAckArguments::new(delivery_tag, false))
                        .await
                    {
                        log("ack_error", json!({ "error": err.to_string() })).await;
                    }
                }
                Err(err) => {
                    log("process_error", json!({ "error": err.to_string() })).await;
                    if let Err(nack_err) = rabbit_channel
                        .basic_nack(BasicNackArguments::new(delivery_tag, false, true))
                        .await
                    {
                        log("nack_error", json!({ "error": nack_err.to_string() })).await;
                    }
                }
            }
        }
    };

    tokio::select! {
        _ = consumer_loop => {},
        _ = shutdown_signal() => {},
    }

    let _ = rabbit::rabbitmq_close(rabbit_channel, rabbit_connection).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subnet_prefix_accepts_dotted_triplet() {
        assert!(is_valid_subnet_prefix("192.168.1"));
        assert!(is_valid_subnet_prefix("10.0.0"));
        assert!(is_valid_subnet_prefix("255.255.255"));
    }

    #[test]
    fn subnet_prefix_rejects_bad_input() {
        assert!(!is_valid_subnet_prefix(""));
        assert!(!is_valid_subnet_prefix("192.168"));
        assert!(!is_valid_subnet_prefix("192.168.1.1"));
        assert!(!is_valid_subnet_prefix("192.168.256"));
        assert!(!is_valid_subnet_prefix("192.168.1/24"));
        assert!(!is_valid_subnet_prefix("192.168.a"));
        assert!(!is_valid_subnet_prefix("; rm -rf /"));
    }

    #[test]
    fn normalize_matches_db_insert_keying() {
        // DB insert binds parts[1] of splitn(3, '/') after "\\\\" -> "/"; the
        // dedup key in the service must match it char-for-char.
        assert_eq!(
            normalize_share_path("/volume1/testshare"),
            Some("volume1".to_string())
        );
        assert_eq!(
            normalize_share_path("\\\\server\\share"),
            Some("server\\share".to_string())
        );
    }

    #[test]
    fn normalize_rejects_empty_component() {
        assert_eq!(normalize_share_path("//"), None);
        assert_eq!(normalize_share_path(""), None);
    }
}
