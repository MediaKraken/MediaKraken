use futures_util::StreamExt;
use ssdp_client::{SearchResponse, SearchTarget, search};
use std::time::Duration;

pub async fn mk_lib_hardware_ssdp_search(
    target: &str,
) -> Result<Vec<SearchResponse>, Box<dyn std::error::Error>> {
    let search_target = match target.parse::<SearchTarget>() {
        Ok(st) => st,
        Err(_) => return Ok(Vec::new()),
    };

    let stream = search(&search_target, Duration::from_secs(5), 2, None).await?;
    Ok(stream
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect())
}

pub fn mk_lib_hardware_ssdp_filter_locations(
    responses: &[SearchResponse],
    keywords: &[&str],
) -> Vec<String> {
    if keywords.is_empty() {
        return responses.iter().map(|r| r.location().to_string()).collect();
    }
    responses
        .iter()
        .filter_map(|resp| {
            let haystack = format!("{} {} {}", resp.server(), resp.usn(), resp.location())
                .to_ascii_lowercase();
            if keywords
                .iter()
                .any(|kw| haystack.contains(&kw.to_ascii_lowercase()))
            {
                Some(resp.location().to_string())
            } else {
                None
            }
        })
        .collect()
}
