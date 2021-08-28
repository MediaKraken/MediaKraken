use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database_sqlx/src/mk_lib_database.rs"]
mod mk_lib_database;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database_sqlx/src/mk_lib_database_metadata.rs"]
mod mk_lib_database_metadata;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database_sqlx/src/mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_logging/src/mk_lib_logging.rs"]
mod mk_lib_logging;

#[cfg(not(debug_assertions))]
#[path = "mk_lib_database.rs"]
mod mk_lib_database;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_database_metadata.rs"]
mod mk_lib_database_metadata;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_database_version.rs"]
mod mk_lib_database_version;
#[cfg(not(debug_assertions))]
#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkmetadata";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;

    // open the database
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           false).await;

    // setup last used id's per thread
    let mut metadata_last_id: i32;
    let mut metadata_last_title: String;
    let mut metadata_last_year: i8;

    // process all the "Z" records
    loop {
        // grab new batch of records to process by content provider
        for row_data in db_connection.db_download_read_provider("Z").await()
        {
            let mut metadata_uuid: Uuid;


            // check for dupes by name/year
            file_name = guessit(row_data["mdq_path"]);
            if (file_name["title"]) == list {
                file_name["title"] = common_string.com_string_guessit_list(file_name["title"]);
            }
            if "title" in file_name {
            if "year" in file_name {
            if type (file_name["year"]) == list {
            file_name["year"] = file_name["year"][0];
            }
            if file_name["title"].lower() == metadata_last_title
            and file_name["year"] == metadata_last_year {
            // matches last media scanned, so set with that metadata id
            let metadata_uuid = metadata_last_id;
            }}
            else if file_name["title"].lower() == metadata_last_title {
            // matches last media scanned, so set with that metadata id
            let metadata_uuid = metadata_last_id;
        }
            // doesn"t match the last file, so set the file to be id"d
            if metadata_uuid == None {
                // begin id process
                metadata_uuid = metadata_identification::metadata_identification(
                db_connection,
                row_data,
                file_name).await();
            }
            // allow NONE to be set so, unmatched stuff can work for skipping
            let metadata_last_id = metadata_uuid
            let metadata_last_title = file_name["title"].lower();
            try:
            let metadata_last_year = file_name["year"];
            except
            KeyError:
            let metadata_last_year = None;
            else {
            // invalid guessit guess so set to ZZ to skip for now
            db_connection.db_download_update_provider("ZZ", row_data["mdq_id"]).await();
        }
        }
        // update the media row with the json media id AND THE proper NAME!!!
        if metadata_uuid
        is
        not
        None {
            db_connection.db_begin();
            db_connection.db_update_media_id(row_data["mdq_provider_id"],
            metadata_uuid);
            db_connection.db_download_delete(row_data["mdq_id"]);
            db_connection.db_commit();
        }
    }
    sleep(Duration::from_secs(1)).await;
}


    // stop logging
    mk_lib_logging::mk_logging_post_elk("info",
    json ! ({"STOP": "STOP"}),
    LOGGING_INDEX_NAME).await;
    Ok(())
}