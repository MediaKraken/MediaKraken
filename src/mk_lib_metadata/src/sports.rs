/*

async def metadata_sports_lookup(db_connection, download_data):
    """
    Lookup sporting event by name
    """
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'function':
                                                                             inspect.stack()[0][
                                                                                 3],
                                                                         'locals': locals(),
                                                                         'caller':
                                                                             inspect.stack()[1][
                                                                                 3]})
    # don't bother checking title/year as the main_server_metadata_api_worker does it already
    if not hasattr(metadata_sports_lookup, "metadata_last_id"):
        # it doesn't exist yet, so initialize it
        metadata_sports_lookup.metadata_last_id = None
        metadata_sports_lookup.metadata_last_imdb = None
        metadata_sports_lookup.metadata_last_tmdb = None
        metadata_sports_lookup.metadata_last_thesportsdb = None
    metadata_uuid = None  # so not found checks verify later

    stripped_name = os.path.basename(
        download_data['mdq_path'].replace('_', ' ').rsplit('(', 1)[0].strip())
    metadata_uuid = await db_connection.db_meta_sports_guid_by_event_name(stripped_name)
    if metadata_uuid is None and THESPORTSDB_CONNECTION is not None:
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "searching": stripped_name})
        thesportsdb_data = \
            THESPORTSDB_CONNECTION.com_meta_thesportsdb_search_event_by_name(stripped_name)
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "sports return": thesportsdb_data})
        # "valid" key returned in case of null response........or event none
        if thesportsdb_data is not None:
            thesportsdb_data = json.loads(thesportsdb_data)
            if thesportsdb_data['event'] is not None:
                # TODO "find" the right event by name?  if multiples?
                metadata_uuid = await db_connection.db_meta_sports_guid_by_thesportsdb(
                    thesportsdb_data['event'][0]['idEvent'])
                if metadata_uuid is None:
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
    # set last values to negate lookups for same title/show
    metadata_sports_lookup.metadata_last_id = metadata_uuid
    metadata_sports_lookup.metadata_last_imdb = imdb_id
    metadata_sports_lookup.metadata_last_tmdb = tmdb_id
    metadata_sports_lookup.metadata_last_thesportsdb = thesportsdb_id
    return metadata_uuid

 */
// cargo test -- --show-output
#[cfg(test)]
mod test_mk_lib_common {
    use super::*;

    macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
  }
}