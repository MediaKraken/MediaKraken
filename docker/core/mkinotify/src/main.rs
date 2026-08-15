use inotify::{EventMask, Inotify, WatchMask};
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;

const CHANNEL_BUFFER: usize = 4096;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (_sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkinotify").await?;

    let mut inotify = Inotify::init().map_err(|e| format!("Failed to initialize inotify: {e}"))?;

    for row_data in
        mk_lib_database::mk_lib_database_library::mk_lib_database_library_read(&sqlx_pool_ro)
            .await?
    {
        let lib_path: String = row_data.mm_media_dir_path;
        match inotify.watches().add(
            &lib_path,
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        ) {
            Ok(lib_path) => println!("Loaded add inotify watch: {:?}", lib_path),
            Err(e) => eprintln!("Failed to add inotify watch for {}: {e}", lib_path),
        }
    }

    let (tx, mut rx) = mpsc::channel::<String>(CHANNEL_BUFFER);

    // The blocking read_events_blocking() call must run off the async runtime; forward formatted
    // payloads over a channel to an async publisher below.
    let mut reader_handle = tokio::task::spawn_blocking(move || {
        let mut buffer = [0u8; 4096];

        loop {
            match inotify.read_events_blocking(&mut buffer) {
                Ok(events) => {
                    for event in events {
                        // Build the payload string here so the async side only ever publishes.
                        let (event_type, is_dir) = if event.mask.contains(EventMask::CREATE) {
                            ("Create", event.mask.contains(EventMask::ISDIR))
                        } else if event.mask.contains(EventMask::DELETE) {
                            ("Delete", event.mask.contains(EventMask::ISDIR))
                        } else if event.mask.contains(EventMask::MODIFY) {
                            ("Modify", event.mask.contains(EventMask::ISDIR))
                        } else {
                            continue;
                        };

                        let kind = if is_dir { "Dir" } else { "File" };
                        let payload = format!(
                            "{{'Type': '{} {}', 'JSON': {:?}}}",
                            event_type, kind, event.name
                        );

                        if tx.blocking_send(payload).is_err() {
                            eprintln!("inotify event channel closed, stopping reader");
                            return;
                        }
                    }
                }
                Err(e) => {
                    // Log and keep trying rather than taking down the whole daemon on a transient read error.
                    eprintln!("Failed to read inotify events: {e}");
                }
            }
        }
    });

    // inotify is a Linux-only kernel interface, so unix signal handling is always available here.
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;

    loop {
        tokio::select! {
            maybe_payload = rx.recv() => {
                // Channel closed (reader task exited) → nothing left to publish.
                let Some(payload) = maybe_payload else { break };

                if let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                    rabbit_channel.clone(),
                    "mk_inotify",
                    payload,
                )
                .await
                {
                    // Log-and-continue: a transient broker hiccup must not take down the whole event pipeline.
                    eprintln!("failed to publish inotify event: {error}");
                }
            }
            _ = tokio::signal::ctrl_c() => {
                eprintln!("received ctrl-c, shutting down");
                break;
            }
            _ = terminate.recv() => {
                eprintln!("received SIGTERM, shutting down");
                break;
            }
        }
    }

    // The blocking reader can be parked inside read_events_blocking(), so the join is bounded to stay within Docker's grace window.
    match tokio::time::timeout(Duration::from_secs(5), &mut reader_handle).await {
        Ok(result) => result.map_err(|e| format!("inotify reader task failed: {e}"))?,
        Err(_) => eprintln!("timed out waiting for inotify reader; proceeding with shutdown"),
    }

    Ok(())
}
