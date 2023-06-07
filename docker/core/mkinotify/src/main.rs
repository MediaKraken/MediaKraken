use inotify::{EventMask, Inotify, WatchMask};
use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use mk_lib_rabbitmq;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}))
            .await
            .unwrap();
    }

    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkinotify")
            .await
            .unwrap();

    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    for row_data in
        mk_lib_database::mk_lib_database_library::mk_lib_database_library_read(&sqlx_pool, 0, 99999)
            .await
            .unwrap()
    {
        let lib_path: String = row_data.mm_media_dir_path;
        inotify
            .add_watch(
                &lib_path,
                WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
            )
            .expect("Failed to add inotify watch");
    }

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");
        // process all the events
        for event in events {
            if event.mask.contains(EventMask::CREATE) {
                if event.mask.contains(EventMask::ISDIR) {
                    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                        rabbit_channel.clone(),
                        "mk_inotify",
                        format!("{{'Type': 'Dir Create', 'JSON': {:?}}}", event.name),
                    )
                    .await
                    .unwrap();
                    #[cfg(debug_assertions)]
                    {
                        mk_lib_logging::mk_logging_post_elk(
                            std::module_path!(),
                            json!({ "Directory created": event.name }),
                        )
                        .await
                        .unwrap();
                    }
                } else {
                    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                        rabbit_channel.clone(),
                        "mk_inotify",
                        format!("{{'Type': 'File Create', 'JSON': {:?}}}", event.name),
                    )
                    .await
                    .unwrap();
                    #[cfg(debug_assertions)]
                    {
                        mk_lib_logging::mk_logging_post_elk(
                            std::module_path!(),
                            json!({ "File created": event.name }),
                        )
                        .await
                        .unwrap();
                    }
                }
            } else if event.mask.contains(EventMask::DELETE) {
                if event.mask.contains(EventMask::ISDIR) {
                    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                        rabbit_channel.clone(),
                        "mk_inotify",
                        format!("{{'Type': 'Dir Delete', 'JSON': {:?}}}", event.name),
                    )
                    .await
                    .unwrap();
                    #[cfg(debug_assertions)]
                    {
                        mk_lib_logging::mk_logging_post_elk(
                            std::module_path!(),
                            json!({ "Directory deleted": event.name }),
                        )
                        .await
                        .unwrap();
                    }
                } else {
                    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                        rabbit_channel.clone(),
                        "mk_inotify",
                        format!("{{'Type': 'File Delete', 'JSON': {:?}}}", event.name),
                    )
                    .await
                    .unwrap();
                    #[cfg(debug_assertions)]
                    {
                        mk_lib_logging::mk_logging_post_elk(
                            std::module_path!(),
                            json!({ "File deleted": event.name }),
                        )
                        .await
                        .unwrap();
                    }
                }
            } else if event.mask.contains(EventMask::MODIFY) {
                if event.mask.contains(EventMask::ISDIR) {
                    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                        rabbit_channel.clone(),
                        "mk_inotify",
                        format!("{{'Type': 'Dir Modify', 'JSON': {:?}}}", event.name),
                    )
                    .await
                    .unwrap();
                    #[cfg(debug_assertions)]
                    {
                        mk_lib_logging::mk_logging_post_elk(
                            std::module_path!(),
                            json!({ "Directory modified": event.name }),
                        )
                        .await
                        .unwrap();
                    }
                } else {
                    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                        rabbit_channel.clone(),
                        "mk_inotify",
                        format!("{{'Type': 'File Modify', 'JSON': {:?}}}", event.name),
                    )
                    .await
                    .unwrap();
                    #[cfg(debug_assertions)]
                    {
                        mk_lib_logging::mk_logging_post_elk(
                            std::module_path!(),
                            json!({ "File modified": event.name }),
                        )
                        .await
                        .unwrap();
                    }
                }
            }
        }
    }
}
