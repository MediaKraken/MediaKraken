use sqlx::postgres::PgRow;
use uuid::Uuid;

pub async fn mk_lib_database_metadata_game_system_by_uuid(pool: &sqlx::PgPool,
                                                          game_sys_uuid: uuid::Uuid)
                                                          -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select * from mm_metadata_game_systems_info \
        where gs_id = $1")
        .bind(game_sys_uuid)
        .fetch_one(pool)
        .await?;
    Ok(rows)
}
/*

async def db_meta_game_system_list_count(self, search_value=None, db_connection=None):
    """
    Return game system count
    """
    if search_value is not None:
        return await db_conn.fetchval(
            'select count(*) from mm_metadata_game_systems_info'
            ' where gs_game_system_name % $1', search_value)
    else:
        return await db_conn.fetchval(
            'select count(*) from mm_metadata_game_systems_info')


async def db_meta_game_system_list(self, offset=0, records=None, search_value=None,
                                   db_connection=None):
    """
    # return list of game systems
    """
    # TODO might need to sort by release year as well for machines with multiple releases
    if search_value is not None:
        return await db_conn.fetch('select gs_id,gs_game_system_name,'
                                   'gs_game_system_json->\'description\','
                                   'gs_game_system_json->\'year\','
                                   'gs_game_system_alias'
                                   ' from mm_metadata_game_systems_info'
                                   ' where gs_game_system_name % $1'
                                   ' order by gs_game_system_json->\'description\''
                                   ' offset $2 limit $2',
                                   search_value, offset, records)
    else:
        return await db_conn.fetch('select gs_id,gs_game_system_name,'
                                   'gs_game_system_json->\'description\','
                                   'gs_game_system_json->\'year\','
                                   'gs_game_system_alias'
                                   ' from mm_metadata_game_systems_info'
                                   ' order by gs_game_system_json->\'description\''
                                   ' offset $1 limit $2',
                                   offset, records)

def db_meta_games_system_insert(self, platform_name,
                                platform_alias, platform_json=None):
    """
    # insert game system
    """
    new_guid = uuid.uuid4()
    self.db_cursor.execute('insert into mm_metadata_game_systems_info(gs_id,'
                           ' gs_game_system_name,'
                           ' gs_game_system_alias,'
                           ' gs_game_system_json)'
                           ' values (%s, %s, %s, %s)',
                           (new_guid, platform_name, platform_alias, platform_json))
    self.db_commit()
    return new_guid


def db_meta_games_system_guid_by_short_name(self, short_name):
    self.db_cursor.execute('select gs_id'
                           ' from mm_metadata_game_systems_info'
                           ' where gs_game_system_name = %s', (short_name,))
    try:
        return self.db_cursor.fetchone()['gs_id']
    except:
        return None


def db_meta_games_system_game_count(self, short_name):
    self.db_cursor.execute('select gs_id'
                           ' from mm_metadata_game_systems_info'
                           ' where gs_game_system_name = %s', (short_name,))
    try:
        return self.db_cursor.fetchone()['gs_id']
    except:
        return None


def db_meta_game_system_upsert(self, system_name, system_alias=None, system_json=None):
    new_guid = uuid.uuid4()
    self.db_cursor.execute('INSERT INTO mm_metadata_game_systems_info'
                           ' (gs_id,'
                           ' gs_game_system_name,'
                           ' gs_game_system_alias,'
                           ' gs_game_system_json)'
                           ' VALUES (%s, %s, %s, %s)'
                           ' ON CONFLICT (gs_game_system_name)'
                           ' DO UPDATE SET gs_game_system_alias = %s, gs_game_system_json = %s',
                           (new_guid, system_name, system_alias, system_json,
                            system_alias, system_json))
    self.db_cursor.commit()
    return new_guid

 */