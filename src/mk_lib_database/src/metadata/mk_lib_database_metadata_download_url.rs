
pub async fn mk_lib_database_metadata_download_url_exists(
    sqlx_pool: &sqlx::PgPool,
    download_url: String,
) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        r#"select exists(
            select 1
            from mm_downloaded
            where mm_downloaded_url = $1
            limit 1
        ) as found_record
        limit 1"#,
    )
    .bind(download_url)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_download_url_insert(
    sqlx_pool: &sqlx::PgPool,
    download_url: String,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_downloaded (
        mm_download_guid,
        mm_downloaded_url,
        )
        values ($1, $2)"#,
    )
    .bind(uuid::Uuid::now_v7())
    .bind(download_url)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}
