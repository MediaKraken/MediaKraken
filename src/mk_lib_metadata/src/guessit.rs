#![cfg_attr(debug_assertions, allow(dead_code))]

use serde_json::json;
use sqlx::{types::Json, types::Uuid};
use std::error::Error;
use std::path::Path;
use stdext::function_name;
use torrent_name_parser::Metadata;

use crate::mk_lib_logging;

use crate::database::mk_lib_database_metadata_download_queue;
use crate::database::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;

// #[path = "../identification.rs"]
// mod metadata_identification;

// // setup last used id's per thread
// static mut metadata_last_uuid: Uuid = uuid::Uuid::nil();
// static mut metadata_last_title: String = String::new();
// static mut metadata_last_year: i32 = 0;

pub async fn metadata_guessit(
    sqlx_pool: &sqlx::PgPool,
    download_data: &DBDownloadQueueByProviderList,
) -> Result<(uuid::Uuid, Metadata), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut metadata_uuid: uuid::Uuid = uuid::Uuid::nil();
    // check for dupes by name/year
    let file_name = Path::new(&download_data.mm_download_path.as_ref().unwrap())
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Guessit File": file_name }),
        )
        .await
        .unwrap();
    }
    let guessit_data: Metadata = Metadata::from(&file_name).unwrap();
    if guessit_data.title().len() > 0 {
        // if guessit_data.year().is_some() {
        //     if guessit_data.title().to_lowercase() == metadata_last_title
        //         && guessit_data.year().unwrap() == metadata_last_year
        //     {
        //         // matches last media scanned, so set with that metadata id
        //         metadata_uuid = metadata_last_uuid
        //     }
        // } else if guessit_data.title().to_lowercase() == metadata_last_title {
        //     // matches last media scanned, so set with that metadata id
        //     metadata_uuid = metadata_last_uuid
        // }
        // if metadata_uuid == uuid::Uuid::nil() {
        //     // begin id process
        //     metadata_uuid = metadata_identification::metadata_identification(
        //         &sqlx_pool,
        //         download_data,
        //         guessit_data,
        //     )
        //     .await
        //     .unwrap();
        // }
        // allow none to be set so unmatched stuff can work for skipping
        // metadata_last_uuid = metadata_uuid;
        // metadata_last_title = guessit_data.title().to_lowercase();
        // if guessit_data.year().is_some() {
        //     metadata_last_year = guessit_data.year().unwrap();
        // } else {
        //     metadata_last_year = 0;
        // }
    } else {
        database::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_update_provider(&sqlx_pool,
                                                                                                                 "ZZ".to_string(),
                                                                                                                 download_data.mm_download_guid).await.unwrap();
    }
    Ok((metadata_uuid, guessit_data))
}
