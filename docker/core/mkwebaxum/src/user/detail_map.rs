//! Maps the raw JSON blobs stored in the `mm_metadata_*` tables into the
//! `{"mm_metadata_json": {...}}` contract expected by the detail templates.
//!
//! Every function is a pure value transform: (stored blob, indexed columns,
//! local-image path) -> contract value. The handlers are responsible for the
//! queries and for the not-found / error responses.
//!
//! Stored blob shapes (as produced by the fetcher writers in `mk_lib_metadata`):
//! - book: OpenLibrary work JSON (`title`, `author_name`, `first_publish_date`, ...)
//! - game / game_system: MAME XML -> JSON (`machine` object: `name`, `description`,
//!   `year`, `manufacturer`)
//! - album: MusicBrainz release JSON (`title`, `artist-credit-phrase`, `date-year`,
//!   `label`, `format`, `mediums` -> `tracks`)
//! - person: TMDB person JSON (`name`, `biography`, `birthday`, `deathday`,
//!   `place_of_birth`)
//! - tv / collection: TMDB JSON (`name`, `overview`, `first_air_date`, `genres`)
//! - sports: TheSportsDB event JSON (`strEvent`, `strSport`, `dateEvent`, ...)
//!
//! Local image blobs are read with `->>'Poster'` (TMDB-style top-level key,
//! movie/tv/person/collection writers) with a fallback to the nested
//! `Images` object form. Stored paths are relative to the metadata image
//! root and are served under the `/metadata` route.

use serde_json::{Value, json};

/// String value at `key`, or `Value::Null` when absent or not a string.
fn str_at(blob: &Value, key: &str) -> Value {
    blob.get(key)
        .and_then(Value::as_str)
        .map(Value::from)
        .unwrap_or(Value::Null)
}

/// First string value found across `keys`, or `Value::Null`.
fn str_at_first(blob: &Value, keys: &[&str]) -> Value {
    for key in keys {
        if let Some(value) = blob.get(*key).and_then(Value::as_str) {
            return Value::from(value);
        }
    }
    Value::Null
}

/// String value at a nested path (e.g. `["machine", "year"]`), or `Value::Null`.
fn nested_str(blob: &Value, path: &[&str]) -> Value {
    let mut current = Some(blob);
    for key in path {
        current = current.and_then(|value| value.get(*key));
    }
    current
        .and_then(Value::as_str)
        .map(Value::from)
        .unwrap_or(Value::Null)
}

/// First non-null of `a` / `b`.
fn or_null(a: Value, b: Value) -> Value {
    if a.is_null() { b } else { a }
}

/// First 4 characters of the first date string found across `keys`
/// (`"1994-05-16"` -> `"1994"`), or `Value::Null`.
fn year_from_date(blob: &Value, keys: &[&str]) -> Value {
    for key in keys {
        if let Some(value) = blob.get(*key).and_then(Value::as_str)
            && value.len() >= 4
        {
            return Value::from(&value[..4]);
        }
    }
    Value::Null
}

/// `genres` array as a list of names. Accepts `["Drama"]` or `[{"name": "Drama"}]`.
fn genres_of(blob: &Value) -> Value {
    let names = blob
        .get("genres")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    item.as_str()
                        .or_else(|| item.get("name").and_then(Value::as_str))
                })
                .map(Value::from)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Value::from(names)
}

/// Number at `key` rendered as a string (scores, player counts), or `Value::Null`.
fn number_as_string(blob: &Value, key: &str) -> Value {
    match blob.get(key) {
        Some(Value::String(s)) => Value::from(s.clone()),
        Some(Value::Number(n)) => Value::from(n.to_string()),
        _ => Value::Null,
    }
}

/// Stored local-image path turned into a directly usable `/metadata` URL,
/// or `Value::Null` when empty. Absolute URLs pass through unchanged.
fn cover_url(path: &str) -> Value {
    if path.is_empty() {
        return Value::Null;
    }
    if path.starts_with("http://") || path.starts_with("https://") {
        return Value::from(path);
    }
    Value::from(format!("/metadata{path}"))
}

/// `mm_metadata_album_localimage` -> cover path. Legacy code reads the nested
/// `{"Images": {"musicbrainz": "<path>"}}` form; the flat `{"Poster": ...}`
/// form is accepted as a fallback.
pub fn album_cover_path(localimage: &Value) -> &str {
    localimage
        .get("Images")
        .and_then(|images| images.get("musicbrainz"))
        .and_then(Value::as_str)
        .or_else(|| localimage.get("Poster").and_then(Value::as_str))
        .unwrap_or_default()
}

