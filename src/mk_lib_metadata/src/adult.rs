#![cfg_attr(debug_assertions, allow(dead_code))]

use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use std::error::Error;
use stdext::function_name;
use torrent_name_parser::Metadata;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "provider/pornhub.rs"]
mod provider_pornhub;

#[path = "provider/tmdb.rs"]
mod provider_tmdb;

#[path = "../mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;
use crate::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;

pub struct MetadataAdultLastLookup {
    metadata_last_id: Uuid,
    metadata_last_imdb: String,
    metadata_last_tmdb: String,
}

pub async fn metadata_adult_lookup(
    sqlx_pool: &sqlx::PgPool,
    download_data: &DBDownloadQueueByProviderList,
    file_name: Metadata,
) -> Result<Uuid, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // don't bother checking title/year as the main_server_metadata_api_worker does it already
    let mut metadata_uuid = uuid::Uuid::nil(); // so not found checks verify later
    Ok(metadata_uuid)
}

/*

pub async fn metadata_adult_lookup(db_connection, download_data, file_name):

    // determine provider id's from nfo/xml if they exist
    nfo_data, xml_data = await metadata_nfo_xml.nfo_xml_file(download_data['Path'])
    imdb_id, tmdb_id = await metadata_nfo_xml.nfo_xml_id_lookup(nfo_data, xml_data)
    if imdb_id != None or tmdb_id != None:
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "meta adult look": imdb_id,
                                                                             'tmdb': tmdb_id})
    // if same as last, return last id and save lookup
    if imdb_id != None and imdb_id == metadata_adult_lookup.metadata_last_imdb:
        await db_connection.db_download_delete(download_data['mdq_id'])
        await db_connection.db_commit()
        // don't need to set last......since they are equal
        return metadata_adult_lookup.metadata_last_id
    if tmdb_id != None and tmdb_id == metadata_adult_lookup.metadata_last_tmdb:
        await db_connection.db_download_delete(download_data['mdq_id'])
        await db_connection.db_commit()
        // don't need to set last......since they are equal
        return metadata_adult_lookup.metadata_last_id
    // doesn't match last id's so continue lookup
    // if ids from nfo/xml, query local db to see if exist
    if tmdb_id != None:
        metadata_uuid = await db_connection.db_meta_guid_by_tmdb(tmdb_id)
    if imdb_id != None and metadata_uuid is None:
        metadata_uuid = await db_connection.db_meta_guid_by_imdb(imdb_id)
    // if ids from nfo/xml on local db
    if metadata_uuid != None:
        await db_connection.db_download_delete(download_data['mdq_id'])
        await db_connection.db_commit()
        // fall through here to set last id's
    else:
        // check to see if id is known from nfo/xml but not in db yet so fetch data
        if tmdb_id != None or imdb_id != None:
            if tmdb_id != None:
                provider_id = tmdb_id
            else:
                provider_id = imdb_id
            dl_meta = await db_connection.db_download_que_exists(download_data['mdq_id'],
                                                                 common_global.DLMediaType.Movie.value,
                                                                 'pornhub',
                                                                 provider_id)
            if dl_meta is None:
                metadata_uuid = download_data['mdq_new_uuid']
                await db_connection.db_begin()
                await db_connection.db_download_update(guid=download_data['mdq_id'],
                                                       status='Fetch')
                # set provider last so it's not picked up by the wrong thread too early
                await db_connection.db_download_update_provider('pornhub', download_data['mdq_id'])
                await db_connection.db_commit()
            else:
                await db_connection.db_download_delete(download_data['mdq_id'])
                await db_connection.db_commit()
                metadata_uuid = dl_meta
    if metadata_uuid == None:
        // no ids found on the local database so begin name/year searches
        // db lookup by name and year (if available)
        if file_name.contains_key("year"):
            metadata_uuid = await db_connection.db_find_metadata_guid(file_name['title'],
                                                                      file_name['year'])
        else:
            metadata_uuid = await db_connection.db_find_metadata_guid(file_name['title'], None)
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "meta adult db meta": metadata_uuid})
        if metadata_uuid == None:
            metadata_uuid = download_data['mdq_new_uuid']
            // no matches by name/year on local database
            // search themoviedb since not matched above via DB or nfo/xml
            // save the updated status
            await db_connection.db_begin()
            await db_connection.db_download_update(guid=download_data['mdq_id'],
                                                   status='Search')
            // set provider last so it's not picked up by the wrong thread
            await db_connection.db_download_update_provider('pornhub', download_data['mdq_id'])
            await db_connection.db_commit()
    // set last values to negate lookups for same title/show
    metadata_adult_lookup.metadata_last_id = metadata_uuid
    metadata_adult_lookup.metadata_last_imdb = imdb_id
    metadata_adult_lookup.metadata_last_tmdb = tmdb_id
    return metadata_uuid

 */
