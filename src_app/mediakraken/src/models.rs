use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LibrarySummary {
    pub server_name: String,
    pub version: String,
    pub movie_count: u64,
    pub show_count: u64,
    pub music_album_count: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UpcLookupResult {
    pub upc: String,
    pub owned: bool,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub media_type: Option<String>,
    #[serde(default)]
    pub year: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HardwareDevice {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub powered: bool,
    #[serde(default)]
    pub volume: Option<u8>,
    #[serde(default)]
    pub muted: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HardwareDeviceList {
    pub devices: Vec<HardwareDevice>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaKind {
    Movie,
    Show,
    Audio,
}

impl MediaKind {
    pub fn endpoint(self) -> &'static str {
        match self {
            MediaKind::Movie => "movies",
            MediaKind::Show => "shows",
            MediaKind::Audio => "audio",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            MediaKind::Movie => "Movies",
            MediaKind::Show => "TV Shows",
            MediaKind::Audio => "Audio",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MediaItem {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub year: Option<u32>,
    #[serde(default)]
    pub media_type: Option<String>,
    #[serde(default)]
    pub poster_url: Option<String>,
    #[serde(default)]
    pub overview: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MediaList {
    pub items: Vec<MediaItem>,
}
