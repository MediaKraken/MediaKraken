#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::types::Uuid;

#[path = "provider/imvdb.rs"]
mod provider_imvdb;

pub struct MetadataMusicVideoLastLookup {
    metadata_last_id: Uuid,
    metadata_last_band: String,
    metadata_last_song: String,
}

pub async fn metadata_music_video_lookup(pool: &sqlx::PgPool,
                                   file_name: String) {
    let mut metadata_uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000")?;  // so not found checks verify later
}

/*

// imvdb lookup
pub async fn metadata_music_video_lookup(db_connection, file_name):
    """
    Lookup by name on music video database
    """
    // check for same variables


    // determine names
    if file_name.find('-') != -1:
        band_name, song_name = os.path.splitext(
            os.path.basename(file_name.lower()))[0].split('-', 1)
        try:
            song_name = song_name.split('(', 1)[0].strip()
        except:
            pass
        // set name for lookups
        band_name = band_name.strip().replace(' ', '-')
        song_name = song_name.strip().replace(' ', '-')
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'mv title': band_name,
                                                                             'song': song_name})
        // if same as last, return last id and save lookup
        if band_name == metadata_music_video_lookup.metadata_last_band \
                and song_name == metadata_music_video_lookup.metadata_last_song:
            return metadata_music_video_lookup.metadata_last_id
        metadata_uuid = db_connection.db_meta_music_video_lookup(band_name, song_name)
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "uuid": metadata_uuid})
        if metadata_uuid == []:
            metadata_uuid = None
        if metadata_uuid is None:
            if IMVDB_CONNECTION != None:
                imvdb_json = IMVDB_CONNECTION.com_imvdb_search_video(band_name, song_name)
                await common_logging_elasticsearch_httpx.com_es_httpx_post_async(
                    message_type='info',
                    message_text={
                        "imvdb return": imvdb_json})
                if imvdb_json != None:
                    # parse the results and insert/update
                    for video_data in imvdb_json['results']:
                        // the results are bit crazy....hence the breakup and insert
                        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(
                            message_type='info',
                            message_text={
                                "vid data": video_data})
                        if db_connection.db_meta_music_video_count(str(video_data['id'])) == 0:
                            # need to submit a fetch record for limiter and rest of video data
                            if db_connection.db_download_que_exists(None,
                                                                    common_global.DLMediaType.Music_Video.value,
                                                                    'imvdb',
                                                                    str(video_data['id'])) is None:
                                db_connection.db_download_insert(provider='imvdb',
                                                                 que_type=common_global.DLMediaType.Music_Video.value,
                                                                 down_json={"Status": "Fetch",
                                                                            "ProviderMetaID":
                                                                                video_data['id']},
                                                                 down_new_uuid=uuid.uuid4(),
                                                                 )
                            # db_connection.db_meta_music_video_add(video_data['artists'][0]['slug'],
                            #                                       video_data['song_slug'],  {'imvdb': str(video_data['id'])},
                            #                                       video_data,
                            #                                         {'Images': {'imvdb': None}})
                    // try after inserting new records
                    metadata_uuid = db_connection.db_meta_music_video_lookup(
                        band_name, song_name)
                    if metadata_uuid == []:
                        metadata_uuid = None
        // set last values to negate lookups for same song
        metadata_music_video_lookup.metadata_last_id = metadata_uuid
        metadata_music_video_lookup.metadata_last_band = band_name
        metadata_music_video_lookup.metadata_last_song = song_name
        return metadata_uuid
    else:
        return None
 */
