#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rand::{thread_rng, Rng};

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn meta_image_file_path(
    media_type: String,
) -> Result<String, Box<dyn std::error::Error>> {
    // This is the SAVE path.  Do NOT shorten the path to static.
    // This is the SAVE path.  Do NOT shorten the path to static.
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const STRING_LEN: usize = 2;
    let mut rng = rand::thread_rng();
    let file_path_random: String = (0..STRING_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    let file_path_random_two: String = (0..STRING_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    let file_path = format!(
        "/mediakraken/static/meta/{}/{}/{}",
        &media_type, &file_path_random, &file_path_random_two
    );
    // This is the SAVE path.  Do NOT shorten the path to static.
    // This is the SAVE path.  Do NOT shorten the path to static.
    Ok(file_path)
}
