pub async fn mk_lib_database_download_queue_by_provider(pool: &sqlx::PgPool, provider_name: &str)
                                                        -> Result<i32, sqlx::Error> {
    let rows = sqlx::query("select mm_download_guid, \
                               mm_download_que_type, \
                               mm_download_new_uuid, \
                               mm_download_provider_id, \
                               mm_download_status \
                               from mm_metadata_download_que \
                               where mm_download_provider = $1 \
                               order by mm_download_que_type limit 25")
        .bind(provider_name)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn mk_lib_database_download_queue_delete(pool: &sqlx::PgPool, download_guid: Uuid)
                                                   -> Result<(), sqlx::Error> {
    sqlx::query("delete from mm_metadata_download_que where mm_download_guid = $1")
        .bind(download_guid)
        .execute(pool)
        .await?;
    Ok(())
}
