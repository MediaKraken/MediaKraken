// https://isbndb.com/apidocs/v2

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

/*

class CommonMetadataISBNdb:
    """
    Class for interfacing with isbndb
    """

    def __init__(self, option_config_json):
        self.api_key = option_config_json['API']['isbndb']

    # http://isbndb.com/api/v2/docs/authors
    # http://isbndb.com/api/v2/json/[your-api-key]/author/richards_rowland
    async def com_isbndb_author(self, author_name):
        """
        Grab the author
        """
        return await common_network_async.mk_network_fetch_from_url_async(
            'http://isbndb.com/api/v2/json/'
            + self.api_key + '/author/' + author_name,
            None)

    # http://isbndb.com/api/v2/xml/mykey/books?q=science

    # http://isbndb.com/api/v2/docs/publishers
    # http://isbndb.com/api/v2/json/[your-api-key]/publisher/chapman_hall_crc
    async def com_isbndb_publisher(self, publisher_name):
        """
        Grab the publisher
        """
        return await common_network_async.mk_network_fetch_from_url_async(
            'http://isbndb.com/api/v2/json/'
            + self.api_key + '/publisher/'
            + publisher_name, None)

    # http://isbndb.com/api/v2/docs/subjects
    # http://isbndb.com/api/v2/json/[your-api-key]/subject/brain_diseases_diagnosis

    # http://isbndb.com/api/v2/docs/categories
    # http://isbndb.com/api/v2/json/[your-api-key]/category/alphabetically.authors.r.i

    # http://isbndb.com/api/v2/docs/prices
    # http://isbndb.com/api/v2/json/[your-api-key]/prices/084930315X
    # http://isbndb.com/api/v2/json/[your-api-key]/prices/9780849303159
    # http://isbndb.com/api/v2/json/[your-api-key]/prices/principles_of_solid_mechanics
    async def com_isbndb_prices(self, book_info):
        """
        Grab prices
        """
        return await common_network_async.mk_network_fetch_from_url_async(
            'http://isbndb.com/api/v2/json/'
            + self.api_key + '/prices/'
            + book_info, None)

    # http://isbndb.com/api/v2/docs/books
    # http://isbndb.com/api/v2/json/[your-api-key]/book/084930315X
    # http://isbndb.com/api/v2/json/[your-api-key]/book/9780849303159
    # http://isbndb.com/api/v2/json/[your-api-key]/book/principles_of_solid_mechanics
    async def com_isbndb_books(self, book_info):
        """
        Search
        """
        return requests.get('http://isbndb.com/api/v2/json/'
                            + self.api_key + '/book/' + book_info, timeout=5)


async def metadata_periodicals_search_isbndb(db_connection, lookup_name):
    """
    search isbndb
    """
    metadata_uuid = None
    if common_global.api_instance != None:
        api_response = await common_global.api_instance.com_isbndb_books(lookup_name)
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'response': api_response})
        if api_response.status_code == 200:
            // TODO verify decent results before insert
            await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                             message_text={
                                                                                 'resp json': api_response.json()})
            if 'error' in api_response.json():
                await common_logging_elasticsearch_httpx.com_es_httpx_post_async(
                    message_type='info',
                    message_text={
                        'stuff': 'error skipp'})
            else:
                metadata_uuid = await db_connection.db_meta_book_insert(api_response.json())
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'meta book uuid': metadata_uuid})
    return metadata_uuid

 */