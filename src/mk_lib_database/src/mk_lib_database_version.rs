use crate::mk_lib_database_postgresql;
use crate::mk_lib_database_version_schema;
use tokio::time::{Duration, sleep};

pub static DATABASE_VERSION: i32 = 83;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_version_value() {
        assert_eq!(DATABASE_VERSION, 83);
    }

    #[test]
    fn test_database_version_is_positive() {
        assert!(DATABASE_VERSION > 0);
    }
}

pub async fn mk_lib_database_postgresql_version(
    sqlx_pool: &sqlx::PgPool,
) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as(r#"SELECT version();"#)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_version(sqlx_pool: &sqlx::PgPool) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as(r#"select mm_version_number from mm_version"#)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_version_check(
    sqlx_pool: &sqlx::PgPool,
    update_schema: bool,
) -> Result<bool, sqlx::Error> {
    // see if db exists, with timeout to prevent infinite looping
    println!("Checking database version...");
    let table_check_timeout = Duration::from_secs(60);
    let start = tokio::time::Instant::now();
    let mut table_exists = false;
    while start.elapsed() < table_check_timeout {
        match mk_lib_database_postgresql::mk_lib_database_table_exists(sqlx_pool, "mm_version").await {
            Ok(exists) => {
                if exists {
                    table_exists = true;
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error checking table existence: {e}");
                return Err(e);
            }
        }
        sleep(Duration::from_secs(5)).await;
    }
    if !table_exists {
        return Err(sqlx::Error::PoolTimedOut);
    }

    // start version check
    let version_no: i32 = mk_lib_database_version(sqlx_pool).await?;
    let mut version_match: bool = DATABASE_VERSION == version_no;

    if !version_match {
        if update_schema {
            println!("Database upgrade from {version_no} to version {DATABASE_VERSION}");
            mk_lib_database_version_schema::mk_lib_database_update_schema(sqlx_pool, version_no)
                .await?;
            version_match = true;
        } else {
            let version_wait_timeout = Duration::from_secs(300);
            let start_wait = tokio::time::Instant::now();
            while start_wait.elapsed() < version_wait_timeout {
                sleep(Duration::from_secs(5)).await;
                let current_version: i32 = mk_lib_database_version(sqlx_pool).await?;
                if DATABASE_VERSION == current_version {
                    version_match = true;
                    break;
                }
            }
        }
    }
    println!("Database version: {version_no}, expected version: {DATABASE_VERSION}");
    Ok(version_match)
}
