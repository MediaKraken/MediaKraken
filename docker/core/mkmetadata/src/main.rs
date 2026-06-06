use mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_by_provider;
use mk_lib_database::mk_lib_database::mk_lib_database_open_pool;
use mk_lib_database::mk_lib_database_option_status::{APIJson, mk_lib_database_option_api_read};
use mk_lib_database::mk_lib_database_version::mk_lib_database_version_check;
use mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push;
use mk_lib_metadata::base::metadata_process;
use mk_lib_network::mk_lib_network_limiter::API_LIMIT;
use ratelimit::Ratelimiter;
use serde_json::json;
use std::env;
use std::error::Error;
use tokio::signal;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};

const POLL_INTERVAL: Duration = Duration::from_secs(1);
const DAILY_WINDOW_SECS: u64 = 86_400;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

fn build_limiter(tokens: u64, window: Duration) -> Ratelimiter {
    Ratelimiter::builder(tokens, window)
        .max_tokens(tokens)
        .initial_available(tokens)
        .build()
        .expect("ratelimiter build failed")
}

// Waits until every limiter has a token available, or returns true if
// shutdown was requested while waiting. Token consumption is atomic: if a
// later limiter would block, the already-consumed earlier tokens would be
// lost, so check tightest (shortest-window) limiter first.
async fn await_limiters(limiters: &[Ratelimiter], shutdown_rx: &mut watch::Receiver<bool>) -> bool {
    for limiter in limiters {
        while let Err(wait) = limiter.try_wait() {
            tokio::select! {
                _ = sleep(wait) => {}
                _ = shutdown_rx.wait_for(|v| *v) => return true,
            }
        }
    }
    false
}

fn spawn_provider_loop(
    pool: sqlx::PgPool,
    provider: &'static str,
    api_key: String,
    limiters: Vec<Ratelimiter>,
    debug: bool,
    shutdown_rx: watch::Receiver<bool>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        run_provider_loop(pool, provider, api_key, limiters, debug, shutdown_rx).await;
    })
}

