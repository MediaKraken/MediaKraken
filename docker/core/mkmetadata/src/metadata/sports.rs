#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::types::Uuid;

#[path = "provider/thesportsdb.rs"]
mod provider_thesportsdb;

pub struct MetadataSportsLastLookup {
    metadata_last_id: Uuid,
    metadata_last_imdb: String,
    metadata_last_tmdb: String,
    metadata_last_thesportsdb: String,
}

pub async fn metadata_sports_lookup(pool: &sqlx::PgPool,
                                    download_data: serde_json::Value)
                                    -> Result<Uuid, sqlx::Error> {
    // don't bother checking title/year as the main_server_metadata_api_worker does it already
    let mut metadata_uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();  // so not found checks verify later
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
