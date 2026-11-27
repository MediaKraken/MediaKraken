// https://upcdiscdb.com/

use select::document::Document;
use select::predicate::{Attr, Class};
use serde_json::{Value, json};

const UPCDISCDB_SEARCH_URL: &str = "https://upcdiscdb.com/search";
const UPCDISCDB_BASE_URL: &str = "https://upcdiscdb.com";
const UPCDISCDB_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

/// Submit a UPC/EAN to UPCDiscDB and screen-scrape the resulting release page.
///
/// The search endpoint redirects (search -> /barcode/<upc> -> /releases/<slug>)
/// when the barcode is known; unknown barcodes land on a 404 "Barcode not found"
/// page, in which case `{"found": false, ...}` is returned.
pub async fn provider_upcdiscdb_lookup_by_upc(
    _sqlx_pool: &sqlx::PgPool,
    upc_code: &str,
    _api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let upc = upc_code.trim();
    if upc.is_empty() {
        return Err("upcdiscdb: UPC code is empty".into());
    }

    let mut url = reqwest::Url::parse(UPCDISCDB_SEARCH_URL)?;
    url.query_pairs_mut().append_pair("q", upc);

    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, UPCDISCDB_USER_AGENT)
        .send()
        .await?;
    let status = response.status();
    let html = response.text().await?;

    if status.as_u16() == 404 || !html.contains("release__title") {
        return Ok(json!({
            "provider": "upcdiscdb",
            "found": false,
            "upc": upc,
        }));
    }

    parse_upcdiscdb_release_page(upc, &html)
}