/// Book (OpenLibrary) blob -> contract.
pub fn book(blob: &Value, cover_path: &str) -> Value {
    let author = match blob.get("author_name") {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(", "),
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    };
    let publisher = blob
        .get("publishers")
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .map(|first| {
            first
                .as_str()
                .or_else(|| first.get("name").and_then(Value::as_str))
                .unwrap_or_default()
        })
        .unwrap_or_default();
    let isbn = blob
        .get("isbn13")
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .and_then(Value::as_str)
        .or_else(|| {
            blob.get("isbn10")
                .and_then(Value::as_array)
                .and_then(|items| items.first())
                .and_then(Value::as_str)
        })
        .unwrap_or_default();
    json!({
        "mm_metadata_json": {
            "name": str_at(blob, "title"),
            "author": Value::from(author),
            "overview": str_at(blob, "description"),
            "year": year_from_date(blob, &["first_publish_date"]),
            "genre": Value::Array(Vec::new()),
            "publisher": Value::from(publisher),
            "isbn": Value::from(isbn),
            "cover": cover_url(cover_path),
        }
    })
}

/// Game (MAME machine XML) blob -> contract. `name` / `platform` come from
/// indexed columns / the system join; the blob supplies the rest.
pub fn game(blob: &Value, name: &str, platform: &str, cover_path: &str) -> Value {
    json!({
        "mm_metadata_json": {
            "name": if name.is_empty() {
                nested_str(blob, &["machine", "name"])
            } else {
                Value::from(name)
            },
            "overview": or_null(
                nested_str(blob, &["machine", "description"]),
                str_at(blob, "description"),
            ),
            "year": or_null(nested_str(blob, &["machine", "year"]), str_at(blob, "year")),
            "genre": Value::Array(Vec::new()),
            "developer": or_null(
                nested_str(blob, &["machine", "manufacturer"]),
                str_at(blob, "manufacturer"),
            ),
            "publisher": Value::Null,
            "platform": if platform.is_empty() {
                Value::Null
            } else {
                Value::from(platform)
            },
            "cover": cover_url(cover_path),
        }
    })
}

/// Game system (MAME system XML) blob -> contract.
pub fn game_system(blob: &Value) -> Value {
    json!({
        "mm_metadata_json": {
            "name": str_at(blob, "name"),
            "overview": str_at(blob, "description"),
            "year": str_at(blob, "year"),
            "developer": str_at(blob, "developer"),
            "manufacturer": str_at(blob, "manufacturer"),
            "cover": Value::Null,
        }
    })
}

/// Track list flattened from the MusicBrainz `mediums` -> `tracks` structure.
fn album_tracks(blob: &Value) -> Value {
    let mut tracks: Vec<Value> = Vec::new();
    if let Some(mediums) = blob.get("mediums").and_then(Value::as_array) {
        for medium in mediums {
            if let Some(items) = medium.get("tracks").and_then(Value::as_array) {
                for track in items {
                    let length = track
                        .get("length")
                        .and_then(Value::as_u64)
                        .or_else(|| {
                            track
                                .get("length")
                                .and_then(Value::as_str)
                                .and_then(|s| s.parse().ok())
                        })
                        .unwrap_or(0);
                    tracks.push(json!({
                        "name": str_at(track, "recording-name"),
                        "artist": str_at(track, "artist-credit-phrase"),
                        "duration": format!("{}:{}", length / 60, format!("{:02}", length % 60)),
                    }));
                }
            }
        }
    }
    Value::from(tracks)
}

/// Label / format value: accepts `{"name": "..."}` objects as well as plain strings.
fn object_name(blob: &Value, key: &str) -> Value {
    blob.get(key)
        .and_then(|value| {
            value
                .get("name")
                .and_then(Value::as_str)
                .or_else(|| value.as_str())
        })
        .map(Value::from)
        .unwrap_or(Value::Null)
}

/// Album (MusicBrainz release) blob -> contract.
pub fn album(blob: &Value, name: &str, cover_path: &str) -> Value {
    let artist = match blob.get("artist-credit-phrase").and_then(Value::as_str) {
        Some(phrase) => phrase.to_string(),
        None => blob
            .get("artist-credit")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.get("name").and_then(Value::as_str))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default(),
    };
    json!({
        "mm_metadata_json": {
            "name": if name.is_empty() {
                str_at(blob, "title")
            } else {
                Value::from(name)
            },
            "artist": Value::from(artist),
            "overview": Value::Null,
            "year": year_from_date(blob, &["date-year", "date"]),
            "label": object_name(blob, "label"),
            "format": object_name(blob, "format"),
            "genre": Value::Array(Vec::new()),
            "cover": cover_url(cover_path),
        },
        "mm_metadata_tracks": album_tracks(blob),
    })
}

