pub mod adult;
pub mod anime;
pub mod base;
pub mod book;
pub mod game;
pub mod guessit;
pub mod image_path;
pub mod mk_lib_metadata_id3;
pub mod mk_lib_metadata_m3u8;
pub mod movie;
pub mod music;
pub mod music_video;
pub mod nfo_xml;
pub mod sports;
pub mod tv;

#[path = "provider"]
pub mod metadata_provider {
    pub mod anidb;
    pub mod chart_lyrics;
    pub mod comicvine;
    #[cfg(feature = "discid")]
    pub mod discid;
    pub mod flickr;
    pub mod giant_bomb;
    pub mod gutenberg;
    pub mod imdb;
    pub mod imvdb;
    pub mod isbndb;
    pub mod lastfm;
    pub mod mangadex;
    pub mod musicbrainz;
    pub mod myanimelist;
    pub mod omdb;
    pub mod open_library;
    pub mod opensubtitles;
    pub mod pitchfork;
    pub mod pornhub;
    pub mod shoutcast;
    pub mod shutterstock;
    pub mod soundcloud;
    pub mod televisiontunes;
    pub mod theaudiodb;
    pub mod thegamesdb;
    pub mod thesportsdb;
    pub mod tmdb;
    pub mod twitch;
    pub mod vimeo;
    pub mod youtube;
}