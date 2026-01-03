use mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;
use std::error::Error;
use torrent_name_parser::Metadata;

#[path = "provider/imdb.rs"]
mod provider_imdb;

#[path = "provider/omdb.rs"]
mod provider_omdb;

#[path = "provider/tmdb.rs"]
mod provider_tmdb;

pub struct MetadataPersonLastLookup {
    metadata_last_id: uuid::Uuid,
    metadata_last_imdb: String,
    metadata_last_tmdb: String,
}

pub async fn metadata_person_lookup(
    _sqlx_pool: &sqlx::PgPool,
    _download_data: &DBDownloadQueueByProviderList,
    _file_name: Metadata,
) -> Result<uuid::Uuid, Box<dyn Error>> {
    // don't bother checking title/year as the main_server_metadata_api_worker does it already
    let metadata_uuid = uuid::Uuid::nil(); // so not found checks verify later
    Ok(metadata_uuid)
}
