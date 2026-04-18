use fanotify::high_level::{
    FAN_CLOSE_WRITE, FAN_CREATE, FAN_DELETE, FAN_EVENT_ON_CHILD, FAN_MODIFY, FAN_MOVED_FROM,
    FAN_MOVED_TO, FAN_ONDIR, FanEvent, Fanotify, FanotifyMode,
};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration as StdDuration, Instant};
use tokio::signal;
use tokio::sync::mpsc;
use tokio::time::{Duration, MissedTickBehavior, interval};

const DEDUPE_WINDOW_MS: u64 = 1500;
const CHANNEL_BUFFER: usize = 4096;
const DEDUPE_CLEANUP_SECS: u64 = 30;
const PUBLISH_QUEUE: &str = "mk_inotify";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    CreateWrite,
    Create,
    Delete,
    Move,
    Modify,
}

impl Action {
    fn as_str(self) -> &'static str {
        match self {
            Action::CreateWrite => "Create/Write",
            Action::Create => "Create",
            Action::Delete => "Delete",
            Action::Move => "Move",
            Action::Modify => "Modify",
        }
    }
}

fn classify_event(events: &[FanEvent]) -> Option<Action> {
    if events.iter().any(|e| matches!(e, FanEvent::CloseWrite)) {
        Some(Action::CreateWrite)
    } else if events.iter().any(|e| matches!(e, FanEvent::Create)) {
        Some(Action::Create)
    } else if events.iter().any(|e| matches!(e, FanEvent::Delete)) {
        Some(Action::Delete)
    } else if events
        .iter()
        .any(|e| matches!(e, FanEvent::MovedFrom | FanEvent::MovedTo))
    {
        Some(Action::Move)
    } else if events.iter().any(|e| matches!(e, FanEvent::Modify)) {
        Some(Action::Modify)
    } else {
        None
    }
}

fn build_payload(action: Action, path: &str, pid: i32) -> String {
    json!({
        "Type": action.as_str(),
        "Path": path,
        "Pid": pid,
    })
    .to_string()
}

struct RawEvent {
    action: Action,
    path: String,
    pid: i32,
}

async fn log_event(payload: serde_json::Value) {
    if let Err(err) = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(payload).await {
        eprintln!("mkfanotify: loki push failed: {err}");
    }
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120).await?;

    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkfanotify").await?;

    let fanotify = Fanotify::new_blocking(FanotifyMode::CONTENT)?;

    let mut watches_installed: usize = 0;
    let rows = mk_lib_database::mk_lib_database_library::mk_lib_database_library_read(
        &sqlx_pool_ro,
    )
    .await?;

    for row_data in rows {
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
            Ok(_) => {
                watches_installed += 1;
                log_event(json!({
                    "Module": std::module_path!(),
                    "Event": "watch_installed",
                    "Path": lib_path,
                }))
                .await;
            }
            Err(err) => {
                log_event(json!({
                    "Module": std::module_path!(),
                    "Event": "watch_failed",
                    "Path": lib_path,
                    "Error": err.to_string(),
                }))
                .await;
            }
        }
    }

    if watches_installed == 0 {
        log_event(json!({
            "Module": std::module_path!(),
            "Event": "no_watches_installed",
        }))
        .await;
        return Err("mkfanotify: no library paths watched".into());
    }

    let (tx, mut rx) = mpsc::channel::<RawEvent>(CHANNEL_BUFFER);

    let watcher_handle = tokio::task::spawn_blocking(move || {
        loop {
            let events = fanotify.read_event();
            for event in events {
                let Some(action) = classify_event(&event.events) else {
                    continue;
                };
                let raw = RawEvent {
                    action,
                    path: event.path,
                    pid: event.pid,
                };
                if tx.blocking_send(raw).is_err() {
                    return;
                }
            }
        }
    });

    let dedupe_window = StdDuration::from_millis(DEDUPE_WINDOW_MS);
    let mut recently_sent: HashMap<(Action, String), Instant> = HashMap::new();
    let mut cleanup_ticker = interval(Duration::from_secs(DEDUPE_CLEANUP_SECS));
    cleanup_ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    cleanup_ticker.tick().await;

    loop {
        tokio::select! {
            _ = shutdown_signal() => {
                log_event(json!({
                    "Module": std::module_path!(),
                    "Event": "shutdown",
                }))
                .await;
                break;
            }
            _ = cleanup_ticker.tick() => {
                let now = Instant::now();
                recently_sent.retain(|_, last_seen| {
                    now.duration_since(*last_seen) < dedupe_window
                });
            }
            maybe_event = rx.recv() => {
                let Some(raw) = maybe_event else { break; };
                let now = Instant::now();
                let key = (raw.action, raw.path.clone());
                if let Some(last) = recently_sent.get(&key)
                    && now.duration_since(*last) < dedupe_window
                {
                    continue;
                }
                recently_sent.insert(key, now);

                let payload = build_payload(raw.action, &raw.path, raw.pid);
                if let Err(err) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                    rabbit_channel.clone(),
                    PUBLISH_QUEUE,
                    payload,
                )
                .await
                {
                    log_event(json!({
                        "Module": std::module_path!(),
                        "Event": "publish_failed",
                        "Path": raw.path,
                        "Action": raw.action.as_str(),
                        "Error": err.to_string(),
                    }))
                    .await;
                }
            }
        }
    }

    drop(rx);
    if let Err(err) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(
        rabbit_channel,
        rabbit_connection,
    )
    .await
    {
        log_event(json!({
            "Module": std::module_path!(),
            "Event": "rabbitmq_close_failed",
            "Error": err.to_string(),
        }))
        .await;
    }
    watcher_handle.abort();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_prefers_close_write_over_create_and_modify() {
        let evs = [FanEvent::Modify, FanEvent::CloseWrite, FanEvent::Create];
        assert_eq!(classify_event(&evs), Some(Action::CreateWrite));
    }

    #[test]
    fn classify_prefers_create_over_modify() {
        let evs = [FanEvent::Modify, FanEvent::Create];
        assert_eq!(classify_event(&evs), Some(Action::Create));
    }

    #[test]
    fn classify_delete() {
        assert_eq!(classify_event(&[FanEvent::Delete]), Some(Action::Delete));
    }

    #[test]
    fn classify_move_from_or_to() {
        assert_eq!(classify_event(&[FanEvent::MovedFrom]), Some(Action::Move));
        assert_eq!(classify_event(&[FanEvent::MovedTo]), Some(Action::Move));
    }

    #[test]
    fn classify_modify() {
        assert_eq!(classify_event(&[FanEvent::Modify]), Some(Action::Modify));
    }

    #[test]
    fn classify_empty_is_none() {
        assert_eq!(classify_event(&[]), None);
    }

    #[test]
    fn action_strings_stable() {
        assert_eq!(Action::CreateWrite.as_str(), "Create/Write");
        assert_eq!(Action::Create.as_str(), "Create");
        assert_eq!(Action::Delete.as_str(), "Delete");
        assert_eq!(Action::Move.as_str(), "Move");
        assert_eq!(Action::Modify.as_str(), "Modify");
    }

    #[test]
    fn payload_shape() {
        let p = build_payload(Action::Create, "/a/b", 42);
        let v: serde_json::Value = serde_json::from_str(&p).unwrap();
        assert_eq!(v["Type"], "Create");
        assert_eq!(v["Path"], "/a/b");
        assert_eq!(v["Pid"], 42);
    }
}
