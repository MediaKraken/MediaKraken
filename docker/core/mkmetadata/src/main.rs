use mk_lib_database;
use mk_lib_metadata;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Debug)]
struct APIJson {
    themoviedb: String,
    musicbrainz: Option<String>,
    thesportsdb: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    let _result =
        mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
            .await;

    // pull options/api keys and set structs to contain the data
    let option_json: serde_json::Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_api_read(&sqlx_pool)
            .await
            .unwrap();
    let option_api: APIJson = serde_json::from_value(option_json).unwrap();

    // launch thread per provider
    let _handle_tmdb = tokio::spawn(async move {
        let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
            .await
            .unwrap();
        loop {
            let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool, "themoviedb").await.unwrap();
            for download_data in metadata_to_process {
                mk_lib_metadata::base::metadata_process(
                    &sqlx_pool,
                    "themoviedb".to_string(),
                    download_data,
                    option_api.themoviedb.as_str(),
                )
                .await
                .unwrap();
            }
            sleep(Duration::from_secs(1)).await;
        }
        // sqlx_pool.close().await;
    });
    if option_api.musicbrainz.is_some() {
        let musicbrainz_api_key = option_api.musicbrainz.unwrap();
        let _handle_musicbrainz = tokio::spawn(async move {
            let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
                .await
                .unwrap();
            loop {
                let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool, "musicbrainz").await.unwrap();
                for download_data in metadata_to_process {
                    mk_lib_metadata::base::metadata_process(
                        &sqlx_pool,
                        "musicbrainz".to_string(),
                        download_data,
                        musicbrainz_api_key.as_str(),
                    )
                    .await
                    .unwrap();
                }
                sleep(Duration::from_secs(1)).await;
            }
            // sqlx_pool.close().await;
        });
    };
    let _handle_thesportsdb = tokio::spawn(async move {
        let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
            .await
            .unwrap();
        loop {
            let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool, "thesportsdb").await.unwrap();
            for download_data in metadata_to_process {
                mk_lib_metadata::base::metadata_process(
                    &sqlx_pool,
                    "thesportsdb".to_string(),
                    download_data,
                    option_api.thesportsdb.as_str(),
                )
                .await
                .unwrap();
            }
            sleep(Duration::from_secs(1)).await;
        }
        // sqlx_pool.close().await;
    });

    // process all the "Z" records
    loop {
        // grab new batch of records to process by content provider
        let metadata_to_process =
            mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(
                &sqlx_pool, "Z",
            )
            .await
            .unwrap();
        println!("here");
        for download_data in metadata_to_process {
            println!("DL Data: {:?}", download_data);
            // process the "Z" record
            let metadata_uuid = mk_lib_metadata::base::metadata_process(
                &sqlx_pool,
                &download_data,
            )
            .await
            .unwrap();
            println!("here2");
            // update the media row with the json media id and the proper name
            if metadata_uuid != uuid::Uuid::nil() {
                mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_update_metadata_guid(
                    &sqlx_pool,
                    &download_data.mm_download_provider_id.unwrap(),
                    metadata_uuid,
                    &download_data.mm_download_guid,
                )
                .await
                .unwrap();
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
    // TODO unreachable....so, do I care
    // sqlx_pool.close().await;
    //handle_tmdb.join().unwrap();
    //handle_tmdb.take().map(JoinHandle::join);
    //handle_musicbrainz.join().unwrap();
    //handle_musicbrainz.take().map(JoinHandle::join);
    //handle_thesportsdb.join().unwrap();
    //handle_thesportsdb.take().map(JoinHandle::join);
}
