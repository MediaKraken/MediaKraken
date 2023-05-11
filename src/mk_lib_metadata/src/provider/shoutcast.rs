// https://directory.shoutcast.com/Developer







/*

class CommonMetadataShoutcast:
    """
    Class for interfacing with Shoutcast
    """

    def __init__(self, option_config_json):
        self.shoutcast_api_key = option_config_json['API']['shoutcast']
        self.shoutcast_url = 'http://api.shoutcast.com/legacy/'

    pub async fn com_shoutcast_generate_options(self, rec_limit=None, bit_rate=None, media_type=None):
        options = ''
        if rec_limit != None:
            options += '&limit=%s' % rec_limit
        if bit_rate != None:
            options += '&br=%s' % bit_rate
        if media_type != None:
            # MP3 = audio/mpeg and AAC+ = audio/aacp
            if media_type.lower() == 'mp3':
                media_type = 'audio/mpeg'
            else:
                media_type = 'audio/aacp'
            options += '&mt=%s' % media_type
        return options

    pub async fn com_shoutcast_top_500(self, rec_limit=None, bit_rate=None, media_type=None):
        """
        Grab top 500 stations
        """
        return json.loads(await common_network_async.mk_network_fetch_from_url_async(
            self.shoutcast_url + 'Top500?k=' + self.shoutcast_api_key
            + self.com_shoutcast_generate_options(rec_limit, bit_rate, media_type), None))

    pub async fn com_shoutcast_keyword(self, search_string, rec_limit=None, bit_rate=None,
                                    media_type=None):
        """
        Grab stations by keyword
        """
        return json.loads(await common_network_async.mk_network_fetch_from_url_async(
            self.shoutcast_url + 'stationsearch?k=' + self.shoutcast_api_key
            + ('&search=%s' % search_string.replace(' ', '+'))
            + self.com_shoutcast_generate_options(rec_limit, bit_rate, media_type), None))

    pub async fn com_shoutcast_genre(self, genre_string, rec_limit=None, bit_rate=None,
                                  media_type=None):
        """
        Grab stations by genre
        """
        return json.loads(await common_network_async.mk_network_fetch_from_url_async(
            self.shoutcast_url + 'stationsearch?k=' + self.shoutcast_api_key
            + ('&genresearch=%s' % genre_string.replace(' ', '+'))
            + self.com_shoutcast_generate_options(rec_limit, bit_rate, media_type), None))

    pub async fn com_shoutcast_genre_list(self):
        """
        Grab genre list
        """
        return json.loads(await common_network_async.mk_network_fetch_from_url_async(
            self.shoutcast_url + 'genrelist?k=' + self.shoutcast_api_key, None))

// TODO
# Get Secondary Genres

 */
