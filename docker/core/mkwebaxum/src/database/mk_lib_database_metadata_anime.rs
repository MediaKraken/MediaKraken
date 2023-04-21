#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::mk_lib_logging;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;

/*

// TODO port query
def db_metadata_anime_insert(self, ani_media_id_json, ani_name, ani_json,
                               ani_image_local, ani_user_json, mapping_data, before_data):
    """
    Insert new anidb entries into database
    """
    new_guid = uuid.uuid4()
    self.db_cursor.execute('insert into mm_metadata_anime(mm_metadata_anime_guid,'
                           ' mm_metadata_anime_media_id,'
                           ' mm_media_anime_name,'
                           ' mm_metadata_anime_json,'
                           ' mm_metadata_anime_localimage_json,'
                           ' mm_metadata_anime_user_json,'
                           ' mm_metadata_anime_mapping,'
                           ' mm_metadata_anime_mapping_before)'
                           ' values ($1,$2,$3,$4,$5,$6,$7,$8)',
                           (new_guid, ani_media_id_json, ani_name, ani_json,
                            ani_image_local, ani_user_json, mapping_data, before_data))
    self.db_commit()
    return new_guid


// TODO port query
def db_meta_anime_search(self, title_to_search):
    """
    search for title
    """
    // TODO hit movie and tv db's as well?
    self.db_cursor.execute('select mm_metadata_anime_guid'
                           ' from mm_metadata_anime'
                           ' where mm_media_anime_name = $1', (title_to_search,))
    try:
        return self.db_cursor.fetchone()[0]
    except:
        return None


// TODO port query
def db_meta_anime_update_meta_id(self, media_id_json, mapping_json, mapping_before):
    """
    Update the media id json from scudlee data
    """
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text={
        'ani_id_json': media_id_json})
    self.db_cursor.execute('update mm_metadata_anime set mm_metadata_anime_media_id = $1,'
                           ' mm_metadata_anime_mapping = $2,'
                           ' mm_metadata_anime_mapping_before = $3'
                           ' where mm_metadata_anime_media_id->'anidb' ? $4',
                           (media_id_json, mapping_json, mapping_before,
                            json.loads(media_id_json)['anidb']))
    self.db_commit()


// TODO port query
def db_meta_anime_meta_by_id(self, anidb_id):
    """
    Return count of records with id
    """
    self.db_cursor.execute('select mm_metadata_anime_guid'
                           ' from mm_metadata_anime'
                           ' where mm_metadata_anime_media_id->'anidb' ? $1', (anidb_id,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_anime_guid']
    except:
        return None

 */
