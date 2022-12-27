#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/polyfloyd/rust-id3
// id3 = "1.2.0"

use id3::{Tag, TagLike};

pub async fn mk_lib_metadata_id3_get_tag_info(file_name: String) {
    et tag = Tag::read_from_path(file_name)?;
    if let Some(artist) = tag.artist() {
        #[cfg(debug_assertions)]
        {
        println!("artist: {}", artist);}
    }
    if let Some(title) = tag.title() {
        #[cfg(debug_assertions)]
        {
        println!("title: {}", title);}
    }
    if let Some(album) = tag.album() {
        #[cfg(debug_assertions)]
        {println!("album: {}", album);}
    }
}