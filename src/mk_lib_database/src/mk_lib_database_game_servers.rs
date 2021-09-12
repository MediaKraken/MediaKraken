use sqlx::postgres::PgRow;
use uuid::Uuid;

pub async fn mk_lib_database_dedicated_server_read(pool: &sqlx::PgPool,
                                                   offset: i32, limit: i32)
                                                   -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_game_server_guid, mm_game_server_name, \
        mm_game_server_json from mm_game_dedicated_servers \
        order by mm_game_server_name offset $1 limit $2")
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/*

async def db_game_server_upsert(self, server_name, server_json, db_connection=None):
    """
    Upsert a game server into the database
    """
    new_guid = uuid.uuid4()
    await db_conn.execute('INSERT INTO mm_game_dedicated_servers (mm_game_server_guid,'
                          ' mm_game_server_name,'
                          ' mm_game_server_json)'
                          ' VALUES ($1, $2, $3)'
                          ' ON CONFLICT (mm_game_server_name)'
                          ' DO UPDATE SET mm_game_server_json = $4',
                          new_guid, server_name, server_json,
                          server_json)
    return new_guid


async def db_game_server_delete(self, record_uuid, db_connection=None):
    """
    Delete game_server
    """
    await db_conn.execute('delete from mm_game_dedicated_servers'
                          ' where mm_game_server_guid = $1',
                          record_uuid)


async def db_game_server_detail(self, record_uuid, db_connection=None):
    """
    game server info
    """
    return await db_conn.fetchrow('select mm_game_server_name,'
                                  ' mm_game_server_json'
                                  ' from mm_game_dedicated_servers'
                                  ' where mm_game_server_guid = %s', record_uuid)


async def db_game_server_list_count(self, db_connection=None):
    """
    Return number of game servers
    """
    return await db_conn.fetchval('select count(*) from mm_game_dedicated_servers')

 */