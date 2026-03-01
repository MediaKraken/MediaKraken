use crate::mk_lib_database::MediaStatusUpdatePayload;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

pub async fn mk_lib_database_metadata_exists_movie(
    sqlx_pool: &sqlx::PgPool,
    metadata_id: i32,
) -> Result<bool, sqlx::Error> {
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
    pub mm_metadata_movie_name_alt: Option<String>,
    pub mm_date: String, // DateTime<Utc>,
    pub mm_poster: String,
    pub mm_status_user_json: Option<serde_json::Value>,
    pub mm_metadata_genre_json: serde_json::Value,
    pub mm_metadata_availibility: String,
    pub mm_metadata_movie_tagline: Option<String>,
    pub mm_metadata_runtime: i32,
    pub mm_metadata_vote_average: Option<f64>,
    pub photo_updated: DateTime<Utc>, // Maps to TIMESTAMPTZ
}

pub async fn mk_lib_database_metadata_movie_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    user_id: i64,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaMovieList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_metadata_movie_guid, mm_metadata_movie_name,
             mm_metadata_movie_name_alt,
             mm_metadata_movie_json->>'release_date' as mm_date,
             mm_metadata_movie_localimage_json->>'Poster' as mm_poster,
             'unavailable' as mm_availibility,
             (mm_metadata_movie_json->'runtime')::int as mm_metadata_runtime,
             mm_metadata_movie_json->>'tagline' as mm_metadata_tagline,
             (mm_metadata_movie_json->'genres')::jsonb as mm_genre,
             mm_status_user_json,
             ROUND((mm_metadata_movie_json->'vote_average')::numeric, 1)::float as mm_metadata_vote_average,
             photo_updated
             from mm_metadata_movie
             LEFT JOIN mm_metadata_user_status
             ON mm_metadata_user_status.mm_status_type_movie = mm_metadata_movie.mm_metadata_movie_guid
             and mm_metadata_user_status.mm_status_user_id = $1
             WHERE mm_metadata_movie_name &@ $2
             or mm_metadata_movie_name_alt &@ $3
             offset $4 limit $5"#,
        )
        .bind(&user_id)
        .bind(&search_value)
        .bind(&search_value)
        .bind(offset)
        .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    } else {
        sqlx::query_as(
            r#"select mm_metadata_movie_guid, mm_metadata_movie_name,
            mm_metadata_movie_name_alt,
            mm_metadata_movie_json->>'release_date' as mm_date,
            mm_metadata_movie_localimage_json->>'Poster' as mm_poster,
            'unavailable' as mm_availibility,
            (mm_metadata_movie_json->'runtime')::int as mm_metadata_runtime,
            mm_metadata_movie_json->>'tagline' as mm_metadata_tagline,
            (mm_metadata_movie_json->'genres')::jsonb as mm_genre,
            mm_status_user_json,
            ROUND((mm_metadata_movie_json->'vote_average')::numeric, 1)::float as mm_metadata_vote_average,
            photo_updated
            from mm_metadata_movie
            LEFT JOIN mm_metadata_user_status
            ON mm_metadata_user_status.mm_status_type_movie = mm_metadata_movie.mm_metadata_movie_guid
            and mm_metadata_user_status.mm_status_user_id = $1
            order by LOWER(mm_metadata_movie_name), mm_date
            offset $2 limit $3"#,
        )
        .bind(&user_id)
        .bind(offset)
        .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    }
}

pub async fn mk_lib_database_metadata_movie_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            "select count(*) from mm_metadata_movie \
            where mm_metadata_movie_name &@ $1",
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
    let mut original_name = None;
    if !data_json["original_title"].is_null() && data_json["title"] != data_json["original_title"] {
        original_name = Some(data_json["original_title"].as_str().unwrap());
    }
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        "insert into mm_metadata_movie (mm_metadata_movie_guid, \
        mm_metadata_movie_media_id, \
        mm_metadata_movie_name, \
        mm_metadata_movie_name_alt, \
        mm_metadata_movie_json, \
        mm_metadata_movie_localimage_json) \
        values ($1,$2,$3,$4,$5,$6)",
    )
    .bind(uuid_id)
    .bind(series_id)
    .bind(data_json["title"].as_str().unwrap().to_string())
    .bind(original_name)
    .bind(data_json)
    .bind(data_image_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_metadata_movie_guid_by_tmdb(
    sqlx_pool: &sqlx::PgPool,
    uuid_id: Uuid,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        "select mm_metadata_guid from mm_metadata_movie where mm_metadata_movie_media_id = $1",
    )
    .bind(uuid_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_movie_detail_by_guid(
    sqlx_pool: &sqlx::PgPool,
    uuid_id: Uuid,
) -> Result<PgRow, sqlx::Error> {
    let row = sqlx::query(
        "select mm_metadata_movie_media_id, \
        mm_metadata_movie_json, \
        mm_metadata_movie_localimage_json, \
        mm_metadata_movie_user_json \
        from mm_metadata_movie \
        where mm_metadata_movie_guid = $1",
    )
    .bind(uuid_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row)
}

pub async fn mk_lib_database_metadata_movie_status(
    sqlx_pool: &sqlx::PgPool,
    payload: MediaStatusUpdatePayload,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    println!("here is the payload2: {:?}", payload);
    let guid = uuid::Uuid::now_v7();
    sqlx::query(
        r#"
        INSERT INTO mm_metadata_user_status (
            mm_status_guid,
            mm_status_user_id,
            mm_status_user_json,
            mm_status_type_movie
        )
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (mm_status_user_id, mm_status_type_movie)
        DO UPDATE
        SET mm_status_user_json = EXCLUDED.mm_status_user_json
        "#,
    )
    .bind(guid) // $1
    .bind(user_id) // $2
    .bind(sqlx::types::Json(&payload)) // $3
    .bind(payload.guid) // $4
    .execute(sqlx_pool)
    .await?;
    Ok(())
}

/*

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
