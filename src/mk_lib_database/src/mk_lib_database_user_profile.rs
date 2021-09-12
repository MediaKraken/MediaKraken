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

 */