use uuid::Uuid;
use sqlx::postgres::PgRow;

/*

async def db_music_video_list_count(self, search_value=None, db_connection=None):
    """
    Music video count
    """
    if search_value is not None:
        return await db_conn.fetchval('select count(*)'
                                      ' from mm_metadata_music_video, mm_media'
                                      ' where mm_media_metadata_guid'
                                      ' = mm_metadata_music_video_guid group'
                                      ' and mm_media_music_video_song % $1',
                                      search_value)
    else:
        return await db_conn.fetchval('select count(*)'
                                      ' from mm_metadata_music_video, mm_media'
                                      ' where mm_media_metadata_guid'
                                      ' = mm_metadata_music_video_guid')

 */