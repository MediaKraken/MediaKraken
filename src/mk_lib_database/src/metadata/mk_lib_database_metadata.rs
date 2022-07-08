#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Json, types::Uuid};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetadataGenreCountList {
    genre: String,
    mm_count: i64,
}

pub async fn mk_lib_database_metadata_genre_count_read(
    pool: &sqlx::PgPool,
) -> Result<Vec<DBMetadataGenreCountList>, sqlx::Error> {
    let select_query = sqlx::query(
        "select \
        jsonb_array_elements_text(mm_metadata_json->'genres')b as genre, \
        count(mm_metadata_json->'genres') as mm_count from mm_metadata_movie group by genre \
        order by jsonb_array_elements_text(mm_metadata_json->'genres')b",
    );
    let table_rows: Vec<DBMetadataGenreCountList> = select_query
        .map(|row: PgRow| DBMetadataGenreCountList {
            genre: row.get("genre"),
            mm_count: row.get("mm_count"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetadataGenreList {
    genre: String,
}

pub async fn mk_lib_database_metadata_genre_read(
    pool: &sqlx::PgPool,
    offset: i32,
    limit: i32,
) -> Result<Vec<DBMetadataGenreList>, sqlx::Error> {
    let select_query = sqlx::query(
        "select distinct \
        jsonb_array_elements_text(mm_metadata_json->'genres')b as genre from mm_metadata_movie \
        order by jsonb_array_elements_text(mm_metadata_json->'genres')b offset $1 limit $2",
    )
    .bind(offset)
    .bind(limit);
    let table_rows: Vec<DBMetadataGenreList> = select_query
        .map(|row: PgRow| DBMetadataGenreList {
            genre: row.get("genre"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_genre_count(pool: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "select distinct jsonb_array_elements_text(mm_metadata_json->'genres')b \
        from mm_metadata_movie",
    )
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

/*




// TODO port query
def db_meta_fetch_media_id_json(self, media_id_id,
                                collection_media=False):
    """
    # grab the current metadata json id
    """
    if not collection_media:
        self.db_cursor.execute('select mm_metadata_guid,'
                               ' mm_metadata_media_id'
                               ' from mm_metadata_movie'
                               ' where mm_metadata_media_id = $1',
                               (media_id_id,))
    else:
        self.db_cursor.execute('select mm_metadata_collection_guid,'
                               ' mm_metadata_collection_media_ids'
                               ' from mm_metadata_collection'
                               ' where mm_metadata_collection_media_ids->>$1 = $2',
                               (media_id_id,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None




// TODO port query
def db_find_metadata_guid(self, media_name, media_release_year):
    """
    Lookup id by name/year
    """
    metadata_guid = None
    if media_release_year != None:
        # for year and -3/+3 year as well
        self.db_cursor.execute('select mm_metadata_guid from mm_metadata_movie'
                               ' where (LOWER(mm_metadata_name) = $1'
                               ' or LOWER(mm_metadata_json->>'original_title') = $2)'
                               ' and substring(mm_metadata_json->>'release_date' from 0 for 5)'
                               ' in ($3,$4,$5,$6,$7,$8,$9)',
                               (media_name.lower(), media_name.lower(), str(media_release_year),
                                str(int(media_release_year) + 1),
                                str(int(media_release_year) + 2),
                                str(int(media_release_year) + 3),
                                str(int(media_release_year) - 1),
                                str(int(media_release_year) - 2),
                                str(int(media_release_year) - 3)))
    else:
        self.db_cursor.execute('select mm_metadata_guid from mm_metadata_movie'
                               ' where (LOWER(mm_metadata_name) = $1'
                               ' or LOWER(mm_metadata_json->>'original_title') = $2)',
                               (media_name.lower(), media_name.lower()))
    for row_data in self.db_cursor.fetchall():
        // TODO should probably handle multiple results better.   Perhaps a notification?
        metadata_guid = row_data['mm_metadata_guid']
        common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text={
            "db find metadata guid": metadata_guid})
        break
    return metadata_guid


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


// TODO port query
def db_meta_queue_list(self, user_id, offset=0, records=None, search_value=None):
    // TODO sort by release date as well
    // TODO use the search value
    self.db_cursor.execute('(select mm_metadata_guid,'
                           ' mm_metadata_name'
                           ' from mm_metadata_movie '
                           ' where mm_metadata_user_json->'UserStats'->$1->>'queue' = '
                           ''True''
                           ' order by LOWER(mm_metadata_name))'
                           ' UNION ALL '
                           '(select mm_metadata_tvshow_guid,'
                           ' mm_metadata_tvshow_name'
                           ' from mm_metadata_tvshow '
                           ' where mm_metadata_tvshow_user_json->'UserStats'->$2->>'queue' '
                           '= 'True''
                           ' order by LOWER(mm_metadata_tvshow_name))'
                           ' UNION ALL '
                           '(select mm_metadata_music_guid,'
                           ' mm_metadata_music_name'
                           ' from mm_metadata_music '
                           ' where mm_metadata_music_user_json->'UserStats'->$3->>'queue' '
                           '= 'True''
                           ' order by LOWER(mm_metadata_music_name))'
                           ' UNION ALL '
                           '(select mm_metadata_music_video_guid,'
                           ' mm_media_music_video_band'
                           ' from mm_metadata_music_video '
                           ' where '
                           'mm_metadata_music_video_user_json->'UserStats'->$4->>'queue' '
                           '= 'True''
                           ' order by LOWER(mm_media_music_video_band))'
                           ' offset $5 limit $6',
                           (user_id, user_id, user_id, user_id, offset, records))
    return self.db_cursor.fetchall()

 */
