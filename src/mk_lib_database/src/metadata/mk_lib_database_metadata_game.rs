use sqlx::postgres::PgRow;
use uuid::Uuid;

pub async fn mk_lib_database_metadata_game_by_uuid(pool: &sqlx::PgPool, game_uuid: uuid::Uuid)
                                                   -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select gi_id, gi_system_id, gi_game_info_json \
        from mm_metadata_game_software_info where gi_id = $1")
        .bind(game_uuid)
        .fetch_one(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_metadata_game_by_sha1(pool: &sqlx::PgPool, sha1_hash: String)
                                                   -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select gi_id from mm_metadata_game_software_info \
        where gi_game_info_sha1 = $1")
        .bind(sha1_hash)
        .fetch_one(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_metadata_game_by_blake3(pool: &sqlx::PgPool, blake3_hash: String)
                                                     -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select gi_id from mm_metadata_game_software_info \
        where gi_game_info_blake3 = $1")
        .bind(blake3_hash)
        .fetch_one(pool)
        .await?;
    Ok(rows)
}

// pub async fn mk_lib_database_metadata_game_read(pool: &sqlx::PgPool,
//                                                 offset: i32, limit: i32)
//                                                 -> Result<Vec<PgRow>, sqlx::Error> {
//     let rows: Vec<PgRow> = sqlx::query("select gi_id, gi_game_info_short_name \
//         from mm_metadata_game_software_info \
//         where gi_system_id is null and gi_gc_category is null \
//         order by gi_short_name offset $1 limit $2")
//         .bind(offset)
//         .bind(limit)
//         .fetch_all(pool)
//         .await?;
//     Ok(rows)
// }

pub async fn mk_lib_database_metadata_game_count(pool: &sqlx::PgPool,
                                                 search_value: String)
                                                 -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_game_software_info \
            where gi_game_info_name %% %s")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_game_software_info")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_metadata_game_read(pool: &sqlx::PgPool,
                                                 search_value: String,
                                                 offset: i32, limit: i32)
                                                 -> Result<Vec<PgRow>, sqlx::Error> {
    if search_value != "" {
        let rows = sqlx::query("select gi_id,gi_game_info_short_name, gi_game_info_name, \
             gi_game_info_json->\"year\", gs_game_system_json->\"description\" \
             from mm_metadata_game_software_info, mm_metadata_game_systems_info \
             where gi_system_id = gs_id  and gi_game_info_name % $1 \
             order by gi_game_info_name, gi_game_info_json->\"year\" \
             offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    } else {
        let rows = sqlx::query("select gi_id,gi_game_info_short_name, gi_game_info_name, \
            gi_game_info_json->\"year\", gs_game_system_json->\"description\" \
            from mm_metadata_game_software_info, mm_metadata_game_systems_info \
            where gi_system_id = gs_id order by gi_game_info_name, gi_game_info_json->\"year\" \
            offset $1 limit $2")
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

/*

async def db_meta_game_insert(self, game_system_id, game_short_name, game_name, game_json,
                              db_connection=None):
    """
    Insert game
    """
    new_game_id = uuid.uuid4()
    await db_conn.execute("insert into mm_metadata_game_software_info(gi_id,"
                          " gi_system_id,"
                          " gi_game_info_short_name,"
                          " gi_game_info_name,"
                          " gi_game_info_json)"
                          " values ($1, $2, $3, $4, $5)",
                          new_game_id, game_system_id, game_short_name, game_name,
                          game_json)
    return new_game_id


async def db_meta_game_update(self, game_system_id, game_short_name, game_name, game_json,
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


async def db_meta_game_by_name(self, game_short_name, game_name, db_connection=None):
    """
    # return game info by name
    """
    return await db_conn.fetch("select gi_id, gi_system_id,"
                               " gi_game_info_json"
                               " from mm_metadata_game_software_info"
                               " where gi_game_info_name = $1"
                               " or game_short_name = $2", game_name, game_short_name)


async def db_meta_game_update_by_guid(self, game_id, game_json, db_connection=None):
    """
    Update game by uuid
    """
    await db_conn.execute("update mm_metadata_game_software_info"
                          " set gi_game_info_json = $1"
                          " where gi_system_id = $2",
                          game_json, game_id)


def db_meta_game_by_system_count(self, guid):
    """
    # game list by system count
    """
    self.db_cursor.execute("select count(*) from mm_metadata_game_software_info,"
                           " mm_metadata_game_systems_info"
                           " where gi_system_id = gs_id"
                           " and gs_id = %s", (guid,))
    return self.db_cursor.fetchone()[0]


def db_meta_game_by_system(self, guid, offset=0, records=None):
    """
    # game list by system count
    """
    self.db_cursor.execute("select * from mm_metadata_game_software_info,"
                           " mm_metadata_game_systems_info"
                           " where gi_system_id = gs_id"
                           " and gs_id = %s"
                           " offset %s, limit %s", (guid, offset, records))
    try:
        return self.db_cursor.fetchone()
    except:
        return None


def db_meta_game_by_name_and_system(self, game_name, game_system_short_name):
    """
    # game by name and system short name
    """
    if game_system_short_name is None:
        self.db_cursor.execute("select gi_id, gi_game_info_json"
                               " from mm_metadata_game_software_info"
                               " where gi_game_info_name = %s and gi_system_id IS NULL",
                               (game_name,))
    else:
        self.db_cursor.execute("select gi_id, gi_game_info_json"
                               " from mm_metadata_game_software_info"
                               " where gi_game_info_name = %s and gi_system_id = %s",
                               (game_name, game_system_short_name))
    return self.db_cursor.fetchall()


# poster, backdrop, etc
def db_meta_game_image_random(self, return_image_type="Poster"):
    """
    Find random game image
    """
    # TODO little bobby tables
    self.db_cursor.execute("select gi_game_info_json->\"Images\"->\"thegamesdb\"->>\""
                           + return_image_type + "\" as image_json,gi_id"
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


def db_meta_game_category_by_name(self, category_name):
    self.db_cursor.execute(
        "select gc_id from mm_game_category where gc_category = %s", (category_name,))
    try:
        return self.db_cursor.fetchone()
    except:
        return None


def db_meta_game_category_add(self, category_name):
    category_uuid = uuid.uuid4()
    self.db_cursor.execute("insert into mm_game_category (gc_id, gc_category)"
                           " values (%s, %s)",
                           (category_uuid, category_name))
    self.db_cursor.commit()
    return category_uuid

 */