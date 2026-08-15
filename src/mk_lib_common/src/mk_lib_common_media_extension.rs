pub static MEDIA_EXTENSION: [&str; 31] = [
    "webm", "mkv", "flv", "vob", "ogv", "ogg", "drc", "mng", "avi", "mov", "qt", "wmv", "wma",
    "yuv", "rm", "rmvb", "asf", "mp4", "m4p", "m4v", "mpg", "mp2", "mpeg", "mpe", "mp3", "flac",
    "mpv", "m2v", "nsv", "pdf", "lrc",
];

pub static SUBTITLE_EXTENSION: [&str; 7] = ["srt", "smi", "ssa", "ass", "vtt", "sub", "idx"];

pub static MEDIA_EXTENSION_SKIP_FFMPEG: [&str; 6] = ["pdf", "zip", "7z", "iso", "chd", "lrc"];

pub static GAME_EXTENSION: [&str; 5] = ["iso", "chd", "zip", "7z", "rar"];

pub static COMIC_BOOK_EXTENSION: [&str; 4] = ["cbr", "cbz", "cbt", "pdf"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_extension_count() {
        assert_eq!(MEDIA_EXTENSION.len(), 31);
    }

    #[test]
    fn test_media_extension_contains_video() {
        assert!(MEDIA_EXTENSION.contains(&"mp4"));
        assert!(MEDIA_EXTENSION.contains(&"mkv"));
        assert!(MEDIA_EXTENSION.contains(&"avi"));
        assert!(MEDIA_EXTENSION.contains(&"webm"));
    }

    #[test]
    fn test_media_extension_contains_audio() {
        assert!(MEDIA_EXTENSION.contains(&"mp3"));
        assert!(MEDIA_EXTENSION.contains(&"flac"));
        assert!(MEDIA_EXTENSION.contains(&"ogg"));
    }

    #[test]
    fn test_subtitle_extension_count() {
        assert_eq!(SUBTITLE_EXTENSION.len(), 7);
    }

    #[test]
    fn test_subtitle_extension_values() {
        assert!(SUBTITLE_EXTENSION.contains(&"srt"));
        assert!(SUBTITLE_EXTENSION.contains(&"vtt"));
        assert!(SUBTITLE_EXTENSION.contains(&"ass"));
    }

    #[test]
    fn test_media_extension_skip_ffmpeg_count() {
        assert_eq!(MEDIA_EXTENSION_SKIP_FFMPEG.len(), 6);
    }

    #[test]
    fn test_media_extension_skip_ffmpeg_values() {
        assert!(MEDIA_EXTENSION_SKIP_FFMPEG.contains(&"pdf"));
        assert!(MEDIA_EXTENSION_SKIP_FFMPEG.contains(&"zip"));
        assert!(MEDIA_EXTENSION_SKIP_FFMPEG.contains(&"iso"));
    }

    #[test]
    fn test_game_extension_count() {
        assert_eq!(GAME_EXTENSION.len(), 5);
    }

    #[test]
    fn test_game_extension_values() {
        assert!(GAME_EXTENSION.contains(&"iso"));
        assert!(GAME_EXTENSION.contains(&"chd"));
        assert!(GAME_EXTENSION.contains(&"zip"));
    }

    #[test]
    fn test_comic_book_extension_count() {
        assert_eq!(COMIC_BOOK_EXTENSION.len(), 4);
    }

    #[test]
    fn test_comic_book_extension_values() {
        assert!(COMIC_BOOK_EXTENSION.contains(&"cbr"));
        assert!(COMIC_BOOK_EXTENSION.contains(&"cbz"));
        assert!(COMIC_BOOK_EXTENSION.contains(&"pdf"));
    }

    #[test]
    fn test_pdf_in_both_media_and_comic() {
        assert!(MEDIA_EXTENSION.contains(&"pdf"));
        assert!(COMIC_BOOK_EXTENSION.contains(&"pdf"));
    }

    #[test]
    fn test_no_duplicate_extensions_in_media() {
        let mut sorted = MEDIA_EXTENSION.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), MEDIA_EXTENSION.len());
    }
}
