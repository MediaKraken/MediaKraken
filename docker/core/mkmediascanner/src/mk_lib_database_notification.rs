use sqlx::postgres::PgRow;
use uuid::Uuid;

pub async fn mk_lib_database_notification_read(pool: &sqlx::PgPool,
                                               offset: i32, limit: i32)
                                               -> Result<Vec<PgRow>, sqlx::Error> {
    let rows = sqlx::query("select mm_notification_guid, mm_notification_text, \
        mm_notification_time, \
        mm_notification_dismissable from mm_notification \
        order by mm_notification_time desc offset $1 limit $2")
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_notification_insert(pool: &sqlx::PgPool,
                                                 mm_notification_text: String,
                                                 mm_notification_dismissable: bool)
                                                 -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_notification (mm_notification_guid, \
        mm_notification_text, \
        mm_notification_time = NOW(), \
        mm_notification_dismissable) \
        values ($1, $2, $3)")
        .bind(Uuid::new_v4())
        .bind(mm_notification_text)
        .bind(mm_notification_dismissable)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}


pub async fn mk_lib_database_notification_delete(pool: &sqlx::PgPool,
                                                 mk_notification_guid: uuid::Uuid)
                                                 -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("delete from mm_notification where mm_notification_guid = $1")
        .bind(mk_notification_guid)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}