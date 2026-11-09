use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetadataGenreCountList {
    pub genre: String,
    pub mm_count: i64,
}

pub async fn mk_lib_database_metadata_genre_count_read(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<DBMetadataGenreCountList>, sqlx::Error> {
    let table_rows: Vec<DBMetadataGenreCountList> = sqlx::query_as(
        r#"select jsonb_array_elements_text(mm_metadata_json->'genres')b as genre, count(mm_metadata_json->'genres') as mm_count from mm_metadata_movie group by genre order by jsonb_array_elements_text(mm_metadata_json->'genres')b"#,
    )
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetadataGenreList {
    genre: String,
}

pub async fn mk_lib_database_metadata_genre_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetadataGenreList>, sqlx::Error> {
    let table_rows: Vec<DBMetadataGenreList> = sqlx::query_as(
        r#"select distinct jsonb_array_elements_text(mm_metadata_json->'genres')b as genre from mm_metadata_movie order by jsonb_array_elements_text(mm_metadata_json->'genres')b offset $1 limit $2"#,
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_genre_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select distinct jsonb_array_elements_text(mm_metadata_json->'genres')b from mm_metadata_movie"#,
    )
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/// Fetch the movie metadata record for a movie media id.
///
/// NOTE: the Python original also had a collection branch
/// (`mm_metadata_collection_media_ids->>$1 = $2`) that passed a single bind
/// for two placeholders, with unclear key/value semantics; not ported.
pub async fn mk_lib_database_metadata_fetch_media_id_json(
    sqlx_pool: &sqlx::PgPool,
    media_id: i32,
) -> Result<Option<(uuid::Uuid, i32)>, sqlx::Error> {
    let row: Option<(uuid::Uuid, i32)> = sqlx::query_as(
        r#"select mm_metadata_movie_guid, mm_metadata_movie_media_id
        from mm_metadata_movie
        where mm_metadata_movie_media_id = $1"#,
    )
    .bind(media_id)
    .fetch_optional(sqlx_pool)
    .await?;
    Ok(row)
}

/// Lookup a movie metadata guid by name (and release year, when given).
///
/// With a year, matches on `name` or `original_title` against the
/// `release_date` year plus/minus 3 years. Returns the first match.
pub async fn mk_lib_database_metadata_find_guid(
    sqlx_pool: &sqlx::PgPool,
    media_name: &str,
    media_release_year: Option<i32>,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let name = media_name.to_lowercase();
    let row: Option<(uuid::Uuid,)> = match media_release_year {
        Some(year) => {
            // for the year and -3/+3 year as well
            sqlx::query_as(
                r#"select mm_metadata_movie_guid from mm_metadata_movie
                where (LOWER(mm_metadata_movie_name) = $1
                or LOWER(mm_metadata_movie_json->>'original_title') = $2)
                and substring(mm_metadata_movie_json->>'release_date' from 0 for 5)
                in ($3,$4,$5,$6,$7,$8,$9)"#,
            )
            .bind(&name)
            .bind(&name)
            .bind(year.to_string())
            .bind((year + 1).to_string())
            .bind((year + 2).to_string())
            .bind((year + 3).to_string())
            .bind((year - 1).to_string())
            .bind((year - 2).to_string())
            .bind((year - 3).to_string())
            .fetch_optional(sqlx_pool)
            .await?
        }
        None => {
            sqlx::query_as(
                r#"select mm_metadata_movie_guid from mm_metadata_movie
                where (LOWER(mm_metadata_movie_name) = $1
                or LOWER(mm_metadata_movie_json->>'original_title') = $2)"#,
            )
            .bind(&name)
            .bind(&name)
            .fetch_optional(sqlx_pool)
            .await?
        }
    };
    // TODO should probably handle multiple results better. Perhaps a notification?
    Ok(row.map(|row| row.0))
}

/*
// Not ported: the Python original calls `db_meta_fetch_series_media_id_json`,
// which does not exist anywhere, and its movie-branch fetch
// (`db_meta_fetch_media_id_json` with `->>$1 = $2` on two placeholders and
// one bind) is broken. Legacy scudlee import path.

// TODO port query
def db_meta_update_media_id_from_scudlee(self, media_tvid, media_imdbid,
                                         media_aniid):
    """
     # update the mediaid in metadata
     """
    # do tvdb first due to datadump
    if media_tvid != None:
        media_type = 'thetvdb'
        media_id = media_tvid
    else if media_imdbid != None:
        media_type = 'imdb'
        media_id = media_imdbid
    else if media_aniid != None:
        media_type = 'anidb'
        media_id = media_aniid
    # lookup id from metadata json or collections
    row_data = self.db_meta_fetch_media_id_json(media_id, False)
    # do the update if a record is found
    if row_data != None:
        # update json data
        common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                             message_text={"id": media_tvid,
                                                                           'imdb': media_imdbid,
                                                                           'ani': media_aniid})
        json_data = json.loads(row_data['mm_metadata_media_id'])
        if media_imdbid != None:
            json_data.update({'imdb': media_imdbid})
        if media_tvid != None:
            json_data.update({'thetvdb': media_tvid})
        if media_aniid != None:
            json_data.update({'anidb': media_aniid})
        self.db_cursor.execute('update mm_metadata_movie set mm_metadata_media_id = $1'
                               ' where mm_metadata_guid = $2',
                               (json.dumps(json_data), row_data['mm_metadata_guid']))
    # lookup id from series
    row_data = self.db_meta_fetch_series_media_id_json(media_type, media_id)
    # do the update if a record is found
    if row_data != None:
        # update json data
        common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                             message_text={"id2": media_tvid,
                                                                           'imdb': media_imdbid,
                                                                           'anidb': media_aniid})
        json_data = json.loads(row_data['mm_metadata_media_tvshow_id'])
        if media_imdbid != None:
            json_data.update({'imdb': media_imdbid})
        if media_tvid != None:
            json_data.update({'thetvdb': media_tvid})
        if media_aniid != None:
            json_data.update({'anidb': media_aniid})
        self.db_cursor.execute('update mm_metadata_tvshow set mm_metadata_media_tvshow_id = $1'
                               ' where mm_metadata_tvshow_guid = $2', (json.dumps(json_data),
                                                                       row_data[
                                                                           'mm_metadata_tvshow_guid']))
*/

/// Queued metadata items across movie, tv show, music and music video.
///
/// NOTE: the Python original selected `mm_metadata_guid` / `mm_metadata_name`
/// (no such movie columns) and `mm_media_music_video_band` (typo); ported to
/// `mm_metadata_movie_*` / `mm_metadata_music_video_band`.
pub async fn mk_lib_database_metadata_queue_list(
    sqlx_pool: &sqlx::PgPool,
    user_id: i64,
    offset: i64,
    records: Option<i64>,
) -> Result<Vec<(uuid::Uuid, String)>, sqlx::Error> {
    // TODO sort by release date as well
    // TODO use the search value
    let user_key = user_id.to_string();
    match records {
        Some(records) => {
            let rows: Vec<(uuid::Uuid, String)> = sqlx::query_as(
                r#"(select mm_metadata_movie_guid, mm_metadata_movie_name
                from mm_metadata_movie
                where mm_metadata_movie_user_json->'UserStats'->$1->>'queue' = 'True'
                order by LOWER(mm_metadata_movie_name))
                UNION ALL
                (select mm_metadata_tvshow_guid, mm_metadata_tvshow_name
                from mm_metadata_tvshow
                where mm_metadata_tvshow_user_json->'UserStats'->$2->>'queue' = 'True'
                order by LOWER(mm_metadata_tvshow_name))
                UNION ALL
                (select mm_metadata_music_guid, mm_metadata_music_name
                from mm_metadata_music
                where mm_metadata_music_user_json->'UserStats'->$3->>'queue' = 'True'
                order by LOWER(mm_metadata_music_name))
                UNION ALL
                (select mm_metadata_music_video_guid, mm_metadata_music_video_band
                from mm_metadata_music_video
                where mm_metadata_music_video_user_json->'UserStats'->$4->>'queue' = 'True'
                order by LOWER(mm_metadata_music_video_band))
                offset $5 limit $6"#,
            )
            .bind(&user_key)
            .bind(&user_key)
            .bind(&user_key)
            .bind(&user_key)
            .bind(offset)
            .bind(records)
            .fetch_all(sqlx_pool)
            .await?;
            Ok(rows)
        }
        None => {
            let rows: Vec<(uuid::Uuid, String)> = sqlx::query_as(
                r#"(select mm_metadata_movie_guid, mm_metadata_movie_name
                from mm_metadata_movie
                where mm_metadata_movie_user_json->'UserStats'->$1->>'queue' = 'True'
                order by LOWER(mm_metadata_movie_name))
                UNION ALL
                (select mm_metadata_tvshow_guid, mm_metadata_tvshow_name
                from mm_metadata_tvshow
                where mm_metadata_tvshow_user_json->'UserStats'->$2->>'queue' = 'True'
                order by LOWER(mm_metadata_tvshow_name))
                UNION ALL
                (select mm_metadata_music_guid, mm_metadata_music_name
                from mm_metadata_music
                where mm_metadata_music_user_json->'UserStats'->$3->>'queue' = 'True'
                order by LOWER(mm_metadata_music_name))
                UNION ALL
                (select mm_metadata_music_video_guid, mm_metadata_music_video_band
                from mm_metadata_music_video
                where mm_metadata_music_video_user_json->'UserStats'->$4->>'queue' = 'True'
                order by LOWER(mm_metadata_music_video_band))
                offset $5"#,
            )
            .bind(&user_key)
            .bind(&user_key)
            .bind(&user_key)
            .bind(&user_key)
            .bind(offset)
            .fetch_all(sqlx_pool)
            .await?;
            Ok(rows)
        }
    }
}
