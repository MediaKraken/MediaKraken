#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/LesnyRumcajs/wakey/releases/tag/v0.1.1

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn mk_lib_network_wol(mac_addr: String) {
    // "01:02:03:04:05:06"
    let wol = wakey::WolPacket::from_string(mac_addr, ':');
    let mut wol_worked: bool = false;
    if wol.send_magic().is_ok() {
        wol_worked = true;
    }
    wol_worked
}
