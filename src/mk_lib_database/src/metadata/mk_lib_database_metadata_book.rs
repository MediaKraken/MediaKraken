use uuid::Uuid;
use sqlx::postgres::PgRow;

pub async fn mk_lib_database_metadata_book_by_uuid(pool: &sqlx::PgPool, book_uuid: uuid::Uuid)
                                                   -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_metadata_book_json from mm_metadata_book \
        where mm_metadata_book_guid = $1")
        .bind(book_uuid)
        .fetch_one(pool)
        .await?;
    Ok(rows)
}

/*

async def db_meta_periodical_list(self, offset=0, records=None, search_value=None,
                                  db_connection=None):
    """
    periodical list
    """
    # TODO sort by release date
    if search_value is not None:
        return await db_conn.fetch('select mm_metadata_book_guid,'
                                   ' mm_metadata_book_name'
                                   ' from mm_metadata_book'
                                   ' where mm_metadata_book_name % $1'
                                   ' order by mm_metadata_book_name'
                                   ' offset $2 limit $3',
                                   search_value,
                                   offset, records)
    else:
        return await db_conn.fetch('select mm_metadata_book_guid,'
                                   ' mm_metadata_book_name'
                                   ' from mm_metadata_book'
                                   ' order by mm_metadata_book_name'
                                   ' offset $1 limit $2',
                                   offset, records)


async def db_meta_periodical_list_count(self, search_value=None, db_connection=None):
    """
    periodical list count
    """
    if search_value is not None:
        return await db_conn.fetchval('select count(*)'
                                      ' from mm_metadata_book'
                                      ' where mm_metadata_book_name % $1',
                                      search_value, )
    else:
        return await db_conn.fetchval('select count(*) from mm_metadata_book')

 */