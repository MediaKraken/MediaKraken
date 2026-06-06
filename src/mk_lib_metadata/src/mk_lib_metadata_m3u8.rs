// https://github.com/sile/hls_m3u8

use hls_m3u8::MediaPlaylist;
use std::error::Error;

const M3U_HEADER: &str = "EXTM3U\n";
const M3U_LINE_HEADER: &str = "EXTINF:";

pub async fn mk_lib_metadata_m3u8_validate_playlist(
    playlist: &str,
) -> Result<MediaPlaylist<'_>, Box<dyn Error>> {
    Ok(playlist.parse::<MediaPlaylist>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_m3u8_valid_simple_playlist() {
        let playlist = "#EXTM3U\n#EXTINF:111,Track One\ntrack1.mp3\n#EXTINF:222,Track Two\ntrack2.mp3\n";
        let result = mk_lib_metadata_m3u8_validate_playlist(playlist).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_m3u8_empty_string_rejected() {
        let result = mk_lib_metadata_m3u8_validate_playlist("").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_m3u8_missing_header_rejected() {
        let playlist = "track1.mp3\ntrack2.mp3\n";
        let result = mk_lib_metadata_m3u8_validate_playlist(playlist).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_m3u8_single_track() {
        let playlist = "#EXTM3U\n#EXTINF:60,Single Track\nsingle.mp3\n";
        let result = mk_lib_metadata_m3u8_validate_playlist(playlist).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_m3u8_zero_duration() {
        let playlist = "#EXTM3U\n#EXTINF:0,Zero Duration\nzero.mp3\n";
        let result = mk_lib_metadata_m3u8_validate_playlist(playlist).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_m3u_header_constant() {
        assert_eq!(M3U_HEADER, "EXTM3U\n");
    }

    #[test]
    fn test_m3u_line_header_constant() {
        assert_eq!(M3U_LINE_HEADER, "EXTINF:");
    }
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
