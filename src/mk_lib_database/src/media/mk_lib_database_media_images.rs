

/*

async def db_image_count(self, class_guid, search_value=None, db_connection=None):
    """
    Image list count
    """
    return await db_conn.fetchval('select count(*) from mm_media'
                                  ' where mm_media_class_guid = $1', class_guid)


async def db_image_list(self, class_guid, offset=0, records=None, search_value=None,
                        db_connection=None):
    """
    Image list
    """
    return await db_conn.fetch('select mm_media_path from mm_media'
                               ' where mm_media_class_guid = $1 offset $2 limit $3',
                               class_guid, offset, records)

 */