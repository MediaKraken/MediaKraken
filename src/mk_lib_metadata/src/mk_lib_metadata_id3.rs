// https://github.com/polyfloyd/rust-id3

use id3::{Tag, TagLike};

pub async fn mk_lib_metadata_id3_get_tag_info(file_name: String) {
    let Ok(tag) = Tag::read_from_path(file_name) else {
        return;
    };
    if let Some(_artist) = tag.artist() {
        #[cfg(debug_assertions)]
        {
            // mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "artist": artist }))
            //     .await
            //     .unwrap();
        }
    }
    if let Some(_title) = tag.title() {
        #[cfg(debug_assertions)]
        {
            // mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "title": title }))
            //     .await
            //     .unwrap();
        }
    }
    if let Some(_album) = tag.album() {
        #[cfg(debug_assertions)]
        {
            // mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "album": album }))
            //     .await
            //     .unwrap();
        }
    }
}
