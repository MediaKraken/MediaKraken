

/*

async def db_media_album_count(self, search_value=None, db_connection=None):
    """
    Album count
    """
    if search_value is not None:
        return await db_conn.fetchval('select count(*) from mm_metadata_album, mm_media'
                                      ' where mm_media_metadata_guid'
                                      ' = mm_metadata_album_guid '
                                      ' and mm_metadata_album_name % $1',
                                      search_value)
    else:
        return await db_conn.fetchval(
            'select count(*) from (select distinct mm_metadata_album_guid'
            ' from mm_metadata_album, mm_media'
            ' where mm_media_metadata_guid = mm_metadata_album_guid) as temp')


async def db_media_album_list(self, offset=0, per_page=None, search_value=None, db_connection=None):
    """
    Album list
    """
    # TODO only grab the image part of the json for list, might want runtime, etc as well
    if search_value is not None:
        return await db_conn.fetch('select mm_metadata_album_guid,'
                                   ' mm_metadata_album_name,'
                                   ' mm_metadata_album_json'
                                   ' from mm_metadata_album, mm_media'
                                   ' where mm_media_metadata_guid'
                                   ' = mm_metadata_album_guid'
                                   ' and mm_metadata_album_name % $1'
                                   ' group by mm_metadata_album_guid'
                                   ' order by LOWER(mm_metadata_album_name)'
                                   ' offset $2 limit $3',
                                   search_value, offset, per_page)
    else:
        return await db_conn.fetch('select mm_metadata_album_guid,'
                                   ' mm_metadata_album_name,'
                                   ' mm_metadata_album_json'
                                   ' from mm_metadata_album, mm_media'
                                   ' where mm_media_metadata_guid'
                                   ' = mm_metadata_album_guid'
                                   ' group by mm_metadata_album_guid'
                                   ' order by LOWER(mm_metadata_album_name)'
                                   ' offset $1 limit $2',
                                   offset, per_page)

 */