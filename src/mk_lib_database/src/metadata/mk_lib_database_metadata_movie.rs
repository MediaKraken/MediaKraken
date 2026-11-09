use crate::mk_lib_database::MediaStatusUpdatePayload;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

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

#[allow(clippy::too_many_arguments)]
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
                OR lower(mm_metadata_movie_primary_lang_gen) = lower($5)
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
        .bind(user_id)
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
                OR lower(mm_metadata_movie_primary_lang_gen) = lower($4)
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
        .bind(user_id)
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
        r#"select lower(mm_metadata_movie_primary_lang_gen) as primary_language
        from mm_metadata_movie
        where coalesce(mm_metadata_movie_primary_lang_gen, '') <> ''
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
                OR lower(mm_metadata_movie_primary_lang_gen) = lower($4)
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
                OR lower(mm_metadata_movie_primary_lang_gen) = lower($4)
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
        original_name = Some(data_json["original_title"].as_str().unwrap_or(""));
    }
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_movie (mm_metadata_movie_guid, mm_metadata_movie_media_id, mm_metadata_movie_name, mm_metadata_movie_name_alt, mm_metadata_movie_json, mm_metadata_movie_localimage_json) values ($1,$2,$3,$4,$5,$6)"#,
    )
    .bind(uuid_id)
    .bind(series_id)
    .bind(data_json["title"].as_str().unwrap_or("").to_string())
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
    .bind(user_id)
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

/// Read in movie metadata via a media guid.
///
/// NOTE: the Python original joined `mm_media_metadata_guid = mm_metadata_guid`
/// (no such movie column); ported as `= mm_metadata_movie_guid`.
pub async fn mk_lib_database_metadata_movie_by_media_uuid(
    sqlx_pool: &sqlx::PgPool,
    media_guid: uuid::Uuid,
) -> Result<Option<(serde_json::Value, serde_json::Value)>, sqlx::Error> {
    let row: Option<(serde_json::Value, serde_json::Value)> = sqlx::query_as(
        r#"select mm_metadata_movie_json, mm_metadata_movie_localimage_json
        from mm_media, mm_metadata_movie
        where mm_media_metadata_guid = mm_metadata_movie_guid
        and mm_media_guid = $1"#,
    )
    .bind(media_guid)
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row)
}

/// Find a random movie image of the requested type (e.g. "Poster").
///
/// NOTE: the Python original selected `->'Images'->'themoviedb'->>type` but
/// filtered on `->'Images'->>type` (missing the themoviedb hop); ported as-is.
/// The image type is embedded in a JSON path in the SQL, so it is restricted
/// to `[A-Za-z0-9_-]`; the Python original was injectable ("little bobby
/// tables").
pub async fn mk_lib_database_metadata_movie_image_random(
    sqlx_pool: &sqlx::PgPool,
    return_image_type: &str,
) -> Result<Option<(serde_json::Value, uuid::Uuid)>, sqlx::Error> {
    if !is_valid_image_type(return_image_type) {
        return Err(sqlx::Error::Configuration(
            format!("invalid image type: {return_image_type}").into(),
        ));
    }
    let sql = format!(
        r#"select mm_metadata_movie_localimage_json->'Images'->'themoviedb'->{return_image_type} as image_json,
        mm_metadata_movie_guid
        from mm_media, mm_metadata_movie
        where mm_media_metadata_guid = mm_metadata_movie_guid
        and (mm_metadata_movie_localimage_json->'Images'->>'{return_image_type}')::text != 'null'
        order by random() limit 1"#,
    );
    let row: Option<(serde_json::Value, uuid::Uuid)> = sqlx::query_as(sqlx::AssertSqlSafe(sql))
        .fetch_optional(sqlx_pool)
        .await?;
    Ok(row)
}

/// Update the cast/crew json for the movie with the given metadata guid.
///
/// Only the `cast`/`crew` keys of `cast_crew_json` are copied, under
/// `Cast`/`Crew`, into the stored movie json.
pub async fn mk_lib_database_metadata_movie_update_castcrew(
    sqlx_pool: &sqlx::PgPool,
    cast_crew_json: &serde_json::Value,
    metadata_guid: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    let row: Option<(serde_json::Value,)> = sqlx::query_as(
        r#"select mm_metadata_movie_json
        from mm_metadata_movie
        where mm_metadata_movie_guid = $1"#,
    )
    .bind(metadata_guid)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some((mut movie_json,)) = row else {
        return Err(sqlx::Error::RowNotFound);
    };
    // TODO for dumping 'meta'
    if let Some(cast) = cast_crew_json.get("cast") {
        movie_json["Cast"] = cast.clone();
    }
    // TODO for dumping 'meta'
    if let Some(crew) = cast_crew_json.get("crew") {
        movie_json["Crew"] = crew.clone();
    }
    sqlx::query(
        r#"update mm_metadata_movie set mm_metadata_movie_json = $1
        where mm_metadata_movie_guid = $2"#,
    )
    .bind(movie_json)
    .bind(metadata_guid)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

/// Update a movie record by its tmdb media id.
///
/// NOTE: the Python original bound a json `series_id_json` to the integer
/// `mm_metadata_movie_media_id` column (flagged "wrong" in the original
/// comment); ported with an explicit `media_id: i32` instead.
pub async fn mk_lib_database_metadata_movie_update(
    sqlx_pool: &sqlx::PgPool,
    media_id: i32,
    result_json: &serde_json::Value,
    image_json: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"update mm_metadata_movie
        set mm_metadata_movie_media_id = $1,
            mm_metadata_movie_name = $2,
            mm_metadata_movie_json = $3,
            mm_metadata_movie_localimage_json = $4
        where mm_metadata_movie_media_id = $5"#,
    )
    .bind(media_id)
    .bind(result_json["title"].as_str().unwrap_or("").to_string())
    .bind(result_json)
    .bind(image_json)
    .bind(result_json["id"].as_i64().unwrap_or(0) as i32)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

/// True when the image type is safe to embed in a SQL JSON path.
fn is_valid_image_type(image_type: &str) -> bool {
    !image_type.is_empty()
        && image_type
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}
