use reqwest::Url;
use select::document::Document;
use select::predicate::Class;

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
        .append_pair("_nkw", upc_code.trim())
        .append_pair("_sop", "12");

    let html = reqwest::Client::new()
        .get(search_url)
        .header(reqwest::header::USER_AGENT, 
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let mut results = Vec::new();
    let document = Document::from(html.as_str());

    for node in document.find(Class("s-item__title")) {
        let title = node.text().trim().to_string();
        if title.is_empty() || title == "Shop on eBay" {
            continue;
        }

        let (year, media_format) = extract_year_and_media_format(&title);
        results.push(EbayMediaResult {
            title,
            year,
            media_format,
        });
    }

    Ok(results)
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
    use super::{detect_media_format, extract_year, extract_year_and_media_format};

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
}
