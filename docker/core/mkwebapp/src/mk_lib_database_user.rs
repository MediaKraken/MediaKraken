#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use serde::{Serialize, Deserialize};

pub async fn mk_lib_database_user_exists(pool: &sqlx::PgPool,
                                         user_name: String)
                                         -> Result<bool, sqlx::Error> {
    let row: (bool, ) = sqlx::query_as("select exists(select 1 from mm_user \
        where username = $1 limit 1) limit 1")
        .bind(user_name)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBUserList {
	id: uuid::Uuid,
	username: String,
	email: String,
	created_at: String,
	active: String,
	is_admin: String,
	lang: String,
}

pub async fn mk_lib_database_user_read(pool: &sqlx::PgPool,
                                       offset: i32, limit: i32)
                                       -> Result<Vec<DBUserList>, sqlx::Error> {
    let select_query = sqlx::query("select id, username, email, created_at, active, \
         is_admin, lang from mm_user offset $1 limit $2) order by LOWER(username)")
        .bind(offset)
        .bind(limit);
    let table_rows: Vec<DBUserList> = select_query
		.map(|row: PgRow| DBUserList {
			id: row.get("id"),
			username: row.get("username"),
			email: row.get("email"),
			created_at: row.get("created_at"),
			active: row.get("active"),
			is_admin: row.get("is_admin"),
			lang: row.get("lang"),
		})
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_user_count(pool: &sqlx::PgPool,
                                        user_name: String)
                                        -> Result<i32, sqlx::Error> {
    if user_name != "" {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_user")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query_as("select count(*) from mm_user where username = $1")
            .bind(user_name)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

pub async fn mk_lib_database_user_delete(pool: &sqlx::PgPool,
                                         user_uuid: uuid::Uuid)
                                         -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("delete from mm_user where id = $1")
        .bind(user_uuid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
/*

// TODO port query
pub async fn db_user_insert(self, user_name, user_email, user_password):
    """
    # insert user
    """
    if await self.db_user_count(user_name=None) == 0:
        user_admin = true
    else:
        user_admin = false
    return await db_conn.execute(
        'insert into mm_user (username, email, password, active, is_admin, user_json, created_at)'
        ' values ($1, $2, crypt($3, gen_salt('bf', 10)), True, $4, '{"per_page": 30}','
        ' current_timestamp)'
        ' returning id',
        user_name, user_email, user_password, user_admin), user_admin, 30


// TODO port query
pub async fn db_user_login(self, user_name, user_password):
    """
    # verify user logon
    """
    result = await db_conn.fetchrow('select id, active, is_admin,'
                                    ' user_json->'per_page' as per_page'
                                    ' from mm_user where username = $1'
                                    ' and password = crypt($2, password)',
                                    user_name, user_password)
    if result != None:
        print(result, flush=True)
        if result['active'] is False:
            return 'inactive_account', None, None
        return result['id'], result['is_admin'], result['per_page']
    return 'invalid_password', None, None

// TODO port query
pub async fn db_user_detail(self, guid):
    return await db_conn.fetchrow('select * from mm_user'
                                  ' where id = $1', guid)
 */