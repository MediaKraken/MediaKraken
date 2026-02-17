use crate::mk_lib_database_version_schema;
use crate::mk_lib_database_postgresql;
use tokio::time::{sleep, Duration};

pub static DATABASE_VERSION: i32 = 78;

pub async fn mk_lib_database_postgresql_version(
    sqlx_pool: &sqlx::PgPool,
) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("SELECT version();")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_version(sqlx_pool: &sqlx::PgPool) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as("select mm_version_number from mm_version")
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_version_check(
    sqlx_pool: &sqlx::PgPool,
    update_schema: bool,
) -> Result<bool, sqlx::Error> {
    // see if db exists
    while mk_lib_database_postgresql::mk_lib_database_table_exits(
        &sqlx_pool,
        "mm_version",
    )
    .await
    .unwrap() == false
    {
        sleep(Duration::from_secs(5)).await;
    }
    // start version check
    let mut version_match: bool = false;
    let version_no: i32 = mk_lib_database_version(&sqlx_pool).await.unwrap();
    if DATABASE_VERSION == version_no {
        version_match = true;
    }
    if version_match == false {
        if update_schema == true {
            // do db updates here
            mk_lib_database_version_schema::mk_lib_database_update_schema(&sqlx_pool, version_no)
                .await?;
            version_match = true;
        } else {
            loop {
                sleep(Duration::from_secs(5)).await;
                let version_no: i32 = mk_lib_database_version(&sqlx_pool).await.unwrap();
                if DATABASE_VERSION == version_no {
                    version_match = true;
                    break;
                }
            }
        }
    }
    Ok(version_match)
}
