#![cfg_attr(debug_assertions, allow(dead_code))]

// http://www.chartlyrics.com/api.aspx

use serde_json::json;
use std::error::Error;
use stdext::function_name;

use crate::mk_lib_logging;

use crate::mk_lib_network;

pub async fn provider_chart_lyrics_fetch(
    sqlx_pool: &sqlx::PgPool,
    artist_name: String,
    song_name: String,
) {
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
