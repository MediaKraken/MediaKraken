use sqlx::postgres::PgRow;

pub async fn mk_lib_database_library_read(pool: &sqlx::PgPool)
                                          -> Result<Vec<PgRow>, sqlx::Error> {
    let rows = sqlx::query("select mm_media_dir_guid, mm_media_dir_path \
        from mm_library_dir")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

#[allow(dead_code)]
pub async fn mk_lib_database_library_path_audit(pool: &sqlx::PgPool)
                                                -> Result<Vec<PgRow>, sqlx::Error> {
    let rows = sqlx::query("select mm_media_dir_guid, mm_media_dir_path, \
        mm_media_dir_class_enum, \
        mm_media_dir_last_scanned from mm_library_dir")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

#[allow(dead_code)]
pub async fn mk_lib_database_library_path_status(pool: &sqlx::PgPool)
                                                 -> Result<Vec<PgRow>, sqlx::Error> {
    let rows = sqlx::query("select mm_media_dir_path, mm_media_dir_status \
        from mm_library_dir where mm_media_dir_status IS NOT NULL \
        order by mm_media_dir_path")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

#[allow(dead_code)]
pub async fn mk_lib_database_library_path_status_update(pool: &sqlx::PgPool,
                                                        library_uuid: uuid::Uuid,
                                                        library_status_json: serde_json::Value)
                                                        -> Result<(), sqlx::Error> {
    sqlx::query("update mm_library_dir set mm_media_dir_status = $1 \
        where mm_media_dir_guid = $2")
        .bind(library_status_json)
        .bind(library_uuid)
        .execute(pool)
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn mk_lib_database_library_path_timestamp_update(pool: &sqlx::PgPool,
                                                           library_uuid: uuid::Uuid)
                                                           -> Result<(), sqlx::Error> {
    sqlx::query("update mm_library_dir set mm_media_dir_last_scanned = NOW() \
        where mm_media_dir_guid = $1")
        .bind(library_uuid)
        .execute(pool)
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn mk_lib_database_library_file_exists(pool: &sqlx::PgPool,
                                                 file_name: String)
                                                 -> Result<bool, sqlx::Error> {
    let row: (bool, ) = sqlx::query_as("select exists(select 1 from mm_media \
        where mm_media_path = $1 limit 1) \
        as found_record limit 1")
        .bind(file_name)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
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