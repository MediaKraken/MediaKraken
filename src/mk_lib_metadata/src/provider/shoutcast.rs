// https://directory.shoutcast.com/Developer

use mk_lib_network;

const SHOUTCAST_API_URL: &str = "http://api.shoutcast.com/legacy/";

pub async fn mk_provider_shoutcast_top500(
    api_key: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}Top500?k={}", SHOUTCAST_API_URL, api_key);
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(url)
        .await
        .unwrap();
    Ok(json_data)
}

pub async fn mk_provider_shoutcast_genre_list(
    api_key: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}genrelist?k={}", SHOUTCAST_API_URL, api_key);
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(url)
        .await
        .unwrap();
    Ok(json_data)
}

/*
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

*/