fn parse_upcdiscdb_release_page(
    upc: &str,
    html: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let doc = Document::from(html);

    // Primary data source: the JSON-LD (schema.org Product) block in the head.
    let product = find_json_ld_product(&doc);

    let release_format = product
        .as_ref()
        .and_then(|p| p.get("category"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| first_text(&doc, "release__title-detail"));

    let mut release = json!({
        "name": product.as_ref().and_then(|p| p.get("name")).and_then(Value::as_str),
        "title": first_text(&doc, "release__title-name"),
        "scope": first_text(&doc, "release__scope"),
        "format": release_format,
        "genres": doc
            .find(Class("release__eyebrow-item--genre"))
            .map(|el| el.text().trim().to_string())
            .filter(|text| !text.is_empty())
            .collect::<Vec<_>>(),
        "release_date": product
            .as_ref()
            .and_then(|p| p.get("releaseDate"))
            .and_then(Value::as_str),
        "description": product
            .as_ref()
            .and_then(|p| p.get("description"))
            .and_then(Value::as_str),
        "image": product
            .as_ref()
            .and_then(|p| p.get("image"))
            .and_then(Value::as_str),
        "url": product.as_ref().and_then(|p| p.get("url")).and_then(Value::as_str),
        "release_card_number": first_text(&doc, "rental-card__number")
            .map(|number| number.trim_start_matches("No.").trim().to_string()),
        "series": series_from_subject_of(product.as_ref()),
    });

    // "Release card" table (Format, Contents, Region, First released, IMDb, ...)
    let release_card: Vec<Value> = doc
        .find(Class("rental-card__row"))
        .filter_map(|row| {
            let key = row
                .find(Class("rental-card__key"))
                .next()?
                .text()
                .trim()
                .to_string();
            let value = row
                .find(Class("rental-card__value"))
                .next()?
                .text()
                .trim()
                .to_string();
            (!key.is_empty()).then(|| json!({ "key": key, "value": value }))
        })
        .collect();
    release["release_card"] = Value::Array(release_card);

    // All barcodes printed on the release (a release can carry several).
    let barcodes: Vec<Value> = doc
        .find(Class("barcode-label"))
        .map(|label| {
            let label_text = |class: &str| -> Option<String> {
                label
                    .find(Class(class))
                    .next()
                    .map(|el| el.text().trim().to_string())
                    .filter(|text| !text.is_empty())
            };
            json!({
                "type": label_text("barcode-label__type"),
                "number": label_text("barcode-label__number"),
                "street_date": label_text("barcode-label__date")
                    .map(|date| date.trim_start_matches("Street date").trim().to_string()),
                "is_scanned_upc": label
                    .find(Class("barcode-label__number"))
                    .next()
                    .map(|el| el.text().trim() == upc)
                    .unwrap_or(false),
            })
        })
        .collect();

    Ok(json!({
        "provider": "upcdiscdb",
        "found": true,
        "upc": upc,
        "release": release,
        "barcodes": barcodes,
    }))
}

/// Extract the schema.org Product node from the page's application/ld+json.
fn find_json_ld_product(doc: &Document) -> Option<Value> {
    doc.find(Attr("type", "application/ld+json"))
        .map(|el| el.text())
        .find_map(|raw| serde_json::from_str::<Value>(raw.trim()).ok())
        .and_then(|value| {
            value
                .get("@graph")?
                .as_array()?
                .iter()
                .find(|node| node.get("@type").and_then(|t| t.as_str()) == Some("Product"))
                .cloned()
        })
}

/// Turn the Product `subjectOf` (Movie or TVSeason) into a series object with
/// explicit imdb_id / tmdb_id fields.
fn series_from_subject_of(product: Option<&Value>) -> Value {
    let Some(subject_of) = product.and_then(|p| p.get("subjectOf")) else {
        return Value::Null;
    };

    let mut imdb_id: Option<String> = None;
    let mut tmdb_id: Option<String> = None;
    find_external_ids(subject_of, &mut imdb_id, &mut tmdb_id);

    let mut series = subject_of.clone();
    if let Some(obj) = series.as_object_mut() {
        if let Some(id) = imdb_id {
            obj.insert("imdb_id".to_string(), Value::String(id));
        }
        if let Some(id) = tmdb_id {
            obj.insert("tmdb_id".to_string(), Value::String(id));
        }
    }
    series
}

/// Recursively walk a JSON-LD node for `sameAs` URLs, pulling out the
/// IMDb and TMDB identifiers.
fn find_external_ids(node: &Value, imdb_id: &mut Option<String>, tmdb_id: &mut Option<String>) {
    if let Some(same_as) = node.get("sameAs").and_then(|v| v.as_array()) {
        for url_value in same_as.iter().filter_map(|v| v.as_str()) {
            if imdb_id.is_none() {
                *imdb_id = parse_imdb_id(url_value);
            }
            if tmdb_id.is_none() {
                *tmdb_id = parse_tmdb_id(url_value);
            }
        }
    }
    if let Some(obj) = node.as_object() {
        for value in obj.values() {
            find_external_ids(value, imdb_id, tmdb_id);
        }
    }
}

fn parse_imdb_id(url: &str) -> Option<String> {
    let marker = "imdb.com/title/";
    let start = url.find(marker)? + marker.len();
    let id: String = url[start..]
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .collect();
    (!id.is_empty()).then_some(id)
}

fn parse_tmdb_id(url: &str) -> Option<String> {
    if !url.contains("themoviedb.org") {
        return None;
    }
    url.rsplit('/')
        .find(|segment| !segment.is_empty())
        .map(str::to_string)
}

/// First non-empty trimmed text of an element with the given class, or None.
fn first_text(doc: &Document, class: &str) -> Option<String> {
    doc.find(Class(class))
        .next()
        .map(|el| el.text().trim().to_string())
        .filter(|text| !text.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Minimal replica of the structure of a real UPCDiscDB release page.
    const RELEASE_PAGE_HTML: &str = r#"
        <html>
            <head>
                <script type="application/ld+json">{"@context":"https://schema.org","@graph":[{"@type":"Product","@id":"https://upcdiscdb.com/releases/barry-season-1-dvd#release","name":"Barry Season 1 DVD","url":"https://upcdiscdb.com/releases/barry-season-1-dvd","description":"Barry Season 1 on DVD, Region 1. First released October 2, 2018. Barcode: UPC-A 883929644742.","image":"https://upcdiscdb.com/storage/covers/barry-season-1-dvd-28354-600.webp?v=1791825049","category":"DVD","releaseDate":"2018-10-02","gtin":"883929644742","subjectOf":{"@type":"TVSeason","name":"Barry Season 1","seasonNumber":1,"partOfSeries":{"@type":"TVSeries","name":"Barry","startDate":"2018","sameAs":["https://www.imdb.com/title/tt5348176/","https://www.themoviedb.org/tv/73107"]}}}]}</script>
            </head>
            <body>
                <section class="release">
                    <div class="release__body">
                        <p class="release__eyebrow">
                            <span class="format-badge format-badge--dvd">DVD</span>
                            <span class="release__eyebrow-item release__eyebrow-item--genre">Comedy</span>
                            <span class="release__eyebrow-item release__eyebrow-item--genre">Crime</span>
                        </p>
                        <h1 class="release__title">
                            <span class="release__title-name">Barry</span>
                            <span class="release__scope">Season 1</span>
                            <span class="release__title-detail">DVD</span>
                        </h1>
                        <div class="rental-card">
                            <p class="rental-card__head">
                                <span class="rental-card__heading">Release card</span>
                                <span class="rental-card__number">No. 028354</span>
                            </p>
                            <dl class="rental-card__rows">
                                <div class="rental-card__row">
                                    <dt class="rental-card__key">Format</dt>
                                    <dd class="rental-card__value">DVD</dd>
                                </div>
                                <div class="rental-card__row">
                                    <dt class="rental-card__key">Region</dt>
                                    <dd class="rental-card__value">1</dd>
                                </div>
                                <div class="rental-card__row">
                                    <dt class="rental-card__key">First released</dt>
                                    <dd class="rental-card__value">October 2, 2018</dd>
                                </div>
                                <div class="rental-card__row">
                                    <dt class="rental-card__key">IMDb</dt>
                                    <dd class="rental-card__value">
                                        <a class="rental-card__link" href="https://www.imdb.com/title/tt5348176/">tt5348176</a>
                                    </dd>
                                </div>
                            </dl>
                        </div>
                    </div>
                </section>
                <section class="release-section release-columns__barcodes">
                    <ul class="barcode-labels">
                        <li class="barcode-label barcode-label--scanned">
                            <p class="barcode-label__head">
                                <span class="barcode-label__type">UPC-A</span>
                            </p>
                            <p class="barcode-label__number">883929644742</p>
                            <p class="barcode-label__foot">
                                <span class="barcode-label__date">Street date Oct 2, 2018</span>
                            </p>
                        </li>
                    </ul>
                </section>
            </body>
        </html>
    "#;

    #[test]
    fn parses_release_page_fields() {
        let value = parse_upcdiscdb_release_page("883929644742", RELEASE_PAGE_HTML).unwrap();

        assert_eq!(value["found"], json!(true));
        assert_eq!(value["upc"], json!("883929644742"));
        assert_eq!(value["release"]["name"], json!("Barry Season 1 DVD"));
        assert_eq!(value["release"]["title"], json!("Barry"));
        assert_eq!(value["release"]["scope"], json!("Season 1"));
        assert_eq!(value["release"]["format"], json!("DVD"));
        assert_eq!(value["release"]["release_date"], json!("2018-10-02"));
        assert_eq!(value["release"]["release_card_number"], json!("028354"));
        assert_eq!(value["release"]["genres"][0], json!("Comedy"));
        assert_eq!(value["release"]["series"]["imdb_id"], json!("tt5348176"));
        assert_eq!(value["release"]["series"]["tmdb_id"], json!("73107"));
        assert_eq!(value["release"]["series"]["name"], json!("Barry Season 1"));

        let release_card = value["release"]["release_card"].as_array().unwrap();
        let keys: Vec<&str> = release_card
            .iter()
            .map(|row| row["key"].as_str().unwrap())
            .collect();
        assert!(keys.contains(&"Region"));
        assert!(keys.contains(&"First released"));

        let barcodes = value["barcodes"].as_array().unwrap();
        assert_eq!(barcodes.len(), 1);
        assert_eq!(barcodes[0]["type"], json!("UPC-A"));
        assert_eq!(barcodes[0]["number"], json!("883929644742"));
        assert_eq!(barcodes[0]["street_date"], json!("Oct 2, 2018"));
        assert_eq!(barcodes[0]["is_scanned_upc"], json!(true));
    }

    #[test]
    fn parses_format_from_title_detail_when_no_json_ld() {
        let html = r#"
            <html>
                <body>
                    <h1 class="release__title">
                        <span class="release__title-name">Hoffa</span>
                        <span class="release__title-detail">DVD</span>
                    </h1>
                </body>
            </html>
        "#;
        let value = parse_upcdiscdb_release_page("012345678901", html).unwrap();
        assert_eq!(value["release"]["name"], Value::Null);
        assert_eq!(value["release"]["title"], json!("Hoffa"));
        assert_eq!(value["release"]["format"], json!("DVD"));
        assert_eq!(value["barcodes"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn extracts_external_ids() {
        assert_eq!(
            parse_imdb_id("https://www.imdb.com/title/tt5348176/"),
            Some("tt5348176".to_string())
        );
        assert_eq!(parse_imdb_id("https://example.com/tt123"), None);
        assert_eq!(
            parse_tmdb_id("https://www.themoviedb.org/tv/73107"),
            Some("73107".to_string())
        );
        assert_eq!(
            parse_tmdb_id("https://www.themoviedb.org/movie/603/"),
            Some("603".to_string())
        );
        assert_eq!(parse_tmdb_id("https://www.imdb.com/title/tt123/"), None);
    }
}
