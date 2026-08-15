use crate::mk_lib_hardware_ssdp;
use url::Url;

const TIVO_SSDP_SEARCH_TARGET: &str = "TiVoMediaServer:1";

pub async fn mk_lib_hardware_tivo_discover() -> Vec<Url> {
    let Ok(responses) =
        mk_lib_hardware_ssdp::mk_lib_hardware_ssdp_search(TIVO_SSDP_SEARCH_TARGET).await
    else {
        return Vec::new();
    };

    responses
        .iter()
        .filter_map(|resp| Url::parse(resp.location()).ok())
        .collect()
}
