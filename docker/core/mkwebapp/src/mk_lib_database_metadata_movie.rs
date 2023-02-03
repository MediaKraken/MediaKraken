#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};
use stdext::function_name;
use serde_json::json;

pub async fn mk_lib_database_metadata_exists_movie(
    sqlx_pool: &sqlx::PgPool,
    metadata_id: i32,
) -> Result<bool, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (bool,) = sqlx::query_as(
        "select exists(select 1 from mm_metadata_movie \
        where mm_metadata_movie_media_id = $1 limit 1) as found_record limit 1",
    )
    .bind(metadata_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMovieList {
    pub mm_metadata_guid: uuid::Uuid,
    pub mm_metadata_name: String,
    pub mm_date: String,    // DateTime<Utc>,
    pub mm_poster: String,
    pub mm_metadata_user_json: Option<serde_json::Value>,
}

pub async fn mk_lib_database_metadata_movie_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i32,
    limit: i32,
) -> Result<Vec<DBMetaMovieList>, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let select_query;
    if search_value != "" {
        select_query = sqlx::query(
            "select mm_metadata_movie_guid, mm_metadata_movie_name, \
             mm_metadata_movie_json->>'release_date' as mm_date, \
             mm_metadata_movie_localimage_json->>'Poster' as mm_poster, \
             mm_metadata_movie_user_json \
             from mm_metadata_movie \
             where mm_metadata_movie_name % $1 \
             order by mm_metadata_movie_name, mm_date offset $2 limit $3",
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit);
    } else {
        select_query = sqlx::query(
            "select mm_metadata_movie_guid, mm_metadata_movie_name, \
            mm_metadata_movie_json->>'release_date' as mm_date, \
            mm_metadata_movie_localimage_json->>'Poster' as mm_poster, \
            mm_metadata_movie_user_json \
            from mm_metadata_movie \
            order by mm_metadata_movie_name, mm_date \
            offset $1 limit $2",
        )
        .bind(offset)
        .bind(limit);
    }
    let table_rows: Vec<DBMetaMovieList> = select_query
        .map(|row: PgRow| DBMetaMovieList {
            mm_metadata_guid: row.get("mm_metadata_movie_guid"),
            mm_metadata_name: row.get("mm_metadata_movie_name"),
            mm_date: row.get("mm_date"),
            mm_poster: row.get("mm_poster"),
            mm_metadata_user_json: row.get("mm_metadata_movie_user_json"),
        })
        .fetch_all(sqlx_pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_movie_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    if search_value != "" {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_movie \
            where mm_metadata_movie_name % $1",
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as("select count(*) from mm_metadata_movie")
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_movie_insert(
    sqlx_pool: &sqlx::PgPool,
    uuid_id: Uuid,
    series_id: i32,
    data_json: &serde_json::Value,
    data_image_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_metadata_movie (mm_metadata_movie_guid, \
        mm_metadata_movie_media_id, \
        mm_metadata_movie_name, \
        mm_metadata_movie_json, \
        mm_metadata_movie_localimage_json) \
        values ($1,$2,$3,$4,$5)",
    )
    .bind(uuid_id)
    .bind(series_id)
    .bind(data_json["title"].as_str().unwrap().to_string())
    .bind(data_json)
    .bind(data_image_json)
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_movie_guid_by_tmdb(
    sqlx_pool: &sqlx::PgPool,
    uuid_id: Uuid,
) -> Result<uuid::Uuid, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let row: (uuid::Uuid,) = sqlx::query_as(
        "select mm_metadata_guid from mm_metadata_movie where mm_metadata_movie_media_id = $1",
    )
    .bind(uuid_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/*

// TODO port query
def db_meta_tmdb_count(self, tmdb_id):
    """
    # see if metadata exists via themovedbid
    """
    self.db_cursor.execute('select exists(select 1 from mm_metadata_movie'
                           ' where mm_metadata_movie_media_id = $1 limit 1) limit 1', (tmdb_id,))
    return self.db_cursor.fetchone()[0]



// TODO port query
def db_read_media_metadata(self, media_guid):
    """
    # read in the media with corresponding metadata
    """
    self.db_cursor.execute('select mm_metadata_movie_guid,'
                           ' mm_metadata_movie_media_id,'
                           ' mm_metadata_movie_name,'
                           ' mm_metadata_movie_json,'
                           ' mm_metadata_movie_localimage_json,'
                           ' mm_metadata_movie_user_json'
                           ' from mm_metadata_movie'
                           ' where mm_metadata_movie_guid = $1', (media_guid,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None


// TODO port query
pub async fn db_meta_movie_by_media_uuid(self, media_guid):
    """
    # read in metadata via media id
    """
    return await db_conn.fetchrow('select mm_metadata_movie_json,'
                                  ' mm_metadata_movie_localimage_json'
                                  ' from mm_media, mm_metadata_movie'
                                  ' where mm_media_metadata_guid = mm_metadata_guid'
                                  ' and mm_media_guid = $1', media_guid)


// TODO port query
pub async fn db_meta_movie_detail(self, media_guid):
    """
    # read in the media with corresponding metadata
    """
    return await db_conn.fetchrow('select mm_metadata_movie_guid,'
                                  ' mm_metadata_movie_media_id,'
                                  ' mm_metadata_movie_name,'
                                  ' mm_metadata__moviejson,'
                                  ' mm_metadata_movie_localimage_json,'
                                  ' mm_metadata_movie_user_json'
                                  ' from mm_metadata_movie'
                                  ' where mm_metadata_movie_guid = $1',
                                  media_guid)


// TODO port query
pub async fn db_meta_movie_status_update(self, metadata_guid, user_id, status_text,
                                      db_connection=None):
    """
    # set status's for metadata
    """
    # do before the select to save db lock time
    if status_text == 'watched' or status_text == 'requested':
        status_setting = true
    else:
        status_setting = status_text
        status_text = 'Rating'
    // grab the user json for the metadata
    json_data = await db_conn.fetchrow('SELECT mm_metadata_movie_user_json'
                                       ' from mm_metadata_movie'
                                       ' where mm_metadata_guid = $1 FOR UPDATE',
                                       metadata_guid)
    // split this off so coroutine doesn't get mad
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
pub async fn db_meta_movie_json_update(self, media_guid, metadata_json):
    """
    # update the metadata json
    """
    await db_conn.execute('update mm_metadata_movie'
                          ' set mm_metadata_movie_user_json = $1'
                          ' where mm_metadata_movie_guid = $2',
                          metadata_json, media_guid)
    await db_conn.execute('commit')


# poster, backdrop, etc
// TODO port query
def db_meta_movie_image_random(self, return_image_type='Poster'):
    """
    Find random movie image
    """
    // TODO little bobby tables
    self.db_cursor.execute('select mm_metadata_movie_localimage_json->'Images'->'themoviedb'->>''
                           + return_image_type + '' as image_json,mm_metadata_movie_guid'
                                                 ' from mm_media,mm_metadata_movie'
                                                 ' where mm_media_metadata_guid = mm_metadata_movie_guid'
                                                 ' and (mm_metadata_movie_localimage_json->'Images'->>''
                           + return_image_type + ''' + ')::text != 'null''
                                                        ' order by random() limit 1')
    try:
        // then if no results.....a None will except which will then pass None, None
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
    self.db_cursor.execute('select mm_metadata_movie_json'
                           ' from mm_metadata_movie'
                           ' where mm_metadata_guid = $1', (metadata_id,))
    cast_crew_json_row = self.db_cursor.fetchone()[0]
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text={
        'castrow': cast_crew_json_row})
    // TODO for dumping 'meta'
    if 'cast' in cast_crew_json:
        cast_crew_json_row.update({'Cast': cast_crew_json['cast']})
    // TODO for dumping 'meta'
    if 'crew' in cast_crew_json:
        cast_crew_json_row.update({'Crew': cast_crew_json['crew']})
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                         message_text={'upt': cast_crew_json_row})
    self.db_cursor.execute('update mm_metadata_movie set mm_metadata_movie_json = $1'
                           ' where mm_metadata_movie_guid = $2',
                           (json.dumps(cast_crew_json_row), metadata_id))
    self.db_commit()

    // TODO port query
def db_meta_update(self, series_id_json, result_json, image_json):
    """
    # update record by tmdb
    """
    // um, mm_metadata_media_id is wrong
    self.db_cursor.execute('update mm_metadata_movie set mm_metadata_movie_media_id = $1,'
                           ' mm_metadata_movie_name = $2,'
                           ' mm_metadata_movie_json = $3,'
                           ' mm_metadata_movie_localimage_json = $4'
                           ' where mm_metadata_movie_media_id = $5',
                           (series_id_json, result_json['title'],
                            json.dumps(result_json), json.dumps(image_json),
                            result_json['id']))
    self.db_commit()

 */
