#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/lumeohq/onvif-rs
// onvif = { git = "https://github.com/lumeohq/onvif-rs" }

use onvif::discovery;

const MAX_CONCURRENT_JUMPERS: usize = 100;

pub async fn mk_lib_hardware_onvif_discovery() {
    discovery::discover(std::time::Duration::from_secs(1))
        .await
        .unwrap()
        .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| async move {
            #[cfg(debug_assertions)]
            {
                println!("Device found: {:?}", addr);
            }
        })
        .await;
}
