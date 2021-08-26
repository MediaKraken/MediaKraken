
pub async fn mk_lib_database_update_schema(pool: &sqlx::PgPool, version_no: i32)
                                           -> Result<bool, sqlx::Error> {
    if version_no < 43 {
        mk_lib_database_version_update(&pool,
                                       43).await?;
    }
    Ok(true)
}

#[allow(dead_code)]
pub async fn mk_lib_database_version_update(pool: &sqlx::PgPool,
                                            version_number: i32)
                                            -> Result<(), sqlx::Error> {
    sqlx::query("update mm_version set mm_version_number = $1")
        .bind(version_number)
        .execute(pool)
        .await?;
    Ok(())
}