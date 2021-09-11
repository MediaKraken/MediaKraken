use sqlx::postgres::PgRow;

pub async fn mk_lib_database_option_read(pool: &sqlx::PgPool)
                                         -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value, ) = sqlx::query_as("select mm_options_json from mm_options_and_status")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_status_read(pool: &sqlx::PgPool)
                                         -> Result<serde_json::Value, sqlx::Error> {
    let row: (serde_json::Value, ) = sqlx::query_as("select mm_status_json from mm_options_and_status")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_option_status_read(pool: &sqlx::PgPool)
                                                -> Result<Vec<PgRow>, sqlx::Error> {
    let rows = sqlx::query("select mm_options_json, mm_status_json \
        from mm_options_and_status")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}