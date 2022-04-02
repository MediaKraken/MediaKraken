use sqlx::{types::Uuid, types::Json};
use sqlx::postgres::PgRow;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_media_update_metadata_guid(pool: &sqlx::PgPool,
                                                        mm_media_guid: Uuid,
                                                        mm_metadata_guid: Uuid,)
                                                        -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("update mm_media set mm_media_metadata_guid = $1 \
        where mm_media_guid = $2")
        .bind(mm_metadata_guid)
        .bind(mm_media_guid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_media_unmatched_count(pool: &sqlx::PgPool)
                                                   -> Result<(i32), sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select count(*) from mm_media \
        where mm_media_metadata_guid is NULL")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaUnmatchedList {
	mm_media_guid: uuid::Uuid,
	mm_media_path: String,
}

pub async fn mk_lib_database_media_unmatched_read(pool: &sqlx::PgPool,
                                                  offset: i32, limit: i32)
                                                  -> Result<Vec<DBMediaUnmatchedList>, sqlx::Error> {
    let select_query = sqlx::query("select mm_media_guid, \
        mm_media_path from mm_media \
        where mm_media_metadata_guid is NULL \
        order by mm_media_path offset $1 limit $2")
        .bind(offset)
        .bind(limit);
    let table_rows: Vec<DBCronList> = select_query
		.map(|row: PgRow| DBCronList {
			mm_media_guid: row.get("mm_media_guid"),
			mm_media_path: row.get("mm_media_path"),
		})
		.fetch_all(pool)
		.await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_matched_count(pool: &sqlx::PgPool)
                                                 -> Result<(i32), sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select count(*) from mm_media \
        where mm_media_metadata_guid is not NULL")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaKnownList {
    mm_media_path: String,
}

