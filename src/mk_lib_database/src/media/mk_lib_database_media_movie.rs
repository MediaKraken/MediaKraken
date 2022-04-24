#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use serde::{Serialize, Deserialize};

#[path = "mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;

pub async fn mk_lib_database_media_movie_genre_count(pool: &sqlx::PgPool)
                                                     -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_metadata_json->'genres' as gen, \
        count(mm_metadata_json->'genres') as gen_count \
        from ((select distinct on (mm_media_metadata_guid) \
        mm_metadata_json from mm_media, mm_metadata_movie \
        where mm_media_class_guid = $1 \
        and mm_media_metadata_guid = mm_metadata_guid) union (select distinct \
        on (mmr_media_metadata_guid) mm_metadata_json from mm_media_remote, \
        mm_metadata_movie where mmr_media_class_guid = $2 \
        and mmr_media_metadata_guid = mm_metadata_guid)) \
        as temp group by gen")
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE)
        .bind(mk_lib_common_enum_media_type::DLMediaType::MOVIE)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_media_movie_random(pool: &sqlx::PgPool)
                                                -> Result<(Uuid, Uuid), sqlx::Error> {
    let row: (Uuid, Uuid) = sqlx::query_as("select mm_metadata_guid, mm_media_guid \
        from mm_media, mm_metadata_movie \
        where mm_media_metadata_guid = mm_metadata_guid \
        and random() < 0.01 limit 1")
        .fetch_one(pool)
        .await?;
    Ok(row)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaMovieList {
    mm_metadata_music_video_guid: uuid::Uuid,
}

pub async fn mk_lib_database_media_movie_read(pool: &sqlx::PgPool,
                                              search_value: String,
                                              offset: i32, limit: i32)
                                              -> Result<Vec<DBMediaMovieList>, sqlx::Error> {
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMediaMovieList> = select_query
        .map(|row: PgRow| DBMediaMovieList {
            mm_metadata_music_video_guid: row.get("mm_metadata_music_video_guid"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_movie_count(pool: &sqlx::PgPool,
                                               search_value: String)
                                               -> Result<i32, sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query_as("")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

/*

def db_read_media_metadata_movie_both(self, media_guid):
    """
    # read in metadata and ffprobe by id
    """
    self.db_cursor.execute('select mm_media_ffprobe_json,'
                           'mm_metadata_json,'
                           'mm_metadata_localimage_json'
                           ' from mm_media, mm_metadata_movie'
                           ' where mm_media_metadata_guid = mm_metadata_guid'
                           ' and mm_media_guid = $1', (media_guid,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None

def db_read_media_list_by_uuid(self, media_guid):
    self.db_cursor.execute('select mm_media_ffprobe_json'
                           ' from mm_media'
                           ' where mm_media_metadata_guid in (select mm_metadata_guid from '
                           'mm_media where mm_media_guild = $1)', (media_guid,))
    video_data = []
    for file_data in self.db_cursor.fetchall():
        # go through streams
        audio_streams = []
        subtitle_streams = ['None']
        if 'streams' in file_data['FFprobe'] and file_data['FFprobe']['streams'] != None:
            for stream_info in file_data['FFprobe']['streams']:
                common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                                     message_text={
                                                                         "info": stream_info})
                stream_language = ''
                stream_title = ''
                stream_codec = ''
                try:
                    stream_language = stream_info['tags']['language'] + ' - '
                except:
                    pass
                try:
                    stream_title = stream_info['tags']['title'] + ' - '
                except:
                    pass
                try:
                    stream_codec \
                        = stream_info['codec_long_name'].rsplit('(', 1)[1].replace(')', '') \
                          + ' - '
                except:
                    pass
                if stream_info['codec_type'] == 'audio':
                    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                                         message_text={
                                                                             'stuff': 'audio'})
                    audio_streams.append((stream_codec + stream_language
                                          + stream_title)[:-3])
                else if stream_info['codec_type'] == 'subtitle':
                    subtitle_streams.append(stream_language)
                    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                                         message_text={
                                                                             'stuff': 'subtitle'})
    return video_data

// TODO port query
pub async fn db_media_movie_list(self, class_guid, list_type=None, list_genre='all',
                              list_limit=0, group_collection=False, offset=None,
                              include_remote=False,
                              search_text=None):
    """
    # web media return
    """
    if list_genre == 'all':
        if list_type == "recent_addition":
            if not group_collection:
                if not include_remote:
                    if offset is None:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid)'
                                                   ' mm_media_name, mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_media_json->>'DateAdded' >= $2) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date''
                                                   ' asc',
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"))
                    else:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid) mm_media_name,'
                                                   ' mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path, mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_media_json->>'DateAdded' >= $2) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc'
                                                   ' offset $3 limit $4',
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"),
                                                   offset, list_limit)
                else:
                    if offset is None:
                        return await db_conn.fetch('select mm_media_metadata_guid)'
                                                   ' mm_media_name, mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path, mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_media_json->>'DateAdded' >= $2)'
                                                   ' union (select distinct on (mmr_media_metadata_guid) mm_media_name,'
                                                   ' mmr_media_guid, mmr_media_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster, NULL as '
                                                   'mmr_media_path, mm_metadata_json'
                                                   ' from mm_media_remote, mm_metadata_movie'
                                                   ' where mmr_media_class_guid = $3'
                                                   ' and mmr_media_metadata_guid'
                                                   ' = mm_metadata_guid and mmr_media_json->>'DateAdded' >= $4) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"),
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"))
                    else:
                        return await db_conn.fetch('mm_media_metadata_guid)'
                                                   ' mm_media_name, mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path, mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_media_json->>'DateAdded' >= $2)'
                                                   ' union (select distinct on (mmr_media_metadata_guid) mm_media_name,'
                                                   ' mmr_media_guid, mmr_media_json, '
                                                   'mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster, NULL as '
                                                   'mmr_media_path,'
                                                   ' mm_metadata_json'
                                                   '  from mm_media_remote, mm_metadata_movie'
                                                   ' where mmr_media_class_guid = $3'
                                                   ' and mmr_media_metadata_guid'
                                                   ' = mm_metadata_guid and mmr_media_json->>'DateAdded' >= $4) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc'
                                                   ' offset $5 limit $6',
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"),
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"),
                                                   offset, list_limit)
            else:
                if offset is None:
                    return await db_conn.fetch('select 1')
                else:
                    return await db_conn.fetch('select 1')
        else:
            if not group_collection:
                if not include_remote:
                    if offset is None:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid) mm_media_name,'
                                                   ' mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   class_guid)
                    else:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid) mm_media_name,'
                                                   ' mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   ' offset $2 limit $3',
                                                   class_guid, offset, list_limit)
                else:
                    if offset is None:
                        return await db_conn.fetch('select * from ((select distinct'
                                                   ' on (mm_media_metadata_guid) mm_media_name,'
                                                   ' mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid)'
                                                   ' union (select distinct on (mmr_media_metadata_guid) mm_media_name,'
                                                   ' mmr_media_guid,'
                                                   ' mmr_media_json, '
                                                   'mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster, NULL as '
                                                   'mmr_media_path, mm_metadata_json'
                                                   '  from mm_media_remote, mm_metadata_movie'
                                                   ' where mmr_media_class_guid = $2'
                                                   ' and mmr_media_metadata_guid'
                                                   ' = mm_metadata_guid)) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   class_guid, class_guid)
                    else:
                        return await db_conn.fetch('select * from ((select distinct'
                                                   ' on (mm_media_metadata_guid) mm_media_name,'
                                                   ' mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid)'
                                                   ' union (select distinct on (mmr_media_metadata_guid) mm_media_name,'
                                                   ' mmr_media_guid, mmr_media_json, '
                                                   'mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster, NULL as '
                                                   'mmr_media_path, mm_metadata_json'
                                                   '  from mm_media_remote, mm_metadata_movie'
                                                   ' where mmr_media_class_guid = $2'
                                                   ' and mmr_media_metadata_guid'
                                                   ' = mm_metadata_guid)) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc'
                                                   ' offset $3 limit $4',
                                                   class_guid, class_guid, offset,
                                                   list_limit)
            else:
                if not include_remote:
                    if offset is None:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid) mm_media_name as name,'
                                                   ' mm_media_guid as guid,'
                                                   ' mm_metadata_user_json as mediajson,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path as mediapath'
                                                   ' from mm_media, mm_metadata_movie,'
                                                   ' mm_metadata_json'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and (mm_metadata_json->>'belongs_to_collection') is null'
                                                   ' union select mm_metadata_collection_name as name,'
                                                   ' mm_metadata_collection_guid as guid,'
                                                   ' nullb as metajson,'
                                                   ' mm_media_path as mediapath'
                                                   ' from mm_metadata_collection) as temp'
                                                   ' order by LOWER(name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   class_guid)
                    else:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid)'
                                                   ' mm_media_name as name,'
                                                   ' mm_media_guid as guid,'
                                                   ' mm_metadata_user_json as mediajson,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path as mediapath,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media,'
                                                   ' mm_metadata_movie where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and (mm_metadata_json->>'belongs_to_collection') is null'
                                                   ' union select mm_metadata_collection_name as name,'
                                                   ' mm_metadata_collection_guid as guid,'
                                                   ' nullb as metajson,'
                                                   ' mm_media_path as mediapath,'
                                                   ' mm_metadata_json'
                                                   ' from mm_metadata_collection) as temp'
                                                   ' order by LOWER(name),'
                                                   ' mm_metadata_json->>'release_date' asc'
                                                   ' offset $2 limit $3',
                                                   class_guid, offset, list_limit)
                else:
                    if offset is None:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid)'
                                                   ' mm_media_name as name,'
                                                   ' mm_media_guid as guid,'
                                                   ' mm_metadata_user_json as mediajson,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path as mediapath,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and (mm_metadata_json->>'belongs_to_collection') is null'
                                                   // TODO put back in
                                                   #                        ' union select mm_metadata_collection_name as name,'
                                                   #                        ' mm_metadata_collection_guid as guid,'
                                                   #                        ' nullb as mediajson, nullb as metajson,'
                                                   #                        ' nullb as metaimagejson, mm_media_path as mediapath'
                                                   #                        ' from mm_metadata_collection'
                                                   ') as temp'
                                                   ' order by LOWER(name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   class_guid)
                    else:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid)'
                                                   ' mm_media_name as name,'
                                                   ' mm_media_guid as guid,'
                                                   ' mm_metadata_user_json as mediajson,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path as mediapath,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and (mm_metadata_json->>'belongs_to_collection') is null'
                                                   // TODO put back in
                                                   #                        ' union select mm_metadata_collection_name as name,'
                                                   #                        ' mm_metadata_collection_guid as guid,'
                                                   #                        ' nullb as mediajson, nullb as metajson,'
                                                   #                        ' nullb as metaimagejson, mm_media_path as mediapath'
                                                   #                        ' from mm_metadata_collection'
                                                   ') as temp'
                                                   ' order by LOWER(name),'
                                                   ' mm_metadata_json->>'release_date' asc'
                                                   ' offset $2 limit $3',
                                                   class_guid, offset, list_limit)
    else:
        if list_type == "recent_addition":
            if not group_collection:
                if not include_remote:
                    if offset is None:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid)'
                                                   ' mm_media_name, mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_media_json->>'DateAdded' >= $2'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? $3) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"),
                                                   list_genre)
                    else:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid) mm_media_name,'
                                                   ' mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_media_json->>'DateAdded' >= $2'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? $3) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc'
                                                   ' offset $4 limit $5',
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"),
                                                   list_genre, offset, list_limit)
                else:
                    if offset is None:
                        return await db_conn.fetch('select * from ((select distinct'
                                                   ' on (mm_media_metadata_guid) mm_media_name,'
                                                   ' mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_media_json->>'DateAdded' >= $2'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? $3)'
                                                   ' union (select distinct on (mmr_media_metadata_guid) mm_media_name,'
                                                   ' mmr_media_guid,'
                                                   ' mmr_media_json, '
                                                   'mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster, NULL as '
                                                   'mmr_media_path, mm_metadata_json'
                                                   '  from mm_media_remote, mm_metadata_movie'
                                                   ' where mmr_media_class_guid = $4'
                                                   ' and mmr_media_metadata_guid = mm_metadata_guid'
                                                   ' and mmr_media_json->>'DateAdded' >= $5'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? %6)) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"),
                                                   list_genre, class_guid,
                                                   (datetime.datetime.now()
                                                    - datetime.timedelta(
                                                               days=7)).strftime(
                                                       "%Y-%m-%d"),
                                                   list_genre)
                    else:
                        return await db_conn.fetch('select * from ((select distinct'
                                                   ' on (mm_media_metadata_guid)'
                                                   ' mm_media_name, mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_media_json->>'DateAdded' >= $2'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? $3)'
                                                   ' union (select distinct on (mmr_media_metadata_guid) mm_media_name,'
                                                   ' mmr_media_guid,'
                                                   ' mmr_media_json, '
                                                   'mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster, NULL as '
                                                   'mmr_media_path,'
                                                   ' mm_metadata_json'
                                                   '  from mm_media_remote, mm_metadata_movie'
                                                   ' where mmr_media_class_guid = $4'
                                                   ' and mmr_media_metadata_guid = mm_metadata_guid'
                                                   ' and mmr_media_json->>'DateAdded' >= $5'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? $6)) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc'
                                                   ' offset $7 limit $8',
                                                   class_guid, (datetime.datetime.now()
                                                                - datetime.timedelta(
                                        days=7)).strftime(
                                "%Y-%m-%d"),
                                                   list_genre, class_guid,
                                                   (datetime.datetime.now()
                                                    - datetime.timedelta(
                                                               days=7)).strftime(
                                                       "%Y-%m-%d"),
                                                   list_genre, offset, list_limit)

            else:
                return await db_conn.fetch('select 1')
        else:
            if not group_collection:
                if not include_remote:
                    if offset is None:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid)'
                                                   ' mm_media_name, mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? $2) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   class_guid, list_genre)
                    else:
                        return await db_conn.fetch('select * from (select distinct'
                                                   ' on (mm_media_metadata_guid)'
                                                   ' mm_media_name, mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path, mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? $2) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc'
                                                   ' offset $3 limit $4',
                                                   class_guid, list_genre, offset,
                                                   list_limit)

                else:
                    if offset is None:
                        return await db_conn.fetch('select * from ((select distinct'
                                                   ' on (mm_media_metadata_guid)'
                                                   ' mm_media_name, mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media, mm_metadata_movie'
                                                   ' where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? $2)'
                                                   ' union (select distinct on (mmr_media_metadata_guid)'
                                                   ' mm_media_name,'
                                                   ' mmr_media_guid,'
                                                   ' mmr_media_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster, NULL as '
                                                   'mmr_media_path,'
                                                   ' mm_metadata_json'
                                                   '  from mm_media_remote, mm_metadata_movie'
                                                   ' where mmr_media_class_guid = $3'
                                                   ' and mmr_media_metadata_guid'
                                                   ' = mm_metadata_guid and mm_metadata_json->'genres'->0->'name' ? $4)) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc',
                                                   class_guid, list_genre, class_guid,
                                                   list_genre)
                    else:
                        return await db_conn.fetch('select * from ((select distinct'
                                                   ' on (mm_media_metadata_guid) mm_media_name,'
                                                   ' mm_media_guid,'
                                                   ' mm_metadata_user_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' mm_media_path, mm_metadata_json'
                                                   ' from mm_media,'
                                                   ' mm_metadata_movie where mm_media_class_guid = $1'
                                                   ' and mm_media_metadata_guid = mm_metadata_guid'
                                                   ' and mm_metadata_json->'genres'->0->'name' ? $2)'
                                                   ' union (select distinct on (mmr_media_metadata_guid)'
                                                   ' mm_media_name,'
                                                   ' mmr_media_guid,'
                                                   ' mmr_media_json,'
                                                   ' mm_metadata_localimage_json->'Poster''
                                                   ' as mm_poster,'
                                                   ' NULL as mmr_media_path,'
                                                   ' mm_metadata_json'
                                                   ' from mm_media_remote, mm_metadata_movie'
                                                   ' where mmr_media_class_guid = $3'
                                                   ' and mmr_media_metadata_guid'
                                                   ' = mm_metadata_guid and mm_metadata_json->'genres'->0->'name' ? $4)) as temp'
                                                   ' order by LOWER(mm_media_name),'
                                                   ' mm_metadata_json->>'release_date' asc'
                                                   ' offset $5 limit $6',
                                                   class_guid, list_genre, class_guid,
                                                   list_genre,
                                                   offset, list_limit)
            else:
                if offset is None:
                    return await db_conn.fetch('select 1')
                else:
                    return await db_conn.fetch('select 1')


// TODO port query
pub async fn db_media_movie_list_count(self, class_guid, list_type=None,
                                    list_genre='all',
                                    group_collection=False, include_remote=False, search_text=None,
                                    db_connection=None):
    """
    # web media count
    """
    # messageWords[0]=="movie" or messageWords[0]=='in_progress' or messageWords[0]=='video':
    if list_genre == 'all':
        if list_type == "recent_addition":
            if not group_collection:
                if not include_remote:
                    return await db_conn.fetchval('select count(*) from (select distinct'
                                                  ' mm_metadata_guid'
                                                  ' from mm_media, mm_metadata_movie'
                                                  ' where mm_media_class_guid = $1'
                                                  ' and mm_media_metadata_guid'
                                                  ' = mm_metadata_guid'
                                                  ' and mm_media_json->>'DateAdded' >= $2)'
                                                  ' as temp',
                                                  class_guid, (datetime.datetime.now()
                                                               - datetime.timedelta(
                                    days=7)).strftime("%Y-%m-%d"), )
                else:
                    return await db_conn.fetchval(
                        'select count(*) from ((select distinct'
                        ' mm_metadata_guid from mm_media, mm_metadata_movie'
                        ' where mm_media_class_guid = $1'
                        ' and mm_media_metadata_guid'
                        ' = mm_metadata_guid'
                        ' and mm_media_json->>'DateAdded' >= $2)'
                        ' union (select distinct mmr_metadata_guid'
                        ' from mm_media_remote,'
                        ' mm_metadata_movie where mmr_media_class_guid = $3'
                        ' and mmr_media_metadata_guid = mm_metadata_guid'
                        ' and mm_media_json->>'DateAdded' >= $4)) as temp',
                        class_guid, (datetime.datetime.now()
                                     - datetime.timedelta(
                                    days=7)).strftime(
                            "%Y-%m-%d"),
                        class_guid, (datetime.datetime.now()
                                     - datetime.timedelta(
                                    days=7)).strftime(
                            "%Y-%m-%d"))
            else:
                return await db_conn.fetchval('select 1')
        else:
            if not group_collection:
                if not include_remote:
                    return await db_conn.fetchval('select count(*) from (select distinct'
                                                  ' mm_metadata_guid'
                                                  ' from mm_media, mm_metadata_movie'
                                                  ' where mm_media_class_guid = $1'
                                                  ' and mm_media_metadata_guid'
                                                  ' = mm_metadata_guid) as temp',
                                                  class_guid)
                else:
                    return await db_conn.fetchval(
                        'select count(*) from ((select distinct'
                        ' mm_metadata_guid from mm_media, mm_metadata_movie'
                        ' where mm_media_class_guid = $1 and mm_media_metadata_guid'
                        ' = mm_metadata_guid)'
                        ' union (select distinct mm_metadata_guid'
                        ' from mm_media_remote, mm_metadata_movie'
                        ' where mmr_media_class_guid = $2'
                        ' and mmr_media_metadata_guid = mm_metadata_guid)) as temp',
                        class_guid, class_guid)
            else:
                if not include_remote:
                    return await db_conn.fetchval('select count(*) as row_count'
                                                  ' from ((select distinct mm_metadata_guid from mm_media,'
                                                  ' mm_metadata_movie where mm_media_class_guid = $1'
                                                  ' and mm_media_metadata_guid = mm_metadata_guid'
                                                  ' and (mm_metadata_json->>'belongs_to_collection') is null)'
                                                  ' union (select count(*) from xxxx as row_count)) as temp',
                                                  class_guid, class_guid)
                else:
                    return await db_conn.fetchval('select 1')
    else:
        if list_type == "recent_addition":
            if not group_collection:
                if not include_remote:
                    return await db_conn.fetchval('select count(*) from (select distinct'
                                                  ' mm_metadata_guid from mm_media,'
                                                  ' mm_metadata_movie'
                                                  ' where mm_media_class_guid = $1'
                                                  ' and mm_media_metadata_guid'
                                                  ' = mm_metadata_guid and mm_media_json->>'DateAdded' >= $2'
                                                  ' and mm_metadata_json->'genres'->0->'name' ? $3) as temp',
                                                  class_guid, (datetime.datetime.now()
                                                               - datetime.timedelta(
                                    days=7)).strftime(
                            "%Y-%m-%d"), list_genre)
                else:
                    return await db_conn.fetchval(
                        'select count(*) from ((select distinct'
                        ' mm_metadata_guid from mm_media, mm_metadata_movie'
                        ' where mm_media_class_guid = $1 and mm_media_metadata_guid'
                        ' = mm_metadata_guid and mm_media_json->>'DateAdded' >= $2'
                        ' and mm_metadata_json->'genres'->0->'name' ? $3)'
                        ' union (select distinct mmr_metadata_guid from mm_media_remote,'
                        ' mm_metadata_movie where mmr_media_class_guid = $4'
                        ' and mmr_media_metadata_guid = mm_metadata_guid'
                        ' and mmr_media_json->>'DateAdded' >= $5'
                        ' and mm_metadata_json->'genres'->0->'name' ? $6)) as temp',
                        class_guid, (datetime.datetime.now()
                                     - datetime.timedelta(
                                    days=7)).strftime(
                            "%Y-%m-%d"), list_genre,
                        class_guid, (datetime.datetime.now()
                                     - datetime.timedelta(
                                    days=7)).strftime(
                            "%Y-%m-%d"), list_genre)
            else:
                return await db_conn.fetchval('select 1')
        else:
            if not group_collection:
                if not include_remote:
                    return await db_conn.fetchval('select count(*) from (select distinct'
                                                  ' mm_metadata_guid from mm_media,'
                                                  ' mm_metadata_movie'
                                                  ' where mm_media_class_guid = $1 '
                                                  'and mm_media_metadata_guid'
                                                  ' = mm_metadata_guid and mm_metadata_json->'genres'->0->'name' ? $2)'
                                                  ' as temp', class_guid, list_genre)
                else:
                    return await db_conn.fetchval(
                        'select count(*) from ((select distinct'
                        ' mm_metadata_guid'
                        ' from mm_media, mm_metadata_movie'
                        ' where mm_media_class_guid = $1 and mm_media_metadata_guid'
                        ' = mm_metadata_guid and mm_metadata_json->'genres'->0->'name' ? $2)'
                        ' union (select distinct mmr_media_metadata_guid from mm_media_remote,'
                        ' mm_metadata_movie where mmr_media_class_guid = $3'
                        ' and mmr_media_metadata_guid = mm_metadata_guid'
                        ' and mm_metadata_json->'genres'->0->'name' ? $4)) as temp',
                        class_guid, list_genre, class_guid,
                        list_genre)
            else:
                return await db_conn.fetchval('select 1')

 */