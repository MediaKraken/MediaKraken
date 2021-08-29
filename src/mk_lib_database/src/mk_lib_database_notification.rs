use uuid::Uuid;

pub async fn mk_lib_database_notification_read(pool: &sqlx::PgPool,
                                               offset: i32, limit: i32)
                                               -> Result<Vec<Row>, sqlx::Error> {
    let rows = client
        .query("select mm_notification_guid, mm_notification_text, mm_notification_time,\
        mm_notification_dismissable from mm_notification \
        order by mm_notification_time desc offset $1 limit $2", &[&offset, &limit])
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_notification_insert(pool: &sqlx::PgPool,
                                                 mm_notification_text: String,
                                                 mm_notification_dismissable: bool)
                                                 -> Result<(), sqlx::Error> {
    client
        .query("insert into mm_notification (mm_notification_guid, \
        mm_notification_text, \
        mm_notification_time = NOW(), \
        mm_notification_dismissable) \
        values ($1, $2, $3)",
               &[&Uuid::new_v4(), &mm_notification_text,
                   &mm_notification_dismissable]).await?;
    Ok(())
}


pub async fn mk_lib_database_notification_delete(pool: &sqlx::PgPool,
                                                 mk_notification_guid: uuid::Uuid)
                                                 -> Result<Vec<Row>, sqlx::Error> {
    let rows = client
        .query("delete from mm_notification where mm_notification_guid = $1",
               &[&mk_notification_guid])
        .await?;
    Ok(())
}

// // cargo test -- --show-output
// #[cfg(test)]
// mod test_mk_lib_common {
//     use super::*;
//
//     macro_rules! aw {
//     ($e:expr) => {
//         tokio_test::block_on($e)
//     };
//   }
// }