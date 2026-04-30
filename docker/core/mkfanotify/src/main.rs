use fanotify::high_level::{
    FanEvent, Fanotify, FanotifyMode, FAN_CLOSE_WRITE, FAN_CREATE, FAN_DELETE,
    FAN_EVENT_ON_CHILD, FAN_MODIFY, FAN_MOVED_FROM, FAN_MOVED_TO, FAN_ONDIR,
};
use mk_lib_database;
use mk_lib_rabbitmq;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

const DEDUPE_WINDOW_MS: u64 = 1500;
const CHANNEL_BUFFER: usize = 4096;
const CLEANUP_EVERY_EVENTS: u64 = 1000;

const EVENT_TYPE_FILE: u8 = 1;

const ACTION_CREATE_WRITE: u8 = 1;
const ACTION_CREATE: u8 = 2;
const ACTION_DELETE: u8 = 3;
const ACTION_MOVE: u8 = 4;
const ACTION_MODIFY: u8 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;

    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkfanotify").await?;

    let fanotify = Fanotify::new_blocking(FanotifyMode::CONTENT)?;

    for row_data in
        mk_lib_database::mk_lib_database_library::mk_lib_database_library_read(&sqlx_pool_ro)
            .await?
    {
        let lib_path = row_data.mm_media_dir_path;

        let mask = FAN_MODIFY
            | FAN_CLOSE_WRITE
            | FAN_CREATE
            | FAN_DELETE
            | FAN_MOVED_FROM
            | FAN_MOVED_TO
            | FAN_EVENT_ON_CHILD
            | FAN_ONDIR;

        match fanotify.add_path(mask, &lib_path) {
            Ok(_) => println!("Loaded fanotify watch for: {}", lib_path),
            Err(e) => eprintln!("Failed to add fanotify watch for {}: {}", lib_path, e),
        }
    }

    let (tx, mut rx) = mpsc::channel::<String>(CHANNEL_BUFFER);

    let watcher_handle = tokio::task::spawn_blocking(move || {
        let dedupe_window = Duration::from_millis(DEDUPE_WINDOW_MS);
        let mut recently_sent: HashMap<(u8, u8, String), Instant> = HashMap::new();
        let mut processed_events: u64 = 0;

        loop {
            let events = fanotify.read_event();

            for event in events {
                let action = if event.events.iter().any(|e| matches!(e, FanEvent::CloseWrite)) {
                    Some(ACTION_CREATE_WRITE)
                } else if event.events.iter().any(|e| matches!(e, FanEvent::Create)) {
                    Some(ACTION_CREATE)
                } else if event.events.iter().any(|e| matches!(e, FanEvent::Delete)) {
                    Some(ACTION_DELETE)
                } else if event
                    .events
                    .iter()
                    .any(|e| matches!(e, FanEvent::MovedFrom | FanEvent::MovedTo))
                {
                    Some(ACTION_MOVE)
                } else if event.events.iter().any(|e| matches!(e, FanEvent::Modify)) {
                    Some(ACTION_MODIFY)
                } else {
                    None
                };

                let Some(action) = action else {
                    continue;
                };

                let now = Instant::now();
                let dedupe_key = (EVENT_TYPE_FILE, action, event.path.clone());

                let should_publish = match recently_sent.get(&dedupe_key) {
                    Some(last_seen) => now.duration_since(*last_seen) >= dedupe_window,
                    None => true,
                };

                if !should_publish {
                    continue;
                }

                recently_sent.insert(dedupe_key, now);

                let action_text = match action {
                    ACTION_CREATE_WRITE => "Create/Write",
                    ACTION_CREATE => "Create",
                    ACTION_DELETE => "Delete",
                    ACTION_MOVE => "Move",
                    ACTION_MODIFY => "Modify",
                    _ => "Unknown",
                };

                let payload = json!({
                    "Type": action_text,
                    "Path": event.path,
                    "Pid": event.pid,
                })
                .to_string();

                if tx.blocking_send(payload).is_err() {
                    return;
                }

                processed_events += 1;
                if processed_events % CLEANUP_EVERY_EVENTS == 0 {
                    let now = Instant::now();
                    recently_sent.retain(|_, last_seen| {
                        now.duration_since(*last_seen) < dedupe_window
                    });
                }
            }
        }
    });

    while let Some(payload) = rx.recv().await {
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
            rabbit_channel.clone(),
            "mk_inotify",
            payload,
        )
        .await?;
    }

    watcher_handle.await?;
    Ok(())
}