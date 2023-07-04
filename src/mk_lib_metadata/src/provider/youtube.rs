// https://github.com/Mithronn/rusty_ytdl

use rusty_ytdl::Video;
use select::document::Document;
use select::predicate::{Attr, Name};
use std::error::Error;
use std::io::Read;
use std::io::Write;
use std::str;

pub async fn provider_youtube_video_fetch(video_url: &str, download_path: &str) {
    let video = Video::new(video_url).unwrap();
    let video_download_buffer = video.download().await;
    if video_download_buffer.is_ok() {
        let path = std::path::Path::new(download_path);
        let mut file = std::fs::File::create(path).unwrap();
        let _info = file.write_all(&video_download_buffer.unwrap());
    }
}

pub async fn provider_youtube_trending(country_code: &str) -> Result<(), Box<dyn Error>> {
    let mut link_list: Vec<String> = Vec::new();
    let url = format!("https://www.youtube.com/feed/trending?gl={}", country_code);
    let response = reqwest::get(&url).await?.text().await?;
    let document = Document::from(response.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    // for node in document.find(Attr("href", "/watch")) {
    //     link_list.push(node.attr("href").unwrap().to_string());
    // }
    // let link_list2: Vec<String> = link_list
    //     .into_iter()
    //     .step_by(2)
    //     .map(|link| format!("www.youtube.com{}", link))
    //     .collect();
    // Ok(link_list2)
    Ok(())
}
