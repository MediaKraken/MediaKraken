/*

class CommonMetadataOMDB:
    """
    Class for interfacing with omdb
    """

    def __init__(self):
        pass

    async def com_omdb_get(self, media_title, media_year, media_fullplot, media_tomatoes):
        omdb.get(title=media_title, year=media_year, fullplot=media_fullplot,
                 tomatoes=media_tomatoes)

    async def com_omdb_search(self, media_title):
        """
        Search
        """
        omdb.search(media_title)

    async def com_omdb_search_movie(self, media_title):
        """
        Search movie
        """
        omdb.search_movie(media_title)

    async def com_omdb_search_episode(self, media_title):
        """
        Search episode
        """
        omdb.search_episode(media_title)

    async def com_omdb_search_series(self, media_title):
        """
        Search series
        """
        omdb.search_series(media_title)

    async def com_omdb_imdb(self, imdbid):
        """
        Info by IMDB id
        """
        omdb.imdbid(imdbid)

    async def com_omdb_title(self, media_title):
        """
        Grab by title
        """
        omdb.title(media_title)

    async def com_omdb_default(self):
        """
        Set defaults for data returned
        """
        omdb.set_default('tomatoes', True)

    async def com_omdb_request(self, media_title, media_year, media_fullplot, media_tomatoes):
        omdb.request(media_title, media_year, media_fullplot, media_tomatoes)

 */