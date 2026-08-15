use crate::mk_lib_hardware_ssdp;
use url::Url;

const ALEXA_DISCOVERY_TARGET: &str = "urn:schemas-upnp-org:device:MediaRenderer:1";

pub async fn mk_lib_hardware_alexa_discover() -> Vec<Url> {
    let Ok(responses) =
        mk_lib_hardware_ssdp::mk_lib_hardware_ssdp_search(ALEXA_DISCOVERY_TARGET).await
    else {
        return Vec::new();
    };

    mk_lib_hardware_ssdp::mk_lib_hardware_ssdp_filter_locations(&responses, &["amazon", "alexa"])
        .into_iter()
        .filter_map(|loc| Url::parse(&loc).ok())
        .collect()
}
