use mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;
use parse_torrent_title::ParsedTitle;
use serde_json::json;
use std::error::Error;
use std::sync::Mutex;

use crate::base::metadata_debug_log;
use crate::guessit::parsed_year;
use crate::nfo_xml;

/// Last lookup state for movies.
///
/// The Python original kept these values as class attributes on the
/// `metadata_movie_lookup` function, so they are process-wide here as well:
/// repeated lookups for the same ids within a worker run resolve without
/// re-hitting the database.
pub struct MetadataMovieLastLookup {
    metadata_last_id: uuid::Uuid,
    metadata_last_imdb: Option<String>,
    metadata_last_tmdb: Option<String>,
}

impl MetadataMovieLastLookup {
    const fn new() -> Self {
        Self {
            metadata_last_id: uuid::Uuid::nil(),
            metadata_last_imdb: None,
            metadata_last_tmdb: None,
        }
    }
}

impl Default for MetadataMovieLastLookup {
    fn default() -> Self {
        Self::new()
    }
}

static MOVIE_LAST_LOOKUP: Mutex<MetadataMovieLastLookup> =
    Mutex::new(MetadataMovieLastLookup::new());

/// Locks the last lookup state, recovering from a poisoned mutex so one
/// panicking thread doesn't wedge the lookup for the rest of the process.
/// Callers must not hold the guard across an await.
fn movie_last_lookup() -> std::sync::MutexGuard<'static, MetadataMovieLastLookup> {
    MOVIE_LAST_LOOKUP
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub async fn metadata_movie_lookup(
    sqlx_pool: &sqlx::PgPool,
    download_data: &DBDownloadQueueByProviderList,
    file_name: ParsedTitle,
) -> Result<uuid::Uuid, Box<dyn Error>> {
    // don't bother checking title/year as the main_server_metadata_api_worker does it already
    let mut metadata_uuid = uuid::Uuid::nil(); // so not found checks verify later

    // determine provider id's from nfo/xml if they exist
    let mut imdb_id: Option<String> = None;
    let mut tmdb_id: Option<String> = None;
    if let Some(download_path) = download_data.mm_download_path.as_ref() {
        let (nfo_data, xml_data) = nfo_xml::nfo_xml_file(download_path).await;
        (imdb_id, tmdb_id) = nfo_xml::nfo_xml_id_lookup(nfo_data.as_ref(), xml_data.as_ref()).await;
    }
    metadata_debug_log(json!({
        "Type": "MovieLookupNfo",
        "Module": std::module_path!(),
        "DL Guid": download_data.mm_download_guid,
        "ImdbId": imdb_id,
        "TmdbId": tmdb_id,
    }))
    .await;

    // if same as last, return last id and save lookup. The guard's scope
    // ends before any await: a std MutexGuard is !Send.
    let cached_id = {
        let last = movie_last_lookup();
        let is_last = (imdb_id.is_some()
            && imdb_id.as_deref() == last.metadata_last_imdb.as_deref())
            || (tmdb_id.is_some() && tmdb_id.as_deref() == last.metadata_last_tmdb.as_deref());
        is_last.then(|| last.metadata_last_id)
    };
    if let Some(cached_id) = cached_id {
        // don't need to set last......since they are equal
        metadata_debug_log(json!({
            "Type": "MovieLookupCacheHit",
            "Module": std::module_path!(),
            "DL Guid": download_data.mm_download_guid,
            "MetadataUuid": cached_id,
        }))
        .await;
        return Ok(cached_id);
    }

    // doesn't match last id's so continue lookup
    // if ids from nfo/xml, query local db to see if exist
    // A non-numeric tmdb id is treated as "not found locally".
    if let Some(tmdb_id) = tmdb_id.as_deref()
        && let Ok(tmdb_id_num) = tmdb_id.parse::<i32>()
        && let Some(guid) =
            mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_guid_by_tmdb(
                sqlx_pool,
                tmdb_id_num,
            )
            .await?
    {
        metadata_uuid = guid;
        metadata_debug_log(json!({
            "Type": "MovieLookupDbTmdbId",
            "Module": std::module_path!(),
            "DL Guid": download_data.mm_download_guid,
            "TmdbId": tmdb_id_num,
            "MetadataUuid": guid,
        }))
        .await;
    }

    /*
    // not ported: the "Fetch" update below also stores the provider id in
    // mm_download_provider_id and no mk_lib_database function updates that
    // column on an existing queue row yet. Setting the status to "Fetch"
    // without it would make the Fetch worker fail on a missing provider_id,
    // so the whole branch stays off until the db helper exists.
    if metadata_uuid == None {
        // check to see if id is known from nfo/xml but not in db yet so fetch data
        if tmdb_id != None || imdb_id != None {
            if tmdb_id != None {
                provider_id = str(tmdb_id);
            } else {
                provider_id = imdb_id;
            }
            dl_meta = db_connection.db_download_que_exists(dl_row.get("mdq_id"),
                                                           common_global.DLMediaType.Movie.value,
                                                           "themoviedb",
                                                           provider_id);
            if dl_meta == None {
                let metadata_uuid = dl_row.get("mdq_new_uuid");
                db_connection.db_begin();
                db_connection.db_download_update(guid = dl_row.get("mdq_id"),
                                                 status = "Fetch",
                                                 provider_guid = provider_id);
                // set provider last so it's not picked up by the wrong thread too early
                db_connection.db_download_update_provider("themoviedb",
                                                          dl_row.get("mdq_id"));
                db_connection.db_commit();
            } else {
                let metadata_uuid = dl_meta;
            }
        }
    }
    */

    // leave this AFTER the dl check as it looks at tmdbid and imdb for values!
    if metadata_uuid == uuid::Uuid::nil() {
        // no ids found on the local database so begin name/year searches
        // db lookup by name and year (if available)
        metadata_uuid =
            mk_lib_database::database_metadata::mk_lib_database_metadata::mk_lib_database_metadata_find_guid(
                sqlx_pool,
                file_name.title(),
                parsed_year(&file_name),
            )
            .await?
            .unwrap_or(uuid::Uuid::nil());
        if metadata_uuid != uuid::Uuid::nil() {
            metadata_debug_log(json!({
                "Type": "MovieLookupDbNameYear",
                "Module": std::module_path!(),
                "DL Guid": download_data.mm_download_guid,
                "Title": file_name.title(),
                "Year": parsed_year(&file_name),
                "MetadataUuid": metadata_uuid,
            }))
            .await;
        }
    }

    if metadata_uuid == uuid::Uuid::nil() {
        metadata_uuid = download_data.mm_download_new_uuid;
        metadata_debug_log(json!({
            "Type": "MovieLookupNoLocalMatch",
            "Module": std::module_path!(),
            "DL Guid": download_data.mm_download_guid,
            "Title": file_name.title(),
            "Year": parsed_year(&file_name),
            "ImdbId": imdb_id,
            "TmdbId": tmdb_id,
            "Note": "row re-pointed at themoviedb for the remote search",
        }))
        .await;
        // no matches by name/year on local database
        // search themoviedb since not matched above via DB or nfo/xml
        // save the updated status
        mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_status_update(
            sqlx_pool,
            download_data.mm_download_guid,
            "Search".to_string(),
        )
        .await?;
        // set provider last so it's not picked up by the wrong thread
        mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_update_provider(
            sqlx_pool,
            "themoviedb".to_string(),
            download_data.mm_download_guid,
        )
        .await?;
    }

    // set last values to negate lookups for same title/show. A miss
    // must not be cached: on a miss metadata_uuid is this row's
    // mm_download_new_uuid placeholder, and caching it would make a
    // later file with the same NFO id resolve to a media guid that may
    // never become a metadata record.
    if metadata_uuid != uuid::Uuid::nil() && metadata_uuid != download_data.mm_download_new_uuid {
        let mut last = movie_last_lookup();
        last.metadata_last_id = metadata_uuid;
        last.metadata_last_imdb = imdb_id;
        last.metadata_last_tmdb = tmdb_id;
    }

    Ok(metadata_uuid)
}
