use tokio::time::{Duration, sleep};

pub static DATABASE_VERSION: i32 = 43;

#[path = "./mk_lib_database_version_schema.rs"]
mod mk_lib_database_version_schema;

#[allow(dead_code)]
pub async fn mk_lib_database_version(pool: &sqlx::PgPool)
                                     -> Result<i32, sqlx::Error> {
    let row: (i32, ) = sqlx::query_as("select mm_version_number from mm_version")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_version_check(pool: &sqlx::PgPool,
                                           update_schema: bool)
                                           -> Result<bool, sqlx::Error> {
    let mut version_match: bool = false;
    let version_no: i32 = mk_lib_database_version(&pool).await.unwrap();
    if DATABASE_VERSION == version_no {
        version_match = true;
    }
    if version_match == false {
        if update_schema == true {
            // do db updates here
            mk_lib_database_version_schema::mk_lib_database_update_schema(&pool,
                                                                          version_no).await?;
            version_match = true;
        } else {
            loop {
                sleep(Duration::from_secs(5)).await;
                let version_no: i32 = mk_lib_database_version(&pool).await.unwrap();
                if DATABASE_VERSION == version_no {
                    version_match = true;
                    break;
                }
            }
        }
    }
    Ok(version_match)
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