#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// http://dev-guide.pitchfork.com/docs.html

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

/*

class CommonMetadataPitchfork:
    """
    Class for interfacing with pitchfork
    """

    def __init__(self):
        self.pitchfork_api = None

    pub async fn com_pitchfork_search(self, artist_name, album_title):
        """
        Search via name and title
        """
        self.pitchfork_api = pitchfork.search(artist_name, album_title)

    pub async fn com_pitchfork_album_title(self):
        """
        Album title
        """
        return self.pitchfork_api.album()

    pub async fn com_pitchfork_album_label(self):
        """
        Album label
        """
        return self.pitchfork_api.label()

    pub async fn com_pitchfork_album_review(self):
        """
        Album review
        """
        return self.pitchfork_api.editorial()

    pub async fn com_pitchfork_album_cover_art_link(self):
        """
        Get album coverart link
        """
        return self.pitchfork_api.cover()

    pub async fn com_pitchfork_album_review_score(self):
        """
        Get review score
        return self.pitchfork_api.score()

 */