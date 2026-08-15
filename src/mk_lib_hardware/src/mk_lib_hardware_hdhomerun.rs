use crate::mk_lib_hardware_ssdp;
use mk_lib_network;
use serde::Deserialize;
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

#[cfg(test)]
mod tests {
    use super::HDHomeRunChannel;
    use serde_json;

    #[test]
    fn hdhomerun_channel_deserialize_from_json() {
        let json_str = r#"{"GuideNumber":"3.1","GuideName":"ABC","URL":"http://192.168.1.100:5004/auto/v3.1"}"#;
        let channel: HDHomeRunChannel = serde_json::from_str(json_str).unwrap();
        assert_eq!(channel.guide_number, "3.1");
        assert_eq!(channel.guide_name, "ABC");
        assert_eq!(channel.stream_url, "http://192.168.1.100:5004/auto/v3.1");
    }

    #[test]
    fn hdhomerun_channel_clone() {
        let channel = HDHomeRunChannel {
            guide_number: "5.2".to_string(),
            guide_name: "NBC".to_string(),
            stream_url: "http://192.168.1.100:5004/auto/v5.2".to_string(),
        };
        let cloned = channel.clone();
        assert_eq!(channel.guide_number, cloned.guide_number);
        assert_eq!(channel.guide_name, cloned.guide_name);
        assert_eq!(channel.stream_url, cloned.stream_url);
    }

    #[test]
    fn hdhomerun_channel_debug_format() {
        let channel = HDHomeRunChannel {
            guide_number: "1.1".to_string(),
            guide_name: "CBS".to_string(),
            stream_url: "http://192.168.1.100:5004/auto/v1.1".to_string(),
        };
        let debug_str = format!("{:?}", channel);
        assert!(debug_str.contains("HDHomeRunChannel"));
    }
}

pub async fn mk_lib_hardware_hdhomerun_discover() -> Vec<Url> {
    let Ok(responses) =
        mk_lib_hardware_ssdp::mk_lib_hardware_ssdp_search(HDHOMERUN_DISCOVERY_TARGET).await
    else {
        return Vec::new();
    };

    mk_lib_hardware_ssdp::mk_lib_hardware_ssdp_filter_locations(&responses, &["hdhomerun"])
        .into_iter()
        .filter_map(|loc| Url::parse(&loc).ok())
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
