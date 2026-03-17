use mk_lib_network;
use serde::Deserialize;
use ssdp::header::{HeaderMut, HeaderRef, MX, Man, ST};
use ssdp::message::{Multicast, SearchRequest};
use url::Url;

const HDHOMERUN_DISCOVERY_TARGET: &str = "upnp:rootdevice";

#[derive(Debug, Clone, Deserialize)]
pub struct HDHomeRunChannel {
    #[serde(rename = "GuideNumber")]
    pub guide_number: String,
    #[serde(rename = "GuideName")]
    pub guide_name: String,
    #[serde(rename = "URL")]
    pub stream_url: String,
}

pub async fn mk_lib_hardware_hdhomerun_discover() -> Vec<Url> {
    let mut request = SearchRequest::new();
    request.set(Man);
    request.set(MX(5));

    let Some(target) = ssdp::FieldMap::new(HDHOMERUN_DISCOVERY_TARGET) else {
        return Vec::new();
    };

    request.set(ST::Target(target));

    let Ok(responses) = request.multicast() else {
        return Vec::new();
    };

    responses
        .into_iter()
        .filter_map(|(response, _)| {
            let looks_like_hdhomerun = response
                .get_raw("SERVER")
                .into_iter()
                .flatten()
                .chain(response.get_raw("USN").into_iter().flatten())
                .chain(response.get_raw("ST").into_iter().flatten())
                .any(|value| {
                    String::from_utf8_lossy(value)
                        .to_ascii_lowercase()
                        .contains("hdhomerun")
                });

            if !looks_like_hdhomerun {
                return None;
            }

            let location = response.get_raw("LOCATION")?.first()?;
            Url::parse(&String::from_utf8_lossy(location)).ok()
        })
        .collect()
}

pub async fn mk_lib_hardware_hdhomerun_channel_discover() -> Vec<HDHomeRunChannel> {
    let mut channels = Vec::new();

    for device_location in mk_lib_hardware_hdhomerun_discover().await {
        let mut lineup_url = device_location;
        lineup_url.set_path("/lineup.json");
        lineup_url.set_query(None);
        lineup_url.set_fragment(None);

        let Ok(lineup_json) =
            mk_lib_network::mk_lib_network::mk_data_from_url_to_json(lineup_url.to_string()).await
        else {
            continue;
        };

        let Ok(mut device_channels) = serde_json::from_value::<Vec<HDHomeRunChannel>>(lineup_json)
        else {
            continue;
        };

        channels.append(&mut device_channels);
    }

    channels
}