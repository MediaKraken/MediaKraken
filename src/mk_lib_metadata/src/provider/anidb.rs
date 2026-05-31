// https://anidb.net/

use mk_lib_network;
use std::error::Error;

pub async fn provider_anidb_fetch_titles_file() -> Result<(), Box<dyn Error>> {
    mk_lib_network::mk_lib_network::mk_download_file_from_url(
        "http://anidb.net/api/anime-titles.xml.gz".to_string(),
        "/mediakraken/cache/anidb_titles.gz",
    )
    .await?;
    Ok(())
}
