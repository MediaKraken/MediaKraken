#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;

pub async fn mk_lib_database_update_schema(pool: &sqlx::PgPool, version_no: i64)
                                           -> Result<bool, sqlx::Error> {
    if version_no < 43 {
        mk_lib_database_version::mk_lib_database_version_update(&pool,
                                                                43).await?;
    }
    Ok(true)
}