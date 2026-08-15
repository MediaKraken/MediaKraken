use crate::mk_lib_hardware_ssdp;
use url::Url;

const SYNOLOGY_DISCOVERY_TARGET: &str = "ssdp:all";

pub async fn mk_lib_hardware_synology_discover() -> Vec<Url> {
    let Ok(responses) =
        mk_lib_hardware_ssdp::mk_lib_hardware_ssdp_search(SYNOLOGY_DISCOVERY_TARGET).await
    else {
        return Vec::new();
    };

    mk_lib_hardware_ssdp::mk_lib_hardware_ssdp_filter_locations(&responses, &["synology"])
        .into_iter()
        .filter_map(|loc| Url::parse(&loc).ok())
        .collect()
}
