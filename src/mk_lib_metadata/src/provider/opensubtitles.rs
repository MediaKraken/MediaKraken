/*

class CommonMetadataOpenSubtitles:
    """
    Class for interfacing with Opensubtitles
    """

    def __init__(self, user_name, user_password):
        self.opensubtitles_inst = OpenSubtitles()
        self.token = self.opensubtitles_inst.login(user_name, user_password)

    async def com_meta_opensub_search(self, file_name):
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        f = File(file_name)
        return self.opensubtitles_inst.search_subtitles([{'sublanguageid': 'all',
                                                          'moviehash': f.get_hash(),
                                                          'moviebytesize': f.size}])

    async def com_meta_opensub_ping(self):
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        self.opensubtitles_inst.no_operation()

    async def com_meta_opensub_logoff(self):
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        self.opensubtitles_inst.logout()

 */