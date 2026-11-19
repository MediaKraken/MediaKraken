use mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;
use parse_torrent_title::ParsedTitle;
use serde_json::json;
use std::error::Error;
use std::sync::Mutex;

use crate::nfo_xml;

/// Last lookup state for TV shows.
///
/// The Python original kept these values as class attributes on the
/// `metadata_tv_lookup` function, so they are process-wide here as well:
/// repeated lookups for the same ids within a worker run resolve without
/// re-hitting the database.
pub struct MetadataTVLastLookup {
    metadata_last_id: uuid::Uuid,
    metadata_last_imdb: Option<String>,
    metadata_last_tvdb: Option<String>,
    metadata_last_tmdb: Option<String>,
}

impl MetadataTVLastLookup {
    const fn new() -> Self {
        Self {
            metadata_last_id: uuid::Uuid::nil(),
            metadata_last_imdb: None,
            metadata_last_tvdb: None,
            metadata_last_tmdb: None,
        }
    }
}

impl Default for MetadataTVLastLookup {
    fn default() -> Self {
        Self::new()
    }
}

static TV_LAST_LOOKUP: Mutex<MetadataTVLastLookup> = Mutex::new(MetadataTVLastLookup::new());

/// Locks the last lookup state, recovering from a poisoned mutex so one
/// panicking thread doesn't wedge the lookup for the rest of the process.
/// Callers must not hold the guard across an await.
fn tv_last_lookup() -> std::sync::MutexGuard<'static, MetadataTVLastLookup> {
    TV_LAST_LOOKUP
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub async fn metadata_tv_lookup(
    sqlx_pool: &sqlx::PgPool,
    download_data: &DBDownloadQueueByProviderList,
    file_name: ParsedTitle,
) -> Result<uuid::Uuid, Box<dyn Error>> {
    // don't bother checking title/year as the main_server_metadata_api_worker does it already
    let mut metadata_uuid = uuid::Uuid::nil(); // so not found checks verify later

    // determine provider id's from nfo/xml if they exist
    let mut imdb_id: Option<String> = None;
    let mut tvdb_id: Option<String> = None;
    let mut tmdb_id: Option<String> = None;
    if let Some(download_path) = download_data.mm_download_path.as_ref() {
        let nfo_data = nfo_xml::nfo_file_tv(download_path).await;
        (imdb_id, tvdb_id, tmdb_id) = nfo_xml::nfo_id_lookup_tv_full(nfo_data.as_ref()).await;
    }

    // if same as last, return last id and save lookup
    // check these dupes as the nfo/xml files might not exist to pull the metadata id from
    {
        let last = tv_last_lookup();
        if (imdb_id.is_some() && imdb_id.as_deref() == last.metadata_last_imdb.as_deref())
            || (tvdb_id.is_some() && tvdb_id.as_deref() == last.metadata_last_tvdb.as_deref())
            || (tmdb_id.is_some() && tmdb_id.as_deref() == last.metadata_last_tmdb.as_deref())
        {
            // don't need to set last......since they are equal
            return Ok(last.metadata_last_id);
        }
    }

    // doesn't match last id's so continue lookup
    // if ids from nfo/xml, query local db to see if exist
    /*
    // not ported: no TV id lookups exist in the published mk_lib_database
    // yet. Add these and wire them in here:
    // - tmdb: select mm_metadata_tvshow_guid from mm_metadata_tvshow
    //   where mm_metadata_media_tvshow_id = $1
    // - tvdb / imdb: match on the id keys in mm_metadata_tvshow_json
    if tmdb_id != None {
        metadata_uuid = await db_connection.db_metatv_guid_by_tmdb(tmdb_id)
    }
    // keep these separate just in case imdb is there but tmdb isn't
    if tvdb_id != None and metadata_uuid is None:
        metadata_uuid = await db_connection.db_metatv_guid_by_tvdb(tvdb_id)
    if imdb_id != None and metadata_uuid is None:
        metadata_uuid = await db_connection.db_metatv_guid_by_imdb(imdb_id)
    */
    /*
    // not ported: the "Fetch" update below also stores the provider id in
    // mm_download_provider_id and no mk_lib_database function updates that
    // column on an existing queue row yet. Setting the status to "Fetch"
    // without it would make the Fetch worker fail on a missing provider_id
    // (metadata_fetch reads download_data.mm_download_provider_id), so the
    // whole branch stays off until the db helper exists.
    // if ids from nfo/xml on local db
    if metadata_uuid is None:
        // id is known from nfo/xml but not in db yet so fetch data
        if tmdb_id != None or imdb_id != None:
            if tmdb_id != None:
                provider_id = str(tmdb_id)
            else:
                provider_id = imdb_id
            dl_meta = await db_connection.db_download_que_exists(download_data['mdq_id'],
                                                                 common_global.DLMediaType.TV.value,
                                                                 'themoviedb', provider_id)
            if dl_meta is None:
                metadata_uuid = download_data['mdq_new_uuid']
                await db_connection.db_begin()
                await db_connection.db_download_update(guid=download_data['mdq_id'],
                                                       status='Fetch',
                                                       provider_guid=provider_id)
                // set provider last so it's not picked up by the wrong thread too early
                await db_connection.db_download_update_provider('themoviedb',
                                                                download_data['mdq_id'])
                await db_connection.db_commit()
            else:
                metadata_uuid = dl_meta
        else if tvdb_id != None:
            dl_meta = await db_connection.db_download_que_exists(download_data['mdq_id'],
                                                                 common_global.DLMediaType.TV.value,
                                                                 'thetvdb', str(tvdb_id))
            if dl_meta is None:
                metadata_uuid = download_data['mdq_new_uuid']
                await db_connection.db_begin()
                await db_connection.db_download_update(guid=download_data['mdq_id'],
                                                       status='Fetch',
                                                       provider_guid=tvdb_id)
                // set provider last so it's not picked up by the wrong thread too early
                await db_connection.db_download_update_provider('thetvdb', download_data['mdq_id'])
                await db_connection.db_commit()
            else:
                metadata_uuid = dl_meta
    */

    if metadata_uuid == uuid::Uuid::nil() {
        // no ids found on the local database so begin name/year searches
        let _ = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(
            json!({ "stuff": "tv db lookup", "file": file_name.title() }),
        )
        .await;
        /*
        // not ported: no TV show name/year lookup exists in the published
        // mk_lib_database yet (mk_lib_database_metadata_find_guid is
        // movie-only). Add a db_metatv_guid_by_tvshow_name equivalent
        // (match mm_metadata_tvshow_name, year from the json air date).
        // db lookup by name and year (if available)
        if "year" in file_name:
            metadata_uuid = await db_connection.db_metatv_guid_by_tvshow_name(file_name['title'],
                                                                               file_name['year'])
        else:
            metadata_uuid = await db_connection.db_metatv_guid_by_tvshow_name(file_name['title'],
                                                                               None)
        */
        let tv_db_meta = (metadata_uuid != uuid::Uuid::nil()).then_some(metadata_uuid);
        let _ = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(
            json!({ "tv db meta": tv_db_meta }),
        )
        .await;

        if metadata_uuid == uuid::Uuid::nil() {
            metadata_uuid = download_data.mm_download_new_uuid;
            // no matches by name/year
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
    }

    // set last values to negate lookups for same show
    let mut last = tv_last_lookup();
    last.metadata_last_id = metadata_uuid;
    last.metadata_last_imdb = imdb_id;
    last.metadata_last_tvdb = tvdb_id;
    last.metadata_last_tmdb = tmdb_id;

    Ok(metadata_uuid)
}

/*

pub async fn metadata_tv_lookup(db_connection, download_data, file_name):

    // determine provider id's from nfo/xml if they exist
    nfo_data = await metadata_nfo_xml.nfo_file_tv(download_data['Path'])
    imdb_id, tvdb_id, tmdb_id = await metadata_nfo_xml.nfo_id_lookup_tv(nfo_data)
    // if same as last, return last id and save lookup
    // check these dupes as the nfo/xml files might not exist to pull the metadata id from
    if imdb_id != None and imdb_id == metadata_tv_lookup.metadata_last_imdb:
        // don't need to set last......since they are equal
        return metadata_tv_lookup.metadata_last_id
    if tvdb_id != None and tvdb_id == metadata_tv_lookup.metadata_last_tvdb:
        // don't need to set last......since they are equal
        return metadata_tv_lookup.metadata_last_id
    if tmdb_id != None and tmdb_id == metadata_tv_lookup.metadata_last_tmdb:
        // don't need to set last......since they are equal
        return metadata_tv_lookup.metadata_last_id
    // if ids from nfo/xml, query local db to see if exist
    if tmdb_id != None:
        metadata_uuid = await db_connection.db_metatv_guid_by_tmdb(tmdb_id)
    if tvdb_id != None and metadata_uuid is None:
        metadata_uuid = await db_connection.db_metatv_guid_by_tvdb(tvdb_id)
    if imdb_id != None and metadata_uuid is None:
        metadata_uuid = await db_connection.db_metatv_guid_by_imdb(imdb_id)
    // if ids from nfo/xml on local db
    if metadata_uuid is None:
        // id is known from nfo/xml but not in db yet so fetch data
        if tmdb_id != None or imdb_id != None:
            if tmdb_id != None:
                provider_id = str(tmdb_id)
            else:
                provider_id = imdb_id
            dl_meta = await db_connection.db_download_que_exists(download_data['mdq_id'],
                                                                 common_global.DLMediaType.TV.value,
                                                                 'themoviedb', provider_id)
            if dl_meta is None:
                metadata_uuid = download_data['mdq_new_uuid']
                await db_connection.db_begin()
                await db_connection.db_download_update(guid=download_data['mdq_id'],
                                                       status='Fetch',
                                                       provider_guid=provider_id)
                // set provider last so it's not picked up by the wrong thread too early
                await db_connection.db_download_update_provider('themoviedb',
                                                                download_data['mdq_id'])
                await db_connection.db_commit()
            else:
                metadata_uuid = dl_meta
        else if tvdb_id != None:
            dl_meta = await db_connection.db_download_que_exists(download_data['mdq_id'],
                                                                 common_global.DLMediaType.TV.value,
                                                                 'thetvdb', str(tvdb_id))
            if dl_meta is None:
                metadata_uuid = download_data['mdq_new_uuid']
                await db_connection.db_begin()
                await db_connection.db_download_update(guid=download_data['mdq_id'],
                                                       status='Fetch',
                                                       provider_guid=tvdb_id)
                // set provider last so it's not picked up by the wrong thread too early
                await db_connection.db_download_update_provider('thetvdb', download_data['mdq_id'])
                await db_connection.db_commit()
            else:
                metadata_uuid = dl_meta
    if metadata_uuid is None:
        // no ids found on the local database so begin name/year searches
        await mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(message_type='info',
                                                                         message_text=
                                                                         {'stuff': "tv db lookup",
                                                                          'file': str(file_name)})
        // db lookup by name and year (if available)
        if 'year' in file_name:
            metadata_uuid = await db_connection.db_metatv_guid_by_tvshow_name(file_name['title'],
                                                                              file_name['year'])
        else:
            metadata_uuid = await db_connection.db_metatv_guid_by_tvshow_name(file_name['title'],
                                                                              None)
        await mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(message_type='info',
                                                                         message_text={
                                                                             "tv db meta": metadata_uuid})
        if metadata_uuid is None:
            // no matches by name/year
            // search themoviedb since not matched above via DB or nfo/xml
            // save the updated status
            await db_connection.db_begin()
            await db_connection.db_download_update(guid=download_data["mdq_id"],
                                                   status="Search")
            // set provider last so it's not picked up by the wrong thread
            await db_connection.db_download_update_provider("themoviedb", download_data["mdq_id"])
            await db_connection.db_commit()
    // set last values to negate lookups for same show
    metadata_tv_lookup.metadata_last_id = metadata_uuid
    metadata_tv_lookup.metadata_last_imdb = imdb_id
    metadata_tv_lookup.metadata_last_tvdb = tvdb_id
    metadata_tv_lookup.metadata_last_tmdb = tmdb_id
    return metadata_uuid

 */
