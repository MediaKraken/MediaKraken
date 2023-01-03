#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::error::Error;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

/*

class CommonMetadataIMDB:
    """
    Class for interfacing with imdb
    """

    def __init__(self, cache=True, cache_dir=None):
        // open connection to imdb
        if cache != None:
            if cache_dir != None:
                self.imdb = Imdb(cache=True, cache_dir=cache_dir)
            else:
                self.imdb = Imdb(cache=True)
        else:
            self.imdb = Imdb()

    pub async fn com_imdb_title_search(self, media_title):
        """
        # fetch info from title
        """
        return self.imdb.search_for_title(media_title)

    pub async fn com_imdb_id_search(self, media_id):
        """
        # fetch info by ttid
        """
        return self.imdb.get_title_by_id(media_id)

    pub async fn com_imdb_person_by_id(self, person_id):
        """
        # fetch person info by id
        """
        return self.imdb.get_person_by_id(person_id)

    pub async fn com_imdb_person_images_by_id(self, person_id):
        """
        # fetch person images by id
        """
        return self.imdb.get_person_images(person_id)

    pub async fn com_imdb_title_review_by_id(self, media_id):
        """
        # fetch the title review
        """
        return self.imdb.get_title_reviews(media_id)

 */