/// Music video blob -> contract. `band` / `song` come from indexed columns.
pub fn music_video(blob: &Value, band: &str, song: &str, cover_path: &str) -> Value {
    json!({
        "mm_metadata_json": {
            "name": if song.is_empty() {
                str_at(blob, "title")
            } else {
                Value::from(song)
            },
            "artist": if band.is_empty() {
                str_at(blob, "artist")
            } else {
                Value::from(band)
            },
            "overview": str_at(blob, "description"),
            "year": year_from_date(blob, &["year"]),
            "genre": Value::Array(Vec::new()),
            "director": str_at(blob, "director"),
            "cover": cover_url(cover_path),
        }
    })
}

/// Person (TMDB) blob -> contract.
pub fn person(blob: &Value, name: &str, photo_path: &str) -> Value {
    json!({
        "mm_metadata_json": {
            "name": if name.is_empty() {
                str_at(blob, "name")
            } else {
                Value::from(name)
            },
            "overview": str_at(blob, "biography"),
            "born": str_at(blob, "birthday"),
            "died": str_at(blob, "deathday"),
            "nationality": str_at(blob, "place_of_birth"),
            "photo": cover_url(photo_path),
        }
    })
}

/// TV show (TMDB) blob -> contract. No season table exists, so the season
/// list is always empty.
pub fn tv(blob: &Value, name: &str, cover_path: &str) -> Value {
    json!({
        "mm_metadata_json": {
            "name": if name.is_empty() {
                str_at(blob, "name")
            } else {
                Value::from(name)
            },
            "overview": str_at(blob, "overview"),
            "year": year_from_date(blob, &["first_air_date"]),
            "genre": genres_of(blob),
            "poster": cover_url(cover_path),
        },
        "mm_metadata_seasons": Value::Array(Vec::new()),
    })
}

/// Sports event (TheSportsDB) blob -> contract. Both the raw provider keys
/// (`strEvent`, ...) and friendly keys (`title`, ...) are accepted.
pub fn sports(blob: &Value, cover_path: &str) -> Value {
    json!({
        "mm_metadata_json": {
            "title": str_at_first(blob, &["title", "strEvent"]),
            "sport": str_at_first(blob, &["sport", "strSport"]),
            "league": str_at_first(blob, &["league", "strLeague"]),
            "season": str_at_first(blob, &["season", "strSeason"]),
            "release_date": str_at_first(blob, &["release_date", "dateEvent"]),
            "venue": str_at_first(blob, &["venue", "strVenue"]),
            "team_home": str_at_first(blob, &["team_home", "strHomeTeam"]),
            "team_away": str_at_first(blob, &["team_away", "strAwayTeam"]),
            "home_score": number_as_string(blob, "intHomeScore"),
            "away_score": number_as_string(blob, "intAwayScore"),
            "poster": cover_url(cover_path),
            "overview": str_at_first(blob, &["overview", "strDescriptionEN"]),
        }
    })
}

/// Collection (TMDB) blob -> contract.
pub fn collection(blob: &Value, cover_path: &str) -> Value {
    json!({
        "mm_metadata_json": {
            "name": str_at(blob, "name"),
            "overview": str_at(blob, "overview"),
            "year": Value::Null,
            "genre": genres_of(blob),
            "cover": cover_url(cover_path),
        }
    })
}

/// Game server blob -> contract. `name` comes from the indexed column.
pub fn game_server(blob: &Value, name: &str) -> Value {
    let cover_path = blob
        .get("Poster")
        .and_then(Value::as_str)
        .or_else(|| {
            blob.get("Images")
                .and_then(|images| images.get("Poster"))
                .and_then(Value::as_str)
        })
        .unwrap_or_default();
    json!({
        "mm_metadata_json": {
            "name": if name.is_empty() {
                str_at(blob, "name")
            } else {
                Value::from(name)
            },
            "overview": str_at_first(blob, &["overview", "description"]),
            "game": str_at_first(blob, &["game", "game_name"]),
            "address": str_at_first(blob, &["address", "host"]),
            "version": str_at(blob, "version"),
            "max_players": number_as_string(blob, "max_players"),
            "cover": cover_url(cover_path),
        }
    })
}