pub async fn mk_lib_database_media_known(pool: &sqlx::PgPool,
                                         offset: i32, limit: i32)
                                         -> Result<Vec<DBMediaKnownList>, sqlx::Error> {
    let select_query = sqlx::query("select mm_media_path \
        from mm_media where mm_media_guid \
        order by mm_media_path offset $1 limit $2")
        .bind(offset)
        .bind(limit);
    let table_rows: Vec<DBMediaKnownList> = select_query
		.map(|row: PgRow| DBMediaKnownList {
			mm_media_path: row.get("mm_media_path"),
		})
		.fetch_all(pool)
		.await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_media_insert(pool: &sqlx::PgPool,
                                          mm_media_guid: Uuid,
                                          mm_media_class_enum: i16,
                                          mm_media_path: String,
                                          mm_media_metadata_guid: Uuid,
                                          mm_media_ffprobe_json: serde_json::Value,
                                          mm_media_json: serde_json::Value)
                                          -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_media (mm_media_guid, mm_media_class_enum, \
         mm_media_path, mm_media_metadata_guid, mm_media_ffprobe_json, mm_media_json) \
         values ($1, $2, $3, $4, $5, $6)")
        .bind(mm_media_guid)
        .bind(mm_media_class_enum)
        .bind(mm_media_path)
        .bind(mm_media_metadata_guid)
        .bind(mm_media_ffprobe_json)
        .bind(mm_media_json)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_media_duplicate_detail_count(pool: &sqlx::PgPool,
                                                          mm_metadata_guid: Uuid)
                                                          -> Result<(i32), sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select count(*) from mm_media \
        where mm_media_metadata_guid = $1")
        .bind(mm_metadata_guid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_media_duplicate_count(pool: &sqlx::PgPool)
                                                   -> Result<(i32), sqlx::Error> {
    // TODO technically this will "dupe" things like subtitles atm
    let row: (i32, ) = sqlx::query_as("select count(*) from (select mm_media_metadata_guid \
        from mm_media where mm_media_metadata_guid is not null \
        group by mm_media_metadata_guid HAVING count(*) > 1) as total")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_media_path_by_uuid(pool: &sqlx::PgPool,
                                                mm_media_guid: Uuid)
                                                -> Result<(String), sqlx::Error> {
    let row: (String, ) = sqlx::query_as("select mm_media_path from mm_media \
        where mm_media_guid = $1")
        .bind(mm_media_guid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaDuplicateList {
	mm_media_metadata_guid: uuid::Uuid,
	mm_media_name: String,
	mm_count: i64,
}

pub async fn mk_lib_database_media_duplicate(pool: &sqlx::PgPool, offset: i32, limit: i32)
                                             -> Result<(DBMediaDuplicateList), sqlx::Error> {
    // TODO technically this will "dupe" things like subtitles atm
    let select_query = sqlx::query("select mm_media_metadata_guid, \
        mm_media_name, count(*) as mm_count \
        from mm_media, mm_metadata_movie \
        where mm_media_metadata_guid is not null \
        and mm_media_metadata_guid = mm_metadata_guid \
        group by mm_media_metadata_guid, \
        mm_media_name HAVING count(*) > 1 order by LOWER(mm_media_name) \
        offset $1 limit $2")
        .bind(offset)
        .bind(limit);
    let table_rows: Vec<DBMediaDuplicateList> = select_query
        .map(|row: PgRow| DBMediaDuplicateList {
            mm_media_metadata_guid: row.get("mm_media_metadata_guid"),
            mm_media_name: row.get("mm_media_name"),
            mm_count: row.get("mm_count"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMediaDuplicateDetailList {
	mm_media_guid: uuid::Uuid,
	mm_media_path: String,
	mm_media_ffprobe_json: Json,
}

pub async fn mk_lib_database_media_duplicate_detail(pool: &sqlx::PgPool,
                                                    mm_metadata_guid: Uuid,
                                                    offset: i32, limit: i32)
                                                    -> Result<(DBMediaDuplicateDetailList), sqlx::Error> {
    let select_query = sqlx::query("select mm_media_guid, \
        mm_media_path, mm_media_ffprobe_json \
        from mm_media where mm_media_guid \
        in (select mm_media_guid from mm_media \
        where mm_media_metadata_guid = $1 offset $2 limit $3")
        .bind(mm_metadata_guid)
        .bind(offset)
        .bind(limit);
    let table_rows: Vec<DBMediaDuplicateDetailList> = select_query
        .map(|row: PgRow| DBMediaDuplicateDetailList {
            mm_media_guid: row.get("mm_media_guid"),
            mm_media_path: row.get("mm_media_path"),
            mm_media_ffprobe_json: row.get("mm_media_ffprobe_json"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

/*
// TODO port query
def db_read_media(self, media_guid=None):
    """
    # read in all media unless guid specified
    """
    if media_guid is not None:
        self.db_cursor.execute(
            'select * from mm_media'
            ' where mm_media_guid = $1', (media_guid,))
        try:
            return self.db_cursor.fetchone()
        except:
            return None
    else:
        self.db_cursor.execute('select * from mm_media')
        return self.db_cursor.fetchall()


// TODO port query
def db_media_rating_update(self, media_guid, user_id, status_text):
    """
    # set favorite status for media
    """
    self.db_cursor.execute('SELECT mm_media_json from mm_media'
                           ' where mm_media_guid = $1 FOR UPDATE', (media_guid,))
    if status_text == 'watched' or status_text == 'mismatch':
        status_setting = True
    else:
        status_setting = status_text
        status_text = 'Rating'
    try:
        json_data = self.db_cursor.fetchone()['mm_media_json']
        if 'UserStats' not in json_data:
            json_data['UserStats'] = {}
        if user_id in json_data['UserStats']:
            json_data['UserStats'][user_id][status_text] = status_setting
        else:
            json_data['UserStats'][user_id] = {status_text: status_setting}
        self.db_update_media_json(media_guid, json.dumps(json_data))
        self.db_commit()
    except:
        self.db_rollback()
        return None


// TODO port query
def db_media_watched_checkpoint_update(self, media_guid, user_id, ffmpeg_time):
    """
    # set checkpoint for media (so can pick up where left off per user)
    """
    self.db_cursor.execute('SELECT mm_media_json from mm_media'
                           ' where mm_media_guid = $1 FOR UPDATE', (media_guid,))
    json_data = self.db_cursor.fetchone()['mm_media_json']
    if 'UserStats' not in json_data:
        json_data['UserStats'] = {}
    if user_id in json_data['UserStats']:
        json_data['UserStats'][user_id]['ffmpeg_checkpoint'] = ffmpeg_time
    else:
        json_data['UserStats'][user_id] = {'ffmpeg_checkpoint': ffmpeg_time}
    self.db_update_media_json(media_guid, json.dumps(json_data))
    self.db_commit()





// TODO port query
def db_update_media_json(self, media_guid, mediajson):
    """
    # update the mediajson
    """
    self.db_cursor.execute('update mm_media set mm_media_json = $1'
                           ' where mm_media_guid = $2',
                           (mediajson, media_guid))


// TODO port query
def db_media_by_metadata_guid(self, metadata_guid, media_class_uuid):
    """
    # fetch all media with METADATA match
    """
    self.db_cursor.execute('select mm_media_name,'
                           'mm_media_guid'
                           ' from mm_media, mm_metadata_movie'
                           ' where mm_media_metadata_guid = mm_metadata_guid'
                           ' and mm_media_metadata_guid = $1'
                           ' and mm_media_class_guid = $2',
                           (metadata_guid, media_class_uuid))
    return self.db_cursor.fetchall()


// TODO port query
def db_media_image_path(self, media_id):
    """
    # grab image path for media id NOT metadataid
    """
    self.db_cursor.execute('select mm_metadata_localimage_json->'Images' as mm_image'
                           ' from mm_media, mm_metadata_movie'
                           ' where mm_media_metadata_guid = mm_metadata_guid'
                           ' and mm_media_guid = $1', (media_id,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_localimage_json']
    except:
        return None


// TODO port query
def db_read_media_metadata_both(self, media_guid):
    """
    # read in metadata by id
    """
    self.db_cursor.execute('select mm_media_name,'
                           'mm_media_metadata_guid,'
                           'mm_media_ffprobe_json,'
                           'mm_media_json,'
                           'mm_metadata_json,'
                           'mm_metadata_localimage_json,'
                           'mm_metadata_media_id'
                           ' from mm_media, mm_metadata_movie'
                           ' where mm_media_metadata_guid = mm_metadata_guid'
                           ' and mm_media_guid = $1', (media_guid,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None


// TODO port query
def db_read_media_path_like(self, media_path):
    """
    # do a like class path match for trailers and extras
    """
    # use like since I won't be using the "root" directory but media within it
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                         message_text={'path like': media_path})
    self.db_cursor.execute('select mm_media_metadata_guid'
                           ' from mm_media'
                           ' where mm_media_path LIKE $1'
                           ' and mm_media_metadata_guid IS NOT NULL limit 1',
                           ((media_path + '%'),))
    try:
        return self.db_cursor.fetchone()['mm_media_metadata_guid']
    except:
        return None


// TODO port query
def db_read_media_new(self, offset=None, records=None, search_value=None, days_old=7):
    """
    # new media
    """
    if offset is None:
        self.db_cursor.execute('select mm_media_name,'
                               ' mm_media_guid,'
                               ' mm_media_class_guid'
                               ' from mm_media, mm_metadata_movie'
                               ' where mm_media_metadata_guid = mm_metadata_guid'
                               ' and mm_media_json->>'DateAdded' >= $1'
                               ' order by LOWER(mm_media_name),'
                               ' mm_media_class_guid',
                               ((datetime.datetime.now()
                                 - datetime.timedelta(days=days_old)).strftime("%Y-%m-%d"),))
    else:
        self.db_cursor.execute('select mm_media_name,'
                               ' mm_media_guid,'
                               ' mm_media_class_guid'
                               ' from mm_media, mm_metadata_movie'
                               ' where mm_media_metadata_guid = mm_metadata_guid'
                               ' and mm_media_json->>'DateAdded' >= $1'
                               ' order by LOWER(mm_media_name),'
                               ' mm_media_class_guid offset $2 limit $3',
                               ((datetime.datetime.now()
                                 - datetime.timedelta(days=days_old)).strftime("%Y-%m-%d"),
                                offset, records))
    return self.db_cursor.fetchall()


// TODO port query
def db_media_ffmeg_update(self, media_guid, ffmpeg_json):
    """
    Update the ffprobe json data
    """
    self.db_cursor.execute('update mm_media set mm_media_ffprobe_json = $1'
                           ' where mm_media_guid = $2', (ffmpeg_json, media_guid))


// TODO port query
def db_ffprobe_data(self, guid):
    self.db_cursor.execute('select mm_media_ffprobe_json from mm_media'
                           ' where mm_media_guid = $1', (guid,))
    return self.db_cursor.fetchone()[0]


// TODO port query
def db_ffprobe_all_media_guid(self, media_uuid, media_class_uuid):
    """
    # fetch all media with METADATA match
    """
    self.db_cursor.execute(
        'select distinct mm_media_guid,'
        'mm_media_ffprobe_json'
        ' from mm_media, mm_metadata_movie'
        ' where mm_media_metadata_guid = '
        '(select mm_media_metadata_guid'
        ' from mm_media where mm_media_guid = $1)'
        ' and mm_media_class_guid = $2',
        (media_uuid, media_class_uuid))
    return self.db_cursor.fetchall()

 */