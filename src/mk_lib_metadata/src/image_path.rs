use rand::Rng;
use rand::RngExt;

pub async fn meta_image_file_path(
    media_type: String,
) -> Result<String, Box<dyn std::error::Error>> {
    // This is the SAVE path.  Do NOT shorten the path to static.
    // This is the SAVE path.  Do NOT shorten the path to static.
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const STRING_LEN: usize = 2;
    let mut rng = rand::rng();
    let file_path_random: String = (0..STRING_LEN)
        .map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char)
        .collect();
    let file_path_random_two: String = (0..STRING_LEN)
        .map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char)
        .collect();
    let file_path: String = format!(
        "/mediakraken/metadata/{}/{}/{}",
        &media_type, &file_path_random, &file_path_random_two
    );
    // This is the SAVE path.  Do NOT shorten the path to static.
    // This is the SAVE path.  Do NOT shorten the path to static.
    Ok(file_path)
}
