// https://gitlab.com/timosaarinen/flickr-rust

use flickr::FlickrAPI;

pub async fn provider_tmdb_movie_fetch(api_key: &str, api_secret: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut flickr = FlickrAPI::new(api_key, api_secret);
    let res = flickr.favorites().get_list().perform()?;
    println!("{:#?}", res.photos.unwrap());
    Ok(())
}
