use uuid::Uuid;
use sqlx::postgres::PgRow;

pub async fn mk_lib_database_metadata_music_album_read(pool: &sqlx::PgPool,
                                                 search_value: String,
                                                 offset: i32, limit: i32)
                                                 -> Result<Vec<PgRow>, sqlx::Error> {
    // TODO, only grab the poster locale from json
    // TODO order by release year
    if search_value != "" {
        let rows = sqlx::query("select mm_metadata_album_guid, mm_metadata_album_name, \
            mm_metadata_album_json, mm_metadata_album_localimage \
            from mm_metadata_album where mm_metadata_album_name % $1 \
            order by LOWER(mm_metadata_album_name) \
            offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    } else {
        let rows = sqlx::query("select mm_metadata_album_guid, mm_metadata_album_name, \
            mm_metadata_album_json, mm_metadata_album_localimage \
            from mm_metadata_album order by LOWER(mm_metadata_album_name) \
            offset $1 limit $2")
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

/*
async def db_meta_music_album_by_guid(self, guid, db_connection=None):
    """
    # return album data by guid
    """
    return await db_conn.fetchrow('select * from mm_metadata_album'
                                  ' where mm_metadata_album_guid = $1',
                                  guid)


async def db_meta_music_songs_by_album_guid(self, guid, db_connection=None):
    """
    # return song list from album guid
    """
    return await db_conn.fetch('select * from mm_metadata_music'
                               ' where blah = $1'
                               ' order by lower(mm_metadata_music_name)',
                               guid)

 */