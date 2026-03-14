use ssdp::header::{HeaderMut, HeaderRef, Man, MX, ST};
use ssdp::message::{Multicast, SearchRequest};
use url::Url;

const ALEXA_DISCOVERY_TARGET: &str = "urn:schemas-upnp-org:device:MediaRenderer:1";

pub async fn mk_lib_hardware_alexa_discover() -> Vec<Url> {
    let mut request = SearchRequest::new();
    request.set(Man);
    request.set(MX(5));

    let Ok(target) = ssdp::FieldMap::new(ALEXA_DISCOVERY_TARGET) else {
        return Vec::new();
    };

    request.set(ST::Target(target));

    let Ok(responses) = request.multicast() else {
        return Vec::new();
    };

    responses
        .into_iter()
        .filter_map(|(response, _)| {
            let looks_like_alexa = response
                .get_raw("SERVER")
                .into_iter()
                .flatten()
                .chain(response.get_raw("USN").into_iter().flatten())
                .any(|value| {
                    let value = String::from_utf8_lossy(value);
                    let lowercase_value = value.to_ascii_lowercase();
                    lowercase_value.contains("amazon") || lowercase_value.contains("alexa")
                });

            if !looks_like_alexa {
                return None;
            }

            let location = response.get_raw("Location")?.first()?;
            Url::parse(&String::from_utf8_lossy(location)).ok()
        })
        .collect()
}
