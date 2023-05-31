// https://github.com/lumeohq/onvif-rs
// onvif = { git = "https://github.com/lumeohq/onvif-rs" }

use futures_util::stream::StreamExt;
use mk_lib_logging::mk_lib_logging;
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
    discovery::DiscoveryBuilder::default()
        .run()
        .await
        .unwrap()
        .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| async move {
            #[cfg(debug_assertions)]
            {
                mk_lib_logging::mk_logging_post_elk(
                    std::module_path!(),
                    json!({ "Device found": format!("{:?}", addr) }),
                )
                .await
                .unwrap();
            }
        })
        .await;
}
