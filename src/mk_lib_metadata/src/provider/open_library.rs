// https://openlibrary.org/

// https://openlibrary.org/developers/api

// https://openlibrary.org/data/ol_cdump_latest.txt.gz
// use above to preload book metadata

// covers?
// https://archive.org/details/amazon_2007_covers

// https://github.com/bspradling/open-library

use mk_lib_network;
use std::error::Error;

pub async fn provider_openlibrary_cover_fetch(
    isbn: String,
    file_path: String,
) -> Result<(), Box<dyn Error>> {
    let _fetch_result = mk_lib_network::mk_lib_network::mk_download_file_from_url(
        format!("http://covers.openlibrary.org/b/isbn/{}-L.jpg", isbn).to_string(),
        &file_path,
    )
    .await
    .unwrap();
    Ok(())
}
