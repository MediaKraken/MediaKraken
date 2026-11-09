use mk_lib_database;
use mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;
use mk_lib_hash;
use std::error::Error;
use std::path::Path;

pub async fn metadata_game_lookup(
    sqlx_pool: &sqlx::PgPool,
    download_data: &DBDownloadQueueByProviderList,
) -> Result<uuid::Uuid, Box<dyn Error>> {
    let download_path = download_data
        .mm_download_path
        .as_ref()
        .ok_or("mm_download_path missing from download queue record")?;
    let file_name = Path::new(download_path)
        .file_name()
        .ok_or("download path has no file component")?
        .to_str()
        .ok_or("download path is not valid UTF-8")?
        .to_string();
    // stored game names don't include the archive/file extension, so match on the stem only
    let game_name = Path::new(&file_name)
        .file_stem()
        .ok_or("download file name has no stem")?
        .to_str()
        .ok_or("download file name stem is not valid UTF-8")?
        .to_string();

    let mut metadata_uuid =
        mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_uuid_by_name_and_system(
            sqlx_pool,
            game_name,
            "systemfakeshortname".to_string(),
        )
        .await?;

    if metadata_uuid == uuid::Uuid::nil() {
        let sha1_hash = mk_lib_hash::mk_lib_hash_sha1::mk_file_hash_sha1(download_path).await?;
        metadata_uuid =
            mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_by_sha1(
                sqlx_pool, sha1_hash,
            )
            .await?;
    }
    Ok(metadata_uuid)
}
