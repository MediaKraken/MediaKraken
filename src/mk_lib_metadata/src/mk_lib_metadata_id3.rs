// https://github.com/polyfloyd/rust-id3

use id3::{Tag, TagLike};
use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;

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
    let tag = Tag::read_from_path(file_name).unwrap();
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
