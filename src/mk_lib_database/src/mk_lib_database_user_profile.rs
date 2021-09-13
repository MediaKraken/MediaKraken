use sqlx::postgres::PgRow;
use uuid::Uuid;
/*
def db_user_profile_insert(self, profile_name, profile_json):
    """
    insert user profile
    """
    new_user_profile_id = uuid.uuid4()
    self.db_cursor.execute('insert into mm_user_profile (mm_user_profile_guid,'
                           ' mm_user_profile_name,'
                           ' mm_user_profile_json)'
                           ' values (%s, %s, %s)',
                           (new_user_profile_id, profile_name, profile_json))
    self.db_commit()
    return new_user_profile_id

async def db_user_group_insert(self, group_name, group_desc, group_rights_json,
                               db_connection=None):
    """
    insert user group
    """
    new_user_group_id = uuid.uuid4()
    await db_conn.execute('insert into mm_user_group (mm_user_group_guid,'
                          ' mm_user_group_name,'
                          ' mm_user_group_description,'
                          ' mm_user_group_rights_json)'
                          ' values ($1,$2,$3,$4)',
                          new_user_group_id, group_name,
                          group_desc, group_rights_json)
    await db_conn.execute('commit')
    return new_user_group_id

 */