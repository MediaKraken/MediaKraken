#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/coral/lsdp
// lsdp = "0.1.0"

use lsdp::{net::Discover, ClassID};

pub async fn mk_hardware_lenbrook_discovery() {
    let d = Discover::start().await?;
    d.query(lsdp::QueryMessage::new(vec![ClassID::All])).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    for (_, d) in d.inventory().await.lock().await.iter() {
        println!(
            "Found {}: {:?} with data {:?}",
            d.addr, d.records[0].cid, d.records[0].data
        );
    }
}