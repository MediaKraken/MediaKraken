// Port of the Python nfo/xml handling (the original Python source is kept at the
// bottom of this file).
//
// NFO files are XML. `nfo_xml_parse` builds a `serde_json::Value` tree with the
// same shape as Python's `xmltodict.parse`:
// - an element            -> object keyed by its child element names
// - repeated children     -> array
// - plain text            -> string (formatting whitespace trimmed)
// - an empty element      -> null (xmltodict returns None)
// - attributes            -> keys prefixed with "@"
// - attributes and text   -> object with a "#text" entry holding the text
//
// One intentional deviation: a document with several top-level elements (for
// example `multi-episode.nfo`) is not well-formed XML, so `xmltodict` rejects
// it. Here the top-level elements are grouped by name instead, so such files
// remain usable.

use quick_xml::Reader;
use quick_xml::XmlVersion;
use quick_xml::events::Event;
use serde_json::{Map, Value};
use std::error::Error;
use std::path::Path;

/// NFO file names to look for next to a TV media file, in order.
const TV_NFO_FILE_NAMES: [&str; 2] = ["tvinfo.nfo", "tvshow.nfo"];

/// A node of the NFO document while it is being built.
struct NfoNode {
    name: String,
    attrs: Vec<(String, String)>,
    children: Vec<(String, Value)>,
    text: String,
}

impl NfoNode {
    /// Converts the node into a `Value` using the xmltodict-like mapping.
    fn into_value(self) -> Value {
        let text = self.text.trim().to_string();
        let has_attrs = !self.attrs.is_empty();
        let has_children = !self.children.is_empty();

        // element without children: text, attributes or empty
        if !has_children {
            if !has_attrs {
                return if text.is_empty() {
                    Value::Null
                } else {
                    Value::String(text)
                };
            }

            let mut object = Map::new();
            for (key, value) in self.attrs {
                object.insert(format!("@{key}"), Value::String(value));
            }
            if !text.is_empty() {
                object.insert("#text".to_string(), Value::String(text));
            }
            return Value::Object(object);
        }

        // element with children: grouped children plus attributes and text
        let mut object = group_by_name(self.children);
        for (key, value) in self.attrs {
            object.insert(format!("@{key}"), Value::String(value));
        }
        if !text.is_empty() {
            object.insert("#text".to_string(), Value::String(text));
        }
        Value::Object(object)
    }
}

/// Groups `(name, value)` pairs by name, keeping first-seen order.
/// A single entry stays as-is, repeated names become an array.
fn group_by_name(children: Vec<(String, Value)>) -> Map<String, Value> {
    let mut object = Map::new();
    for (name, value) in children {
        match object.get_mut(&name) {
            Some(existing) => {
                if let Value::Array(items) = existing {
                    items.push(value);
                } else {
                    let previous = std::mem::replace(existing, Value::Null);
                    *existing = Value::Array(vec![previous, value]);
                }
            }
            None => {
                object.insert(name, value);
            }
        }
    }
    object
}

/// Parses an NFO (XML) document into a `Value` tree.
///
/// The result is keyed by the top-level element name(s), like
/// `xmltodict.parse` does in Python: `<movie>...</movie>` becomes
/// `{"movie": {...}}`.
pub fn nfo_xml_parse(xml: &str) -> Result<Value, Box<dyn Error>> {
    let mut reader = Reader::from_str(xml);
    let mut buf: Vec<u8> = Vec::new();
    let mut stack: Vec<NfoNode> = Vec::new();
    let mut roots: Vec<(String, Value)> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(start)) => {
                let name = std::str::from_utf8(start.local_name().as_ref())
                    .map(str::to_string)
                    .map_err(|_| "invalid utf-8 in nfo element name")?;
                let mut attrs = Vec::new();
                for attr in start.attributes() {
                    let attr = attr?;
                    let key = std::str::from_utf8(attr.key.local_name().as_ref())
                        .map_err(|_| "invalid utf-8 in nfo attribute name")?
                        .to_string();
                    let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();
                    attrs.push((key, value));
                }
                stack.push(NfoNode {
                    name,
                    attrs,
                    children: Vec::new(),
                    text: String::new(),
                });
            }
            Ok(Event::End(_)) => {
                let mut node = stack.pop().ok_or("unexpected end tag in nfo")?;
                let name = std::mem::take(&mut node.name);
                let value = node.into_value();
                if let Some(parent) = stack.last_mut() {
                    parent.children.push((name, value));
                } else {
                    roots.push((name, value));
                }
            }
            Ok(Event::Text(text)) => {
                if let Some(parent) = stack.last_mut() {
                    let decoded = text.decode()?;
                    let unescaped = quick_xml::escape::unescape(&decoded)?;
                    parent.text.push_str(&unescaped);
                }
            }
            Ok(Event::CData(cdata)) => {
                if let Some(parent) = stack.last_mut() {
                    parent.text.push_str(&cdata.decode()?);
                }
            }
            // entity references: quick-xml emits every `&name;` / `&#nn;`
            // reference as its own event, so expand it here
            Ok(Event::GeneralRef(reference)) => {
                if let Some(parent) = stack.last_mut() {
                    let name = reference.decode()?;
                    let entity = format!("&{name};");
                    let expanded = quick_xml::escape::unescape(&entity)?.into_owned();
                    parent.text.push_str(&expanded);
                }
            }
            Ok(Event::Eof) => break,
            // comments, processing instructions, xml declaration
            Ok(_) => {}
            Err(err) => return Err(err.into()),
        }
        buf.clear();
    }

    if roots.is_empty() {
        return Err("nfo document contains no elements".into());
    }
    Ok(Value::Object(group_by_name(roots)))
}

