#![cfg_attr(debug_assertions, allow(dead_code))]

// https://anidb.net/

use serde_json::json;
use std::error::Error;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

/*

class CommonMetadataANIdb:
    """
    Class for interfacing with anidb
    """

    def __init__(self, db_connection):
        self.adba_connection = None

    pub async fn com_net_anidb_fetch_titles_file(self):
        """
        Fetch the tarball of anime titles
        """
        // check to see if local titles file is older than 24 hours
        if common_file.com_file_modification_timestamp('./cache/anidb_titles.gz') \
                < (time.time() - 86400):
            await common_network_async.mk_network_fetch_from_url_async(
                'http://anidb.net/api/anime-titles.xml.gz',
                './cache/anidb_titles.gz')
            return True  # new file
        return False

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

    pub async fn com_net_anidb_connect(self, user_name, user_password):
        """
        Remote api calls
        """
        self.adba_connection = adba.Connection(log=True)
        try:
            self.adba_connection.auth(user_name, user_password)
        except Exception as err_code:
            common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='error',
                                                                 message_text={"exception msg":
                                                                                   err_code})
        return self.adba_connection

    pub async fn com_net_anidb_logout(self):
        """
        Logout of anidb
        """
        self.adba_connection.logout()

    pub async fn com_net_anidb_stop(self):
        """
        Close the anidb connect and stop the thread
        """
        self.adba_connection.stop()

 */
