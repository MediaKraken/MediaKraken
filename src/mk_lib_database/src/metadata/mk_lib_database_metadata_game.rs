use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

pub async fn mk_lib_database_metadata_game_detail(
    sqlx_pool: &sqlx::PgPool,
    game_uuid: String,
) -> Result<(uuid::Uuid, serde_json::Value), sqlx::Error> {
    let row: (uuid::Uuid, serde_json::Value) = sqlx::query_as(
        r#"select gi_game_info_system_id, gi_game_info_json from mm_metadata_game_software_info where gi_game_info_id = $1"#,
    )
    .bind(game_uuid)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row)
}

pub async fn mk_lib_database_metadata_game_by_sha1(
    sqlx_pool: &sqlx::PgPool,
    sha1_hash: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        r#"select gi_game_info_id from mm_metadata_game_software_info where gi_game_info_sha1 = $1"#,
    )
    .bind(sha1_hash)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_game_by_blake3(
    sqlx_pool: &sqlx::PgPool,
    blake3_hash: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let row: (uuid::Uuid,) = sqlx::query_as(
        r#"select gi_game_info_id from mm_metadata_game_software_info where gi_game_info_blake3 = $1"#,
    )
    .bind(blake3_hash)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_game_count(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    starts_with: String,
    genre: String,
    status_filter: String,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*)
           from mm_metadata_game_software_info
           where
             ($1 = '' or gi_game_info_name &@ $1)
             and (
               $2 = ''
               or ($2 = '#' and left(lower(gi_game_info_name), 1) !~ '^[a-z0-9]$')
               or ($2 <> '#' and lower(gi_game_info_name) like lower($2) || '%')
             )
             and ($3 = '' or lower(coalesce(gi_gc_category, '')) = lower($3))
             and (
               $4 = ''
               or ($4 = 'favorite' and coalesce((gi_game_info_json->'user_status'->>'favorite')::boolean, false))
               or ($4 = 'watched' and coalesce((gi_game_info_json->'user_status'->>'watched')::boolean, false))
               or ($4 = 'unwatched' and not coalesce((gi_game_info_json->'user_status'->>'watched')::boolean, false))
               or ($4 = 'good' and coalesce((gi_game_info_json->'user_status'->>'good')::boolean, false))
               or ($4 = 'bad' and coalesce((gi_game_info_json->'user_status'->>'bad')::boolean, false))
               or ($4 = 'trash' and coalesce((gi_game_info_json->'user_status'->>'trash')::boolean, false))
             )"#,
    )
    .bind(search_value)
    .bind(starts_with)
    .bind(genre)
    .bind(status_filter)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaGameList {
    pub gi_game_info_id: uuid::Uuid,
    pub gi_game_info_short_name: String,
    pub gi_game_info_name: String,
    pub gi_year: Option<String>,
    pub gi_game_info_localimage: Option<serde_json::Value>,
    pub gs_game_system_name: String,
    pub gi_gc_category: Option<String>,
}

pub async fn mk_lib_database_metadata_game_read(
    sqlx_pool: &sqlx::PgPool,
    search_value: String,
    starts_with: String,
    genre: String,
    status_filter: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaGameList>, sqlx::Error> {
    sqlx::query_as(
        r#"select gi_game_info_id,
           gi_game_info_short_name,
           gi_game_info_name,
           gi_game_info_json->'machine'->>'year' as gi_year,
           gi_game_info_localimage,
           gs_game_system_name,
           gi_gc_category
           from mm_metadata_game_software_info, mm_metadata_game_systems_info
           where gi_game_info_system_id = gs_game_system_id
           and ($1 = '' or gi_game_info_name &@ $1)
           and (
               $2 = ''
               or ($2 = '#' and left(lower(gi_game_info_name), 1) !~ '^[a-z0-9]$')
               or ($2 <> '#' and lower(gi_game_info_name) like lower($2) || '%')
           )
           and ($3 = '' or lower(coalesce(gi_gc_category, '')) = lower($3))
           and (
               $4 = ''
               or ($4 = 'favorite' and coalesce((gi_game_info_json->'user_status'->>'favorite')::boolean, false))
               or ($4 = 'watched' and coalesce((gi_game_info_json->'user_status'->>'watched')::boolean, false))
               or ($4 = 'unwatched' and not coalesce((gi_game_info_json->'user_status'->>'watched')::boolean, false))
               or ($4 = 'good' and coalesce((gi_game_info_json->'user_status'->>'good')::boolean, false))
               or ($4 = 'bad' and coalesce((gi_game_info_json->'user_status'->>'bad')::boolean, false))
               or ($4 = 'trash' and coalesce((gi_game_info_json->'user_status'->>'trash')::boolean, false))
           )
           order by gi_game_info_name, gi_year
           offset $5
           limit $6"#,
    )
    .bind(search_value)
    .bind(starts_with)
    .bind(genre)
    .bind(status_filter)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await
}

pub async fn mk_lib_database_metadata_game_genres(
    sqlx_pool: &sqlx::PgPool,
) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as(
        r#"select distinct gi_gc_category
           from mm_metadata_game_software_info
           where coalesce(gi_gc_category, '') <> ''
           order by gi_gc_category"#,
    )
    .fetch_all(sqlx_pool)
    .await?;

    Ok(rows.into_iter().map(|row| row.0).collect())
}

