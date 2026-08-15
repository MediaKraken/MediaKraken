#[non_exhaustive]
pub struct DLUPCMediaType;

impl DLUPCMediaType {
    pub const AUDIOCD: i16 = 1;
    pub const GAME: i16 = 2;
    pub const MEDIA: i16 = 3;
    pub const MISC: i16 = 4;
}

pub struct DLMediaType;

impl DLMediaType {
    pub const MOVIE: i16 = 1;
    pub const TV: i16 = 2;
    pub const PERSON: i16 = 3;
    pub const SPORTS: i16 = 4;
    pub const GAME: i16 = 5;
    pub const PUBLICATION: i16 = 6;
    pub const PICTURE: i16 = 7;
    pub const ANIME: i16 = 8;
    pub const MUSIC: i16 = 9;
    pub const ADULT: i16 = 10;
    pub const COLLECTION: i16 = 11;

    pub const ADULT_IMAGE: i16 = 1000;
    pub const ADULT_SCENE: i16 = 1001;

    pub const GAME_CHD: i16 = 501;
    pub const GAME_CINEMATICS: i16 = 502;
    pub const GAME_ISO: i16 = 504;
    pub const GAME_ROM: i16 = 505;
    pub const GAME_SPEEDRUN: i16 = 506;
    pub const GAME_SUPERPLAY: i16 = 507;
    pub const GAME_TRAILER: i16 = 503;

    pub const MOVIE_HOME: i16 = 111;
    pub const MOVIE_EXTRAS: i16 = 112;
    pub const MOVIE_SOUNDTRACK: i16 = 113;
    pub const MOVIE_SUBTITLE: i16 = 114;
    pub const MOVIE_THEME: i16 = 115;
    pub const MOVIE_TRAILER: i16 = 116;

    pub const MUSIC_ALBUM: i16 = 901;
    pub const MUSIC_LYRICS: i16 = 902;
    pub const MUSIC_SONG: i16 = 903;
    pub const MUSIC_VIDEO: i16 = 904;

    pub const PUBLICATION_BOOK: i16 = 601;
    pub const PUBLICATION_COMIC: i16 = 602;
    pub const PUBLICATION_COMIC_STRIP: i16 = 603;
    pub const PUBLICATION_MAGAZINE: i16 = 604;
    pub const PUBLICATION_GRAPHIC_NOVEL: i16 = 605;

    pub const TV_EPISODE: i16 = 201;
    pub const TV_EXTRAS: i16 = 202;
    pub const TV_SEASON: i16 = 203;
    pub const TV_SUBTITLE: i16 = 204;
    pub const TV_THEME: i16 = 205;
    pub const TV_TRAILER: i16 = 206;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dlupc_media_type_values() {
        assert_eq!(DLUPCMediaType::AUDIOCD, 1);
        assert_eq!(DLUPCMediaType::GAME, 2);
        assert_eq!(DLUPCMediaType::MEDIA, 3);
        assert_eq!(DLUPCMediaType::MISC, 4);
    }

    #[test]
    fn test_dl_media_type_core_values() {
        assert_eq!(DLMediaType::MOVIE, 1);
        assert_eq!(DLMediaType::TV, 2);
        assert_eq!(DLMediaType::PERSON, 3);
        assert_eq!(DLMediaType::SPORTS, 4);
        assert_eq!(DLMediaType::GAME, 5);
        assert_eq!(DLMediaType::PUBLICATION, 6);
        assert_eq!(DLMediaType::PICTURE, 7);
        assert_eq!(DLMediaType::ANIME, 8);
        assert_eq!(DLMediaType::MUSIC, 9);
        assert_eq!(DLMediaType::ADULT, 10);
        assert_eq!(DLMediaType::COLLECTION, 11);
    }

    #[test]
    fn test_dl_media_type_adult_subtypes() {
        assert_eq!(DLMediaType::ADULT_IMAGE, 1000);
        assert_eq!(DLMediaType::ADULT_SCENE, 1001);
    }

    #[test]
    fn test_dl_media_type_game_subtypes() {
        assert_eq!(DLMediaType::GAME_CHD, 501);
        assert_eq!(DLMediaType::GAME_CINEMATICS, 502);
        assert_eq!(DLMediaType::GAME_TRAILER, 503);
        assert_eq!(DLMediaType::GAME_ISO, 504);
        assert_eq!(DLMediaType::GAME_ROM, 505);
        assert_eq!(DLMediaType::GAME_SPEEDRUN, 506);
        assert_eq!(DLMediaType::GAME_SUPERPLAY, 507);
    }

    #[test]
    fn test_dl_media_type_movie_subtypes() {
        assert_eq!(DLMediaType::MOVIE_HOME, 111);
        assert_eq!(DLMediaType::MOVIE_EXTRAS, 112);
        assert_eq!(DLMediaType::MOVIE_SOUNDTRACK, 113);
        assert_eq!(DLMediaType::MOVIE_SUBTITLE, 114);
        assert_eq!(DLMediaType::MOVIE_THEME, 115);
        assert_eq!(DLMediaType::MOVIE_TRAILER, 116);
    }

    #[test]
    fn test_dl_media_type_music_subtypes() {
        assert_eq!(DLMediaType::MUSIC_ALBUM, 901);
        assert_eq!(DLMediaType::MUSIC_LYRICS, 902);
        assert_eq!(DLMediaType::MUSIC_SONG, 903);
        assert_eq!(DLMediaType::MUSIC_VIDEO, 904);
    }

    #[test]
    fn test_dl_media_type_publication_subtypes() {
        assert_eq!(DLMediaType::PUBLICATION_BOOK, 601);
        assert_eq!(DLMediaType::PUBLICATION_COMIC, 602);
        assert_eq!(DLMediaType::PUBLICATION_COMIC_STRIP, 603);
        assert_eq!(DLMediaType::PUBLICATION_MAGAZINE, 604);
        assert_eq!(DLMediaType::PUBLICATION_GRAPHIC_NOVEL, 605);
    }

    #[test]
    fn test_dl_media_type_tv_subtypes() {
        assert_eq!(DLMediaType::TV_EPISODE, 201);
        assert_eq!(DLMediaType::TV_EXTRAS, 202);
        assert_eq!(DLMediaType::TV_SEASON, 203);
        assert_eq!(DLMediaType::TV_SUBTITLE, 204);
        assert_eq!(DLMediaType::TV_THEME, 205);
        assert_eq!(DLMediaType::TV_TRAILER, 206);
    }

    #[test]
    fn test_no_overlapping_values() {
        let core_values = [
            DLMediaType::MOVIE,
            DLMediaType::TV,
            DLMediaType::PERSON,
            DLMediaType::SPORTS,
            DLMediaType::GAME,
            DLMediaType::PUBLICATION,
            DLMediaType::PICTURE,
            DLMediaType::ANIME,
            DLMediaType::MUSIC,
            DLMediaType::ADULT,
            DLMediaType::COLLECTION,
        ];
        for i in 0..core_values.len() {
            for j in (i + 1)..core_values.len() {
                assert_ne!(core_values[i], core_values[j]);
            }
        }
    }
}
