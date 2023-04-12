#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/lumeohq/onvif-rs
// onvif = { git = "https://github.com/lumeohq/onvif-rs" }

use crate::mk_lib_logging;

use onvif::discovery;
use serde_json::json;
use stdext::function_name;

const MAX_CONCURRENT_JUMPERS: usize = 100;

pub async fn mk_lib_hardware_onvif_discovery() {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
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
                .await
                .unwrap();
            }
        })
        .await;
}
