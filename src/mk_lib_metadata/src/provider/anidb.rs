// https://anidb.net/

use mk_lib_network;

pub async fn provider_anidb_fetch_titles_file() {
    mk_lib_network::mk_lib_network::mk_download_file_from_url(
        "http://anidb.net/api/anime-titles.xml.gz".to_string(),
        &"/mediakraken/cache/anidb_titles.gz".to_string(),
    )
    .await
    .unwrap();
}

/*

class CommonMetadataANIdb:
    pub async fn com_net_anidb_save_title_data_to_db(self, title_file='./cache/anidb_titles.gz'):
        """
        Save anidb title data to database
        """
        file_handle = gzip.open(title_file, 'rb')
        anime_aid = None
        anime_title = None
        anime_title_ja = None
        for file_line in file_handle.readlines():
            if file_line.decode('utf-8').find('<anime aid="') != -1:
                anime_aid = file_line.decode(
                    'utf-8').split('"', 1)[1].rsplit('"', 1)[0]
            else if file_line.decode('utf-8').find('title xml:lang="ja"') != -1:
                anime_title_ja = file_line.decode(
                    'utf-8').split('>', 1)[1].rsplit('<', 1)[0]
            else if file_line.decode('utf-8').find('title xml:lang="en"') != -1:
                anime_title = file_line.decode(
                    'utf-8').split('>', 1)[1].rsplit('<', 1)[0]
            else if file_line.decode('utf-8').find('</anime>') != -1:
                if self.db_connection.db_meta_anime_meta_by_id(anime_aid) is None:
                    if anime_title is None:
                        anime_title = anime_title_ja
                    self.db_connection.db_meta_anime_title_insert(
                        {'anidb': anime_aid}, anime_title,
                        None, None, None, None, None)
                // reset each time to handle ja when this doesn't exist
                anime_title = None
        file_handle.close()
        common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                             message_text={'stuff': 'end'})

    pub async fn com_net_anidb_aid_by_title(self, title_to_search):
        """
        Find AID by title
        """
        // check the local DB
        local_db_result = self.db_connection.db_meta_anime_title_search(
            title_to_search)
        if local_db_result is None:
            # check to see if local titles file is older than 24 hours
            if self.com_net_anidb_fetch_titles_file():
                # since new titles file....recheck by title
                self.com_net_anidb_aid_by_title(title_to_search)
            else:
                return None
        else:
            return local_db_result
 */
