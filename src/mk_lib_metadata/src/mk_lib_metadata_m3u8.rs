// https://github.com/sile/hls_m3u8
// hls_m3u8 = 0.4.1
use hls_m3u8::MediaPlaylist;

pub fn mk_lib_metadata_m3u8_validate_playlist(playlist: &str) {
    let valid_playlist = playlist.parse::<MediaPlaylist>().is_ok();
    valid_playlist
}
