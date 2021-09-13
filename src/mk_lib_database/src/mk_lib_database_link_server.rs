
/*
async def db_link_delete(self, sync_guid, db_connection=None):
    """
    Delete server link
    """
    await db_conn.execute('delete from mm_link where mm_link_guid = $1', sync_guid)


async def db_link_list(self, offset=0, records=None, search_value=None, db_connection=None):
    """
    Return list of linked server
    Complete list for admins
    """
    if search_value is not None:
        return await db_conn.fetch('select mm_link_guid,'
                                   ' mm_link_name,'
                                   ' mm_link_json'
                                   ' from mm_link'
                                   ' where mm_link_guid in (select mm_link_guid'
                                   ' from mm_link'
                                   ' where mm_link_name % $1 offset $2 limit $3)',
                                   search_value, offset, records)
    else:
        return await db_conn.fetch('select mm_link_guid,'
                                   ' mm_link_name,'
                                   ' mm_link_json'
                                   ' from mm_link'
                                   ' where mm_link_guid in (select mm_link_guid'
                                   ' from mm_link'
                                   ' offset $1 limit $2)',
                                   offset, records)


async def db_link_list_count(self, search_value=None, db_connection=None):
    """
    Return count of linked servers
    """
    if search_value is not None:
        return await db_conn.fetchval('select count(*)'
                                      ' from mm_link where mm_link_name % $1',
                                      search_value)
    else:
        return await db_conn.fetchval('select count(*) from mm_link')


async def db_link_insert(self, link_json, db_connection=None):
    """
    Insert linked server
    """
    new_guid = uuid.uuid4()
    await db_conn.execute('insert into mm_link (mm_link_guid,'
                          ' mm_link_json)'
                          ' values (%s, %s)', new_guid, link_json)
    await db_conn.db_commit()
    return new_guid

 */