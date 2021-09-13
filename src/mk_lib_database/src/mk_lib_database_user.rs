use sqlx::postgres::PgRow;
use uuid::Uuid;

pub async fn mk_lib_database_user_read(pool: &sqlx::PgPool,
                                       offset: i32, limit: i32)
                                       -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select id, username, email, created_at, active, \
         is_admin, lang from mm_user offset $1 limit $2) order by LOWER(username)")
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/*

async def db_user_count(self, user_name=None, db_connection=None):
    if user_name is None:
        return await db_conn.fetchval('select count(*) from mm_user')
    else:
        return await db_conn.fetchval('select count(*) from mm_user'
                                      ' where username = $1', user_name)


async def db_user_delete(self, user_guid, db_connection=None):
    await db_conn.execute('delete from mm_user where id = $1', user_guid)


async def db_user_detail(self, guid, db_connection=None):
    return await db_conn.fetchrow('select * from mm_user'
                                  ' where id = $1', guid)


async def db_user_exists(self, user_name, db_connection=None):
    return await db_conn.fetchval('select exists(select 1 from mm_user'
                                  ' where username = $1 limit 1) limit 1', user_name)


async def db_user_insert(self, user_name, user_email, user_password, db_connection=None):
    """
    # insert user
    """
    if await self.db_user_count(user_name=None, db_connection=db_conn) == 0:
        user_admin = True
    else:
        user_admin = False
    return await db_conn.execute(
        'insert into mm_user (username, email, password, active, is_admin, user_json, created_at)'
        ' values ($1, $2, crypt($3, gen_salt(\'bf\', 10)), True, $4, \'{"per_page": 30}\','
        ' current_timestamp)'
        ' returning id',
        user_name, user_email, user_password, user_admin), user_admin, 30


async def db_user_login(self, user_name, user_password, db_connection=None):
    """
    # verify user logon
    """
    result = await db_conn.fetchrow('select id, active, is_admin,'
                                    ' user_json->\'per_page\' as per_page'
                                    ' from mm_user where username = $1'
                                    ' and password = crypt($2, password)',
                                    user_name, user_password)
    if result is not None:
        print(result, flush=True)
        if result['active'] is False:
            return 'inactive_account', None, None
        return result['id'], result['is_admin'], result['per_page']
    return 'invalid_password', None, None




 */