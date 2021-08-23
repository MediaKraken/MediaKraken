use tokio::time::{Duration, sleep};

pub static DATABASE_VERSION: i32 = 43;

#[allow(dead_code)]
pub async fn mk_lib_database_version(pool: &rocket::State<sqlx::PgPool>)
                                     -> Result<i32, sqlx::Error> {
    let row = sqlx::query_as("select mm_version_number from mm_version")
        .fetch_one(&**pool)
        .await?;
    Ok(row.get("mm_version_number"))
}

pub async fn mk_lib_database_version_check(pool: &rocket::State<sqlx::PgPool>,
                                           update_schema: bool)
                                           -> Result<bool, sqlx::Error> {
    let mut version_match: bool = false;
    let version_no: i32 = mk_lib_database_version(pool);
    if DATABASE_VERSION == version_no {
        version_match = true;
    }
    if version_match == false {
        if update_schema == true {
            // do db updates here
            mk_lib_database_version_update(pool, 43).await?;
            version_match = true;
        } else {
            loop {
                sleep(Duration::from_secs(5)).await;
                let version_no: i32 = mk_lib_database_version(pool);
                if DATABASE_VERSION == version_no {
                    version_match = true;
                    break;
                }
            }
        }
    }
    Ok(version_match)
}

#[allow(dead_code)]
pub async fn mk_lib_database_version_update(pool: &rocket::State<sqlx::PgPool>,
                                            version_number: i32) -> Result<(), sqlx::Error> {
    pool::query_as("update mm_version set mm_version_number = $1", &[&version_number]).await?;
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