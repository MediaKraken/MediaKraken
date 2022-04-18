#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// http://www.chartlyrics.com/api.aspx

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

pub async fn provider_chart_lyrics_fetch(pool: &sqlx::PgPool,
                                         artist_name: String,
                                         song_name: String) {}

/*
async def com_meta_chart_lyrics(artist_name, song_name):
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