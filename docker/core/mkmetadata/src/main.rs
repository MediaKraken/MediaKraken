use mk_lib_database;
use mk_lib_metadata;
use mk_lib_network;
use ratelimit::Ratelimiter;
use serde_json::json;
use std::env;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120).await?;

    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    // pull options/api keys
    let option_json: serde_json::Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_api_read(
            &sqlx_pool_ro,
        )
        .await?;
    let option_api: mk_lib_database::mk_lib_database_option_status::APIJson =
        serde_json::from_value(option_json)?;

    let debug_enabled = env::var("DEBUG")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    // copy out keys before moving into tasks
    let barcodespider_key = option_api.barcodespider.clone();
    let musicbrainz_key = option_api.musicbrainz.clone();
    let themoviedb_key = option_api.themoviedb.clone();
    let thesportsdb_key = option_api.thesportsdb.clone();
    let upcitemdb_key = option_api.upcitemdb.clone();

    // barcodespider
    if let Some(api_key) = barcodespider_key {
        let sqlx_pool_rw_clone = sqlx_pool_rw.clone();

        let _handle_barcodespider = tokio::spawn(async move {
            let daily_api_call_limiter = Ratelimiter::builder(
                mk_lib_network::mk_lib_network_limiter::API_LIMIT["barcodespider"].2,
                Duration::from_secs(86400),
            )
            .max_tokens(mk_lib_network::mk_lib_network_limiter::API_LIMIT["barcodespider"].2)
            .initial_available(
                mk_lib_network::mk_lib_network_limiter::API_LIMIT["barcodespider"].2,
            )
            .build()
            .unwrap();

            loop {
                let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(
                    &sqlx_pool_rw_clone,
                    "barcodespider",
                )
                .await
                .unwrap();

                for download_data in metadata_to_process {
                    if let Err(wait_time) = daily_api_call_limiter.try_wait() {
                        sleep(wait_time).await;
                        continue;
                    }

                    mk_lib_metadata::base::metadata_process(
                        &sqlx_pool_rw_clone,
                        "barcodespider".to_string(),
                        download_data,
                        api_key.as_str(),
                    )
                    .await
                    .unwrap();
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // musicbrainz
    if let Some(api_key) = musicbrainz_key {
        let sqlx_pool_rw_clone = sqlx_pool_rw.clone();

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

            loop {
                let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(
                    &sqlx_pool_rw_clone,
                    "musicbrainz",
                )
                .await
                .unwrap();

                for download_data in metadata_to_process {
                    if let Err(wait_time) = api_call_limiter.try_wait() {
                        sleep(wait_time).await;
                        continue;
                    }

                    mk_lib_metadata::base::metadata_process(
                        &sqlx_pool_rw_clone,
                        "musicbrainz".to_string(),
                        download_data,
                        api_key.as_str(),
                    )
                    .await
                    .unwrap();
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // themoviedb
    {
        let sqlx_pool_rw_clone = sqlx_pool_rw.clone();
        let api_key = themoviedb_key.clone();
        let debug_enabled = debug_enabled;

        let _handle_tmdb = tokio::spawn(async move {
            let api_call_limiter = Ratelimiter::builder(
                mk_lib_network::mk_lib_network_limiter::API_LIMIT["themoviedb"].0,
                Duration::from_secs(
                    mk_lib_network::mk_lib_network_limiter::API_LIMIT["themoviedb"].1,
                ),
            )
            .max_tokens(mk_lib_network::mk_lib_network_limiter::API_LIMIT["themoviedb"].0)
            .initial_available(mk_lib_network::mk_lib_network_limiter::API_LIMIT["themoviedb"].0)
            .build()
            .unwrap();

            loop {
                let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(
                    &sqlx_pool_rw_clone,
                    "themoviedb",
                )
                .await
                .unwrap();

                for download_data in metadata_to_process {
                    if let Err(wait_time) = api_call_limiter.try_wait() {
                        sleep(wait_time).await;
                        continue;
                    }

                    if debug_enabled {
                        println!("TMDB here2");
                        mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(
                            json!({
                                "Module": std::module_path!(),
                                "DL Guid": download_data.mm_download_guid,
                                "Status": download_data.mm_download_status,
                                "Provider": "themoviedb",
                                "ID": download_data.mm_download_provider_id
                            }),
                        )
                        .await
                        .unwrap();
                    }

                    mk_lib_metadata::base::metadata_process(
                        &sqlx_pool_rw_clone,
                        "themoviedb".to_string(),
                        download_data,
                        api_key.as_str(),
                    )
                    .await
                    .unwrap();
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // thesportsdb
    {
        let sqlx_pool_rw_clone = sqlx_pool_rw.clone();
        let api_key = thesportsdb_key.clone();

        let _handle_thesportsdb = tokio::spawn(async move {
            let api_call_limiter = Ratelimiter::builder(
                mk_lib_network::mk_lib_network_limiter::API_LIMIT["thesportsdb"].0,
                Duration::from_secs(
                    mk_lib_network::mk_lib_network_limiter::API_LIMIT["thesportsdb"].1,
                ),
            )
            .max_tokens(mk_lib_network::mk_lib_network_limiter::API_LIMIT["thesportsdb"].0)
            .initial_available(mk_lib_network::mk_lib_network_limiter::API_LIMIT["thesportsdb"].0)
            .build()
            .unwrap();

            loop {
                let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(
                    &sqlx_pool_rw_clone,
                    "thesportsdb",
                )
                .await
                .unwrap();

                for download_data in metadata_to_process {
                    if let Err(wait_time) = api_call_limiter.try_wait() {
                        sleep(wait_time).await;
                        continue;
                    }

                    mk_lib_metadata::base::metadata_process(
                        &sqlx_pool_rw_clone,
                        "thesportsdb".to_string(),
                        download_data,
                        api_key.as_str(),
                    )
                    .await
                    .unwrap();
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // upcitemdb
    if let Some(api_key) = upcitemdb_key {
        let sqlx_pool_rw_clone = sqlx_pool_rw.clone();

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

            loop {
                let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(
                    &sqlx_pool_rw_clone,
                    "upcitemdb",
                )
                .await
                .unwrap();

                for download_data in metadata_to_process {
                    if let Err(wait_time) = daily_api_call_limiter.try_wait() {
                        sleep(wait_time).await;
                        continue;
                    }

                    if let Err(wait_time) = api_call_limiter.try_wait() {
                        sleep(wait_time).await;
                        continue;
                    }

                    mk_lib_metadata::base::metadata_process(
                        &sqlx_pool_rw_clone,
                        "upcitemdb".to_string(),
                        download_data,
                        api_key.as_str(),
                    )
                    .await
                    .unwrap();
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // process all the "Z" records
    loop {
        let metadata_to_process = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider(
            &sqlx_pool_rw,
            "Z",
        )
        .await?;

        for download_data in metadata_to_process {
            println!("DL Data: {:?}", download_data);

            mk_lib_metadata::base::metadata_process(
                &sqlx_pool_rw,
                "Z".to_string(),
                download_data,
                "",
            )
            .await?;

            println!("here2");
        }

        sleep(Duration::from_secs(1)).await;
    }
}