pub async fn mk_lib_database_metadata_game_uuid_by_name_and_system(
    sqlx_pool: &sqlx::PgPool,
    game_name: String,
    game_system_short_name: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    if !game_system_short_name.is_empty() {
        let row: (uuid::Uuid,) = sqlx::query_as(
            r#"select gi_id from mm_metadata_game_software_info where gi_game_info_name = $1 and game_system_short_name = $2 limit 1"#,
        )
        .bind(game_name)
        .bind(game_system_short_name)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    } else {
        let row: (uuid::Uuid,) = sqlx::query_as(
            r#"select gi_id from mm_metadata_game_software_info where gi_game_info_name = $1 and gi_game_info_system_id IS NULL limit 1"#,
        )
        .bind(game_name)
        .fetch_one(sqlx_pool)
        .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaGameNameMatchList {
    gi_id: uuid::Uuid,
    gi_game_info_json: String,
}

pub async fn mk_lib_database_metadata_game_by_name_and_system(
    sqlx_pool: &sqlx::PgPool,
    game_name: String,
    game_system_short_name: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBMetaGameNameMatchList>, sqlx::Error> {
    if !game_system_short_name.is_empty() {
        // TODO fix game_system_short_name in query below
        sqlx::query_as(
            r#"select gi_id, gi_game_info_json from mm_metadata_game_software_info where gi_game_info_name = $1 and game_system_short_name = $2"#,
        )
        .bind(game_name)
        .bind(game_system_short_name)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    } else {
        sqlx::query_as(
            r#"select gi_id, gi_game_info_json from mm_metadata_game_software_info where gi_game_info_name = $1 and gi_game_info_system_id IS NULL"#,
        )
        .bind(game_name)
        .bind(offset)
        .bind(limit)
        .fetch_all(sqlx_pool)
        .await
    }
}

pub async fn mk_lib_database_metadata_game_insert(
    sqlx_pool: &sqlx::PgPool,
    game_system_id: Uuid,
    game_short_name: String,
    game_name: String,
    game_json: serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_metadata_game_software_info(gi_game_info_id, gi_game_info_system_id, gi_game_info_short_name, gi_game_info_name, gi_game_info_json) values ($1, $2, $3, $4, $5)"#,
    )
    .bind(new_guid)
    .bind(game_system_id)
    .bind(game_short_name)
    .bind(game_name)
    .bind(&game_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/*
// TODO port query
pub async fn db_meta_game_update(self, game_system_id, game_short_name, game_name, game_json,
                              db_connection=None):
    """
    Update game
    """
    await db_conn.execute("update mm_metadata_game_software_info"
                          " set gi_game_info_json = $1"
                          " where gi_system_id = $2"
                          " and gi_game_info_short_name = $3"
                          " and gi_game_info_name = $4",
                          game_json, game_system_id, game_short_name, game_name)


// TODO port query
pub async fn db_meta_game_by_name(self, game_short_name, game_name):
    """
    # return game info by name
    """
    return await db_conn.fetch("select gi_id, gi_system_id,"
                               " gi_game_info_json"
                               " from mm_metadata_game_software_info"
                               " where gi_game_info_name = $1"
                               " or game_short_name = $2", game_name, game_short_name)


// TODO port query
pub async fn db_meta_game_update_by_guid(self, game_id, game_json):
    """
    Update game by uuid
    """
    await db_conn.execute("update mm_metadata_game_software_info"
                          " set gi_game_info_json = $1"
                          " where gi_system_id = $2",
                          game_json, game_id)


// TODO port query
def db_meta_game_by_system_count(self, guid):
    """
    # game list by system count
    """
    self.db_cursor.execute("select count(*) from mm_metadata_game_software_info,"
                           " mm_metadata_game_systems_info"
                           " where gi_system_id = gs_id"
                           " and gs_id = $1", (guid,))
    return self.db_cursor.fetchone()[0]


// TODO port query
def db_meta_game_by_system(self, guid, offset=0, records=None):
    """
    # game list by system count
    """
    self.db_cursor.execute("select * from mm_metadata_game_software_info,"
                           " mm_metadata_game_systems_info"
                           " where gi_system_id = gs_id"
                           " and gs_id = $1"
                           " offset $2, limit $3", (guid, offset, records))
    try:
        return self.db_cursor.fetchone()
    except:
        return None

# poster, backdrop, etc
// TODO port query
def db_meta_game_image_random(self, return_image_type="Poster"):
    """
    Find random game image
    """
    // TODO little bobby tables
    self.db_cursor.execute("select gi_game_info_json->\"Images\"->\"thegamesdb\"->>\""
                           + return_image_type + "\" as image_json, gi_id"
                                                 " from mm_media, mm_metadata_game_software_info"
                                                 " where mm_media_metadata_guid = gi_id"
                                                 " and ("
                                                 "gi_game_info_json->\"Images\"->\"thegamesdb\"->>\""
                           + return_image_type + "\"" + ")::text != \"null\""
                                                        " order by random() limit 1")
    try:
        # then if no results.....a None will except which will then pass None, None
        image_json, metadata_id = self.db_cursor.fetchone()
        return image_json, metadata_id
    except:
        return None, None


// TODO port query
def db_meta_game_category_by_name(self, category_name):
    self.db_cursor.execute(
        "select gc_id from mm_game_category where gc_category = $1", (category_name,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None
 */

pub async fn mk_lib_database_metadata_game_category_insert(
    sqlx_pool: &sqlx::PgPool,
    category_name: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_game_category (gc_id, gc_category)
        values ($1, $2)"#,
    )
    .bind(new_guid)
    .bind(category_name)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}
