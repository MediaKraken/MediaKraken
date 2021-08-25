
pub async fn mk_lib_database_version_check(pool: &sqlx::PgPool, version_no: i64)
                                           -> Result<bool, sqlx::Error> {
    if version_no < 43 {
        let fake = 1;
    }
}