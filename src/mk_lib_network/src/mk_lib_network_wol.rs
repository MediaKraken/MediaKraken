// https://github.com/LesnyRumcajs/wakey

use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;

pub async fn mk_lib_network_wol(mac_addr: String) -> Result<bool, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // "01:02:03:04:05:06"
    let wol = wakey::WolPacket::from_string(mac_addr, ':')?;
    let mut wol_worked: bool = false;
    if wol.send_magic().is_ok() {
        wol_worked = true;
    }
    Ok(wol_worked)
}
