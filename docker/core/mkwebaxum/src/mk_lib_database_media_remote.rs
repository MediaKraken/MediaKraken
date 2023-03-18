#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;
use serde_json::json;

pub async fn mk_lib_database_remote_media_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i32, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (i32,) = sqlx::query_as("select count(*) from mm_media_remote")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

/*
// TODO port query
def db_insert_remote_media(self, media_link_uuid, media_uuid, media_class_uuid,
                           media_metadata_uuid, media_ffprobe_json):
    new_guid = uuid.uuid4()
    self.db_cursor.execute('insert into mm_media_remote (mmr_media_guid,'
                           ' mmr_media_link_id,'
                           ' mmr_media_uuid,'
                           ' mmr_media_class_guid,'
                           ' mmr_media_metadata_guid,'
                           ' mmr_media_ffprobe_json)'
                           ' values ($1,$2,$3,$4,$5,$6)', (new_guid, media_link_uuid, media_uuid,
                                                           media_class_uuid, media_metadata_uuid,
                                                           media_ffprobe_json))
    self.db_commit()
    return new_guid


// TODO port query
def db_read_remote_media(self, media_guid=None):
    if media_guid != None:
        self.db_cursor.execute('select * from mm_media_remote where mmr_media_guid = $1',
                               (media_guid,))
        try:
            return self.db_cursor.fetchone()
        except:
            return None
    else:
        self.db_cursor.execute('select * from mm_media_remote')
        return self.db_cursor.fetchall()





# processed via main_link........
# process new records from network sync event from linked server
# def db_Media_Remote_New_Data(self, link_uuid, link_records):
#    # 0-media guid, 1-type, 2-ffrobe, 3-media id json
#    metadata_guid = None
#    for row_data in link_records:
#        if row_data[1] == 'Movie':
#            if 'themoviedb' in row_data[3]:
#                metadata_guid = db_meta_guid_by_tmdb(row_data[3]['themoviedb'])
#            if metadata_guid is None and 'imdb' in row_data[3]:
#                metadata_guid = db_meta_guid_by_imdb(row_data[3]['imdb'])
#            if metadata_guid is None and 'thetvdb' in row_data[3]:
#                metadata_guid = db_meta_guid_by_tvdb(row_data[3]['thetvdb'])
#        else if row_data[1] == 'TV Show':
#            if 'imdb' in row_data[3]
#                metadata_guid = db_metaTV_guid_by_imdb(row_data[3]['imdb'])
#            if metadata_guid is None and 'thetvdb' in row_data[3]:
#                metadata_guid = db_metatv_guid_by_tvdb(row_data[3]['thetvdb'])
#            if metadata_guid is None and 'tvmaze' in row_data[3]:
#                metadata_guid = db_metaTV_guid_by_tvmaze(row_data[3]['tvmaze'])
#        else if row_data[1] == 'Sports':
#            pass
#        else if row_data[1] == 'Music':
#            pass
#        else if row_data[1] == 'Music Video':
#            pass
#        else if row_data[1] == 'Book':
#            pass
#        else:
#            common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='error', message_text={'stuff':'Link bad data type: %s', row_data[1])
#            return None
#        if metadata_guid != None:
#            self.db_insert_remote_media(link_uuid, row_data[0], \
# self.db_media_uuid_by_class(row_data[1]), metadata_guid[0], json.dumps(row_data[2]))


// TODO port query
def db_media_remote_read_new(self, date_last_sync, sync_movie=None, sync_tv=None,
                             sync_sports=None, sync_music=None, sync_music_video=None,
                             sync_book=None):
    """
    # new media for link
    """
    // TODO add games to this
    let mut first_query = true;
    let mut sync_query = String::new();
    if sync_movie != None:
        sync_query += ('select mm_media_guid, 'Movie','
                       ' mm_media_ffprobe_json,'
                       ' mm_metadata_media_id'
                       ' from mm_media, mm_metadata_movie'
                       ' where mm_media_metadata_guid = mm_metadata_guid'
                       ' and mm_media_json->>'DateAdded' >= $1', (date_last_sync,))
        first_query = false

    if sync_tv != None:
        if not first_query:
            sync_query += ' union all '
        sync_query += ('select mm_media_guid, 'TV Show','
                       ' mm_media_ffprobe_json,'
                       ' mm_metadata_media_tvshow_id'
                       ' from mm_media, mm_metadata_tvshow'
                       ' where mm_metadata_tvshow_guid = mm_metadata_tvshow_guid'
                       ' and mm_media_json->>'DateAdded' >= $1', (date_last_sync,))
        first_query = false

    if sync_sports != None:
        if not first_query:
            sync_query += ' union all '
        sync_query += ('select mm_media_guid, 'Sports','
                       ' mm_media_ffprobe_json,'
                       ' mm_metadata_media_sports_id'
                       ' from mm_media, mm_metadata_sports'
                       ' where mm_metadata_sports_guid = mm_metadata_sports_guid'
                       ' and mm_media_json->>'DateAdded' >= $1', (date_last_sync,))
        first_query = false

    if sync_music != None:
        if not first_query:
            sync_query += ' union all '
        sync_query += ('select mm_media_guid, 'Music','
                       ' mm_media_ffprobe_json,'
                       ' mm_metadata_media_music_id'
                       ' from mm_media, mm_metadata_music'
                       ' where mm_metadata_music_guid = mm_metadata_music_guid'
                       ' and mm_media_json->>'DateAdded' >= $1', (date_last_sync,))
        first_query = false

    if sync_music_video != None:
        if not first_query:
            sync_query += ' union all '
        sync_query += ('select mm_media_guid, 'Music Video','
                       ' mm_media_ffprobe_json,'
                       ' mm_metadata_music_video_media_id'
                       ' from mm_media, mm_metadata_music_video'
                       ' where mm_metadata_music_video_guid = mm_metadata_music_video_guid'
                       ' and mm_media_json->>'DateAdded' >= $1', (date_last_sync,))
        first_query = false

    if sync_book != None:
        if not first_query:
            sync_query += ' union all '
        sync_query += ('select mm_media_guid, 'Book','
                       ' mm_media_ffprobe_json,'
                       ' mm_metadata_book_isbn'
                       ' from mm_media, mm_metadata_book'
                       ' where mm_metadata_book_guid = mm_metadata_book_guid'
                       ' and mm_media_json->>'DateAdded' >= $1', (date_last_sync,))
        first_query = false
    if sync_query != '':
        self.db_cursor.execute(sync_query)
        return self.db_cursor.fetchall()
    else:
        return None

 */
