// https://github.com/Mithronn/rusty_ytdl

use std::io::Write;
use rusty_ytdl::Video;

pub async fn provider_youtube_video_fetch(video_url: &str, download_path: &str) {
    let video = Video::new(video_url).unwrap();
    let video_download_buffer = video.download().await;
    if video_download_buffer.is_ok() {
        let path = std::path::Path::new(download_path);
        let mut file = std::fs::File::create(path).unwrap();
        let _info = file.write_all(&video_download_buffer.unwrap());
    }
}
