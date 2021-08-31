use uuid::Uuid;
use sqlx::postgres::PgRow;

pub async fn mk_lib_database_media_insert(pool: &sqlx::PgPool,
                                          mm_media_guid: Uuid,
                                          mm_media_class_enum: i16,
                                          mm_media_path: String,
                                          mm_media_metadata_guid: Uuid,
                                          mm_media_ffprobe_json: String,
                                          mm_media_json: String)
                                          -> Result<(), sqlx::Error> {
    sqlx::query("insert into mm_media (mm_media_guid, mm_media_class_enum, \
         mm_media_path, mm_media_metadata_guid, mm_media_ffprobe_json, mm_media_json) \
         values ($1, $2, $3, $4, $5, $6)")
        .bind(mm_media_guid)
        .bind(mm_media_class_enum)
        .bind(mm_media_path)
        .bind(mm_media_metadata_guid)
        .bind(mm_media_ffprobe_json)
        .bind(mm_media_json)
        .execute(pool)
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