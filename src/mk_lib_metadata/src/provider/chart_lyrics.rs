// http://www.chartlyrics.com/api.aspx

use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;

pub async fn provider_chart_lyrics_fetch(_artist_name: String, _song_name: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
}

/*
pub async fn com_meta_chart_lyrics(artist_name, song_name):
    """
    Generate url link and fetch lyrics
    """
    lyric_text = urllib.request.urlopen('http://api.chartlyrics.com/apiv1.asmx/SearchLyricDirect?%s'
                                        % urllib.parse.urlencode(
        {'artist': artist_name, 'song': song_name})).read()
    return lyric_text

# com_meta_chart_lyrics('Megadeath','Peace Sells')
# com_meta_chart_lyrics('Metallica','ride the lightning')
*/
