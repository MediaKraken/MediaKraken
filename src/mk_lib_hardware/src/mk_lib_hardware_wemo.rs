#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/Hyperchaotic/weectrl

use weectrl::*;
use tokio_core::reactor::Core;
use futures::stream::Stream;

pub async fn mk_lib_hardware_wemo_discover() {
    let mut core = Core::new().unwrap();
    let discovery: ControllerStream<DeviceInfo> = controller.discover_future(DiscoveryMode::CacheAndBroadcast, true, 3);
    let processor = discovery.for_each(|o| {
        info!(" Got device {:?}", o.unique_id);
        Ok(())
    });
    core.run(processor).unwrap();
}
