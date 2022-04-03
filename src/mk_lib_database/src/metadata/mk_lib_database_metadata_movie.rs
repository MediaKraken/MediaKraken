use sqlx::{FromRow, Row};
use sqlx::{types::Uuid, types::Json};
use sqlx::postgres::PgRow;
use rocket_dyn_templates::serde::{Serialize, Deserialize};
use chrono::prelude::*;

pub async fn mk_lib_database_metadata_exists_movie(pool: &sqlx::PgPool,
                                                   metadata_id: i32)
                                                   -> Result<bool, sqlx::Error> {
    let row: (bool, ) = sqlx::query_as("select exists(select 1 from mm_metadata_movie \
        where mm_metadata_movie_media_id = $1 limit 1) as found_record limit 1")
        .bind(metadata_id)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMovieList {
	mm_metadata_guid: uuid::Uuid,
	mm_metadata_name: String,
	mm_date: DateTime<Utc>,
	mm_poster: String,
	mm_metadata_user_json: serde_json::Value,
}

pub async fn mk_lib_database_metadata_movie_read(pool: &sqlx::PgPool,
                                                 search_value: String,
                                                 offset: i32, limit: i32)
                                                 -> Result<Vec<DBMetaMovieList>, sqlx::Error> {
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("select mm_metadata_guid, mm_metadata_name, \
             mm_metadata_json->'release_date' as mm_date, \
             mm_metadata_localimage_json->'Poster' as mm_poster, \
             mm_metadata_user_json \
             from mm_metadata_movie \
             where mm_metadata_name % $1 \
             order by mm_metadata_name, mm_date offset $2 limit $3)")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("select mm_metadata_guid, mm_metadata_name, \
            mm_metadata_json->'release_date' as mm_date, \
            mm_metadata_localimage_json->'Poster' as mm_poster, \
            mm_metadata_user_json \
            from mm_metadata_movie \
            order by mm_metadata_name, mm_date \
            offset $1 limit $2)")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMetaMovieList> = select_query
        .map(|row: PgRow| DBMetaMovieList {
            mm_metadata_guid: row.get("mm_metadata_guid"),
            mm_metadata_name: row.get("mm_metadata_name"),
            mm_date: row.get("mm_date"),
            mm_poster: row.get("mm_poster"),
            mm_metadata_user_json: row.get("mm_metadata_user_json"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_movie_count(pool: &sqlx::PgPool,
                                                  search_value: String)
                                                  -> Result<i32, sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_movie \
            where mm_metadata_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_metadata_movie")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

/*

// TODO port query
async def db_meta_movie_by_media_uuid(self, media_guid, db_connection=None):
    """
    # read in metadata via media id
    """
    return await db_conn.fetchrow('select mm_metadata_json,'
                                  ' mm_metadata_localimage_json'
                                  ' from mm_media, mm_metadata_movie'
                                  ' where mm_media_metadata_guid = mm_metadata_guid'
                                  ' and mm_media_guid = $1', media_guid)


// TODO port query
async def db_meta_movie_detail(self, media_guid, db_connection=None):
    """
    # read in the media with corresponding metadata
    """
    return await db_conn.fetchrow('select mm_metadata_guid,'
                                  ' mm_metadata_media_id,'
                                  ' mm_metadata_name,'
                                  ' mm_metadata_json,'
                                  ' mm_metadata_localimage_json,'
                                  ' mm_metadata_user_json'
                                  ' from mm_metadata_movie'
                                  ' where mm_metadata_guid = $1',
                                  media_guid)


// TODO port query
async def db_meta_movie_status_update(self, metadata_guid, user_id, status_text,
                                      db_connection=None):
    """
    # set status's for metadata
    """
    # do before the select to save db lock time
    if status_text == 'watched' or status_text == 'requested':
        status_setting = True
    else:
        status_setting = status_text
        status_text = 'Rating'
    # grab the user json for the metadata
    json_data = await db_conn.fetchrow('SELECT mm_metadata_user_json'
                                       ' from mm_metadata_movie'
                                       ' where mm_metadata_guid = $1 FOR UPDATE',
                                       metadata_guid)
    # split this off so coroutine doesn't get mad
    try:
        json_data = json_data['mm_metadata_user_json']
    except:
        json_data = {'UserStats': {}}
    if str(user_id) in json_data['UserStats']:
        json_data['UserStats'][str(user_id)][status_text] = status_setting
    else:
        json_data['UserStats'][str(user_id)] = {status_text: status_setting}
    await self.db_meta_movie_json_update(metadata_guid,
                                        json_data)


// TODO port query
async def db_meta_movie_json_update(self, media_guid, metadata_json, db_connection=None):
    """
    # update the metadata json
    """
    await db_conn.execute('update mm_metadata_movie'
                          ' set mm_metadata_user_json = $1'
                          ' where mm_metadata_guid = $2',
                          metadata_json, media_guid)
    await db_conn.execute('commit')


# poster, backdrop, etc
// TODO port query
def db_meta_movie_image_random(self, return_image_type='Poster'):
    """
    Find random movie image
    """
    # TODO little bobby tables
    self.db_cursor.execute('select mm_metadata_localimage_json->'Images'->'themoviedb'->>''
                           + return_image_type + '' as image_json,mm_metadata_guid'
                                                 ' from mm_media,mm_metadata_movie'
                                                 ' where mm_media_metadata_guid = mm_metadata_guid'
                                                 ' and (mm_metadata_localimage_json->'Images'->>''
                           + return_image_type + ''' + ')::text != 'null''
                                                        ' order by random() limit 1')
    try:
        # then if no results.....a None will except which will then pass None, None
        image_json, metadata_id = self.db_cursor.fetchone()
        return image_json, metadata_id
    except:
        return None, None


// TODO port query
def db_meta_movie_update_castcrew(self, cast_crew_json, metadata_id):
    """
    Update the cast/crew for selected media
    """
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                         message_text={'upt castcrew': metadata_id})
    self.db_cursor.execute('select mm_metadata_json'
                           ' from mm_metadata_movie'
                           ' where mm_metadata_guid = $1', (metadata_id,))
    cast_crew_json_row = self.db_cursor.fetchone()[0]
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text={
        'castrow': cast_crew_json_row})
    # TODO for dumping 'meta'
    if 'cast' in cast_crew_json:
        cast_crew_json_row.update({'Cast': cast_crew_json['cast']})
    # TODO for dumping 'meta'
    if 'crew' in cast_crew_json:
        cast_crew_json_row.update({'Crew': cast_crew_json['crew']})
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                         message_text={'upt': cast_crew_json_row})
    self.db_cursor.execute('update mm_metadata_movie set mm_metadata_json = $1'
                           ' where mm_metadata_guid = $2',
                           (json.dumps(cast_crew_json_row), metadata_id))
    self.db_commit()

 */