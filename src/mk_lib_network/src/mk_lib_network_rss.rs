// https://github.com/rust-syndication/rss

use rss::Channel;
use std::fs::File;
use std::io::BufReader;

pub async fn rss_file_open(file_path: String) -> Result<Channel, Box<dyn std::error::Error>> {
    let file = File::open(file_path).unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
    Ok(channel)
}

pub async fn rss_url_open(url_path: String) -> Result<Channel, Box<dyn std::error::Error>> {
    let content = reqwest::get(url_path)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
