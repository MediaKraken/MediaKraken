use tokio::time::{Duration, sleep};
use tokio_postgres::{Error, Row};

pub static DATABASE_VERSION: i32 = 43;

pub async fn mk_lib_database_version(client: &tokio_postgres::Client) -> Result<i32, Error> {
    let row = client
        .query_one("select mm_version_no from mm_version", &[])
        .await?;
    Ok(row.get("mm_version_no"))
}

pub async fn mk_lib_database_version_check(client: &tokio_postgres::Client,
                                           update_schema: bool) -> Result<bool, Error> {
    let row = client
        .query_one("select mm_version_no from mm_version", &[])
        .await?;
    let mut version_match: bool = false;
    let version_no: i32 = row.get("mm_version_no");
    if DATABASE_VERSION == version_no {
        let version_match = true;
    }
    if version_match == false {
        if update_schema == true {
            // do db updates here
            mk_lib_database_version_update(client, 43).await;
            let version_match = true;
        } else {
            loop {
                sleep(Duration::from_secs(5)).await;
                let row = client
                    .query_one("select mm_version_no from mm_version", &[])
                    .await?;
                let version_no: i32 = row.get("mm_version_no");
                if DATABASE_VERSION == version_no {
                    let version_match = true;
                    break;
                }
            }
        }
    }
    Ok(version_match)
}


pub async fn mk_lib_database_version_update(client: &tokio_postgres::Client,
                                            version_number: i32) -> Result<(), Error> {
    client
        .query("update mm_version set mm_version_no = $1", &[&version_number])
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