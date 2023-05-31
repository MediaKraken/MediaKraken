// https://gitlab.com/timosaarinen/flickr-rust

use flickr::FlickrAPI;
use flickr::methods::favorites::GetListResult;
use flickr::methods::favorites::Photos;

pub async fn provider_flickr_login(
    api_key: &str,
    api_secret: &str,
) -> Result<FlickrAPI, Box<dyn std::error::Error>> {
    let flickr = FlickrAPI::new(api_key, api_secret);
    Ok(flickr)
}

pub async fn provider_flickr_favorites_photo_list(
    mut flickr_api: FlickrAPI,
) -> Result<Photos, Box<dyn std::error::Error>> {
    let res = flickr_api.favorites().get_list().perform()?;
    let results = res.photos.unwrap();
    Ok(results)
}