/// Loads and parses an NFO file. Returns `None` if the file cannot be read or
/// parsed (like the try/except in the Python original).
fn nfo_file_load(nfo_file: &Path) -> Option<Value> {
    let Ok(contents) = std::fs::read_to_string(nfo_file) else {
        return None;
    };
    nfo_xml_parse(&contents).ok()
}

/// Find and load nfo and xml file(s) if they exist
///
/// Checks the directory of the media file for `tvinfo.nfo` first, then
/// `tvshow.nfo`. If the first one exists it is used even when it is not valid
/// XML (returning `None`), mirroring the Python behavior.
pub async fn nfo_file_tv(media_file_path: &str) -> Option<Value> {
    let media_dir = Path::new(media_file_path).parent()?;

    let nfo_file_check = media_dir.join(TV_NFO_FILE_NAMES[0]); // tvinfo.nfo
    if nfo_file_check.is_file() {
        return nfo_file_load(&nfo_file_check);
    }

    let nfo_file_check = media_dir.join(TV_NFO_FILE_NAMES[1]); // tvshow.nfo
    if nfo_file_check.is_file() {
        return nfo_file_load(&nfo_file_check);
    }

    None
}

/// Lookup by id's in nfo/xml files
///
/// Returns `(imdb_id, tmdb_id)`. Both the legacy direct `<imdbid>` /
/// `<tmdbid>` elements and the modern Kodi `<uniqueid type="...">` elements
/// (as used in the files under `docs/data_example/nfo_templates`) are
/// supported. Ids found in `nfo_data` take priority over `xml_data`.
pub async fn nfo_xml_id_lookup(
    nfo_data: Option<&Value>,
    xml_data: Option<&Value>,
) -> (Option<String>, Option<String>) {
    let mut imdb_id = None;
    let mut tmdb_id = None;
    // load both fields for more data in media_id_json on db
    if let Some(nfo_data) = nfo_data
        && let Some(movie_node) = first_named_node(nfo_data, "movie")
    {
        // not all will have imdb
        imdb_id = find_provider_id(movie_node, "imdbid", "imdb");
        // not all nfo's have the movie/tmdb
        tmdb_id = find_provider_id(movie_node, "tmdbid", "tmdb");
    }
    if let Some(xml_data) = xml_data {
        if let Some(movie_node) = first_named_node(xml_data, "movie") {
            // standard nfo/xml file
            if imdb_id.is_none() {
                // not all xmls's will have the imdb
                imdb_id = find_provider_id(movie_node, "imdbid", "imdb");
            }
            if tmdb_id.is_none() {
                // not all xml's have the movie/tmdb
                tmdb_id = find_provider_id(movie_node, "tmdbid", "tmdb");
            }
        } else if let Some(title_node) = first_named_node(xml_data, "Title") {
            // movie.xml
            if imdb_id.is_none() {
                // not all xmls's will have the imdb
                imdb_id = child_text(title_node, "IMDB");
            }
            if tmdb_id.is_none() {
                // not all xml's have the movie/tmdb
                tmdb_id = child_text(title_node, "TMDbId");
            }
        }
    }
    (imdb_id, tmdb_id)
}

