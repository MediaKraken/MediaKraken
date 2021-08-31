pub async fn mk_lib_database_metadata_exists_movie(pool: &sqlx::PgPool,
                                                   metadata_id: i32)
                                                   -> Result<bool, sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select exists(select 1 from mm_metadata_movie \
        where mm_metadata_movie_media_id = $1 limit 1) as found_record limit 1")
        .bind(metadata_id)
        .execute(pool)
        .await?;
    let exists_status: bool = row.get("found_record");
    Ok(exists_status)
}

#[allow(dead_code)]
pub async fn mk_lib_database_metadata_exists_person(pool: &sqlx::PgPool,
                                                    metadata_id: i32)
                                                    -> Result<bool, sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select exists(select 1 from mm_metadata_person \
        where mm_metadata_person_media_id = $1 limit 1) as found_record limit 1")
        .bind(metadata_id)
        .execute(pool)
        .await?;
    let exists_status: bool = row.get("found_record");
    Ok(exists_status)
}

pub async fn mk_lib_database_metadata_exists_tv(pool: &sqlx::PgPool,
                                                metadata_id: i32)
                                                -> Result<bool, sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select exists(select 1 from mm_metadata_tvshow \
        where mm_metadata_media_tvshow_id = $1 limit 1) as found_record limit 1")
        .bind(metadata_id)
        .execute(pool)
        .await?;
    let exists_status: bool = row.get("found_record");
    Ok(exists_status)
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