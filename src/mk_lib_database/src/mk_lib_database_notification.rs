use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBNotificationList {
    pub mm_notification_guid: uuid::Uuid,
    pub mm_notification_text: String,
    pub mm_notification_time: String,
    pub mm_notification_dismissible: bool,
}

pub async fn mk_lib_database_notification_read(
    sqlx_pool: &sqlx::PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<DBNotificationList>, sqlx::Error> {
    let table_rows: Vec<DBNotificationList> = sqlx::query_as(
        "select mm_notification_guid, mm_notification_text, \
        mm_notification_time, \
        mm_notification_dismissible from mm_notification \
        order by mm_notification_time desc offset $1 limit $2",
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_notification_insert(
    sqlx_pool: &sqlx::PgPool,
    mm_notification_text: String,
    mm_notification_dismissable: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "insert into mm_notification (mm_notification_guid, \
        mm_notification_text, \
        mm_notification_time = NOW(), \
        mm_notification_dismissible) \
        values ($1, $2, $3)",
    )
    .bind(Uuid::now_v7())
    .bind(mm_notification_text)
    .bind(mm_notification_dismissable)
    .execute(sqlx_pool)
    .await?;
    Ok(())
}

pub async fn mk_lib_database_notification_delete(
    sqlx_pool: &sqlx::PgPool,
    mk_notification_guid: Uuid,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query("delete from mm_notification where mm_notification_guid = $1")
        .bind(mk_notification_guid)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
