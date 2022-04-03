use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, types::Json};
use rocket_dyn_templates::serde::{Serialize, Deserialize};

/*
# TODO port query
async def db_activity_insert(self, activity_name, activity_overview,
                             activity_short_overview, activity_type, activity_itemid,
                             activity_userid,
                             activity_log_severity, db_connection=None):
    new_guid = uuid.uuid4()
    await db_conn.execute('insert into mm_user_activity (mm_activity_guid,'
                          ' mm_activity_name,'
                          ' mm_activity_overview,'
                          ' mm_activity_short_overview,'
                          ' mm_activity_type,'
                          ' mm_activity_itemid,'
                          ' mm_activity_userid, '
                          ' mm_activity_datecreated,'
                          ' mm_activity_log_severity)'
                          ' values ($1,$2,$3,$4,$5,$6,$7,$8,$9)',
                          (new_guid, activity_name, activity_overview,
                           activity_short_overview,
                           activity_type, activity_itemid, activity_userid,
                           datetime.datetime.now(), activity_log_severity))
    return new_guid

 */

pub async fn mk_lib_database_activity_delete(pool: &sqlx::PgPool,
                                             day_range: i32)
                                             -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("delete from mm_user_activity \
        where mm_activity_datecreated < now() - interval $1 day;")
        .bind(day_range)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
