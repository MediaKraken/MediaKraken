#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/polyfloyd/rust-id3
// id3 = "1.2.0"

use id3::{Tag, TagLike};
use serde_json::json;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn mk_lib_metadata_id3_get_tag_info(file_name: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let tag = Tag::read_from_path(file_name)?;
    if let Some(artist) = tag.artist() {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "artist": artist }))
                .await
                .unwrap();
        }
    }
    if let Some(title) = tag.title() {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "title": title }))
                .await
                .unwrap();
        }
    }
    if let Some(album) = tag.album() {
        #[cfg(debug_assertions)]
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "album": album }))
            .await
            .unwrap();
    }
}
