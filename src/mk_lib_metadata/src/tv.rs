#![cfg_attr(debug_assertions, allow(dead_code))]

use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use std::error::Error;
use stdext::function_name;
use torrent_name_parser::Metadata;

use crate::mk_lib_logging;

#[path = "../mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;
use crate::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;

#[path = "provider/tmdb.rs"]
mod provider_tmdb;

pub struct MetadataTVLastLookup {
    metadata_last_id: Uuid,
    metadata_last_imdb: String,
    metadata_last_tvdb: String,
    metadata_last_tmdb: String,
}

pub async fn metadata_tv_lookup(
    sqlx_pool: &sqlx::PgPool,
    download_data: &DBDownloadQueueByProviderList,
    file_name: Metadata,
) -> Result<Uuid, Box<dyn Error>> {
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
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
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
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
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

/*

pub async fn tv_fetch_save_tmdb(db_connection, tmdb_id, metadata_uuid):
    """
    # tmdb data fetch for tv
    """
    result_json = await common_global.api_instance.com_tmdb_metadata_tv_by_id(tmdb_id)
    // 504	Your request to the backend server timed out. Try again.
    if result_json is None or result_json.status_code == 504:
        time.sleep(60)
        // redo fetch due to 504
        await tv_fetch_save_tmdb(db_connection, tmdb_id, metadata_uuid)
    else if result_json.status_code == 200:
        series_id, result_json, image_json \
            = await common_global.api_instance.com_tmdb_meta_info_build(result_json.json())
        await db_connection.db_metatv_insert_tmdb(metadata_uuid,
                                                  series_id,
                                                  result_json["name"],
                                                  result_json,
                                                  image_json)
        // store the cast and crew
        if result_json.contains_key("credits"):  // cast/crew doesn't exist on all media
            if result_json["credits"].contains_key("cast"):
                await db_connection.db_meta_person_insert_cast_crew("themoviedb",
                                                                    result_json["credits"][
                                                                        "cast"])
            if result_json["credits"].contains_key("crew"):
                await db_connection.db_meta_person_insert_cast_crew("themoviedb",
                                                                    result_json["credits"][
                                                                        "crew"])
    // 429	Your request count (#) is over the allowed limit of (40).
    else if result_json.status_code == 429:
        time.sleep(20)
        // redo fetch due to 504
        await tv_fetch_save_tmdb(db_connection, tmdb_id, metadata_uuid)
    else if result_json.status_code == 404:
        // TODO handle 404's better
        metadata_uuid = None
    else:  // is this is None....
        metadata_uuid = None
    return metadata_uuid

 */
