use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_sync_delete(pool: &sqlx::PgPool,
                                         sync_guid: uuid::Uuid)
                                         -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("delete from mm_sync where mm_sync_guid = $1")
        .bind(sync_guid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

/*
# TODO port query
async def db_sync_progress_update(self, sync_guid, sync_percent, db_connection=None):
    """
    # update progress
    """
    await db_conn.execute('update mm_sync set mm_sync_options_json->\'Progress\' = $1'
                          ' where mm_sync_guid = $2', sync_percent, sync_guid)
    await db_conn.execute('commit')


# TODO port query
async def db_sync_list_count(self, db_connection=None):
    """
    # return count of sync jobs
    """
    return await db_conn.fetchval('select count(*) from mm_sync')


# TODO port query
async def db_sync_insert(self, sync_path, sync_path_to, sync_json, db_connection=None):
    """
    # insert sync job
    """
    new_guid = uuid.uuid4()
    await db_conn.execute('insert into mm_sync (mm_sync_guid,'
                          ' mm_sync_path,'
                          ' mm_sync_path_to,'
                          ' mm_sync_options_json)'
                          ' values ($1, $2, $3, $4)',
                          new_guid, sync_path,
                          sync_path_to, sync_json)
    return new_guid


# TODO port query
async def db_sync_list(self, offset=0, records=None, user_guid=None, db_connection=None):
    """
    # return list of sync jobs
    """
    # TODO by priority, name, year
    if user_guid is None:
        # complete list for admins
        return await db_conn.fetch('select mm_sync_guid uuid,'
                                   ' mm_sync_path,'
                                   ' mm_sync_path_to,'
                                   ' mm_sync_options_json'
                                   ' from mm_sync'
                                   ' where mm_sync_guid in (select mm_sync_guid'
                                   ' from mm_sync'
                                   ' order by mm_sync_options_json->\'Priority\''
                                   ' desc, mm_sync_path'
                                   ' offset $1 limit $2)'
                                   ' order by mm_sync_options_json->\'Priority\''
                                   ' desc, mm_sync_path',
                                   offset, records)
    else:
        return await db_conn.fetch('select mm_sync_guid uuid,'
                                   ' mm_sync_path,'
                                   ' mm_sync_path_to,'
                                   ' mm_sync_options_json'
                                   ' from mm_sync'
                                   ' where mm_sync_guid in (select mm_sync_guid'
                                   ' from mm_sync'
                                   ' where mm_sync_options_json->\'User\'::text = $1'
                                   ' order by mm_sync_options_json->\'Priority\''
                                   ' desc, mm_sync_path offset $2 limit $3)'
                                   ' order by mm_sync_options_json->\'Priority\''
                                   ' desc, mm_sync_path',
                                   str(user_guid), offset, records)

 */