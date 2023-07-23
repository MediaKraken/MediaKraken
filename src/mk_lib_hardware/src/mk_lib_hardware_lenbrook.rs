// https://crates.io/crates/lsdp

use lsdp::{net::Discover, ClassID};
use serde_json::json;
use stdext::function_name;

pub async fn mk_hardware_lenbrook_discovery() -> Result<(), Box<dyn std::error::Error>> {
    let d = Discover::start().await?;
    d.query(lsdp::QueryMessage::new(vec![ClassID::All])).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    for (_, _d) in d.inventory().await.lock().await.iter() {
        // #[cfg(debug_assertions)]
        // {
        //     mk_lib_logging::mk_logging_post_elk(
        //         std::module_path!(),
        //         json!({ "found": d.addr, "records": d.records[0].cid, "data":, d.records[0].data }),
        //     )
        //     .await
        //     .unwrap();
        // }
        continue;
    }
    Ok(())
}
