/*

class CommonMetadataIMDB:
    """
    Class for interfacing with imdb
    """

    def __init__(self, cache=True, cache_dir=None):
        # open connection to imdb
        if cache is not None:
            if cache_dir is not None:
                self.imdb = Imdb(cache=True, cache_dir=cache_dir)
            else:
                self.imdb = Imdb(cache=True)
        else:
            self.imdb = Imdb()

    async def com_imdb_title_search(self, media_title):
        """
        # fetch info from title
        """
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        return self.imdb.search_for_title(media_title)

    async def com_imdb_id_search(self, media_id):
        """
        # fetch info by ttid
        """
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        return self.imdb.get_title_by_id(media_id)

    async def com_imdb_person_by_id(self, person_id):
        """
        # fetch person info by id
        """
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        return self.imdb.get_person_by_id(person_id)

    async def com_imdb_person_images_by_id(self, person_id):
        """
        # fetch person images by id
        """
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        return self.imdb.get_person_images(person_id)

    async def com_imdb_title_review_by_id(self, media_id):
        """
        # fetch the title review
        """
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'function':
                                                                                 inspect.stack()[0][
                                                                                     3],
                                                                             'locals': locals(),
                                                                             'caller':
                                                                                 inspect.stack()[1][
                                                                                     3]})
        return self.imdb.get_title_reviews(media_id)

 */