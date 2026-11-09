use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicList {
    pub mm_metadata_album_guid: uuid::Uuid,
    pub mm_metadata_album_artist: String,
    pub mm_metadata_album_name: String,
    pub mm_metadata_album_json: serde_json::Value,
    pub mm_metadata_album_localimage: String,
}

pub async fn mk_lib_database_metadata_music_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
) -> Result<i64, sqlx::Error> {
    if !search_value.is_empty() {
        let row: (i64,) = sqlx::query_as(
            r#"select count(*) from mm_mtadata_album where mm_metadata_album_name &@ $1"#,
        )
        .bind(search_value)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_metadata_album"#)
            .fetch_one(sqlx_pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_music_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaMusicList>, sqlx::Error> {
    // TODO, only grab the poster locale from json
    // TODO order by release year
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_metadata_album_guid, mm_metadata_album_name, mm_metadata_album_json, mm_metadata_album_localimage from mm_metadata_album where LOWER(mm_metadata_album_name) % LOWER($1) order by LOWER(mm_metadata_album_name) offset $2 limit $3"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select mm_metadata_album_guid, mm_metadata_album_name, mm_metadata_album_json, mm_metadata_album_localimage from mm_metadata_album order by LOWER(mm_metadata_album_name) offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

// Not ported: identical query to `db_meta_album_by_guid` (see
// `mk_lib_database_metadata_album_by_guid`).

// Not ported: identical query to `db_meta_songs_by_album_guid` (see
// `mk_lib_database_metadata_songs_by_album_guid`).

/*
// Not ported: the three tables have no join predicates in the Python
// original (cartesian product), and there is no musician link column on
// `mm_metadata_music` to join on. The Python original already noted that
// album/musician fields "don't exist on the database".

// TODO port query
def db_music_lookup(self, artist_name, album_name, song_title):
    """
     # query to see if song is in local DB
     """
    // TODO the following fields don't exist on the database (album and musician)
    // TODO order by release year
    self.db_cursor.execute('select mm_metadata_music_guid,'
                           ' mm_metadata_media_music_id->'Mbrainz' as mbrainz from '
                           'mm_metadata_music,'
                           ' mm_metadata_album,'
                           ' mm_metadata_musician'
                           ' where lower(mm_metadata_musician_name) = $1'
                           ' and lower(mm_metadata_album_name) = $2'
                           ' and lower(mm_metadata_music_name) = $3',
                           (artist_name.lower(), album_name.lower(), song_title.lower()))
    try:
        return self.db_cursor.fetchone()
    except:
        return None
*/

/// Musician data by guid, all columns.
pub async fn mk_lib_database_metadata_musician_by_guid(
    sqlx_pool: &sqlx::PgPool,
    guid: uuid::Uuid,
) -> Result<Option<PgRow>, sqlx::Error> {
    sqlx::query(
        r#"select * from mm_metadata_musician
        where mm_metadata_musician_guid = $1"#,
    )
    .bind(guid)
    .fetch_optional(sqlx_pool)
    .await
}

/// Insert a musician; returns the new guid.
pub async fn mk_lib_database_metadata_musician_add(
    sqlx_pool: &sqlx::PgPool,
    data_name: &str,
    data_id: &serde_json::Value,
    data_json: &serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_musician (mm_metadata_musician_guid,
        mm_metadata_musician_name, mm_metadata_musician_id, mm_metadata_musician_json)
        values ($1,$2,$3,$4)"#,
    )
    .bind(new_guid)
    .bind(data_name)
    .bind(data_id)
    .bind(data_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/// Album data by guid, all columns.
pub async fn mk_lib_database_metadata_album_by_guid(
    sqlx_pool: &sqlx::PgPool,
    guid: uuid::Uuid,
) -> Result<Option<PgRow>, sqlx::Error> {
    sqlx::query(r#"select * from mm_metadata_album where mm_metadata_album_guid = $1"#)
        .bind(guid)
        .fetch_optional(sqlx_pool)
        .await
}

/// Insert an album; returns the new guid.
pub async fn mk_lib_database_metadata_album_add(
    sqlx_pool: &sqlx::PgPool,
    data_name: &str,
    data_id: &serde_json::Value,
    data_json: &serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_album (mm_metadata_album_guid,
        mm_metadata_album_name, mm_metadata_album_id, mm_metadata_album_json)
        values ($1,$2,$3,$4)"#,
    )
    .bind(new_guid)
    .bind(data_name)
    .bind(data_id)
    .bind(data_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/// Song data by guid, all columns.
pub async fn mk_lib_database_metadata_song_by_guid(
    sqlx_pool: &sqlx::PgPool,
    guid: uuid::Uuid,
) -> Result<Option<PgRow>, sqlx::Error> {
    sqlx::query(r#"select * from mm_metadata_music where mm_metadata_music_guid = $1"#)
        .bind(guid)
        .fetch_optional(sqlx_pool)
        .await
}

/// Insert a song; returns the new guid.
pub async fn mk_lib_database_metadata_song_add(
    sqlx_pool: &sqlx::PgPool,
    data_name: &str,
    data_id: &serde_json::Value,
    data_json: &serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_music (mm_metadata_music_guid, mm_metadata_music_name,
        mm_metadata_media_music_id, mm_metadata_music_json)
        values ($1,$2,$3,$4)"#,
    )
    .bind(new_guid)
    .bind(data_name)
    .bind(data_id)
    .bind(data_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/// Song list from an album guid, all columns.
///
/// NOTE: the Python original filtered on a placeholder column named `blah`;
/// ported as `mm_metadata_music_album_guid` (the album link on
/// `mm_metadata_music`).
pub async fn mk_lib_database_metadata_songs_by_album_guid(
    sqlx_pool: &sqlx::PgPool,
    album_guid: uuid::Uuid,
) -> Result<Vec<PgRow>, sqlx::Error> {
    sqlx::query(
        r#"select * from mm_metadata_music
        where mm_metadata_music_album_guid = $1
        order by lower(mm_metadata_music_name)"#,
    )
    .bind(album_guid)
    .fetch_all(sqlx_pool)
    .await
}

// db_meta_album_list: already ported as `mk_lib_database_metadata_music_read`.

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaMusicianList {
    pub mm_metadata_musician_guid: uuid::Uuid,
    pub mm_metadata_musician_name: String,
    pub mm_metadata_musician_json: serde_json::Value,
}

/// Musician metadata list.
pub async fn mk_lib_database_metadata_musician_list(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaMusicianList>, sqlx::Error> {
    // TODO, only grab the poster locale from json
    if !search_value.is_empty() {
        sqlx::query_as(
            r#"select mm_metadata_musician_guid, mm_metadata_musician_name, mm_metadata_musician_json from mm_metadata_musician where mm_metadata_musician_name % $1 order by LOWER(mm_metadata_musician_name) offset $2 limit $3"#,
        )
        .bind(search_value)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select mm_metadata_musician_guid, mm_metadata_musician_name, mm_metadata_musician_json from mm_metadata_musician order by LOWER(mm_metadata_musician_name) offset $1 limit $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

/*
// Not ported: the SQL is fully commented out in the Python original; only
// the try/except around a never-run fetchone remains.

// TODO port query
def db_meta_album_image_random(self):
    """
     Find random album cover image
     """
    # self.db_cursor.execute('select mm_metadata_localimage_json->'Images'->'themoviedb'->>''
    #     + return_image_type + '' as image_json,mm_metadata_guid from mm_media,mm_metadata_movie'\
    #     ' where mm_media_metadata_guid = mm_metadata_guid'\
    #     ' and (mm_metadata_localimage_json->'Images'->'themoviedb'->>''
    #     + return_image_type + ''' + ')::text != 'null' order by random() limit 1')
    try:
        // then if no results.....a None will except which will then pass None, None
        image_json, metadata_id = self.db_cursor.fetchone()
        return image_json, metadata_id
    except:
        return None, None
*/

/// Music guid by provider uuid (e.g. "Mbrainz").
///
/// The provider is embedded in a JSON path in the SQL, so it is restricted to
/// `[A-Za-z0-9_-]`; the Python original was injectable.
pub async fn mk_lib_database_metadata_music_by_provider_uuid(
    sqlx_pool: &sqlx::PgPool,
    provider: &str,
    provider_uuid: &str,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    if !is_valid_json_path_key(provider) {
        return Err(sqlx::Error::Configuration(
            format!("invalid provider: {provider}").into(),
        ));
    }
    let sql = format!(
        r#"select mm_metadata_music_guid from mm_metadata_music
        where mm_metadata_media_music_id->'{provider}' ? $1"#,
    );
    let row: Option<(uuid::Uuid,)> = sqlx::query_as(sqlx::AssertSqlSafe(sql))
        .bind(provider_uuid)
        .fetch_optional(sqlx_pool)
        .await?;
    Ok(row.map(|row| row.0))
}

/// True when the key is safe to embed in a SQL JSON path.
fn is_valid_json_path_key(key: &str) -> bool {
    !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}
