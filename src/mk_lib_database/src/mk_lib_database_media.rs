use uuid::Uuid;

pub async fn mk_lib_database_media_insert(pool: &sqlx::PgPool,
                                             mm_media_guid: Uuid,
                                             mm_media_class_enum: i16,
                                             mm_media_path: String,
                                             mm_media_metadata_guid: Uuid,
                                             mm_media_ffprobe_json: String,
                                             mm_media_json: String)
                                             -> Result<bool, sqlx::Error> {
    client
        .query_one("insert into mm_media (mm_media_guid, mm_media_class_enum,\
         mm_media_path, mm_media_metadata_guid, mm_media_ffprobe_json, mm_media_json)\
          values ($1, $2, $3, $4, $5, $6)",
                   &[&mm_media_guid, &mm_media_class_enum, &mm_media_path,
                       &mm_media_metadata_guid, &mm_media_ffprobe_json, &mm_media_json]).await?;
    Ok()
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