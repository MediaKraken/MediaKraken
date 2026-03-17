use reqwest::Url;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

const AMAZON_SEARCH_URL: &str = "https://www.amazon.com/s";
const MEDIA_FORMAT_KEYWORDS: [&str; 12] = [
    "4k",
    "blu-ray",
    "dvd",
    "vhs",
    "prime video",
    "audio cd",
    "mp3 cd",
    "vinyl",
    "kindle",
    "hardcover",
    "paperback",
    "digital",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazonUpcResult {
    pub title: String,
    pub year: Option<u16>,
    pub media_format: Option<String>,
}

pub async fn provider_amazon_search_by_upc(
    upc_code: &str,
) -> Result<Option<AmazonUpcResult>, Box<dyn std::error::Error>> {
    let results = provider_amazon_search_results_by_upc(upc_code).await?;
    Ok(results.into_iter().next())
}

pub async fn provider_amazon_search_results_by_upc(
    upc_code: &str,
) -> Result<Vec<AmazonUpcResult>, Box<dyn std::error::Error>> {
    let trimmed_upc = upc_code.trim();
    if trimmed_upc.is_empty() {
        return Ok(Vec::new());
    }

    let mut url = Url::parse(AMAZON_SEARCH_URL)?;
    url.query_pairs_mut().append_pair("k", trimmed_upc);

    let html = reqwest::Client::new()
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
        )
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(parse_amazon_search_results(&html))
}

fn parse_amazon_search_results(html: &str) -> Vec<AmazonUpcResult> {
    let document = Document::from(html);

    document
        .find(Name("div").and(Attr("data-component-type", "s-search-result")))
        .filter_map(|search_result_node| {
            let title = search_result_node
                .find(Name("h2"))
                .next()
                .map(|title_node| title_node.text().trim().to_string())
                .filter(|value| !value.is_empty())?;

            let result_text = search_result_node.text();
            let year = extract_year(&result_text);
            let media_format = extract_media_format(&result_text);

            Some(AmazonUpcResult {
                title,
                year,
                media_format,
            })
        })
        .collect()
}

fn extract_year(input: &str) -> Option<u16> {
    input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|part| part.len() == 4)
        .filter_map(|part| part.parse::<u16>().ok())
        .find(|value| (1900..=2100).contains(value))
}

fn extract_media_format(input: &str) -> Option<String> {
    let lowercase = input.to_ascii_lowercase();

    MEDIA_FORMAT_KEYWORDS.iter().find_map(|keyword| {
        if lowercase.contains(keyword) {
            Some(format_keyword(keyword))
        } else {
            None
        }
    })
}

fn format_keyword(keyword: &str) -> String {
    keyword
        .split(' ')
        .map(|word| {
            if word.eq_ignore_ascii_case("mp3") || word.eq_ignore_ascii_case("4k") {
                word.to_ascii_uppercase()
            } else {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                    None => String::new(),
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_amazon_result_text() {
        let html = r#"
        <div data-component-type="s-search-result">
            <h2>The Matrix [Blu-ray] (1999)</h2>
        </div>
        "#;

        let parsed = parse_amazon_search_results(html);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].title, "The Matrix [Blu-ray] (1999)");
        assert_eq!(parsed[0].year, Some(1999));
        assert_eq!(parsed[0].media_format.as_deref(), Some("Blu-ray"));
    }
}
