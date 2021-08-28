use tokio_postgres::Error;
use uuid::Uuid;

pub async fn mk_lib_database_metadata_download_que_exists(client: &tokio_postgres::Client,
                                                          metadata_provider: String,
                                                          metadata_que_type: i16,
                                                          metadata_provider_id: i32)
                                                          -> Result<bool, Error> {
    let row = client
        .query_one("select exists(select 1 from mm_metadata_download_que \
        where mm_download_provider_id = $1 and mm_download_provider = $2 \
        and mm_download_que_type = $3 and mm_download_status != 'Search' limit 1) \
        as found_record limit 1",
                   &[&metadata_provider_id, &metadata_provider, &metadata_que_type]).await?;
    let id: bool = row.get("found_record");
    Ok(id)
}

pub async fn mk_lib_database_metadata_download_que_insert(client: &tokio_postgres::Client,
                                                          metadata_provider: String,
                                                          metadata_que_type: i16,
                                                          metadata_new_uuid: Uuid,
                                                          metadata_provider_id: i32,
                                                          metadata_status: String)
                                                          -> Result<(), Error> {
    client
        .query_one("insert into mm_metadata_download_que (mm_download_guid, \
        mm_download_provider, \
        mm_download_que_type, \
        mm_download_new_uuid, \
        mm_download__provider_id, \
        mm_download_status) \
        values ($1, $2, $3, $4, $5, $6)",
                   &[&Uuid::new_v4(), &metadata_provider, &metadata_que_type,
                       &metadata_new_uuid, &metadata_provider_id, &metadata_status]).await?;
    Ok(())
}