/// Look up id's in nfo/xml lookup for tv
///
/// Returns `(imdb_id, tmdb_id)` from the `<episodedetails>` element. When the
/// nfo has no `<episodedetails>` (show-level `tvshow.nfo` files) the
/// `<tvshow>` element is used instead.
pub async fn nfo_id_lookup_tv(nfo_data: Option<&Value>) -> (Option<String>, Option<String>) {
    let mut imdb_id = None;
    let mut tmdb_id = None;
    // load both fields for more data in media_id_json on db
    if let Some(nfo_data) = nfo_data {
        let tv_node = first_named_node(nfo_data, "episodedetails")
            .or_else(|| first_named_node(nfo_data, "tvshow"));
        if let Some(tv_node) = tv_node {
            tmdb_id = find_provider_id(tv_node, "tmdbid", "tmdb");
            imdb_id = find_provider_id(tv_node, "imdbid", "imdb");
        }
    }
    (imdb_id, tmdb_id)
}

/// Returns the first top-level element named `name` inside `value`,
/// unwrapping the first entry when it is stored as an array
/// (repeated top-level elements, e.g. multi-episode files).
fn first_named_node<'v>(value: &'v Value, name: &str) -> Option<&'v Value> {
    match value.get(name) {
        Some(Value::Array(items)) => items.first(),
        Some(other) => Some(other),
        None => None,
    }
}

/// Returns the text of a direct child element named `key` of `value`,
/// or `None` when it is missing or empty.
fn child_text(value: &Value, key: &str) -> Option<String> {
    first_named_node(value, key).and_then(node_text)
}

