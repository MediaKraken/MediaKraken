#![cfg_attr(debug_assertions, allow(dead_code))]

use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use std::error::Error;
use stdext::function_name;

use crate::mk_lib_logging;

use crate::database::mk_lib_database_metadata_download_queue;
use crate::database::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;

#[path = "provider/thesportsdb.rs"]
mod provider_thesportsdb;

pub struct MetadataSportsLastLookup {
    metadata_last_id: Uuid,
    metadata_last_imdb: String,
    metadata_last_tmdb: String,
    metadata_last_thesportsdb: String,
}

pub async fn metadata_sports_lookup(
    sqlx_pool: &sqlx::PgPool,
    download_data: &DBDownloadQueueByProviderList,
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

pub async fn metadata_sports_lookup(db_connection, download_data):


    stripped_name = os.path.basename(
        download_data['mdq_path'].replace('_', ' ').rsplit('(', 1)[0].strip())
    metadata_uuid = await db_connection.db_meta_sports_guid_by_event_name(stripped_name)
    if metadata_uuid is None and THESPORTSDB_CONNECTION != None:
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "searching": stripped_name})
        thesportsdb_data = \
            THESPORTSDB_CONNECTION.com_meta_thesportsdb_search_event_by_name(stripped_name)
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "sports return": thesportsdb_data})
        // "valid" key returned in case of null response........or event none
        if thesportsdb_data != None:
            thesportsdb_data = json.loads(thesportsdb_data)
            if thesportsdb_data['event'] != None:
                // TODO "find" the right event by name?  if multiples?
                metadata_uuid = await db_connection.db_meta_sports_guid_by_thesportsdb(
                    thesportsdb_data['event'][0]['idEvent'])
                if metadata_uuid == None:
                    image_json = {'Images': {'thesportsdb': {'Characters': {}, 'Banner': None,
                                                             'Poster': None, 'Backdrop': None,
                                                             "Redo": True}}}
                    await db_connection.db_metathesportsdb_insert({'thesportsdb':
                                                                       thesportsdb_data['event'][0][
                                                                           'idEvent']},
                                                                  thesportsdb_data['event'][0][
                                                                      'strFilename'],
                                                                  thesportsdb_data,
                                                                  image_json)

    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         "metadata_sports return uuid": metadata_uuid})
    // set last values to negate lookups for same title/show
    metadata_sports_lookup.metadata_last_id = metadata_uuid
    metadata_sports_lookup.metadata_last_imdb = imdb_id
    metadata_sports_lookup.metadata_last_tmdb = tmdb_id
    metadata_sports_lookup.metadata_last_thesportsdb = thesportsdb_id
    return metadata_uuid

 */
