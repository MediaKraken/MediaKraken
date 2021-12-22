/*
async def com_meta_chart_lyrics(artist_name, song_name):
    """
    Generate url link and fetch lyrics
    """
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'function':
                                                                             inspect.stack()[0][3],
                                                                         'locals': locals(),
                                                                         'caller':
                                                                             inspect.stack()[1][3]})
    lyric_text = urllib.request.urlopen('http://api.chartlyrics.com/apiv1.asmx/SearchLyricDirect?%s'
                                        % urllib.parse.urlencode(
        {'artist': artist_name, 'song': song_name})).read()
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={'stuff': lyric_text})
    return lyric_text

# com_meta_chart_lyrics('Megadeath','Peace Sells')
# com_meta_chart_lyrics('Metallica','ride the lightning')
*/