use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use sqlx::{types::Uuid, types::Json};
use rocket_dyn_templates::serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicVideoList {
	mm_metadata_music_video_guid: uuid::Uuid,
	mm_media_music_video_band: String,
	mm_media_music_video_song: String,
	mm_metadata_music_video_localimage_json: String,
}

pub async fn mk_lib_database_metadata_music_video_read(pool: &sqlx::PgPool,
                                                       search_value: String,
                                                       offset: i32, limit: i32)
                                                       -> Result<Vec<PgRow>, sqlx::Error> {
    if search_value != "" {
        let rows = sqlx::query("select mm_metadata_music_video_guid, mm_media_music_video_band, \
            mm_media_music_video_song, mm_metadata_music_video_localimage_json \
            from mm_metadata_music_video where mm_media_music_video_song % $1 \
            order by LOWER(mm_media_music_video_band), LOWER(mm_media_music_video_song) \
            offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    } else {
        let rows = sqlx::query("select mm_metadata_music_video_guid, mm_media_music_video_band, \
            mm_media_music_video_song, mm_metadata_music_video_localimage_json \
            from mm_metadata_music_video order by LOWER(mm_media_music_video_band), \
            LOWER(mm_media_music_video_song) offset $1 limit $2")
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

/*

// TODO port query
async def db_meta_music_video_lookup(self, artist_name, song_title, db_connection=None):
    """
    # query to see if song is in local DB
    """
    return await db_conn.fetch('select mm_metadata_music_video_guid from mm_metadata_music_video'
                               ' where lower(mm_media_music_video_band) = $1'
                               ' and lower(mm_media_music_video_song) = $2',
                               artist_name.lower(), song_title.lower())


// TODO port query
async def db_meta_music_video_add(self, new_guid, artist_name, artist_song, id_json,
                                  data_json, image_json, db_connection=None):
    """
    Add metadata for music video
    """
    await db_conn.execute('insert into mm_metadata_music_video (mm_metadata_music_video_guid,'
                          ' mm_metadata_music_video_media_id,'
                          ' mm_media_music_video_band,'
                          ' mm_media_music_video_song,'
                          ' mm_metadata_music_video_json,'
                          ' mm_metadata_music_video_localimage_json)'
                          ' values ($1,$2,$3,$4,$5,$6)',
                          new_guid, id_json, artist_name, artist_song, data_json, image_json)
    await db_conn.db_commit()
    return new_guid


// TODO port query
async def db_meta_music_video_count(self, imvdb_id=None, search_value=None, db_connection=None):
    """
    Return count of music video metadata
    """
    if imvdb_id is None:
        if search_value is not None:
            return await db_conn.fetchval('select count(*) from mm_metadata_music_video'
                                          ' where mm_media_music_video_song % $1',
                                          search_value)
        else:
            return await db_conn.fetchval(
                'select count(*) from mm_metadata_music_video')
    else:
        return await db_conn.fetchval('select count(*) from mm_metadata_music_video'
                                      ' where mm_metadata_music_video_media_id->\'imvdb\' ? $1',
                                      imvdb_id)

 */

pub async fn mk_lib_database_meta_music_video_detail_uuid(pool: &sqlx::PgPool,
                                                          music_video_uuid: uuid::Uuid)
                                                          -> Result<PgRow, sqlx::Error> {
    let row: PgRow = sqlx::query("select mm_media_music_video_band, \
        mm_media_music_video_song, mm_metadata_music_video_json, \
        mm_metadata_music_video_localimage_json from mm_metadata_music_video \
        where mm_metadata_music_video_guid = $1")
        .bind(music_video_uuid)
        .fetch_one(pool)
        .await?;
    Ok(row)
}