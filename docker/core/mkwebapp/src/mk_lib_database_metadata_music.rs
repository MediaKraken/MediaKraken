#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::{types::Uuid, types::Json};
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicList {
    mm_metadata_album_guid: uuid::Uuid,
    mm_metadata_album_name: String,
    mm_metadata_album_json: serde_json::Value,
    mm_metadata_album_localimage: String,
}

pub async fn mk_lib_database_metadata_music_count(pool: &sqlx::PgPool,
                                                 search_value: String)
                                                 -> Result<i32, sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_album \
            where mm_metadata_album_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_album")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_music_read(pool: &sqlx::PgPool,
                                                 search_value: String,
                                                 offset: i32, limit: i32)
                                                 -> Result<Vec<DBMetaMusicList>, sqlx::Error> {
    // TODO, only grab the poster locale from json
    // TODO order by release year
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("select mm_metadata_album_guid, mm_metadata_album_name, \
            mm_metadata_album_json, mm_metadata_album_localimage \
            from mm_metadata_album where mm_metadata_album_name % $1 \
            order by LOWER(mm_metadata_album_name) \
            offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("select mm_metadata_album_guid, mm_metadata_album_name, \
            mm_metadata_album_json, mm_metadata_album_localimage \
            from mm_metadata_album order by LOWER(mm_metadata_album_name) \
            offset $1 limit $2")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMetaMusicList> = select_query
        .map(|row: PgRow| DBMetaMusicList {
            mm_metadata_album_guid: row.get("mm_metadata_album_guid"),
            mm_metadata_album_name: row.get("mm_metadata_album_name"),
            mm_metadata_album_json: row.get("mm_metadata_album_json"),
            mm_metadata_album_localimage: row.get("mm_metadata_album_localimage"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

/*
// TODO port query
pub async fn db_meta_music_album_by_guid(self, guid):
    """
    # return album data by guid
    """
    return await db_conn.fetchrow('select * from mm_metadata_album'
                                  ' where mm_metadata_album_guid = $1',
                                  guid)


// TODO port query
pub async fn db_meta_music_songs_by_album_guid(self, guid):
    """
    # return song list from album guid
    """
    return await db_conn.fetch('select * from mm_metadata_music'
                               ' where blah = $1'
                               ' order by lower(mm_metadata_music_name)',
                               guid)


// TODO port query
def db_music_lookup(self, artist_name, album_name, song_title):
    """
    # query to see if song is in local DB
    """
    // TODO the following fields don't exist on the database (album and musician)
    // TODO order by release year
    self.db_cursor.execute('select mm_metadata_music_guid,'
                           ' mm_metadata_media_music_id->'Mbrainz' as mbrainz from '
                           'mm_metadata_music,'
                           ' mm_metadata_album,'
                           ' mm_metadata_musician'
                           ' where lower(mm_metadata_musician_name) = $1'
                           ' and lower(mm_metadata_album_name) = $2'
                           ' and lower(mm_metadata_music_name) = $3',
                           (artist_name.lower(), album_name.lower(), song_title.lower()))
    try:
        return self.db_cursor.fetchone()
    except:
        return None


// TODO port query
def db_meta_musician_by_guid(self, guid):
    """
    # return musician data by guid
    """
    self.db_cursor.execute('select * from mm_metadata_musician'
                           ' where mm_metadata_musician_guid = $1', (guid,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None


// TODO port query
def db_meta_musician_add(self, data_name, data_id, data_json):
    """
    # insert musician
    """
    new_guid = uuid.uuid4()
    self.db_cursor.execute('insert into mm_metadata_musician (mm_metadata_musician_guid,'
                           ' mm_metadata_musician_name,'
                           ' mm_metadata_musician_id,'
                           ' mm_metadata_musician_json)'
                           ' values ($1,$2,$3,$4)',
                           (new_guid, data_name, data_id, data_json))
    self.db_commit()
    return new_guid


// TODO port query
def db_meta_album_by_guid(self, guid):
    """
    # return album data by guid
    """
    self.db_cursor.execute('select * from mm_metadata_album where mm_metadata_album_guid = $1',
                           (guid,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None


// TODO port query
def db_meta_album_add(self, data_name, data_id, data_json):
    """
    # insert album
    """
    new_guid = uuid.uuid4()
    self.db_cursor.execute('insert into mm_metadata_album (mm_metadata_album_guid,'
                           ' mm_metadata_album_name,'
                           ' mm_metadata_album_id,'
                           ' mm_metadata_album_json)'
                           ' values ($1,$2,$3,$4)',
                           (new_guid, data_name, data_id, data_json))
    self.db_commit()
    return new_guid


// TODO port query
def db_meta_song_by_guid(self, guid):
    """
    # return song data by guid
    """
    self.db_cursor.execute('select * from mm_metadata_music where mm_metadata_music_guid = $1',
                           (guid,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None


// TODO port query
def db_meta_song_add(self, data_name, data_id, data_json):
    """
    # insert song
    """
    new_guid = uuid.uuid4()
    self.db_cursor.execute('insert into mm_metadata_music (mm_metadata_music_guid,'
                           ' mm_metadata_music_name, mm_metadata_media_music_id,'
                           ' mm_metadata_music_json) values ($1,$2,$3,$4)',
                           (new_guid, data_name, data_id, data_json))
    self.db_commit()
    return new_guid


// TODO port query
def db_meta_songs_by_album_guid(self, guid):
    """
    # return song list from ablum guid
    """
    self.db_cursor.execute('select * from mm_metadata_music where blah = $1'
                           ' order by lower(mm_metadata_music_name)', (guid,))
    return self.db_cursor.fetchall()


// TODO port query
def db_meta_album_list(self, offset=0, records=None, search_value=None):
    """
    # return albums metadatalist
    """
    // TODO, only grab the poster locale from json
    // TODO order by release year
    if search_value != None:
        self.db_cursor.execute('select mm_metadata_album_guid, mm_metadata_album_name,'
                               ' mm_metadata_album_json, mm_metadata_album_localimage'
                               ' from mm_metadata_album'
                               ' where mm_metadata_album_name %% $1'
                               ' order by LOWER(mm_metadata_album_name)'
                               ' offset $2 limit $3', (search_value, offset, records))
    else:
        self.db_cursor.execute('select mm_metadata_album_guid, mm_metadata_album_name,'
                               ' mm_metadata_album_json, mm_metadata_album_localimage'
                               ' from mm_metadata_album'
                               ' order by LOWER(mm_metadata_album_name)'
                               ' offset $1 limit $2', (offset, records))
    return self.db_cursor.fetchall()


// TODO port query
def db_meta_musician_list(self, offset=0, records=None, search_value=None):
    """
    # return musician metadatalist
    """
    // TODO, only grab the poster locale from json
    if search_value != None:
        self.db_cursor.execute('select mm_metadata_musician_guid, mm_metadata_musician_name,'
                               ' mm_metadata_musician_json from mm_metadata_musician'
                               ' where mm_metadata_musician_name %% $1'
                               ' order by LOWER(mm_metadata_musician_name) offset $2 limit $3',
                               (search_value, offset, records))
    else:
        self.db_cursor.execute('select mm_metadata_musician_guid, mm_metadata_musician_name,'
                               ' mm_metadata_musician_json from mm_metadata_musician'
                               ' order by LOWER(mm_metadata_musician_name) offset $1 limit $2',
                               (offset, records))
    return self.db_cursor.fetchall()


// TODO port query
def db_meta_album_image_random(self):
    """
    Find random album cover image
    """
    # self.db_cursor.execute('select mm_metadata_localimage_json->'Images'->'themoviedb'->>''
    #     + return_image_type + '' as image_json,mm_metadata_guid from mm_media,mm_metadata_movie'\
    #     ' where mm_media_metadata_guid = mm_metadata_guid'\
    #     ' and (mm_metadata_localimage_json->'Images'->'themoviedb'->>''
    #     + return_image_type + ''' + ')::text != 'null' order by random() limit 1')
    try:
        // then if no results.....a None will except which will then pass None, None
        image_json, metadata_id = self.db_cursor.fetchone()
        return image_json, metadata_id
    except:
        return None, None


// TODO port query
def db_meta_music_by_provider_uuid(self, provider, uuid_id):
    try:
        self.db_cursor.execute('select mm_metadata_music_guid from mm_metadata_music'
                               ' where mm_metadata_media_music_id->'' + provider
                               + '' ? $1', (uuid_id,))
        return self.db_cursor.fetchone()['mm_metadata_music_guid']
    except:
        return None

 */