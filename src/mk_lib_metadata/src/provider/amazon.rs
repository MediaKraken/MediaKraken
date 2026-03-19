use reqwest::Url;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::collections::HashSet;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    let search_terms = build_amazon_upc_search_terms(upc_code);
    if search_terms.is_empty() {
        return Ok(Vec::new());
    }

    let client = reqwest::Client::new();
    let mut all_results = Vec::new();
    let mut seen_results = HashSet::new();

    for search_term in search_terms {
        let mut url = Url::parse(AMAZON_SEARCH_URL)?;
        url.query_pairs_mut().append_pair("k", &search_term);

        let html = client
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

        for result in parse_amazon_search_results(&html) {
            if seen_results.insert(result.clone()) {
                all_results.push(result);
            }
        }

        if !all_results.is_empty() {
            break;
        }
    }

    Ok(all_results)
}

fn build_amazon_upc_search_terms(upc_code: &str) -> Vec<String> {
    let trimmed_upc = upc_code.trim();
    if trimmed_upc.is_empty() {
        return Vec::new();
    }

    let digits_only: String = trimmed_upc
        .chars()
        .filter(|char| char.is_ascii_digit())
        .collect();

    let mut search_terms = Vec::new();
    let mut seen_terms = HashSet::new();

    for candidate in [
        Some(trimmed_upc),
        (!digits_only.is_empty()).then_some(digits_only.as_str()),
    ]
    .into_iter()
    .flatten()
    .flat_map(upc_search_variants)
    {
        if seen_terms.insert(candidate.clone()) {
            search_terms.push(candidate);
        }
    }

    search_terms
}

fn upc_search_variants(upc_code: &str) -> Vec<String> {
    let mut variants = vec![upc_code.to_string()];

    if upc_code.len() == 13 && upc_code.starts_with('0') {
        variants.push(upc_code[1..].to_string());
    }

    if upc_code.len() == 14 && upc_code.starts_with("00") {
        variants.push(upc_code[2..].to_string());
    }

    variants
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
    fn test_build_amazon_upc_search_terms_trims_and_falls_back_to_upc_a() {
        assert_eq!(
            build_amazon_upc_search_terms(" 0191329268810 "),
            vec!["0191329268810".to_string(), "191329268810".to_string()]
        );
    }

    #[test]
    fn test_build_amazon_upc_search_terms_deduplicates_digits_only_input() {
        assert_eq!(
            build_amazon_upc_search_terms("191329268810"),
            vec!["191329268810".to_string()]
        );
    }

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
