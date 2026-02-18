use fanotify::high_level::{Fanotify, FanotifyMode, Event, MarkFlags, MaskFlags};
use mk_lib_database;
use mk_lib_rabbitmq;
use std::error::Error;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Setup Database and RabbitMQ (unchanged logic)
    let (_sqlx_pool_rw, sqlx_pool_ro) = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false).await?;

    let (_rabbit_connection, rabbit_channel) = 
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkfanotify").await?;

    // 2. Initialize Fanotify 
    // Requires CAP_SYS_ADMIN (sudo)
    let fanotify = Fanotify::new_with_nonblocking(FanotifyMode::CONTENT);

    // 3. Add Watches
    for row_data in mk_lib_database::mk_lib_database_library::mk_lib_database_library_read(&sqlx_pool_ro).await? {
        let lib_path = row_data.mm_media_dir_path;
        
        // MarkFlags::FILESYSTEM allows recursive watching of the entire mount point
        // MarkFlags::MOUNT also works if you want to watch the whole drive
        match fanotify.add_path(
            MarkFlags::FAN_MARK_ADD | MarkFlags::FAN_MARK_MOUNT, 
            MaskFlags::FAN_MODIFY | MaskFlags::FAN_CLOSE_WRITE | MaskFlags::FAN_ON_DIR,
            &lib_path
        ) {
            Ok(_) => println!("Loaded fanotify watch on mount for: {}", lib_path),
            Err(e) => eprintln!("Failed to add fanotify watch: {}", e),
        }
    }

    // 4. Event Loop
    loop {
        let events = fanotify.read();
        for event in events {
            let path = event.path; // fanotify crate handles the /proc/self/fd lookup for you
            let event_type = if event.mask.contains(MaskFlags::FAN_ON_DIR) { "Dir" } else { "File" };
            
            // Map fanotify masks back to your logic
            let action = if event.mask.contains(MaskFlags::FAN_MODIFY) {
                "Modify"
            } else if event.mask.contains(MaskFlags::FAN_CLOSE_WRITE) {
                "Create/Write"
            } else {
                "Access"
            };

            let payload = format!("{{'Type': '{} {}', 'Path': {:?}}}", event_type, action, path);

            // Publish to RabbitMQ
            mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                rabbit_channel.clone(),
                "mk_inotify", // Kept name same for downstream compatibility
                payload,
            ).await?;
        }
        
        // Small sleep to prevent CPU spinning in non-blocking mode if no events
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}