#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/lumeohq/onvif-rs
// onvif = { git = "https://github.com/lumeohq/onvif-rs" }

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use onvif::discovery;

const MAX_CONCURRENT_JUMPERS: usize = 100;

pub async fn mk_lib_hardware_onvif_discovery() {
    discovery::discover(std::time::Duration::from_secs(1))
        .await
        .unwrap()
        .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| async move {
            #[cfg(debug_assertions)]
            {
                mk_lib_logging::mk_logging_post_elk(
                    std::module_path!(),
                    json!({ "Device found": addr }),
                )
                .await;
            }
        })
        .await;
}
