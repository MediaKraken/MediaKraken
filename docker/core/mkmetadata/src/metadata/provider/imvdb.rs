#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::error::Error;
use sqlx::types::Uuid;

// https://imvdb.com/developers/api

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

const BASE_API_URL: String = "http://imvdb.com/api/v1".to_string();

/*

class CommonMetadataIMVdb:
    def __init__(self, option_config_json):
        self.headers = {'User-Agent': 'MediaKraken_0.1.6',
                        'IMVDB-APP-KEY': option_config_json['API']['imvdb'],
                        'Accept': 'application/json'}

    pub async fn com_imvdb_video_info(self, video_id):
        """
        Video info
        """
        resp = requests.post(self.base_api_url + "/video/" + video_id
                             + "?include=sources,credits,bts,featured,popularity,countries",
                             headers=self.headers)
        try:
            return resp.json()
        except:
            return None

    pub async fn com_imvdb_search_video(self, artist_name, song_title):
        """
        Search for video by band name and song title
        """
        resp = requests.post(self.base_api_url + "/search/videos?q="
                             + (artist_name.replace(' ', '+') + '+'
                                + song_title.replace(' ', '+')),
                             headers=self.headers)
        try:
            return resp.json()
        except:
            return None

    pub async fn com_imvdb_search_entities(self, artist_name):
        """
        Search by band name
        """
        resp = requests.post(self.base_api_url + "/search/entities?q="
                             + artist_name.replace(' ', '+'), headers=self.headers)
        try:
            return resp.json()
        except:
            return None


pub async fn meta_fetch_save_imvdb(db_connection, imvdb_id, metadata_uuid):
    """
    # fetch from imvdb
    """
    // fetch and save json data via tmdb id
    result_json = await common_global.api_instance.com_imvdb_video_info(imvdb_id)
    if result_json.status_code == 200:
        // set and insert the record
        await db_connection.db_meta_music_video_add(metadata_uuid,
                                                    {'imvdb': str(result_json['id'])},
                                                    result_json['artists'][0]['slug'],
                                                    result_json['song_slug'],
                                                    result_json,
                                                    None)
    else if result_json.status_code == 502:
        time.sleep(300)
        // redo fetch due to 502
        await movie_fetch_save_imvdb(db_connection, imvdb_id, metadata_uuid)
    else if result_json.status_code == 404:
        // TODO handle 404's better
        metadata_uuid = None
    else:  # is this is None....
        metadata_uuid = None
    return metadata_uuid

 */

pub async fn meta_fetch_save_imvdb(pool: &sqlx::PgPool,
                                   imvdb_id: i64,
                                   metadata_uuid: Uuid)
                                   -> Result<Uuid, Box<dyn Error>> {
    let mut metadata_uuid = uuid::Uuid::nil();  // so not found checks verify later
    Ok(metadata_uuid)
}
