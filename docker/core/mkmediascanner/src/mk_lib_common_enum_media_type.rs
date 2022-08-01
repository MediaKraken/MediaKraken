#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![allow(unused)]
#[non_exhaustive]
pub struct DLMediaType;

impl DLMediaType {
    pub const MOVIE: i8 = 1;
    pub const TV: i8 = 2;
    pub const PERSON: i8 = 3;
    pub const SPORTS: i8 = 4;
    pub const GAME: i8 = 5;
    pub const PUBLICATION: i8 = 6;
    pub const PICTURE: i8 = 7;
    pub const ANIME: i8 = 8;
    pub const MUSIC: i8 = 9;
    pub const ADULT: i8 = 10;

    pub const ADULT_IMAGE: i8 = 1000;
    pub const ADULT_MOVIE: i8 = 1001;
    pub const ADULT_SCENE: i8 = 1002;

    pub const GAME_CHD: i8 = 501;
    pub const GAME_CINEMATICS: i8 = 502;
    pub const GAME_INTRO: i8 = 503;
    pub const GAME_ISO: i8 = 504;
    pub const GAME_ROM: i8 = 505;
    pub const GAME_SPEEDRUN: i8 = 506;
    pub const GAME_SUPERPLAY: i8 = 507;

    pub const MOVIE_HOME: i8 = 111;
    pub const MOVIE_EXTRAS: i8 = 112;
    pub const MOVIE_SOUNDTRACK: i8 = 113;
    pub const MOVIE_SUBTITLE: i8 = 114;
    pub const MOVIE_THEME: i8 = 115;
    pub const MOVIE_TRAILER: i8 = 116;

    pub const MUSIC_ALBUM: i8 = 901;
    pub const MUSIC_LYRICS: i8 = 902;
    pub const MUSIC_SONG: i8 = 903;
    pub const MUSIC_VIDEO: i8 = 904;

    pub const PUBLICATION_BOOK: i8 = 601;
    pub const PUBLICATION_COMIC: i8 = 602;
    pub const PUBLICATION_COMIC_STRIP: i8 = 603;
    pub const PUBLICATION_MAGAZINE: i8 = 604;
    pub const PUBLICATION_GRAPHIC_NOVEL: i8 = 605;

    pub const TV_EPISODE: i8 = 201;
    pub const TV_EXTRAS: i8 = 202;
    pub const TV_SEASON: i8 = 203;
    pub const TV_SUBTITLE: i8 = 204;
    pub const TV_THEME: i8 = 205;
    pub const TV_TRAILER: i8 = 206;
}
