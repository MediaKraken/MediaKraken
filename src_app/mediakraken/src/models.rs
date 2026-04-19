use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LibrarySummary {
    pub server_name: String,
    pub version: String,
    pub movie_count: u64,
    pub show_count: u64,
    pub music_album_count: u64,
}
