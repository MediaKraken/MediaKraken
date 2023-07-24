// https://github.com/sile/hls_m3u8

use hls_m3u8::MediaPlaylist;
use std::error::Error;

const M3U_HEADER: &str = "EXTM3U\n";
const M3U_LINE_HEADER: &str = "EXTINF:";

pub async fn mk_lib_metadata_m3u8_validate_playlist(
    playlist: &str,
) -> Result<MediaPlaylist, Box<dyn Error>> {
    let valid_playlist: MediaPlaylist = playlist.parse::<MediaPlaylist>().unwrap();
    Ok(valid_playlist)
}

/*

'''
#EXTM3U
#EXTINF:111,3rd Bass - Al z A-B-Cee z
mp3/3rd Bass/3rd bass - Al z A-B-Cee z.mp3
'''


def com_m3u_write(playlist_data, m3u_file_name):
    """
    Write out m3u from list
    """
    m3u_data = M3U_HEADER
    for playlist_item_seconds, playlist_item_name, playlist_item_filename in playlist_data:
        m3u_data += M3U_LINE_HEADER + playlist_item_seconds + ',' + playlist_item_name + '\n' \
                    + playlist_item_filename + '\n'
    common_file.com_file_save_data(m3u_file_name, m3u_data, False, False, None)
 */
