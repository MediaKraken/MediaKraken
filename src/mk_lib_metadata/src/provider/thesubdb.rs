/*

class CommonMetadataTheSubDB:
    url = 'http://api.thesubdb.com/'
    headers = {
        "User-Agent": ("MediaKraken/1.0 "
                       "(MediaKraken/%s; http://www.mediakraken.org)"
                       % common_version.APP_VERSION)
    }

    async def com_meta_thesubdb_search(self, filename, langs):
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        filehash = common_hash.com_hash_thesubdb(filename)
        async with httpx.AsyncClient() as client:
            response = await client.get(self.url,
                                        params={'action': 'search', 'hash': filehash},
                                        headers=self.headers,
                                        timeout=3.05)
        if response.status_code == 404:
            # no subtitle found
            return []
        subtitles = []
        for lang in response.text.splitlines()[0].split(','):
            if lang in langs:
                sublink = '%s?%s' % (self.url,
                                     urllib.parse.urlencode({'action': 'download',
                                                             'hash': filehash,
                                                             'language': lang}))
                subtitles.append({'lang': lang, 'link': sublink})
        return subtitles

    async def com_meta_thesubdb_download(self, subtitle, stream):
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        async with httpx.AsyncClient() as client:
            response = await client.get(subtitle["link"], headers=self.headers, timeout=5)
        stream.write(response.content)

 */