use ssdp::header::{HeaderMut, HeaderRef, MX, Man, ST};
use ssdp::message::{Multicast, SearchRequest};
use url::Url;

const TIVO_SSDP_SEARCH_TARGET: &str = "TiVoMediaServer:1";

pub fn mk_lib_hardware_tivo_discover() -> Vec<Url> {
    let mut request = SearchRequest::new();
    request.set(Man);
    request.set(MX(5));

    let Some(st_field) = ssdp::FieldMap::new(TIVO_SSDP_SEARCH_TARGET) else {
        return Vec::new();
    };
    request.set(ST::Target(st_field));

    let Ok(responses) = request.multicast() else {
        return Vec::new();
    };

    responses
        .into_iter()
        .filter_map(|(res, _)| {
            let raw_values = res.get_raw("LOCATION")?;
            let first_value = raw_values.first()?;
            let location = std::str::from_utf8(first_value).ok()?.trim();
            Url::parse(location).ok()
        })
        .collect()
}
