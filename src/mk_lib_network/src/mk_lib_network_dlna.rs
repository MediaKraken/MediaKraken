#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde_json::json;

// https://github.com/gabrielmagno/crab-dlna

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use crab_dlna::{
    get_serve_ip, infer_subtitle_from_video, play, Error, MediaStreamingServer, Render, RenderSpec,
};
use std::path::PathBuf;

pub async fn mk_lib_network_dlna_discover() {
    let discover_timeout_secs = 5;
    let renders_discovered = Render::discover(discover_timeout_secs).await.unwrap();
    for render in renders_discovered {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "render": render }))
                .await.unwrap();
        }
    }
}

pub async fn mk_lib_network_dlna_play(filename_to_play: String) -> Result<(), Error> {
    let discover_timeout_secs = 5;
    let render_spec = RenderSpec::Query(discover_timeout_secs, "Kodi".to_string());
    let render = Render::new(render_spec).await?;
    let host_ip = get_serve_ip(&render.host()).await?;
    let video_path = PathBuf::from(filename_to_play);
    let inferred_subtitle_path = infer_subtitle_from_video(&video_path);
    let media_streaming_server =
        MediaStreamingServer::new(&video_path, &inferred_subtitle_path, &host_ip)?;
    play(render, media_streaming_server).await
}
