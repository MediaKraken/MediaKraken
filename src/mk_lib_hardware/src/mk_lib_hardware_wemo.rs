// https://github.com/Hyperchaotic/weectrl

use futures::prelude::*;
use std::time::Duration;
use weectrl::{DiscoveryMode, WeeController};

pub async fn mk_lib_hardware_wemo_discover() {
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
        match res {
            Ok(subscribe_seconds) => println!(
                " - Subscribed {} - {:?} seconds to resubscribe.\n",
                d.friendly_name, subscribe_seconds
            ),
            Err(error) => eprintln!(" - Failed to subscribe {:?} - {:?}", d.friendly_name, error),
        }
    }
}