/// Returns the text content of a node, or `None` when missing or empty.
///
/// Handles both plain text nodes (`Value::String`) and nodes with attributes,
/// where the text lives under the xmltodict-style `"#text"` key.
fn node_text(value: &Value) -> Option<String> {
    let text = match value {
        Value::String(text) => text.as_str(),
        Value::Object(object) => object.get("#text").and_then(Value::as_str)?,
        _ => return None,
    };
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

/// Finds a provider id inside an element node.
///
/// Checks the legacy direct element named `legacy_tag` (e.g. `imdbid`) first,
/// then the modern Kodi `<uniqueid type="...">` elements for `uniqueid_type`
/// (e.g. `imdb`). Empty ids count as missing, like in the Python original.
fn find_provider_id(node: &Value, legacy_tag: &str, uniqueid_type: &str) -> Option<String> {
    if let Some(id) = child_text(node, legacy_tag) {
        return Some(id);
    }

    let uniqueid_value = node.get("uniqueid")?;
    let uniqueids: Vec<&Value> = match uniqueid_value {
        Value::Array(items) => items.iter().collect(),
        other => vec![other],
    };
    for uniqueid in uniqueids {
        let type_matches = uniqueid
            .get("@type")
            .and_then(Value::as_str)
            .is_some_and(|this_type| this_type.eq_ignore_ascii_case(uniqueid_type));
        if type_matches && let Some(id) = node_text(uniqueid) {
            return Some(id);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn parse(xml: &str) -> Value {
        nfo_xml_parse(xml).expect("expected the xml to parse")
    }

    fn template_path(file_name: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/data_example/nfo_templates")
            .join(file_name)
    }

    #[test]
    fn test_parse_text_only_element() {
        let value = parse("<movie><title>The King's Man</title></movie>");
        assert_eq!(value, json!({"movie": {"title": "The King's Man"}}));
    }

    #[test]
    fn test_parse_empty_element_is_null() {
        let value = parse("<movie><outline></outline></movie>");
        assert_eq!(value["movie"]["outline"], Value::Null);
    }

    #[test]
    fn test_parse_whitespace_only_element_is_null() {
        let value = parse("<movie><outline>\n    \n</outline></movie>");
        assert_eq!(value["movie"]["outline"], Value::Null);
    }

    #[test]
    fn test_parse_attributes_and_text() {
        let value = parse(
            "<movie><thumb aspect=\"poster\" preview=\"\">https://image.tmdb.org/t/p/original/a.jpg</thumb></movie>",
        );
        assert_eq!(
            value["movie"]["thumb"],
            json!({
                "@aspect": "poster",
                "@preview": "",
                "#text": "https://image.tmdb.org/t/p/original/a.jpg"
            })
        );
    }

    #[test]
    fn test_parse_repeated_children_become_array() {
        let value = parse(
            "<album><theme>Hanging Out</theme><theme>Partying</theme><theme>TGIF</theme></album>",
        );
        assert_eq!(
            value["album"]["theme"],
            json!(["Hanging Out", "Partying", "TGIF"])
        );
    }

    #[test]
    fn test_parse_preserves_tag_case() {
        let value = parse("<artist><musicBrainzArtistID>64b94289</musicBrainzArtistID></artist>");
        assert_eq!(value["artist"]["musicBrainzArtistID"], json!("64b94289"));
    }

    #[test]
    fn test_parse_unescapes_entities() {
        let value = parse("<album><title>Greatest Hits, Volume I &amp; Volume II</title></album>");
        assert_eq!(
            value["album"]["title"],
            json!("Greatest Hits, Volume I & Volume II")
        );
    }

    #[test]
    fn test_parse_multiple_top_level_elements() {
        let value = parse(
            "<episodedetails><title>One</title></episodedetails>\
             <episodedetails><title>Two</title></episodedetails>",
        );
        assert_eq!(
            value["episodedetails"],
            json!([{"title": "One"}, {"title": "Two"}])
        );
    }

    #[test]
    fn test_parse_malformed_xml_errors() {
        assert!(nfo_xml_parse("<movie><title>Foo</movie>").is_err());
    }

    #[test]
    fn test_parse_empty_document_errors() {
        assert!(nfo_xml_parse("").is_err());
    }

    #[tokio::test]
    async fn test_xml_id_lookup_legacy_movie_nfo() {
        let nfo = parse(
            "<movie><title>Foo</title><imdbid>tt0111111</imdbid><tmdbid>603</tmdbid></movie>",
        );
        let (imdb_id, tmdb_id) = nfo_xml_id_lookup(Some(&nfo), None).await;
        assert_eq!(imdb_id.as_deref(), Some("tt0111111"));
        assert_eq!(tmdb_id.as_deref(), Some("603"));
    }

    #[tokio::test]
    async fn test_xml_id_lookup_empty_ids_ignored() {
        let nfo = parse("<movie><title>Foo</title><imdbid></imdbid><tmdbid>   </tmdbid></movie>");
        let (imdb_id, tmdb_id) = nfo_xml_id_lookup(Some(&nfo), None).await;
        assert_eq!(imdb_id, None);
        assert_eq!(tmdb_id, None);
    }

    #[tokio::test]
    async fn test_xml_id_lookup_uniqueid_format() {
        let nfo = parse(
            "<movie><title>Foo</title>\
             <uniqueid type=\"imdb\">tt6856242</uniqueid>\
             <uniqueid type=\"tmdb\" default=\"true\">476669</uniqueid></movie>",
        );
        let (imdb_id, tmdb_id) = nfo_xml_id_lookup(Some(&nfo), None).await;
        assert_eq!(imdb_id.as_deref(), Some("tt6856242"));
        assert_eq!(tmdb_id.as_deref(), Some("476669"));
    }

    #[tokio::test]
    async fn test_xml_id_lookup_nfo_takes_priority() {
        let nfo = parse("<movie><imdbid>tt0000001</imdbid></movie>");
        let xml = parse(
            "<movie><uniqueid type=\"imdb\">tt9999999</uniqueid>\
             <uniqueid type=\"tmdb\">1</uniqueid></movie>",
        );
        let (imdb_id, tmdb_id) = nfo_xml_id_lookup(Some(&nfo), Some(&xml)).await;
        assert_eq!(imdb_id.as_deref(), Some("tt0000001"));
        assert_eq!(tmdb_id.as_deref(), Some("1"));
    }

    #[tokio::test]
    async fn test_xml_id_lookup_title_format() {
        // movie.xml: the top-level element is Title, not movie
        let xml =
            parse("<Title><Name>Foo</Name><IMDB>tt1234567</IMDB><TMDbId>54321</TMDbId></Title>");
        let (imdb_id, tmdb_id) = nfo_xml_id_lookup(None, Some(&xml)).await;
        assert_eq!(imdb_id.as_deref(), Some("tt1234567"));
        assert_eq!(tmdb_id.as_deref(), Some("54321"));
    }

    #[tokio::test]
    async fn test_xml_id_lookup_none_inputs() {
        let (imdb_id, tmdb_id) = nfo_xml_id_lookup(None, None).await;
        assert_eq!(imdb_id, None);
        assert_eq!(tmdb_id, None);
    }

    #[tokio::test]
    async fn test_tv_id_lookup_uniqueid_format() {
        let nfo = parse(
            "<episodedetails><title>Foo</title>\
             <uniqueid type=\"imdb\">tt9601584</uniqueid>\
             <uniqueid type=\"tmdb\" default=\"true\">1830976</uniqueid>\
             <uniqueid type=\"tvdb\">8042515</uniqueid></episodedetails>",
        );
        let (imdb_id, tmdb_id) = nfo_id_lookup_tv(Some(&nfo)).await;
        assert_eq!(imdb_id.as_deref(), Some("tt9601584"));
        assert_eq!(tmdb_id.as_deref(), Some("1830976"));
    }

    #[tokio::test]
    async fn test_tv_id_lookup_missing_ids() {
        let nfo = parse("<episodedetails><title>Foo</title></episodedetails>");
        let (imdb_id, tmdb_id) = nfo_id_lookup_tv(Some(&nfo)).await;
        assert_eq!(imdb_id, None);
        assert_eq!(tmdb_id, None);
    }

    #[tokio::test]
    async fn test_movie_template_ids() {
        let nfo = parse(&std::fs::read_to_string(template_path("movie.nfo")).unwrap());
        let (imdb_id, tmdb_id) = nfo_xml_id_lookup(Some(&nfo), None).await;
        assert_eq!(imdb_id.as_deref(), Some("tt6856242"));
        assert_eq!(tmdb_id.as_deref(), Some("476669"));
    }

    #[tokio::test]
    async fn test_episode_template_ids() {
        let nfo = parse(&std::fs::read_to_string(template_path("episode.nfo")).unwrap());
        let (imdb_id, tmdb_id) = nfo_id_lookup_tv(Some(&nfo)).await;
        assert_eq!(imdb_id.as_deref(), Some("tt9601584"));
        assert_eq!(tmdb_id.as_deref(), Some("1830976"));
    }

    #[tokio::test]
    async fn test_tvshow_template_ids() {
        let nfo = parse(&std::fs::read_to_string(template_path("tvshow.nfo")).unwrap());
        let (imdb_id, tmdb_id) = nfo_id_lookup_tv(Some(&nfo)).await;
        assert_eq!(imdb_id.as_deref(), Some("tt5171438"));
        assert_eq!(tmdb_id.as_deref(), Some("67198"));
    }

    #[tokio::test]
    async fn test_multi_episode_template_ids() {
        let nfo = parse(&std::fs::read_to_string(template_path("multi-episode.nfo")).unwrap());
        // the first episode is looked up from the grouped top-level elements
        let (imdb_id, tmdb_id) = nfo_id_lookup_tv(Some(&nfo)).await;
        assert_eq!(imdb_id.as_deref(), Some("tt6599052"));
        assert_eq!(tmdb_id.as_deref(), Some("1906721"));
    }

    #[test]
    fn test_music_templates_parse() {
        for file_name in ["album.nfo", "artist.nfo", "music_video.nfo"] {
            let contents = std::fs::read_to_string(template_path(file_name)).unwrap();
            assert!(nfo_xml_parse(&contents).is_ok(), "{file_name} should parse");
        }
    }

    #[test]
    fn test_music_templates_have_no_imdb_or_tmdb_ids() {
        for file_name in ["album.nfo", "artist.nfo", "music_video.nfo"] {
            let contents = std::fs::read_to_string(template_path(file_name)).unwrap();
            let nfo = nfo_xml_parse(&contents).unwrap();
            let (imdb_id, tmdb_id) = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(nfo_xml_id_lookup(Some(&nfo), None));
            assert_eq!(imdb_id, None, "{file_name} should have no imdb id");
            assert_eq!(tmdb_id, None, "{file_name} should have no tmdb id");
        }
    }

    #[tokio::test]
    async fn test_nfo_file_tv_finds_tvinfo() {
        let dir =
            std::env::temp_dir().join(format!("mk_nfo_xml_test_tvinfo_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("tvinfo.nfo"),
            "<tvshow><uniqueid type=\"imdb\">tt5171438</uniqueid></tvshow>",
        )
        .unwrap();

        let nfo_data = nfo_file_tv(dir.join("S01E01.mkv").to_str().unwrap()).await;
        let (imdb_id, _tmdb_id) = nfo_id_lookup_tv(nfo_data.as_ref()).await;
        assert_eq!(imdb_id.as_deref(), Some("tt5171438"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn test_nfo_file_tv_falls_back_to_tvshow() {
        let dir =
            std::env::temp_dir().join(format!("mk_nfo_xml_test_tvshow_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("tvshow.nfo"),
            "<tvshow><uniqueid type=\"tmdb\">67198</uniqueid></tvshow>",
        )
        .unwrap();

        let nfo_data = nfo_file_tv(dir.join("show.mkv").to_str().unwrap()).await;
        let (_imdb_id, tmdb_id) = nfo_id_lookup_tv(nfo_data.as_ref()).await;
        assert_eq!(tmdb_id.as_deref(), Some("67198"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn test_nfo_file_tv_missing_returns_none() {
        let dir =
            std::env::temp_dir().join(format!("mk_nfo_xml_test_empty_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();

        let nfo_data = nfo_file_tv(dir.join("show.mkv").to_str().unwrap()).await;
        assert_eq!(nfo_data, None);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
