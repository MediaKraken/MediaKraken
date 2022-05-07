#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://www.omdbapi.com/

use std::error::Error;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

/*

class CommonMetadataOMDB:
    """
    Class for interfacing with omdb
    """
    pub async fn com_omdb_get(self, media_title, media_year, media_fullplot, media_tomatoes):
        omdb.get(title=media_title, year=media_year, fullplot=media_fullplot,
                 tomatoes=media_tomatoes)

    pub async fn com_omdb_search(self, media_title):
        """
        Search
        """
        omdb.search(media_title)

    pub async fn com_omdb_search_movie(self, media_title):
        """
        Search movie
        """
        omdb.search_movie(media_title)

    pub async fn com_omdb_search_episode(self, media_title):
        """
        Search episode
        """
        omdb.search_episode(media_title)

    pub async fn com_omdb_search_series(self, media_title):
        """
        Search series
        """
        omdb.search_series(media_title)

    pub async fn com_omdb_imdb(self, imdbid):
        """
        Info by IMDB id
        """
        omdb.imdbid(imdbid)

    pub async fn com_omdb_title(self, media_title):
        """
        Grab by title
        """
        omdb.title(media_title)

    pub async fn com_omdb_default(self):
        """
        Set defaults for data returned
        """
        omdb.set_default('tomatoes', True)

    pub async fn com_omdb_request(self, media_title, media_year, media_fullplot, media_tomatoes):
        omdb.request(media_title, media_year, media_fullplot, media_tomatoes)

 */