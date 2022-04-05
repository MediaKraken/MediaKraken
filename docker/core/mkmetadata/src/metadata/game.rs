use sqlx::types::Uuid;

#[path = "provider/giant_bomb.rs"]
mod mk_provider_giant_bomb;

#[path = "provider/thegamesdb.rs"]
mod mk_provider_thegamesdb;

pub async fn metadata_game_lookup(pool: &sqlx::PgPool,
                                   download_data: serde_json::Value) {
    let mut metadata_uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000")?;  // so not found checks verify later
}

/*

async def game_system_update():
    data = await common_global.api_instance.com_meta_gamesdb_platform_list()[
        'Data']['Platforms']['Platform']
    print((type(data)), flush=True)
    print(data, flush=True)
    for game_system in data:
        print(game_system, flush=True)
        game_sys_detail = \
            await \
                common_global.api_instance.com_meta_gamesdb_platform_by_id(game_system['id'])[
                    'Data'][
                    'Platform']
        print((type(game_sys_detail)), flush=True)
        print(game_sys_detail, flush=True)
        break


async def metadata_game_lookup(db_connection, download_data):
    """
    Lookup game metadata
    """
    metadata_uuid = None  // so not found checks verify later
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'game filename':
                                                                             download_data['Path']})
    // TODO determine short name/etc
    for row_data in await db_connection.db_meta_game_by_name(download_data['Path']):
        // TODO handle more than one match
        metadata_uuid = row_data['gi_id']
        break
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         "meta game metadata_uuid B": metadata_uuid})
    if metadata_uuid is None:
        // no matches by name
        // search giantbomb since not matched above via DB or nfo/xml
        // save the updated status
        await db_connection.db_begin()
        await db_connection.db_download_update(guid=download_data['mdq_id'],
                                               status='Search')
        // set provider last so it's not picked up by the wrong thread
        await db_connection.db_download_update_provider("giantbomb", download_data["mdq_id"])
        await db_connection.db_commit()
    return metadata_uuid

 */
