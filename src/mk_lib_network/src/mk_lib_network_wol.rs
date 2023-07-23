// https://github.com/LesnyRumcajs/wakey

pub async fn mk_lib_network_wol(mac_addr: String) -> Result<bool, Box<dyn std::error::Error>> {
    // "01:02:03:04:05:06"
    let wol = wakey::WolPacket::from_string(mac_addr, ':')?;
    let mut wol_worked: bool = false;
    if wol.send_magic().is_ok() {
        wol_worked = true;
    }
    Ok(wol_worked)
}
