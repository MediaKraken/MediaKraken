use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde_json::{json, Value};
use std::error::Error;
use std::net::IpAddr;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();
    let _option_config_json: Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
            .await
            .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkstack_rabbitmq", "mksharescanner")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mksharescanner", &rabbit_channel)
            .await
            .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                // #[cfg(debug_assertions)]
                // {
                //     mk_lib_logging::mk_logging_post_elk(
                //         std::module_path!(),
                //         json!({ "msg body": json_message }),
                //     )
                //     .await
                //     .unwrap();
                // }
                // find and store all network shares
                let share_vec = mk_lib_network::mk_lib_network_nmap::mk_network_share_scan(
                    json_message["Data"].to_string().replace("\"", ""),
                )
                .await
                .unwrap();
                for share_info in share_vec.iter() {
                    mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_insert(
                            &sqlx_pool,
                            share_info.mm_share_ip.parse::<IpAddr>().unwrap(),
                            share_info.mm_share_path.clone(),
                            share_info.mm_share_comment.clone(),
                        )
                        .await.unwrap();
                }
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.unwrap().delivery_tag(),
                )
                .await;
            }
        }
    });
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
