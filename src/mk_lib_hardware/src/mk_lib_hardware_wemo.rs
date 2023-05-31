// https://github.com/Hyperchaotic/weectrl

use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;
use std::time::Duration;
use weectrl::{DiscoveryMode, WeeController};
use futures::prelude::*;

pub async fn mk_lib_hardware_wemo_discover() {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let controller = WeeController::new();
    // Find switches on the network.
    let mut discovery_future = controller.discover_future(
        DiscoveryMode::CacheAndBroadcast,
        true,
        Duration::from_secs(5),
    );    
    while let Some(d) = discovery_future.next().await {
        println!(
            " Found device {}, ID: {}, state: {:?}.",
            d.friendly_name, d.unique_id, d.state
        );
        let res = controller.subscribe(
            &d.unique_id,
            Duration::from_secs(120),
            true,
            Duration::from_secs(5),
        );
        println!(
            " - Subscribed {} - {:?} seconds to resubscribe.\n",
            d.friendly_name,
            res.unwrap()
        );
    }
}
