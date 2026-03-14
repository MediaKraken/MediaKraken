use ssdp::header::{HeaderMut, MX, Man, ST};
use ssdp::message::SearchRequest;
use url::Url;

const TIVO_SSDP_SEARCH_TARGET: &str = "TiVoMediaServer:1";

pub async fn mk_lib_hardware_tivo_discover() -> Vec<Url> {
    let mut request = SearchRequest::new();
    request.set(Man);
    request.set(MX(5));

    let Ok(st_field) = ssdp::FieldMap::new(TIVO_SSDP_SEARCH_TARGET) else {
        return Vec::new();
    };
    request.set(ST::Target(st_field));

    let Ok(responses) = request.multicast() else {
        return Vec::new();
    };

    responses
        .into_iter()
        .filter_map(|(res, _)| {
            let location_header = res.get_raw("Location")?;
            let first_value = location_header.first()?;
            Url::parse(&String::from_utf8_lossy(first_value)).ok()
        })
        .collect()
}
