#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::types::Uuid;
use std::error::Error;
use torrent_name_parser::Metadata;

#[path = "provider/anidb.rs"]
mod provider_anidb;

#[path = "provider/tmdb.rs"]
mod provider_tmdb;

pub struct MetadataAnimeLastLookup {
    metadata_last_id: Uuid,
    metadata_last_imdb: String,
    metadata_last_tmdb: String,
    metadata_last_anidb: String,
}

pub async fn metadata_anime_lookup(pool: &sqlx::PgPool,
                                   download_data: serde_json::Value,
                                   file_name: Metadata)
                                   -> Result<Uuid, sqlx::Error> {
    let mut metadata_uuid = uuid::Uuid::nil();  // so not found checks verify later
    Ok(metadata_uuid)
}

/*

pub async fn metadata_anime_lookup(db_connection, download_data, file_name):


    // determine provider id's from nfo/xml if they exist
    nfo_data, xml_data = await metadata_nfo_xml.nfo_xml_file(file_name)
    imdb_id, tmdb_id, anidb_id = await metadata_nfo_xml.nfo_xml_id_lookup(
        nfo_data, xml_data)
    // if same as last, return last id and save lookup
    if imdb_id != None and imdb_id == metadata_anime_lookup.metadata_last_imdb:
        await db_connection.db_download_delete(download_data['mdq_id'])
        await db_connection.db_commit()
        // don't need to set last......since they are equal
        return metadata_anime_lookup.metadata_last_id
    if tmdb_id != None and tmdb_id == metadata_anime_lookup.metadata_last_tmdb:
        await db_connection.db_download_delete(download_data['mdq_id'])
        await db_connection.db_commit()
        // don't need to set last......since they are equal
        return metadata_anime_lookup.metadata_last_id
    if anidb_id != None and anidb_id == metadata_anime_lookup.metadata_last_anidb:
        await db_connection.db_download_delete(download_data['mdq_id'])
        await db_connection.db_commit()
        // don't need to set last......since they are equal
        return metadata_anime_lookup.metadata_last_id
    // if ids from nfo/xml, query local db to see if exist
    if tmdb_id != None:
        metadata_uuid = await db_connection.db_meta_guid_by_tmdb(tmdb_id)
    if imdb_id != None and metadata_uuid is None:
        metadata_uuid = await db_connection.db_meta_guid_by_imdb(imdb_id)
    if anidb_id != None and metadata_uuid is None:
        metadata_uuid = await db_connection.db_meta_guid_by_anidb(anidb_id)
    // if ids from nfo/xml on local db
    if metadata_uuid != None:
        await db_connection.db_download_delete(download_data['mdq_id'])
        await db_connection.db_commit()
        // fall through here to set last name/year id's
    else:
        // id is known from nfo/xml but not in db yet so fetch data
        if tmdb_id != None or imdb_id != None:
            if tmdb_id != None:
                dl_meta = db_connection.db_download_que_exists(download_data["mdq_id"],
                                                               common_global.DLMediaType.Movie.value,
                                                               "themoviedb", str(tmdb_id))
                if dl_meta is None:
                    metadata_uuid = download_data["mdq_new_uuid"]
                    await db_connection.db_begin()
                    await db_connection.db_download_update(guid=download_data['mdq_id'],
                                                           status='Fetch',
                                                           provider_guid=tmdb_id)
                    // set provider last so it's not picked up by the wrong thread too early
                    await db_connection.db_download_update_provider('themoviedb',
                                                                    download_data['mdq_id'])
                    await db_connection.db_commit()
                else:
                    await db_connection.db_download_delete(download_data['mdq_id'])
                    await db_connection.db_commit()
                    metadata_uuid = dl_meta
            else:
                dl_meta = await db_connection.db_download_que_exists(download_data['mdq_id'],
                                                                     common_global.DLMediaType.Movie.value,
                                                                     'themoviedb', imdb_id)
                if dl_meta is None:
                    metadata_uuid = download_data['mdq_new_uuid']
                    await db_connection.db_begin()
                    await db_connection.db_download_update(guid=download_data['mdq_id'],
                                                           status='Fetch',
                                                           provider_guid=imdb_id)
                    // set provider last so it's not picked up by the wrong thread too early
                    await db_connection.db_download_update_provider('themoviedb',
                                                                    download_data['mdq_id'])
                    await db_connection.db_commit()
                else:
                    await db_connection.db_download_delete(download_data['mdq_id'])
                    await db_connection.db_commit()
                    metadata_uuid = dl_meta
        if metadata_uuid is None and anidb_id != None:
            dl_meta = await db_connection.db_download_que_exists(download_data['mdq_id'],
                                                                 common_global.DLMediaType.Movie.value,
                                                                 'anidb', str(anidb_id))
            if dl_meta is None:
                metadata_uuid = download_data['mdq_new_uuid']
                await db_connection.db_begin()
                await db_connection.db_download_update(guid=download_data['mdq_id'],
                                                       status='Fetch',
                                                       provider_guid=anidb_id)
                // set provider last so it's not picked up by the wrong thread too early
                await db_connection.db_download_update_provider(
                    'anidb', download_data['mdq_id'])
                await db_connection.db_commit()
            else:
                await db_connection.db_download_delete(download_data['mdq_id'])
                await db_connection.db_commit()
                metadata_uuid = dl_meta
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         "meta anime metadata_uuid B": metadata_uuid})
    if metadata_uuid is None:
        // no ids found on the local database so begin name/year searches
        // db lookup by name and year (if available)
        if file_name.contains_key("year"):
            metadata_uuid = await db_connection.db_find_metadata_guid(file_name["title"],
                                                                      file_name["year"])
        else:
            metadata_uuid = await db_connection.db_find_metadata_guid(
                file_name["title"], None)
        if metadata_uuid == None:
            // no matches by name/year
            // search themoviedb since not matched above via DB or nfo/xml
            // save the updated status
            await db_connection.db_begin()
            await db_connection.db_download_update(guid=download_data["mdq_id"],
                                                   status="Search")
            // set provider last so it's not picked up by the wrong thread
            await db_connection.db_download_update_provider("themoviedb", download_data["mdq_id"])
            await db_connection.db_commit()
    // set last values to negate lookups for same title/show
    metadata_anime_lookup.metadata_last_id = metadata_uuid
    metadata_anime_lookup.metadata_last_imdb = imdb_id
    metadata_anime_lookup.metadata_last_tmdb = tmdb_id
    metadata_anime_lookup.metadata_last_anidb = anidb_id
    return metadata_uuid

 */
