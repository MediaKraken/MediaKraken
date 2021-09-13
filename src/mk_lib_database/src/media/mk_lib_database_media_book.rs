use uuid::Uuid;
use sqlx::postgres::PgRow;

/*

async def db_media_book_list(self, offset=0, records=None, search_value=None, db_connection=None):
    if search_value is not None:
        return await db_conn.fetch('select mm_metadata_book_guid,'
                                   ' mm_metadata_book_name'
                                   ' from mm_metadata_book, mm_media'
                                   ' where mm_media_metadata_guid'
                                   ' = mm_metadata_book_guid '
                                   ' and mm_metadata_book_name % $1'
                                   ' order by LOWER(mm_metadata_book_name)'
                                   ' offset $2 limit $3',
                                   search_value, offset, records)
    else:
        return await db_conn.fetch('select mm_metadata_book_guid,'
                                   ' mm_metadata_book_name'
                                   ' from mm_metadata_book, mm_media'
                                   ' where mm_media_metadata_guid'
                                   ' = mm_metadata_book_guid'
                                   ' order by LOWER(mm_metadata_book_name)'
                                   ' offset $1 limit $2',
                                   offset, records)

 */

pub async fn mk_lib_database_media_book_count(pool: &sqlx::PgPool,
                                              search_value: String)
                                              -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_book, \
            mm_media where mm_media_metadata_guid = mm_metadata_book_guid \
            and mm_metadata_book_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_book, \
            mm_media where mm_media_metadata_guid = mm_metadata_book_guid")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}