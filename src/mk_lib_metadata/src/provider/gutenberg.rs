// https://www.gutenberg.org/

// feed of new books
// http://www.gutenberg.org/cache/epub/feeds/today.rss

use mk_lib_network;

pub async fn provider_gutenberg_metadata_download() -> Result<(), Box<dyn std::error::Error>> {
    let _result = mk_lib_network::mk_lib_network::mk_download_file_from_url(
        "https://www.gutenberg.org/cache/epub/feeds/pg_catalog.csv.gz".to_string(),
        &"/mediakraken/cache/pg_catalog.csv.gz".to_string(),
    ).await;
    let _result = mk_lib_network::mk_lib_network::mk_download_file_from_url(
        "https://www.gutenberg.org/cache/epub/feeds/rdf-files.tar.bz2".to_string(),
        &"/mediakraken/cache/rdf-files.tar.bz2".to_string(),
    ).await;
    Ok(())
}
