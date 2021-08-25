use tokio::time::{Duration, sleep};
use tokio_postgres::Error;

pub static DATABASE_VERSION: i64 = 43;

#[allow(dead_code)]
pub async fn mk_lib_database_version(client: &tokio_postgres::Client) -> Result<i32, Error> {
    let row = client
        .query_one("select mm_version_number from mm_version", &[])
        .await?;
    Ok(row.get("mm_version_number"))
}

pub async fn mk_lib_database_version_check(client: &tokio_postgres::Client)
                                           -> Result<bool, Error> {
    let mut version_match: bool = false;
    let version_no: i64 = mk_lib_database_version(&client).await.unwrap();
    if DATABASE_VERSION == version_no {
        version_match = true;
    }
    if version_match == false {
        loop {
            sleep(Duration::from_secs(5)).await;
            let version_no: i64 = mk_lib_database_version(&client).await.unwrap();
            if DATABASE_VERSION == version_no {
                version_match = true;
                break;
            }
        }
    }
    Ok(version_match)
}

#[allow(dead_code)]
pub async fn mk_lib_database_version_update(client: &tokio_postgres::Client,
                                            version_number: i32) -> Result<(), Error> {
    client
        .query("update mm_version set mm_version_number = $1", &[&version_number])
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