async fn run_provider_loop(
    pool: sqlx::PgPool,
    provider: &'static str,
    api_key: String,
    limiters: Vec<Ratelimiter>,
    debug: bool,
    mut shutdown_rx: watch::Receiver<bool>,
) {
    loop {
        if *shutdown_rx.borrow() {
            eprintln!("mkmetadata: {provider} shutdown");
            return;
        }

        let queue = match mk_lib_database_download_queue_by_provider(&pool, provider).await {
            Ok(items) => items,
            Err(err) => {
                eprintln!("mkmetadata: {provider} queue read failed ({err})");
                Vec::new()
            }
        };

        for download_data in queue {
            if await_limiters(&limiters, &mut shutdown_rx).await {
                eprintln!("mkmetadata: {provider} shutdown");
                return;
            }

            if debug
                && let Err(err) = mk_logging_loki_push(json!({
                    "Module": std::module_path!(),
                    "DL Guid": download_data.mm_download_guid,
                    "Status": download_data.mm_download_status,
                    "Provider": provider,
                    "ID": download_data.mm_download_provider_id,
                }))
                .await
            {
                eprintln!("mkmetadata: {provider} loki push failed ({err})");
            }

            if let Err(err) =
                metadata_process(&pool, provider.to_string(), download_data, api_key.as_str()).await
            {
                eprintln!("mkmetadata: {provider} process failed ({err})");
            }
        }

        tokio::select! {
            _ = sleep(POLL_INTERVAL) => {}
            _ = shutdown_rx.wait_for(|v| *v) => {
                eprintln!("mkmetadata: {provider} shutdown");
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (sqlx_pool_rw, sqlx_pool_ro) = mk_lib_database_open_pool(2, 120).await?;

    mk_lib_database_version_check(&sqlx_pool_ro, false).await?;

    let option_json = mk_lib_database_option_api_read(&sqlx_pool_ro).await?;
    let option_api: APIJson = serde_json::from_value(option_json)?;

    let debug_enabled = env::var("DEBUG")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    if let Some(key) = option_api.barcodespider {
        let limit = API_LIMIT["barcodespider"];
        handles.push(spawn_provider_loop(
            sqlx_pool_rw.clone(),
            "barcodespider",
            key,
            vec![build_limiter(
                limit.2,
                Duration::from_secs(DAILY_WINDOW_SECS),
            )],
            debug_enabled,
            shutdown_rx.clone(),
        ));
    }

    if let Some(key) = option_api.musicbrainz {
        let limit = API_LIMIT["musicbrainz"];
        handles.push(spawn_provider_loop(
            sqlx_pool_rw.clone(),
            "musicbrainz",
            key,
            vec![build_limiter(limit.0, Duration::from_secs(limit.1))],
            debug_enabled,
            shutdown_rx.clone(),
        ));
    }

    if !option_api.themoviedb.is_empty() {
        let limit = API_LIMIT["themoviedb"];
        handles.push(spawn_provider_loop(
            sqlx_pool_rw.clone(),
            "themoviedb",
            option_api.themoviedb,
            vec![build_limiter(limit.0, Duration::from_secs(limit.1))],
            debug_enabled,
            shutdown_rx.clone(),
        ));
    }

    if !option_api.thesportsdb.is_empty() {
        let limit = API_LIMIT["thesportsdb"];
        handles.push(spawn_provider_loop(
            sqlx_pool_rw.clone(),
            "thesportsdb",
            option_api.thesportsdb,
            vec![build_limiter(limit.0, Duration::from_secs(limit.1))],
            debug_enabled,
            shutdown_rx.clone(),
        ));
    }

    if let Some(key) = option_api.upcitemdb {
        let limit = API_LIMIT["upcitemdb"];
        // Per-window limiter first so a blocked per-minute check never
        // burns a daily token.
        handles.push(spawn_provider_loop(
            sqlx_pool_rw.clone(),
            "upcitemdb",
            key,
            vec![
                build_limiter(limit.0, Duration::from_secs(limit.1)),
                build_limiter(limit.2, Duration::from_secs(DAILY_WINDOW_SECS)),
            ],
            debug_enabled,
            shutdown_rx.clone(),
        ));
    }

    // "Z" catch-all: no external API, no rate limit.
    handles.push(spawn_provider_loop(
        sqlx_pool_rw.clone(),
        "Z",
        String::new(),
        Vec::new(),
        debug_enabled,
        shutdown_rx.clone(),
    ));

    shutdown_signal().await;
    eprintln!("mkmetadata: shutdown signal received");
    let _ = shutdown_tx.send(true);

    for handle in handles {
        if let Err(err) = handle.await {
            eprintln!("mkmetadata: task join error ({err})");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_limiter_basic() {
        let limiter = build_limiter(10, Duration::from_secs(60));
        let result = limiter.try_wait();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_limiter_max_tokens() {
        let limiter = build_limiter(5, Duration::from_secs(60));
        for _ in 0..5 {
            assert!(limiter.try_wait().is_ok());
        }
        assert!(limiter.try_wait().is_err());
    }

    #[tokio::test]
    async fn test_build_limiter_single_token() {
        let limiter = build_limiter(1, Duration::from_secs(60));
        assert!(limiter.try_wait().is_ok());
        assert!(limiter.try_wait().is_err());
    }

    #[tokio::test]
    async fn test_build_limiter_large_tokens() {
        let limiter = build_limiter(1000, Duration::from_secs(60));
        for _ in 0..1000 {
            assert!(limiter.try_wait().is_ok());
        }
        assert!(limiter.try_wait().is_err());
    }

    #[tokio::test]
    async fn test_build_limiter_short_window() {
        let limiter = build_limiter(2, Duration::from_millis(100));
        assert!(limiter.try_wait().is_ok());
        assert!(limiter.try_wait().is_ok());
        assert!(limiter.try_wait().is_err());
        tokio::time::sleep(Duration::from_millis(150)).await;
        assert!(limiter.try_wait().is_ok());
    }

    #[tokio::test]
    async fn test_build_limiter_different_windows() {
        let limiter1 = build_limiter(5, Duration::from_secs(10));
        let limiter2 = build_limiter(5, Duration::from_secs(60));
        assert!(limiter1.try_wait().is_ok());
        assert!(limiter2.try_wait().is_ok());
    }
}
