use mk_lib_database;
use mk_lib_metadata;
use mk_lib_network;
use nonzero_ext::*;
use ratelimit::Ratelimiter;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use tokio::time::{sleep, Duration};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    let (sqlx_pool_rw, sqlx_pool_ro) = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
        .await
        .unwrap();
    let _result =
        mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
            .await;

    // pull options/api keys and set structs to contain the data
    let option_json: serde_json::Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_api_read(&sqlx_pool)
            .await
            .unwrap();
    let option_api: mk_lib_database::mk_lib_database_option_status::APIJson =
        serde_json::from_value(option_json).unwrap();

    // launch thread per provider
    if option_api.barcodespider.is_some() {
        let _handle_barcodespider = tokio::spawn(async move {
            let daily_api_call_limiter = Ratelimiter::builder(
                mk_lib_network::mk_lib_network_limiter::API_LIMIT["barcodespider"].2,
                Duration::from_secs(86400),
            )
            .max_tokens(mk_lib_network::mk_lib_network_limiter::API_LIMIT["barcodespider"].2)
            .initial_available(mk_lib_network::mk_lib_network_limiter::API_LIMIT["barcodespider"].2)
            .build()
            .unwrap();
            let sqlx_pool =
                mk_lib_database::mk_lib_database::mk_lib_database_open_pool_write(1, 120)
                    .await
                    .unwrap();
            let api_key = option_api.barcodespider.as_ref().unwrap().as_str();
            loop {
                let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool_rw, "barcodespider").await.unwrap();
                for download_data in metadata_to_process {
                    if let Err(sleep) = daily_api_call_limiter.try_wait() {
                        std::thread::sleep(sleep);
                        continue;
                    }
                    mk_lib_metadata::base::metadata_process(
                        &sqlx_pool_rw,
                        "barcodespider".to_string(),
                        download_data,
                        api_key,
                    )
                    .await
                    .unwrap();
                }
                sleep(Duration::from_secs(1)).await;
            }
            // sqlx_pool.close().await;
        });
    }

    if option_api.musicbrainz.is_some() {
        let musicbrainz_api_key = option_api.musicbrainz.unwrap();
        let _handle_musicbrainz = tokio::spawn(async move {
            let api_call_limiter = Ratelimiter::builder(
                mk_lib_network::mk_lib_network_limiter::API_LIMIT["musicbrainz"].0,
                Duration::from_secs(
                    mk_lib_network::mk_lib_network_limiter::API_LIMIT["musicbrainz"].1,
                ),
            )
            .max_tokens(mk_lib_network::mk_lib_network_limiter::API_LIMIT["musicbrainz"].0)
            .initial_available(mk_lib_network::mk_lib_network_limiter::API_LIMIT["musicbrainz"].0)
            .build()
            .unwrap();
            let sqlx_pool =
                mk_lib_database::mk_lib_database::mk_lib_database_open_pool_write(1, 120)
                    .await
                    .unwrap();
            loop {
                let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool_rw, "musicbrainz").await.unwrap();
                for download_data in metadata_to_process {
                    if let Err(sleep) = api_call_limiter.try_wait() {
                        std::thread::sleep(sleep);
                        continue;
                    }
                    mk_lib_metadata::base::metadata_process(
                        &sqlx_pool_rw,
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

    let _handle_tmdb = tokio::spawn(async move {
        let api_call_limiter = Ratelimiter::builder(
            mk_lib_network::mk_lib_network_limiter::API_LIMIT["themoviedb"].0,
            Duration::from_secs(mk_lib_network::mk_lib_network_limiter::API_LIMIT["themoviedb"].1),
        )
        .max_tokens(mk_lib_network::mk_lib_network_limiter::API_LIMIT["themoviedb"].0)
        .initial_available(mk_lib_network::mk_lib_network_limiter::API_LIMIT["themoviedb"].0)
        .build()
        .unwrap();
        let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool_write(1, 120)
            .await
            .unwrap();
        loop {
            let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool_rw, "themoviedb").await.unwrap();
            for download_data in metadata_to_process {
                if let Err(sleep) = api_call_limiter.try_wait() {
                    std::thread::sleep(sleep);
                    continue;
                }
                println!("debug {}", env::var("DEBUG").unwrap());
                if env::var("DEBUG").unwrap() == "true"
                {
                    println!("TMDB here2");
                    mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(
                        json!({ "Module": std::module_path!(), "DL Guid": download_data.mm_download_guid, "Status": download_data.mm_download_status, "Provider": "themoviedb", "ID": download_data.mm_download_provider_id }),
                        )
                        .await
                        .unwrap();
                }
                mk_lib_metadata::base::metadata_process(
                    &sqlx_pool_rw,
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

    let _handle_thesportsdb = tokio::spawn(async move {
        let api_call_limiter = Ratelimiter::builder(
            mk_lib_network::mk_lib_network_limiter::API_LIMIT["thesportsdb"].0,
            Duration::from_secs(mk_lib_network::mk_lib_network_limiter::API_LIMIT["thesportsdb"].1),
        )
        .max_tokens(mk_lib_network::mk_lib_network_limiter::API_LIMIT["thesportsdb"].0)
        .initial_available(mk_lib_network::mk_lib_network_limiter::API_LIMIT["thesportsdb"].0)
        .build()
        .unwrap();
        let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool_write(1, 120)
            .await
            .unwrap();
        loop {
            let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool_rw, "thesportsdb").await.unwrap();
            for download_data in metadata_to_process {
                if let Err(sleep) = api_call_limiter.try_wait() {
                    std::thread::sleep(sleep);
                    continue;
                }
                mk_lib_metadata::base::metadata_process(
                    &sqlx_pool_rw,
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

    if option_api.upcitemdb.is_some() {
        let _handle_upcitemdb = tokio::spawn(async move {
            let daily_api_call_limiter = Ratelimiter::builder(
                mk_lib_network::mk_lib_network_limiter::API_LIMIT["upcitemdb"].2,
                Duration::from_secs(86400),
            )
            .max_tokens(mk_lib_network::mk_lib_network_limiter::API_LIMIT["upcitemdb"].2)
            .initial_available(mk_lib_network::mk_lib_network_limiter::API_LIMIT["upcitemdb"].2)
            .build()
            .unwrap();
            let api_call_limiter = Ratelimiter::builder(
                mk_lib_network::mk_lib_network_limiter::API_LIMIT["upcitemdb"].0,
                Duration::from_secs(
                    mk_lib_network::mk_lib_network_limiter::API_LIMIT["upcitemdb"].1,
                ),
            )
            .max_tokens(mk_lib_network::mk_lib_network_limiter::API_LIMIT["upcitemdb"].0)
            .initial_available(mk_lib_network::mk_lib_network_limiter::API_LIMIT["upcitemdb"].0)
            .build()
            .unwrap();
            let sqlx_pool =
                mk_lib_database::mk_lib_database::mk_lib_database_open_pool_write(1, 120)
                    .await
                    .unwrap();
            let api_key = option_api.upcitemdb.as_ref().unwrap().as_str();
            loop {
                let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(&sqlx_pool_rw, "upcitemdb").await.unwrap();
                for download_data in metadata_to_process {
                    if let Err(sleep) = daily_api_call_limiter.try_wait() {
                        std::thread::sleep(sleep);
                        continue;
                    }
                    if let Err(sleep) = api_call_limiter.try_wait() {
                        std::thread::sleep(sleep);
                        continue;
                    }
                    mk_lib_metadata::base::metadata_process(
                        &sqlx_pool_rw,
                        "upcitemdb".to_string(),
                        download_data,
                        api_key,
                    )
                    .await
                    .unwrap();
                }
                sleep(Duration::from_secs(1)).await;
            }
            // sqlx_pool.close().await;
        });
    }

    // process all the "Z" records
    loop {
        // grab new batch of records to process by content provider
        let metadata_to_process =
            mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(
                &sqlx_pool_rw, "Z",
            )
            .await
            .unwrap();
        for download_data in metadata_to_process {
            println!("DL Data: {:?}", download_data);
            // process the "Z" record
            mk_lib_metadata::base::metadata_process(&sqlx_pool_rw, "Z".to_string(), download_data, "")
                .await
                .unwrap();
            println!("here2");
            // update the media row with the json media id and the proper name
            // if metadata_uuid != uuid::Uuid::nil() {
            //     mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_update_metadata_guid(
            //         &sqlx_pool_rw,
            //         &download_data.mm_download_provider_id.unwrap(),
            //         metadata_uuid,
            //         &download_data.mm_download_guid,
            //     )
            //     .await
            //     .unwrap();
            // }
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
