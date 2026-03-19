use reqwest::Url;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EbayMediaResult {
    pub title: String,
    pub year: Option<i32>,
    pub media_format: Option<String>,
}

/// Search eBay listings by UPC and return parsed title/year/media-format details.
pub async fn provider_ebay_fetch_by_upc(
    upc_code: &str,
) -> Result<Vec<EbayMediaResult>, Box<dyn std::error::Error>> {
    let mut search_url = Url::parse("https://www.ebay.com/sch/i.html")?;
    search_url
        .query_pairs_mut()
        .append_pair("_nkw", &format!("\"{}\"", upc_code.trim()))
        .append_pair("_sop", "12");

    let html = reqwest::Client::new()
        .get(search_url)
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
        )
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(extract_results_from_html(&html))
}

fn extract_results_from_html(html: &str) -> Vec<EbayMediaResult> {
    let document = Document::from(html);
    let mut titles = BTreeSet::new();

    collect_title_nodes(&document, &mut titles);
    collect_json_ld_titles(&document, &mut titles);

    titles
        .into_iter()
        .map(|title| {
            let (year, media_format) = extract_year_and_media_format(&title);
            EbayMediaResult {
                title,
                year,
                media_format,
            }
        })
        .collect()
}

fn collect_title_nodes(document: &Document, titles: &mut BTreeSet<String>) {
    for node in document.find(Class("s-item__title")) {
        push_title(titles, node.text());
    }

    for node in document.find(Attr("role", "heading")) {
        push_title(titles, node.text());
    }
}

fn collect_json_ld_titles(document: &Document, titles: &mut BTreeSet<String>) {
    for node in document.find(Name("script").and(Attr("type", "application/ld+json"))) {
        let Ok(json) = serde_json::from_str::<Value>(&node.text()) else {
            continue;
        };

        collect_titles_from_json(&json, titles);
    }
}

fn collect_titles_from_json(value: &Value, titles: &mut BTreeSet<String>) {
    match value {
        Value::Array(items) => {
            for item in items {
                collect_titles_from_json(item, titles);
            }
        }
        Value::Object(map) => {
            if map.get("@type").and_then(Value::as_str) == Some("Product")
                && let Some(title) = map.get("name").and_then(Value::as_str)
            {
                push_title(titles, title);
            }

            if let Some(item_list) = map.get("itemListElement").and_then(Value::as_array) {
                for item in item_list {
                    if let Some(title) = item
                        .get("item")
                        .and_then(|item| item.get("name"))
                        .and_then(Value::as_str)
                    {
                        push_title(titles, title);
                    }
                }
            }

            for nested in map.values() {
                collect_titles_from_json(nested, titles);
            }
        }
        _ => {}
    }
}

fn push_title(titles: &mut BTreeSet<String>, title: impl AsRef<str>) {
    let normalized = title
        .as_ref()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if normalized.is_empty() || normalized == "Shop on eBay" {
        return;
    }

    titles.insert(normalized);
}

fn extract_year_and_media_format(title: &str) -> (Option<i32>, Option<String>) {
    let year = extract_year(title);
    let media_format = detect_media_format(title);
    (year, media_format)
}

fn extract_year(title: &str) -> Option<i32> {
    title
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter_map(|token| token.parse::<i32>().ok())
        .find(|value| (1900..=2100).contains(value))
}

fn detect_media_format(title: &str) -> Option<String> {
    let lowered = title.to_ascii_lowercase();

    const MEDIA_FORMAT_KEYWORDS: [(&str, &str); 10] = [
        ("4k ultra hd", "4K Ultra HD"),
        ("blu-ray", "Blu-ray"),
        ("bluray", "Blu-ray"),
        ("dvd", "DVD"),
        ("vhs", "VHS"),
        ("laserdisc", "LaserDisc"),
        ("cd", "CD"),
        ("cassette", "Cassette"),
        ("vinyl", "Vinyl"),
        ("digital", "Digital"),
    ];

    MEDIA_FORMAT_KEYWORDS
        .iter()
        .find_map(|(keyword, label)| lowered.contains(keyword).then(|| (*label).to_string()))
}

#[cfg(test)]
mod tests {
    use super::{
        detect_media_format, extract_results_from_html, extract_year, extract_year_and_media_format,
    };

    #[test]
    fn extracts_year_from_title() {
        assert_eq!(extract_year("The Matrix 1999 Blu-ray"), Some(1999));
        assert_eq!(extract_year("Classic Film DVD"), None);
    }

    #[test]
    fn detects_media_formats() {
        assert_eq!(
            detect_media_format("Movie Blu-ray + DVD Combo"),
            Some("Blu-ray".to_string())
        );
        assert_eq!(
            detect_media_format("Brand New Vinyl LP"),
            Some("Vinyl".to_string())
        );
        assert_eq!(detect_media_format("Book Hardcover"), None);
    }

    #[test]
    fn extracts_both_values() {
        assert_eq!(
            extract_year_and_media_format("Alien 1979 4K Ultra HD"),
            (Some(1979), Some("4K Ultra HD".to_string()))
        );
    }

    #[test]
    fn extracts_titles_from_json_ld_when_dom_titles_are_missing() {
        let html = r#"
            <html>
                <body>
                    <script type="application/ld+json">
                        {
                            "@context": "https://schema.org",
                            "@type": "ItemList",
                            "itemListElement": [
                                {
                                    "@type": "ListItem",
                                    "position": 1,
                                    "item": {
                                        "@type": "Product",
                                        "name": "The Matrix 1999 Blu-ray"
                                    }
                                }
                            ]
                        }
                    </script>
                </body>
            </html>
        "#;

        let results = extract_results_from_html(html);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "The Matrix 1999 Blu-ray");
        assert_eq!(results[0].year, Some(1999));
        assert_eq!(results[0].media_format.as_deref(), Some("Blu-ray"));
    }
}
