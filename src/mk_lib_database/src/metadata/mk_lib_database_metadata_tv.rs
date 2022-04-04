use sqlx::{types::Uuid, types::Json};
use sqlx::postgres::PgRow;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_metadata_exists_tv(pool: &sqlx::PgPool,
                                                metadata_id: i32)
                                                -> Result<bool, sqlx::Error> {
    let row: (bool, ) = sqlx::query_as("select exists(select 1 from mm_metadata_tvshow \
        where mm_metadata_media_tvshow_id = $1 limit 1) as found_record limit 1")
        .bind(metadata_id)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

/*

// TODO port query
async def db_metatv_guid_by_tmdb(self, tmdb_uuid, db_connection=None):
    """
    # metadata guid by tmdb id
    """
    return await db_conn.fetchrow('select mm_metadata_tvshow_guid from mm_metadata_tvshow'
                                  ' where mm_metadata_media_tvshow_id->'themoviedb' ? $1',
                                  tmdb_uuid)['mm_metadata_tvshow_guid']


// TODO port query
async def db_meta_tv_detail(self, guid, db_connection=None):
    """
    # return metadata for tvshow
    """
    return await db_conn.fetchrow('select mm_metadata_tvshow_name,'
                                  ' mm_metadata_tvshow_json,'
                                  ' mm_metadata_tvshow_localimage_json,'
                                  ' COALESCE(mm_metadata_tvshow_localimage_json'
                                  '->'Images'->'tvmaze'->>'Poster','
                                  ' mm_metadata_tvshow_localimage_json'
                                  '->'Images'->'thetvdb'->>'Poster')'
                                  ' from mm_metadata_tvshow'
                                  ' where mm_metadata_tvshow_guid = $1',
                                  guid)


// TODO port query
async def db_meta_tv_episode(self, show_guid, season_number, episode_number, db_connection=None):
    """
    # grab episode detail
    """
    return await db_conn.fetchrow(
        'select jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'thetvdb''
        '->'Episode')b->'EpisodeName' as eps_name,'
        ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'thetvdb''
        '->'Episode')b->'FirstAired' as eps_first_air,'
        ' mm_metadata_tvshow_json->'Meta'->'thetvdb'->'Runtime' as eps_runtime,'
        ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'thetvdb''
        '->'Episode')b->'Overview' as eps_overview'
        ' from mm_metadata_tvshow where mm_metadata_tvshow_guid = $1',
        show_guid, str(season_number), str(episode_number))


// TODO port query
async def db_meta_tv_epsisode_by_id(self, show_guid, show_episode_id, db_connection=None):
    """
    # grab episode detail by eps id
    """
    // TODO tvmaze
    // TODO injection fix
    return await db_conn.fetchrow('select eps_data->'EpisodeName' as eps_name,'
                                  ' eps_data->'FirstAired' as eps_first_air,'
                                  ' eps_data->'Runtime' as eps_runtime,'
                                  ' eps_data->'Overview' as eps_overview,'
                                  ' eps_data->'filename' as eps_filename'
                                  ' from (select jsonb_array_elements_text('
                                  'mm_metadata_tvshow_json->'Meta'->'thetvdb''
                                  '->'Meta'->'Episode')b as eps_data'
                                  ' from mm_metadata_tvshow'
                                  ' where mm_metadata_tvshow_guid = $1)'
                                  ' as select_eps_data where eps_data @> '{ "id": "'
                                  + str(show_episode_id) + '" }'',
                                  show_guid)


// TODO port query
async def db_meta_tv_eps_season(self, show_guid, db_connection=None):
    """
    # grab tvmaze ep data for eps per season
    """
    season_data = {}
    for row_data in await db_conn.fetch(
            'select count(*) as ep_count, jsonb_array_elements_text(mm_metadata_tvshow_json'
            '->'Meta'->'thetvdb'->'Meta'->'Episode')b->'SeasonNumber' as season_num'
            ' from mm_metadata_tvshow where mm_metadata_tvshow_guid = $1'
            ' group by season_num', show_guid):
        # if row_data[0] in season_data:
        #     if season_data[row_data[0]] < row_data[1]:
        #         season_data[row_data[0]] = row_data[1]
        # else:
        #     season_data[row_data[0]] = row_data[1]
        season_data[int(row_data['season_num'])] = row_data['ep_count']
    return season_data


// TODO port query
async def db_meta_tv_list(self, offset=0, records=None, search_value=None, db_connection=None):
    """
    # return list of tvshows
    """
    // TODO order by release date
    return await db_conn.fetch('select mm_metadata_tvshow_guid,'
                               ' mm_metadata_tvshow_name,'
                               ' mm_metadata_tvshow_json->'first_air_date''
                               ' as air_date,'
                               ' mm_metadata_tvshow_localimage_json->'Poster''
                               ' as image_json from mm_metadata_tvshow'
                               ' order by LOWER(mm_metadata_tvshow_name),'
                               ' mm_metadata_tvshow_json->'first_air_date''
                               ' offset $1 limit $2',
                               offset, records)


// TODO port query
async def db_meta_tv_list_count(self, search_value=None, db_connection=None):
    """
    # tvshow count
    """
    if search_value is None:
        return await db_conn.fetchval('select count(*) from mm_metadata_tvshow '
                                      'where mm_metadata_tvshow_name % $1',
                                      search_value)
    else:
        return await db_conn.fetchval('select count(*) from mm_metadata_tvshow')


// TODO port query
async def db_meta_tv_season_eps_list(self, show_guid, season_number, db_connection=None):
    """
    # grab episodes within the season
    """
    episode_data = {}
    // TODO security check the seasonumber since from webpage addy - injection
    await db_conn.fetch(
        'select eps_data->'id' as eps_id, eps_data->'EpisodeNumber' as eps_num,'
        ' eps_data->'EpisodeName' as eps_name,'
        ' eps_data->'filename' as eps_filename'
        ' from (select jsonb_array_elements_text('
        'mm_metadata_tvshow_json->'Episode')b as eps_data'
        ' from mm_metadata_tvshow where mm_metadata_tvshow_guid = $1)'
        ' as select_eps_data where eps_data @> '{ "SeasonNumber": "'
        + str(season_number) + '" }'', show_guid)
    # id, episode_number, episode_name, filename
    for row_data in self.db_cursor.fetchall():
        if row_data['eps_filename'] is None:
            episode_data[row_data['eps_num']] \
                = (row_data['eps_name'], 'missing_icon.jpg', row_data['eps_id'],
                   str(season_number))
        else:
            episode_data[row_data['eps_num']] \
                = (
                row_data['eps_name'], row_data['eps_filename'], row_data['eps_id'],
                str(season_number))
    return episode_data


// TODO port query
async def db_meta_tv_count_by_id(self, guid, db_connection=None):
    """
    # does movie exist already by provider id
    """
    return await db_conn.fetchval('select exists(select 1 from mm_metadata_tvshow'
                                  ' where mm_metadata_media_tvshow_id = $1 limit 1) limit 1', guid)



// TODO port query
def db_metatv_insert_tmdb(self, uuid_id, series_id, data_title, data_json,
                          data_image_json):
    """
    # insert metadata from themoviedb
    """
    self.db_cursor.execute('insert into mm_metadata_tvshow (mm_metadata_tvshow_guid,'
                           ' mm_metadata_media_tvshow_id,'
                           ' mm_metadata_tvshow_name,'
                           ' mm_metadata_tvshow_json,'
                           ' mm_metadata_tvshow_localimage_json)'
                           ' values ($1,$2,$3,$4,$5)', (uuid_id, series_id, data_title,
                                                        data_json, data_image_json))
    self.db_commit()


// TODO port query
def db_metatv_guid_by_tvshow_name(self, tvshow_name, tvshow_year=None):
    """
    # metadata guid by name
    """
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text=
    {'db_metatv_guid_by_tvshow_name': str(tvshow_name),
     'year': tvshow_year})
    metadata_guid = None
    if tvshow_year is None:
        self.db_cursor.execute('select mm_metadata_tvshow_guid from mm_metadata_tvshow'
                               ' where LOWER(mm_metadata_tvshow_name) = $1',
                               (tvshow_name.lower(),))
    else:
        // TODO jin index firstaird and premiered
        // TODO check tvmaze as well
        self.db_cursor.execute('select mm_metadata_tvshow_guid from mm_metadata_tvshow'
                               ' where (LOWER(mm_metadata_tvshow_name) = $1)'
                               ' and (substring(mm_metadata_tvshow_json->'Meta'->'thetvdb'->'Meta''
                               '->>'FirstAired' from 0 for 5) in ($2,$3,$4,$5,$6,$7,$8)'
                               ' or substring(mm_metadata_tvshow_json->'Meta'->'tvmaze'->>'premiered''
                               ' from 0 for 5) in ($9,$10,$11,$12,$13,$14,$15))',
                               (tvshow_name.lower(), str(tvshow_year),
                                str(int(tvshow_year) + 1),
                                str(int(tvshow_year) + 2),
                                str(int(tvshow_year) + 3),
                                str(int(tvshow_year) - 1),
                                str(int(tvshow_year) - 2),
                                str(int(tvshow_year) - 3),
                                str(tvshow_year),
                                str(int(tvshow_year) + 1),
                                str(int(tvshow_year) + 2),
                                str(int(tvshow_year) + 3),
                                str(int(tvshow_year) - 1),
                                str(int(tvshow_year) - 2),
                                str(int(tvshow_year) - 3)))
    for row_data in self.db_cursor.fetchall():
        metadata_guid = row_data['mm_metadata_tvshow_guid']
        common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text={
            "db find metadata tv guid":
                metadata_guid})
        break
    return metadata_guid


// TODO port query
def db_metatv_guid_by_imdb(self, imdb_uuid):
    """
    # metadata guid by imdb id
    """
    self.db_cursor.execute('select mm_metadata_tvshow_guid from mm_metadata_tvshow'
                           ' where mm_metadata_media_tvshow_id->'imdb' ? $1', (imdb_uuid,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_tvshow_guid']
    except:
        return None


// TODO port query
def db_metatv_guid_by_tvmaze(self, tvmaze_uuid):
    """
    # metadata guid by tvmaze id
    """
    self.db_cursor.execute('select mm_metadata_tvshow_guid from mm_metadata_tvshow'
                           ' where mm_metadata_media_tvshow_id->'tvmaze' ? $1', (tvmaze_uuid,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_tvshow_guid']
    except:
        return None


// TODO port query
def db_metatv_guid_by_tmdb(self, tmdb_uuid):
    """
    # metadata guid by tmdb id
    """
    self.db_cursor.execute('select mm_metadata_tvshow_guid from mm_metadata_tvshow'
                           ' where mm_metadata_media_tvshow_id->'themoviedb' ? $1',
                           (tmdb_uuid,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_tvshow_guid']
    except:
        return None


// TODO port query
def db_metatv_guid_by_rt(self, rt_uuid):
    """
    # metadata guid by rt id
    """
    self.db_cursor.execute('select mm_metadata_tvshow_guid from mm_metadata_tvshow'
                           ' where mm_metadata_media_tvshow_id->'rottentomatoes' ? $1',
                           (rt_uuid,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_tvshow_guid']
    except:
        return None


// TODO port query
def db_meta_tvshow_list_count(self, search_value=None):
    """
    # tvshow count
    """
    self.db_cursor.execute('select count(*) from mm_metadata_tvshow')
    return self.db_cursor.fetchone()[0]


// TODO port query
def db_meta_tvshow_list(self, offset=0, records=None, search_value=None):
    """
    # return list of tvshows
    """
    // TODO order by release date
    # COALESCE - priority over one column
    self.db_cursor.execute('select mm_metadata_tvshow_guid,mm_metadata_tvshow_name,'
                           ' COALESCE(mm_metadata_tvshow_json->'Meta'->'tvmaze'->'premiered','
                           ' mm_metadata_tvshow_json->'Meta'->'thetvdb'->'Meta'->'Series''
                           '->'FirstAired') as air_date, COALESCE(mm_metadata_tvshow_localimage_json->'Images''
                           '->'tvmaze'->>'Poster', mm_metadata_tvshow_localimage_json->'Images''
                           '->'thetvdb'->>'Poster') as image_json from mm_metadata_tvshow'
                           ' where mm_metadata_tvshow_guid in (select mm_metadata_tvshow_guid'
                           ' from mm_metadata_tvshow order by LOWER(mm_metadata_tvshow_name)'
                           ' offset $1 limit $2) order by LOWER(mm_metadata_tvshow_name)',
                           (offset, records))
    return self.db_cursor.fetchall()


// TODO port query
def db_meta_tvshow_update_image(self, image_json, metadata_uuid):
    """
    # update image json
    """
    self.db_cursor.execute('update mm_metadata_tvshow'
                           ' set mm_metadata_tvshow_localimage_json = $1'
                           ' where mm_metadata_tvshow_guid = $2',
                           (image_json, metadata_uuid))
    self.db_commit()


// TODO port query
def db_meta_tvshow_images_to_update(self, image_type):
    """
    # fetch tv rows to update
    """
    if image_type == 'tvmaze':
        self.db_cursor.execute("select mm_metadata_tvshow_json->'Meta'->'tvmaze','\
            'mm_metadata_tvshow_guid from mm_metadata_tvshow'\
            ' where mm_metadata_tvshow_localimage_json->'Images'->'tvmaze'->'Redo' = 'true'")
    else if image_type == 'thetvdb':
        self.db_cursor.execute("select mm_metadata_tvshow_json->'Meta'->'thetvdb','\
            'mm_metadata_tvshow_guid from mm_metadata_tvshow'\
            ' where mm_metadata_tvshow_localimage_json->'Images'->'thetvdb'->'Redo' = 'true'")
    return self.db_cursor.fetchall()


// TODO port query
def db_meta_tvshow_detail(self, guid):
    """
    # return metadata for tvshow
    """
    self.db_cursor.execute('select mm_metadata_tvshow_name, mm_metadata_tvshow_json,'
                           ' mm_metadata_tvshow_localimage_json,'
                           ' COALESCE(mm_metadata_tvshow_localimage_json'
                           '->'Images'->'tvmaze'->>'Poster','
                           ' mm_metadata_tvshow_localimage_json'
                           '->'Images'->'thetvdb'->>'Poster') from mm_metadata_tvshow'
                           ' where mm_metadata_tvshow_guid = $1', (guid,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None


// TODO port query
def db_read_tvmeta_episodes(self, show_guid):
    """
    # read in the tv episodes metadata by guid
    """
    return self.db_cursor.fetchall()


// TODO port query
def db_read_tvmeta_eps_season(self, show_guid):
    """
    # grab tvmaze ep data for eps per season
    """
    season_data = {}
    # self.db_cursor.execute('select jsonb_array_elements_text(COALESCE((mm_metadata_tvshow_json'
    #                        '->'Meta'->'tvmaze'->'_embedded'->'episodes')b->'season', '
    #                         '(mm_metadata_tvshow_json->'Meta'->'thetvdb'->'Meta'->'Episode')'
    #                         'b->'SeasonNumber')),'
    #                         'jsonb_array_elements_text(COALESCE((mm_metadata_tvshow_json'
    #                         '->'Meta'->'tvmaze'->'_embedded'->'episodes')b->'number','
    #                         '(mm_metadata_tvshow_json->'Meta'->'thetvdb'->'Meta'->'Episode')'
    #                         'b->'EpisodeNumber'))'
    #                         'from mm_metadata_tvshow where mm_metadata_tvshow_guid = $1', (show_guid,))

    self.db_cursor.execute(
        'select count(*) as ep_count, jsonb_array_elements_text(mm_metadata_tvshow_json'
        '->'Meta'->'thetvdb'->'Meta'->'Episode')b->'SeasonNumber' as season_num'
        ' from mm_metadata_tvshow where mm_metadata_tvshow_guid = $1'
        ' group by season_num', (show_guid,))
    for row_data in self.db_cursor.fetchall():
        # if row_data[0] in season_data:
        #     if season_data[row_data[0]] < row_data[1]:
        #         season_data[row_data[0]] = row_data[1]
        # else:
        #     season_data[row_data[0]] = row_data[1]
        season_data[int(row_data['season_num'])] = row_data['ep_count']
    return season_data


// TODO port query
def db_read_tvmeta_season_eps_list(self, show_guid, season_number):
    """
    # grab episodes within the season
    """
    episode_data = {}
    # self.db_cursor.execute('select jsonb_array_elements_text(mm_metadata_tvshow_json'
    #     '->'Meta'->'tvmaze'->'_embedded'->'episodes')b->'season','
    #     ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'tvmaze''
    #     '->'_embedded'->'episodes')b->'number','
    #     ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'tvmaze''
    #     '->'_embedded'->'episodes')b->'name','
    #     ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'tvmaze''
    #     '->'_embedded'->'episodes')b->'id', mm_metadata_tvshow_localimage_json'
    #     '->'Images'->'tvmaze'->'Episodes','

    // TODO security check the seasonumber since from webpage addy - injection
    self.db_cursor.execute(
        'select eps_data->'id' as eps_id, eps_data->'EpisodeNumber' as eps_num,'
        ' eps_data->'EpisodeName' as eps_name,'
        ' eps_data->'filename' as eps_filename'
        ' from (select jsonb_array_elements_text('
        'mm_metadata_tvshow_json->'Meta'->'thetvdb'->'Meta'->'Episode')b as eps_data'
        ' from mm_metadata_tvshow where mm_metadata_tvshow_guid = $1)'
        ' as select_eps_data where eps_data @> '{ "SeasonNumber": "'
        + str(season_number) + '" }'', (show_guid,))
    # id, episode_number, episode_name, filename
    for row_data in self.db_cursor.fetchall():
        if row_data['eps_filename'] is None:
            episode_data[row_data['eps_num']] \
                = (row_data['eps_name'], 'missing_icon.jpg', row_data['eps_id'],
                   str(season_number))
        else:
            episode_data[row_data['eps_num']] \
                = (
                row_data['eps_name'], row_data['eps_filename'], row_data['eps_id'],
                str(season_number))
    return episode_data


// TODO port query
def db_read_tvmeta_epsisode_by_id(self, show_guid, show_episode_id):
    """
    # grab episode detail by eps id
    """
    // TODO tvmaze
    // TODO injection fix
    self.db_cursor.execute('select eps_data->'EpisodeName' as eps_name,'
                           ' eps_data->'FirstAired' as eps_first_air,'
                           ' eps_data->'Runtime' as eps_runtime,'
                           ' eps_data->'Overview' as eps_overview,'
                           ' eps_data->'filename' as eps_filename'
                           ' from (select jsonb_array_elements_text('
                           'mm_metadata_tvshow_json->'Meta'->'thetvdb'->'Meta'->'Episode')b as eps_data'
                           ' from mm_metadata_tvshow where mm_metadata_tvshow_guid = $1)'
                           ' as select_eps_data where eps_data @> '{ "id": "'
                           + str(show_episode_id) + '" }'', (show_guid,))
    return self.db_cursor.fetchone()


// TODO port query
def db_read_tvmeta_episode(self, show_guid, season_number, episode_number):
    """
    # grab episode detail
    """
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                         message_text={"show guid": show_guid,
                                                                       'season': season_number,
                                                                       'eps': episode_number})
    # self.db_cursor.execute('(select
    #     ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'tvmaze''
    #     '->'_embedded'->'episodes')b->'name','
    #     ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'tvmaze''
    #     '->'_embedded'->'episodes')b->'airstamp','
    #     ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'tvmaze''
    #     '->'_embedded'->'episodes')b->'runtime','
    #     ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'tvmaze''
    #     '->'_embedded'->'episodes')b->'summary','

    self.db_cursor.execute(
        'select jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'thetvdb''
        '->'Episode')b->'EpisodeName' as eps_name,'
        ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'thetvdb''
        '->'Episode')b->'FirstAired' as eps_first_air,'
        ' mm_metadata_tvshow_json->'Meta'->'thetvdb'->'Runtime' as eps_runtime,'
        ' jsonb_array_elements_text(mm_metadata_tvshow_json->'Meta'->'thetvdb''
        '->'Episode')b->'Overview' as eps_overview'
        ' from mm_metadata_tvshow where mm_metadata_tvshow_guid = $1',
        (show_guid, str(season_number), str(episode_number)))
    return self.db_cursor.fetchone()


# total episdoes in metadata from tvmaze
# jsonb_array_length(mm_metadata_tvshow_json->'Meta'->'tvmaze'->'_embedded'->'episodes')

# "last" episode season number from tvmaze
# mm_metadata_tvshow_json->'Meta'->'tvmaze'->'_embedded'->'episodes'->(jsonb_array_length(
# mm_metadata_tvshow_json->'Meta'->'tvmaze'->'_embedded'->'episodes')
# - 1)->'season'

# poster, backdrop, etc
// TODO port query
def db_meta_tvshow_image_random(self, return_image_type='Poster'):
    """
    Find random tv show image
    """
    // TODO little bobby tables
    self.db_cursor.execute(
        'select mm_metadata_tvshow_localimage_json->'Images'->'thetvdb'->>''
        + return_image_type + '' as image_json,mm_metadata_guid from mm_media,mm_metadata_tvshow'
                              ' where mm_media_metadata_guid = mm_metadata_guid'
                              ' and (mm_metadata_tvshow_localimage_json->'Images'->'thetvdb'->>''
        + return_image_type + ''' + ')::text != 'null' order by random() limit 1')
    try:
        # then if no results.....a None will except which will then pass None, None
        image_json, metadata_id = self.db_cursor.fetchone()
        return image_json, metadata_id
    except:
        return None, None

 */