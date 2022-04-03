#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

/*

class CommonMetadataIMVdb:
    """
    Class for interfacing with imvdb
    """

    def __init__(self, option_config_json):
        self.headers = {'User-Agent': 'MediaKraken_0.1.6',
                        'IMVDB-APP-KEY': option_config_json['API']['imvdb'],
                        'Accept': 'application/json'}
        self.base_api_url = 'http://imvdb.com/api/v1'

    async def com_imvdb_video_info(self, video_id):
        """
        Video info
        """
        resp = requests.post(self.base_api_url + "/video/" + video_id
                             + "?include=sources,credits,bts,featured,popularity,countries",
                             headers=self.headers)
        try:
            # common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info', message_text= {"imvdb Info Status":
            #                                                      resp.status_code, 'json': resp.json()})
            return resp.json()
        except:
            return None

    async def com_imvdb_search_video(self, artist_name, song_title):
        """
        Search for video by band name and song title
        """
        resp = requests.post(self.base_api_url + "/search/videos?q="
                             + (artist_name.replace(' ', '+') + '+'
                                + song_title.replace(' ', '+')),
                             headers=self.headers)
        try:
            # common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info', message_text= {"imvdb Video Status":
            #                                                      resp.status_code, 'json': resp.json()})
            return resp.json()
        except:
            return None

    async def com_imvdb_search_entities(self, artist_name):
        """
        Search by band name
        """
        resp = requests.post(self.base_api_url + "/search/entities?q="
                             + artist_name.replace(' ', '+'), headers=self.headers)
        try:
            # common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info', message_text= {"imvdb Entities Status":
            #                                                      resp.status_code, 'json': resp.json()})
            return resp.json()
        except:
            return None


async def movie_fetch_save_imvdb(db_connection, imvdb_id, metadata_uuid):
    """
    # fetch from imvdb
    """
    # fetch and save json data via tmdb id
    result_json = await common_global.api_instance.com_imvdb_video_info(imvdb_id)
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         "meta imvdb code": result_json.status_code})
    if result_json.status_code == 200:
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "meta imvdb save fetch result":
                                                                                 result_json.json()})
        # set and insert the record
        await db_connection.db_meta_music_video_add(metadata_uuid,
                                                    {'imvdb': str(result_json['id'])},
                                                    result_json['artists'][0]['slug'],
                                                    result_json['song_slug'],
                                                    result_json,
                                                    None)
    elif result_json.status_code == 502:
        time.sleep(300)
        # redo fetch due to 502
        await movie_fetch_save_imvdb(db_connection, imvdb_id, metadata_uuid)
    elif result_json.status_code == 404:
        # TODO handle 404's better
        metadata_uuid = None
    else:  # is this is None....
        metadata_uuid = None
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'meta imvdb save fetch uuid':
                                                                             metadata_uuid})
    return metadata_uuid

 */