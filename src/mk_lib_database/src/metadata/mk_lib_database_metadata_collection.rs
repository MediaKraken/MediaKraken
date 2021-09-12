use uuid::Uuid;
use sqlx::postgres::PgRow;
/*
async def db_collection_list(self, offset=None, records=None, search_value=None,
                             db_connection=None):
    """
    Return collections list from the database
    """
    if offset is None:
        if search_value is not None:
            return await db_conn.fetch('select mm_metadata_collection_guid,'
                                       ' mm_metadata_collection_name,'
                                       ' mm_metadata_collection_imagelocal_json'
                                       ' from mm_metadata_collection'
                                       ' where mm_metadata_collection_name % $1'
                                       ' order by mm_metadata_collection_name',
                                       search_value)
        else:
            return await db_conn.fetch('select mm_metadata_collection_guid,'
                                       ' mm_metadata_collection_name,'
                                       ' mm_metadata_collection_imagelocal_json'
                                       ' from mm_metadata_collection'
                                       ' order by mm_metadata_collection_name')
    else:
        if search_value is not None:
            return await db_conn.fetch('select mm_metadata_collection_guid,'
                                       ' mm_metadata_collection_name,'
                                       ' mm_metadata_collection_imagelocal_json'
                                       ' from mm_metadata_collection'
                                       ' where mm_metadata_collection_guid'
                                       ' in (select mm_metadata_collection_guid'
                                       ' from mm_metadata_collection'
                                       ' where mm_metadata_collection_name % $1'
                                       ' order by mm_metadata_collection_name'
                                       ' offset $2 limit $3)'
                                       ' order by mm_metadata_collection_name',
                                       search_value, offset, records)
        else:
            return await db_conn.fetch('select mm_metadata_collection_guid,'
                                       ' mm_metadata_collection_name,'
                                       ' mm_metadata_collection_imagelocal_json'
                                       ' from mm_metadata_collection'
                                       ' where mm_metadata_collection_guid'
                                       ' in (select mm_metadata_collection_guid'
                                       ' from mm_metadata_collection'
                                       ' order by mm_metadata_collection_name'
                                       ' offset $1 limit $2) '
                                       'order by mm_metadata_collection_name',
                                       offset, records)


async def db_collection_list_count(self, search_value=None, db_connection=None):
    if search_value is not None:
        return await db_conn.fetchval('select count(*)'
                                      ' from mm_metadata_collection'
                                      ' where mm_metadata_collection_name = $1',
                                      search_value)
    else:
        return await db_conn.fetchval('select count(*)'
                                      ' from mm_metadata_collection')


async def db_collection_read_by_guid(self, media_uuid, db_connection=None):
    """
    Collection details
    """
    return await db_conn.fetchrow('select mm_metadata_collection_json,'
                                  ' mm_metadata_collection_imagelocal_json'
                                  ' from mm_metadata_collection'
                                  ' where mm_metadata_collection_guid = $1',
                                  media_uuid)


async def db_media_collection_scan(self, db_connection=None):
    """
    Returns a list of movies that belong in a collection specifified by tmdb
    """
    return await db_conn.fetch('select mm_metadata_guid, mm_metadata_json'
                               ' from mm_metadata_movie'
                               ' where mm_metadata_json->\'belongs_to_collection\'::text'
                               ' <> \'{}\'::text'
                               ' order by mm_metadata_json->\'belongs_to_collection\'')


async def db_collection_guid_by_name(self, collection_name, db_connection=None):
    """
    Return uuid from collection name
    """
    return await db_conn.fetchval(
        'select mm_metadata_collection_guid from mm_metadata_collection'
        ' where mm_metadata_collection_name->>\'name\' = $1',
        collection_name)


async def db_collection_by_tmdb(self, tmdb_id, db_connection=None):
    """
    Return uuid via tmdb id
    """
    return await db_conn.fetchval(
        'select mm_metadata_collection_guid from mm_metadata_collection'
        ' where mm_metadata_collection_json @> \'{"id":$1}\'', tmdb_id)


async def db_collection_insert(self, collection_name, guid_json, metadata_json,
                               localimage_json, db_connection=None):
    """
    Insert collection into the database
    """
    new_guid = uuid.uuid4()
    await db_conn.execute(
        'insert into mm_metadata_collection (mm_metadata_collection_guid,'
        ' mm_metadata_collection_name, mm_metadata_collection_media_ids,'
        ' mm_metadata_collection_json, mm_metadata_collection_imagelocal_json)'
        ' values ($1,$2,$3,$4,$5)', new_guid, json.dumps(collection_name),
        json.dumps(guid_json),
        json.dumps(metadata_json),
        json.dumps(localimage_json))
    return new_guid


async def db_collection_update(self, collection_guid, guid_json, db_connection=None):
    """
    Update the ids listed within a collection
    """
    await db_conn.execute('update mm_metadata_collection'
                          ' set mm_metadata_collection_media_ids = $1,'
                          ' mm_metadata_collection_json = $2'
                          ' where mm_metadata_collection_guid = $3',
                          TODOfield, json.dumps(guid_json), collection_guid)

 */