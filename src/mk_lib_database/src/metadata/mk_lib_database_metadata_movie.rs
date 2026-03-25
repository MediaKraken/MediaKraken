use crate::mk_lib_database::MediaStatusUpdatePayload;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use sqlx::FromRow;

pub async fn mk_lib_database_metadata_exists_movie(
    sqlx_pool: &sqlx::PgPool,
    metadata_id: i32,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        r#"select exists(select 1 from mm_metadata_movie where mm_metadata_movie_media_id = $1 limit 1) as found_record limit 1"#,
    )
    .bind(metadata_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMovieList {
    pub mm_metadata_movie_guid: uuid::Uuid,
    pub mm_metadata_movie_name: String,
    pub mm_metadata_movie_name_alt: Option<String>,
    pub mm_date: String, // DateTime<Utc>,
    pub mm_poster: String,
    pub mm_status_user_json: Option<serde_json::Value>,
    pub mm_genre: serde_json::Value,
    pub mm_availibility: String,
    pub mm_metadata_tagline: Option<String>,
    pub mm_metadata_runtime: i32,
    pub mm_metadata_vote_average: Option<f64>,
    pub photo_updated: DateTime<Utc>, // Maps to TIMESTAMPTZ
}

pub async fn mk_lib_database_metadata_movie_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    user_id: i64,
    starts_with: String,
    genre_name: String,
    primary_language: String,
    status_filter: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaMovieList>, sqlx::Error> {
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_metadata_movie_guid, 
             mm_metadata_movie_name,
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
             WHERE (
                mm_metadata_movie_name &@ $2
                OR mm_metadata_movie_name_alt &@ $3
             )
             AND (
                $4 = ''
                OR EXISTS (
                    SELECT 1
                    FROM jsonb_array_elements(mm_metadata_movie_json->'genres') AS genre
                    WHERE lower(genre->>'name') = lower($4)
                )
             )
             AND (
                $5 = ''
                OR lower(mm_metadata_movie_primary_lang) = lower($5)
             )
             AND (
                $6 = ''
                OR (
                    $6 = 'unwatched'
                    AND NOT COALESCE((mm_status_user_json->>'watched')::boolean, false)
                )
                OR (
                    $6 <> 'unwatched'
                    AND COALESCE((mm_status_user_json->>$6)::boolean, false)
                )
             )
             offset $7 limit $8"#,
        )
        .bind(&user_id)
         .bind(&search_value)
        .bind(&search_value)
        .bind(&genre_name)
        .bind(&primary_language)
        .bind(&status_filter)
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
            WHERE (
                ($2 = '#' AND left(lower(mm_metadata_movie_name), 1) !~ '^[a-z0-9]$')
                OR ($2 <> '#' AND lower(mm_metadata_movie_name) LIKE lower($2) || '%')
            )
            AND (
                $3 = ''
                OR EXISTS (
                    SELECT 1
                    FROM jsonb_array_elements(mm_metadata_movie_json->'genres') AS genre
                    WHERE lower(genre->>'name') = lower($3)
                )
            )
            AND (
                $4 = ''
                OR lower(mm_metadata_movie_primary_lang) = lower($4)
            )
            AND (
                $5 = ''
                OR (
                    $5 = 'unwatched'
                    AND NOT COALESCE((mm_status_user_json->>'watched')::boolean, false)
                )
                OR (
                    $5 <> 'unwatched'
                    AND COALESCE((mm_status_user_json->>$5)::boolean, false)
                )
            )
            order by LOWER(mm_metadata_movie_name), mm_date
            offset $6 limit $7"#,
        )
        .bind(&user_id)
         .bind(&starts_with)
        .bind(&genre_name)
        .bind(&primary_language)
        .bind(&status_filter)
        .bind(offset)
        .bind(limit)
            .fetch_all(sqlx_pool)
            .await
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMoviePrimaryLanguage {
    pub primary_language: String,
}

