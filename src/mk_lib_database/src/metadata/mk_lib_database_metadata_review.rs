
/*

async def db_review_list_by_tmdb_guid(self, metadata_id, db_connection=None):
    """
    # grab reviews for metadata
    """
    # TODO order by release date
    # TODO order by rating? (optional?)
    return await db_conn.fetch('select mm_review_guid,'
                               ' mm_review_json'
                               ' from mm_review'
                               ' where mm_review_metadata_id = $1',
                               str(metadata_id))

async def db_review_count(self, metadata_id, db_connection=None):
    """
    # count reviews for media
    """
    return await db_conn.fetchval('select count(*) from mm_review'
                                  ' where mm_review_metadata_guid = $1',
                                  metadata_id)


async def db_review_list_by_meta_guid(self, metadata_id, db_connection=None):
    """
    # grab reviews for metadata
    """
    # TODO order by release date
    # TODO order by rating? (optional?)
    return await db_conn.fetch('select mm_review_guid,'
                               'mm_review_json'
                               ' from mm_review'
                               ' where mm_review_metadata_guid = $1',
                               metadata_id)


async def db_review_insert(self, metadata_guid, review_json, db_connection=None):
    """
    # insert record
    """
    new_guid = uuid.uuid4()
    await self.db_cursor.execute('insert into mm_review (mm_review_guid,'
                                 ' mm_review_metadata_guid,'
                                 ' mm_review_json)'
                                 ' values ($1,$2,$3)',
                                 new_guid, metadata_guid, review_json)
    await self.db_cursor.db_commit()
    return new_guid

 */