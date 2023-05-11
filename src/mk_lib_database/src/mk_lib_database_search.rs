








/*
// TODO port query
def db_search(self, search_string, search_type='Local', search_movie=True, search_tvshow=True,
              search_album=True, search_image=True, search_publication=True, search_game=True):
    json_return_data = {}
    if search_type == "Local":
        if search_movie == true:
            // movie section
            self.db_cursor.execute('SELECT mm_metadata_guid,'
                                   ' mm_metadata_name, '
                                   'similarity(mm_metadata_name, $1) AS sml'
                                   ' FROM mm_metadata_movie'
                                   ' WHERE mm_metadata_name % $2'
                                   ' ORDER BY sml DESC, LOWER(mm_metadata_name);',
                                   (search_string, search_string))
            json_return_data['Movie'] = json.dumps(self.db_cursor.fetchall())
        if search_tvshow == true:
            // tv show section
            self.db_cursor.execute('SELECT mm_metadata_tvshow_guid,'
                                   ' mm_metadata_tvshow_name,'
                                   ' similarity(mm_metadata_tvshow_name, $1) AS sml'
                                   ' FROM mm_metadata_tvshow'
                                   ' WHERE mm_metadata_tvshow_name % $2'
                                   ' ORDER BY sml DESC, LOWER(mm_metadata_tvshow_name);',
                                   (search_string, search_string))
            json_return_data['TVShow'] = json.dumps(self.db_cursor.fetchall())
        if search_album == true:
            // album section
            self.db_cursor.execute('SELECT mm_metadata_album_guid,'
                                   ' mm_metadata_album_name,'
                                   ' similarity(mm_metadata_album_name, $1) AS sml'
                                   ' FROM mm_metadata_album'
                                   ' WHERE mm_metadata_album_name % $2'
                                   ' ORDER BY sml DESC, LOWER(mm_metadata_album_name);',
                                   (search_string, search_string))
            json_return_data['Album'] = json.dumps(self.db_cursor.fetchall())
        if search_image == true:
            // TODO image search
            pass
        if search_publication == true:
            // publication section
            self.db_cursor.execute('SELECT mm_metadata_book_guid,'
                                   ' mm_metadata_book_name,'
                                   ' similarity(mm_metadata_book_name, $1) AS sml'
                                   ' FROM mm_metadata_book'
                                   ' WHERE mm_metadata_book_name % $2'
                                   ' ORDER BY sml DESC, LOWER(mm_metadata_book_name);',
                                   (search_string, search_string))
            json_return_data['Publication'] = json.dumps(self.db_cursor.fetchall())
        if search_game == true:
            // game section
            self.db_cursor.execute('SELECT gi_id,'
                                   ' gi_game_info_name,'
                                   ' similarity(gi_game_info_name, $1) AS sml'
                                   ' FROM mm_metadata_game_software_info'
                                   ' WHERE gi_game_info_name % $2'
                                   ' ORDER BY sml DESC, LOWER(gi_game_info_name);',
                                   (search_string, search_string))
            json_return_data['Game'] = json.dumps(self.db_cursor.fetchall())
    return json_return_data
 */