pub async fn mk_lib_database_metadata_movie_languages(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBMetaMoviePrimaryLanguage>, sqlx::Error> {
    sqlx::query_as(
        r#"select lower(mm_metadata_movie_primary_lang) as primary_language
        from mm_metadata_movie
        where coalesce(mm_metadata_movie_primary_lang, '') <> ''
        group by 1
        order by 1"#,
    )
    .fetch_all(sqlx_pool)
    .await
}

pub async fn mk_lib_database_metadata_movie_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    user_id: i64,
    starts_with: String,
    genre_name: String,
    primary_language: String,
    status_filter: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_movie
            LEFT JOIN mm_metadata_user_status
            ON mm_metadata_user_status.mm_status_type_movie = mm_metadata_movie.mm_metadata_movie_guid
            and mm_metadata_user_status.mm_status_user_id = $1
            where mm_metadata_movie_name &@ $2
            AND (
                $3 = ''
                OR EXISTS (
                    SELECT 1
                    FROM jsonb_array_elements(mm_metadata_movie_json->'genres') AS genre
                    WHERE lower(genre->>'name') = lower($3)
                )
            )
            AND (
                $4 = ''
                OR lower(mm_metadata_movie_primary_lang) = lower($4)
            )
            AND (
                $5 = ''
                OR (
                    $5 = 'unwatched'
                    AND NOT COALESCE((mm_status_user_json->>'watched')::boolean, false)
                )
                OR (
                    $5 <> 'unwatched'
                    AND COALESCE((mm_status_user_json->>$5)::boolean, false)
                )
            )"#,
        )
        .bind(user_id)
        .bind(search_value)
        .bind(genre_name)
        .bind(primary_language)
        .bind(status_filter)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_metadata_movie 
            LEFT JOIN mm_metadata_user_status
            ON mm_metadata_user_status.mm_status_type_movie = mm_metadata_movie.mm_metadata_movie_guid
            and mm_metadata_user_status.mm_status_user_id = $1
            WHERE (
                ($2 = '#' AND left(lower(mm_metadata_movie_name), 1) !~ '^[a-z0-9]$')
                OR ($2 <> '#' AND lower(mm_metadata_movie_name) LIKE lower($2) || '%')
            )
            AND (
                $3 = ''
                OR EXISTS (
                    SELECT 1
                    FROM jsonb_array_elements(mm_metadata_movie_json->'genres') AS genre
                    WHERE lower(genre->>'name') = lower($3)
                )
            )
            AND (
                $4 = ''
                OR lower(mm_metadata_movie_primary_lang) = lower($4)
            )
            AND (
                $5 = ''
                OR (
                    $5 = 'unwatched'
                    AND NOT COALESCE((mm_status_user_json->>'watched')::boolean, false)
                )
                OR (
                    $5 <> 'unwatched'
                    AND COALESCE((mm_status_user_json->>$5)::boolean, false)
                )
            )"#,
        )
        .bind(user_id)
        .bind(starts_with)
        .bind(genre_name)
        .bind(primary_language)
        .bind(status_filter)
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
        r#"insert into mm_metadata_movie (mm_metadata_movie_guid, mm_metadata_movie_media_id, mm_metadata_movie_name, mm_metadata_movie_name_alt, mm_metadata_movie_json, mm_metadata_movie_localimage_json) values ($1,$2,$3,$4,$5,$6)"#,
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
        r#"select mm_metadata_guid from mm_metadata_movie where mm_metadata_movie_media_id = $1"#,
    )
    .bind(uuid_id)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMovieDetail {
    pub mm_metadata_movie_guid: uuid::Uuid,
    pub mm_metadata_movie_name: String,
    pub mm_metadata_movie_name_alt: Option<String>,
    pub mm_date: String, // DateTime<Utc>,
    pub mm_poster: String,
    pub mm_backrop: Option<String>,
    pub mm_status_user_json: Option<serde_json::Value>,
    pub mm_genre: serde_json::Value,
    pub mm_availibility: String,
    pub mm_metadata_tagline: Option<String>,
    pub mm_metadata_runtime: i32,
    pub mm_metadata_vote_average: Option<f64>,
    pub photo_updated: DateTime<Utc>, // Maps to TIMESTAMPTZ
    pub mm_metadata_movie_json: serde_json::Value,
}

pub async fn mk_lib_database_metadata_movie_detail_by_guid(
    sqlx_pool: &sqlx::PgPool,
    uuid_id: Uuid,
    user_id: i64,
) -> Result<DBMetaMovieDetail, sqlx::Error> {
    sqlx::query_as(
        r#"select mm_metadata_movie_guid, mm_metadata_movie_name,
             mm_metadata_movie_name_alt,
             mm_metadata_movie_json->>'release_date' as mm_date,
             mm_metadata_movie_localimage_json->>'Poster' as mm_poster,
            mm_metadata_movie_localimage_json->>'Backdrop' as mm_backrop,
             'unavailable' as mm_availibility,
             (mm_metadata_movie_json->'runtime')::int as mm_metadata_runtime,
             mm_metadata_movie_json->>'tagline' as mm_metadata_tagline,
             (mm_metadata_movie_json->'genres')::jsonb as mm_genre,
             mm_status_user_json,
             ROUND((mm_metadata_movie_json->'vote_average')::numeric, 1)::float as mm_metadata_vote_average,
             photo_updated, mm_metadata_movie_json
             from mm_metadata_movie
             LEFT JOIN mm_metadata_user_status
             ON mm_metadata_user_status.mm_status_type_movie = mm_metadata_movie.mm_metadata_movie_guid
             and mm_metadata_user_status.mm_status_user_id = $1
             WHERE mm_metadata_movie_guid = $2"#,
    )
    .bind(&user_id)
    .bind(uuid_id)
    .fetch_one(sqlx_pool)
    .await
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
