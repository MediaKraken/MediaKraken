// https://github.com/lumeohq/onvif-rs
// onvif = { git = "https://github.com/lumeohq/onvif-rs" }

use futures_util::stream::StreamExt;
use onvif::discovery;

const MAX_CONCURRENT_JUMPERS: usize = 100;

pub async fn mk_lib_hardware_onvif_discovery() {
    discovery::DiscoveryBuilder::default()
        .run()
        .await
        .unwrap()
        .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| async move {
            println!("Device: {:?}", addr.urls);
        })
        .await;
}

pub async fn mk_lib_hardware_onvif_device_info() {
    
}
