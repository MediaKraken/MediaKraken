use ssdp::header::{HeaderMut, HeaderRef, MX, Man, ST};
use ssdp::message::{Multicast, SearchRequest};
use url::Url;

const SYNOLOGY_DISCOVERY_TARGET: &str = "ssdp:all";

pub async fn mk_lib_hardware_synology_discover() -> Vec<Url> {
    let mut request = SearchRequest::new();
    request.set(Man);
    request.set(MX(5));

    let Ok(target) = ssdp::FieldMap::new(SYNOLOGY_DISCOVERY_TARGET) else {
        return Vec::new();
    };

    request.set(ST::Target(target));

    let Ok(responses) = request.multicast() else {
        return Vec::new();
    };

    responses
        .into_iter()
        .filter_map(|(response, _)| {
            let looks_like_synology = response
                .get_raw("SERVER")
                .into_iter()
                .flatten()
                .chain(response.get_raw("USN").into_iter().flatten())
                .chain(response.get_raw("Location").into_iter().flatten())
                .any(|value| {
                    String::from_utf8_lossy(value)
                        .to_ascii_lowercase()
                        .contains("synology")
                });

            if !looks_like_synology {
                return None;
            }

            let location = response.get_raw("Location")?.first()?;
            Url::parse(&String::from_utf8_lossy(location)).ok()
        })
        .collect()
}
