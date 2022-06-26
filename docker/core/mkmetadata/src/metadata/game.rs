#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::error::Error;
use sqlx::types::Uuid;
use sqlx::postgres::PgRow;

#[path = "provider/giant_bomb.rs"]
mod provider_giant_bomb;

#[path = "provider/thegamesdb.rs"]
mod mk_provider_thegamesdb;

#[path = "../mk_lib_database_metadata_game.rs"]
mod mk_lib_database_metadata_game;

#[path = "../mk_lib_hash_sha1.rs"]
mod mk_lib_hash_sha1;

pub async fn metadata_game_lookup(pool: &sqlx::PgPool,
                                  download_data: PgRow)
                                  -> Result<Uuid, sqlx::Error> {
    let mut metadata_uuid = uuid::Uuid::nil();  // so not found checks verify later
    // TODO remove the file extension
    metadata_uuid = mk_lib_database_metadata_game::mk_lib_database_metadata_game_by_name_and_system(&pool,
        Path::new(download_data.mdq_path).file_name(), 0).await.unwrap();
    if metadata_uuid == uuid::Uuid::nil() {
        let sha1_hash = mk_lib_hash_sha1::mk_file_hash_sha1(download_data.get("mdq_path")).unwrap();
        metadata_uuid = mk_lib_database_metadata_game::mk_lib_database_metadata_game_by_sha1(&pool, sha1_hash).await.unwrap();
    }
    Ok(metadata_uuid)
}

/*

pub async fn game_system_update():
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


pub async fn metadata_game_lookup(db_connection, download_data):
    """
    Lookup game metadata
    """
    metadata_uuid = None  // so not found checks verify later
    // TODO determine short name/etc
    for row_data in await db_connection.db_meta_game_by_name(download_data["Path"]):
        // TODO handle more than one match
        metadata_uuid = row_data["gi_id"]
        break
    if metadata_uuid == None:
        // no matches by name
        // search giantbomb since not matched above via DB or nfo/xml
        // save the updated status
        await db_connection.db_begin()
        await db_connection.db_download_update(guid=download_data["mdq_id"],
                                               status="Search")
        // set provider last so it's not picked up by the wrong thread
        await db_connection.db_download_update_provider("giantbomb", download_data["mdq_id"])
        await db_connection.db_commit()
    return metadata_uuid

 */
