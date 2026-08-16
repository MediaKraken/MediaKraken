use chrono::Utc;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use num_format::{Locale, ToFormattedString};
use serde_json::json;
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::time::Duration;
use tokio::process::Command;
use uuid::Uuid;

use mk_lib_common::mk_lib_common_enum_media_type::DLMediaType;
use mk_lib_common::mk_lib_common_media_extension::{
    GAME_EXTENSION, MEDIA_EXTENSION, MEDIA_EXTENSION_SKIP_FFMPEG, SUBTITLE_EXTENSION,
};
use mk_lib_database::mk_lib_database_network_share::{DBShareList, parse_share_name};
use mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push;
use mk_lib_share::mk_lib_smb::FileMetadata;

lazy_static! {
    static ref STACK_CD: Regex = Regex::new(r"(?i)-cd\d")?;
    static ref STACK_CD1: Regex = Regex::new(r"(?i)-cd1(?!\d)")?;
    static ref STACK_PART: Regex = Regex::new(r"(?i)-part\d")?;
    static ref STACK_PART1: Regex = Regex::new(r"(?i)-part1(?!\d)")?;
    static ref STACK_DVD: Regex = Regex::new(r"(?i)-dvd\d")?;
    static ref STACK_DVD1: Regex = Regex::new(r"(?i)-dvd1(?!\d)")?;
    static ref STACK_PT: Regex = Regex::new(r"(?i)-pt\d")?;
    static ref STACK_PT1: Regex = Regex::new(r"(?i)-pt1(?!\d)")?;
    static ref STACK_DISK: Regex = Regex::new(r"(?i)-disk\d")?;
    static ref STACK_DISK1: Regex = Regex::new(r"(?i)-disk1(?!\d)")?;
    static ref STACK_DISC: Regex = Regex::new(r"(?i)-disc\d")?;
    static ref STACK_DISC1: Regex = Regex::new(r"(?i)-disc1(?!\d)")?;
}

fn mk_nfs_uri(share_info: &DBShareList, uri: &str) -> String {
    let share_path = share_info.mm_network_share_path.trim_start_matches('/');
    let path_part = uri.trim_start_matches('/');
    if path_part.is_empty() {
        format!("nfs://{}/{}", share_info.mm_network_share_ip, share_path)
    } else {
        format!(
            "nfs://{}/{}/{}",
            share_info.mm_network_share_ip, share_path, path_part
        )
    }
}

// Recursive listing via `nfs-ls -R`. Returns Ok(empty) for empty shares;
// returns Err only when the command itself fails (missing path, auth, etc.).
async fn mk_nfs_tree(
    share_info: &DBShareList,
    uri: &str,
) -> Result<Vec<FileMetadata>, Box<dyn Error>> {
    let output = Command::new("nfs-ls")
        .arg("-R")
        .arg(mk_nfs_uri(share_info, uri))
        .output()
        .await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "nfs-ls failed with status {:?}: {}",
            output.status.code(),
            stderr.trim()
        )
        .into());
    }
    let stdout_data = String::from_utf8(output.stdout)?;
    let base_prefix = mk_nfs_uri(share_info, "");
    let mut file_list = Vec::new();
    for line in stdout_data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        // Drop the nfs://host/share prefix so downstream code sees a server-relative path.
        let relative = trimmed
            .strip_prefix(&base_prefix)
            .unwrap_or(trimmed)
            .trim_start_matches('/');
        if relative.is_empty() || relative == "." || relative == ".." {
            continue;
        }
        let is_dir = relative.ends_with('/');
        let normalized = relative.trim_end_matches('/');
        file_list.push(FileMetadata {
            name: format!("/{normalized}"),
            directory: is_dir,
        });
    }
    Ok(file_list)
}

// Build the `//{ip}/{share}` URI smbclient expects. `mm_network_share_path`
// may be a UNC path (`\\host\share\sub`) or just a share name; either way only
// the share segment belongs in the URI — host comes from `mm_network_share_ip`.
// Mirrors the approach used by mkwebaxum's share-browse handler.
fn mk_smb_share_uri(share_info: &DBShareList) -> Option<String> {
    let share_name = parse_share_name(&share_info.mm_network_share_path)?;
    Some(format!(
        "//{}/{}",
        share_info.mm_network_share_ip, share_name
    ))
}

// Construct an `smbclient` command running the supplied `-c` script, with
// auth/workgroup args. Rejects metacharacters that could break out of the
// quoted `cd` arg in the script.
fn build_smbclient_command(
    share_info: &DBShareList,
    share_uri: &str,
    smb_commands: &[String],
) -> Result<Command, Box<dyn Error>> {
    let mut cmd = Command::new("smbclient");
    cmd.arg(share_uri)
        .arg("-g")
        .arg("-c")
        .arg(smb_commands.join(";"));
    if let Some(workgroup) = share_info.mm_network_share_workgroup.as_deref() {
        if !workgroup.is_empty() {
            cmd.arg("-W").arg(workgroup);
        }
    }
    let user_opt = share_info
        .mm_share_auth_user
        .as_deref()
        .filter(|u| !u.is_empty());
    if let Some(user) = user_opt {
        let pass = share_info
            .mm_share_auth_password
            .as_deref()
            .unwrap_or_default();
        if user.contains('%') || pass.contains('%') || user.contains('\n') || pass.contains('\n') {
            return Err("smb credentials contain disallowed characters".into());
        }
        cmd.arg("-U").arg(format!("{}%{}", user, pass));
    } else {
        cmd.arg("-N");
    }
    Ok(cmd)
}

fn smb_path_is_safe(cleaned: &str) -> bool {
    !cleaned.contains([';', '"', '\n', '\r', '\\'])
}

// Reachability probe: `cd path; ls`. Success = share is reachable and the
// requested directory exists / is accessible.
async fn mk_smb_probe(share_info: &DBShareList, path_on_share: &str) -> bool {
    let Some(share_uri) = mk_smb_share_uri(share_info) else {
        return false;
    };
    let cleaned = path_on_share.trim_start_matches('/');
    if !smb_path_is_safe(cleaned) {
        return false;
    }
    let mut commands: Vec<String> = Vec::new();
    if !cleaned.is_empty() {
        commands.push(format!("cd \"{}\"", cleaned));
    }
    commands.push(String::from("ls"));
    let mut cmd = match build_smbclient_command(share_info, &share_uri, &commands) {
        Ok(c) => c,
        Err(_) => return false,
    };
    matches!(cmd.output().await, Ok(out) if out.status.success())
}

// Recursive listing via `recurse ON; cd path; ls` parsed from `-g` pipe output.
async fn mk_smb_tree(
    share_info: &DBShareList,
    path_on_share: &str,
) -> Result<Vec<FileMetadata>, Box<dyn Error>> {
    let share_uri = mk_smb_share_uri(share_info)
        .ok_or_else(|| Box::<dyn Error>::from("invalid SMB share path"))?;
    let cleaned = path_on_share.trim_start_matches('/');
    if !smb_path_is_safe(cleaned) {
        return Err("smb path contains disallowed characters".into());
    }
    let mut commands: Vec<String> = vec![String::from("recurse ON"), String::from("prompt OFF")];
    if !cleaned.is_empty() {
        commands.push(format!("cd \"{}\"", cleaned));
    }
    commands.push(String::from("ls"));
    let mut cmd = build_smbclient_command(share_info, &share_uri, &commands)?;
    let output = cmd.output().await?;
    // smbclient with `recurse ON` exits non-zero when any subdirectory in the
    // tree fails (e.g. one inaccessible folder), but still emits the entries
    // it could read on stdout. Parse stdout regardless and only surface an
    // error if we got nothing — otherwise a single permission glitch would
    // discard every discovery for the share.
    let stdout_data = String::from_utf8(output.stdout)?;
    let mut file_list: Vec<FileMetadata> = vec![];
    for line in stdout_data.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 2 {
            continue;
        }
        if parts[0] != "D" && parts[0] != "F" {
            continue;
        }
        if parts[1] == "." || parts[1] == ".." {
            continue;
        }
        let path_value = if parts[1].starts_with('/') {
            parts[1].to_string()
        } else if path_on_share.ends_with('/') {
            format!("{}{}", path_on_share, parts[1])
        } else {
            format!("{}/{}", path_on_share, parts[1])
        };
        file_list.push(FileMetadata {
            name: path_value,
            directory: parts[0] == "D",
        });
    }
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let summary = format!(
            "smbclient exited {:?}: {}",
            output.status.code(),
            stderr.trim()
        );
        if file_list.is_empty() {
            return Err(summary.into());
        }
        if let Err(err) = mk_logging_loki_push(json!({
            "level": "warn",
            "message": "smbclient recursive listing exited non-zero; using partial results",
            "module": module_path!(),
            "payload": {
                "path": path_on_share,
                "entries": file_list.len(),
                "error": summary,
            },
        }))
        .await
        {
            eprintln!("mkmediascanner loki push failed ({err})");
        }
    }
    Ok(file_list)
}

fn is_stacked_non_first(base_name: &str) -> bool {
    let is_stacked = STACK_CD.is_match(base_name).unwrap_or(false)
        || STACK_PART.is_match(base_name).unwrap_or(false)
        || STACK_DVD.is_match(base_name).unwrap_or(false)
        || STACK_PT.is_match(base_name).unwrap_or(false)
        || STACK_DISK.is_match(base_name).unwrap_or(false)
        || STACK_DISC.is_match(base_name).unwrap_or(false);
    if !is_stacked {
        return false;
    }
    let is_first = STACK_CD1.is_match(base_name).unwrap_or(false)
        || STACK_PART1.is_match(base_name).unwrap_or(false)
        || STACK_DVD1.is_match(base_name).unwrap_or(false)
        || STACK_PT1.is_match(base_name).unwrap_or(false)
        || STACK_DISK1.is_match(base_name).unwrap_or(false)
        || STACK_DISC1.is_match(base_name).unwrap_or(false);
    !is_first
}

fn path_contains_segment(path: &str, segment: &str) -> bool {
    let forward = format!("/{segment}/");
    let backward = format!("\\{segment}\\");
    path.contains(&forward) || path.contains(&backward)
}

fn path_contains_prefix(path: &str, prefix: &str) -> bool {
    let forward = format!("/{prefix}");
    let backward = format!("\\{prefix}");
    path.contains(&forward) || path.contains(&backward)
}

fn is_tv_class(media_class: i16) -> bool {
    media_class == DLMediaType::TV
        || media_class == DLMediaType::TV_EPISODE
        || media_class == DLMediaType::TV_SEASON
}

// Decide the effective media class and whether the file should get a
// Roku-thumbnail + ffprobe pass. Returns (new_class, generate_roku_thumb).
fn classify_media(original_class: i16, file_extension: &str, file_path: &str) -> (i16, bool) {
    if original_class == DLMediaType::GAME {
        let new_class = if file_extension == "iso" {
            DLMediaType::GAME_ISO
        } else if file_extension == "chd" {
            DLMediaType::GAME_CHD
        } else {
            DLMediaType::GAME_ROM
        };
        return (new_class, false);
    }

    if SUBTITLE_EXTENSION.contains(&file_extension) {
        let new_class = if original_class == DLMediaType::MOVIE {
            DLMediaType::MOVIE_SUBTITLE
        } else if is_tv_class(original_class) {
            DLMediaType::TV_SUBTITLE
        } else {
            original_class
        };
        return (new_class, false);
    }

    if path_contains_segment(file_path, "trailers") {
        let new_class = if original_class == DLMediaType::MOVIE {
            DLMediaType::MOVIE_TRAILER
        } else if is_tv_class(original_class) {
            DLMediaType::TV_TRAILER
        } else {
            original_class
        };
        return (new_class, true);
    }

    let is_theme = path_contains_prefix(file_path, "theme.")
        || (path_contains_segment(file_path, "backdrops")
            && path_contains_prefix(file_path, "theme."));
    if is_theme {
        let new_class = if original_class == DLMediaType::MOVIE {
            DLMediaType::MOVIE_THEME
        } else if is_tv_class(original_class) {
            DLMediaType::TV_THEME
        } else {
            original_class
        };
        return (new_class, true);
    }

    if path_contains_segment(file_path, "extras") {
        let new_class = if original_class == DLMediaType::MOVIE {
            DLMediaType::MOVIE_EXTRAS
        } else if is_tv_class(original_class) {
            DLMediaType::TV_EXTRAS
        } else {
            original_class
        };
        return (new_class, true);
    }

    (original_class, true)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120)
            .await
            .map_err(|e| format!("failed to open database pool: {e}"))?;
    let _ = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(
        &sqlx_pool_ro,
        false,
    )
    .await;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkmediascanner").await?;
    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkmediascanner", &rabbit_channel)
            .await?;

    while let Some(msg) = rabbit_consumer.recv().await {
        // Message content is a wake-up signal; the actual list of paths comes from the DB.
        let audit_rows =
            match mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_audit_read(
                &sqlx_pool_ro,
            )
            .await
            {
                Ok(rows) => rows,
                Err(error) => {
                    if let Err(err) = mk_logging_loki_push(json!({
                        "level": "error",
                        "message": "library_path_audit_read failed; nacking with requeue",
                        "module": module_path!(),
                        "payload": {"error": error.to_string()},
                    }))
                    .await
                    {
                        eprintln!("mkmediascanner loki push failed ({err})");
                    }
                    // Brief backoff so we don't spin on a downed DB, then nack with
                    // requeue so the broker redelivers the trigger (either to us
                    // after recovery or to another worker).
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    if let Some(deliver) = msg.deliver {
                        if let Err(nack_error) = rabbit_channel
                            .basic_nack(amqprs::channel::BasicNackArguments::new(
                                deliver.delivery_tag(),
                                false,
                                true,
                            ))
                            .await
                        {
                            if let Err(err) = mk_logging_loki_push(json!({
                                "level": "error",
                                "message": "rabbitmq nack failed",
                                "module": module_path!(),
                                "payload": {"error": nack_error.to_string()},
                            }))
                            .await
                            {
                                eprintln!("mkmediascanner loki push failed ({err})");
                            }
                        }
                    }
                    continue;
                }
            };

        for row_data in audit_rows {
            if let Err(err) = mk_logging_loki_push(json!({
                "level": "info",
                "message": format!("library_path_audit: {}", row_data.mm_media_dir_path),
                "module": module_path!(),
                "payload": {"info": "Processing library path audit"},
            }))
            .await
            {
                eprintln!("mkmediascanner loki push failed ({err})");
            }
            let share_info = match mk_lib_database::mk_lib_database_network_share::mk_lib_database_network_share_detail(
                &sqlx_pool_ro,
                row_data.mm_media_dir_share_guid,
            )
            .await
            {
                Ok(s) => s,
                Err(error) => {
                    if let Err(err) = mk_logging_loki_push(json!({
                        "level": "error",
                        "message": "network_share_detail failed",
                        "module": module_path!(),
                        "payload": {
                            "share_guid": row_data.mm_media_dir_share_guid,
                            "error": error.to_string(),
                        },
                    }))
                    .await
                    {
                        eprintln!("mkmediascanner loki push failed ({err})");
                    }
                    continue;
                }
            };

            let path_on_share = format!("/{}", row_data.mm_media_dir_path.trim_start_matches('/'));

            // Probe reachability via smbclient (matches the working pattern in
            // mkwebaxum's share-browse handler). Only fall back to NFS if the
            // SMB probe fails. Per-file dedup below handles already-known files,
            // so we don't try to short-circuit scans by directory mtime.
            let smb_reachable = mk_smb_probe(&share_info, &path_on_share).await;

            if let Err(err) = mk_logging_loki_push(json!({
                "level": "info",
                "message": format!("library_path_audit after smbclient: {}", row_data.mm_media_dir_path),
                "module": module_path!(),
                "payload": {"info": "Completed SMB reachability probe", "smb_reachable": smb_reachable},
            }))
            .await
            {
                eprintln!("mkmediascanner loki push failed ({err})");
            }

            let reachable = smb_reachable || mk_nfs_tree(&share_info, &path_on_share).await.is_ok();

            if let Err(err) = mk_logging_loki_push(json!({
                "level": "info",
                "message": format!("reachable {} after smbclient: {}", reachable, row_data.mm_media_dir_path),
                "module": module_path!(),
                "payload": {"info": "reachable"},
            }))
            .await
            {
                eprintln!("mkmediascanner loki push failed ({err})");
            }

            if !reachable {
                let _ = mk_lib_database::mk_lib_database_notification::mk_lib_database_notification_insert(
                    &sqlx_pool_rw,
                    format!("Unable to connect to share: {}", row_data.mm_media_dir_path),
                    true,
                )
                .await;
                continue;
            }

            if let Err(err) = mk_logging_loki_push(json!({
                "level": "info",
                "message": format!("reachable: {}", row_data.mm_media_dir_path),
                "module": module_path!(),
                "payload": {"info": "reachable check complete"},
            }))
            .await
            {
                eprintln!("mkmediascanner loki push failed ({err})");
            }

            let _ = mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_status_update(
                &sqlx_pool_rw,
                row_data.mm_media_dir_guid,
                json!({"Status": "Added to scan", "Pct": 100}),
            )
            .await;
            // Bump timestamp up front so files added mid-scan aren't skipped next round.
            let _ = mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_timestamp_update(
                &sqlx_pool_rw,
                row_data.mm_media_dir_guid,
            )
            .await;
            let _ = mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_status_update(
                &sqlx_pool_rw,
                row_data.mm_media_dir_guid,
                json!({"Status": "File search scan", "Pct": 0.0}),
            )
            .await;

            // Collect every file under the library path. We run a single
            // recursive smbclient subprocess (matching mkwebaxum's working
            // share-browse pattern). If the SMB probe failed we use
            // `nfs-ls -R` instead.
            let files: Vec<FileMetadata> = if smb_reachable {
                match mk_smb_tree(&share_info, &path_on_share).await {
                    Ok(entries) => entries,
                    Err(cli_error) => {
                        if let Err(err) = mk_logging_loki_push(json!({
                            "level": "error",
                            "message": "smbclient recursive listing failed",
                            "module": module_path!(),
                            "payload": {"path": path_on_share, "error": cli_error.to_string()},
                        }))
                        .await
                        {
                            eprintln!("mkmediascanner loki push failed ({err})");
                        }
                        Vec::new()
                    }
                }
            } else {
                match mk_nfs_tree(&share_info, &path_on_share).await {
                    Ok(entries) => entries,
                    Err(error) => {
                        if let Err(err) = mk_logging_loki_push(json!({
                            "level": "error",
                            "message": "nfs-ls recursive listing failed",
                            "module": module_path!(),
                            "payload": {"path": path_on_share, "error": error.to_string()},
                        }))
                        .await
                        {
                            eprintln!("mkmediascanner loki push failed ({err})");
                        }
                        Vec::new()
                    }
                }
            };

            if let Err(err) = mk_logging_loki_push(json!({
                "level": "info",
                "message": format!("b4 total: {}", row_data.mm_media_dir_path),
                "module": module_path!(),
                "payload": {"info": "reachable check complete"},
            }))
            .await
            {
                eprintln!("mkmediascanner loki push failed ({err})");
            }

            let total_candidates = files.iter().filter(|f| !f.directory).count().max(1) as f64;
            let mut scanned: u64 = 0;
            let mut total_files: u64 = 0;
            let original_media_class = row_data.mm_media_dir_class_enum;

            for file_metadata in files {
                if file_metadata.directory {
                    continue;
                }
                scanned += 1;
                if scanned % 100 == 0 {
                    let pct = (scanned as f64 / total_candidates) * 100.0;
                    let _ = mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_status_update(
                        &sqlx_pool_rw,
                        row_data.mm_media_dir_guid,
                        json!({"Status": "File search scan", "Pct": pct}),
                    )
                    .await;
                }

                // On DB error we skip rather than defaulting to "not present";
                // otherwise a transient read outage would fan every file into
                // an insert attempt and flood the queue with duplicates.
                let already_known = match mk_lib_database::mk_lib_database_library::mk_lib_database_library_file_exists(
                    &sqlx_pool_ro,
                    &file_metadata.name,
                )
                .await
                {
                    Ok(known) => known,
                    Err(error) => {
                        if let Err(err) = mk_logging_loki_push(json!({
                            "level": "error",
                            "message": "library_file_exists failed; skipping file",
                            "module": module_path!(),
                            "payload": {"path": file_metadata.name, "error": error.to_string()},
                        }))
                        .await
                        {
                            eprintln!("mkmediascanner loki push failed ({err})");
                        }
                        continue;
                    }
                };
                if already_known {
                    continue;
                }

                let file_lower = file_metadata.name.to_lowercase();
                let Some(file_extension) = Path::new(&file_lower)
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(str::to_owned)
                else {
                    continue;
                };

                let is_media = MEDIA_EXTENSION.contains(&file_extension.as_str());
                let is_subtitle = SUBTITLE_EXTENSION.contains(&file_extension.as_str());
                let is_game = GAME_EXTENSION.contains(&file_extension.as_str());
                if !is_media && !is_subtitle && !is_game {
                    continue;
                }

                total_files += 1;

                let base_file_name = Path::new(&file_metadata.name)
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap_or(&file_metadata.name);
                let save_dl_record = !is_stacked_non_first(base_file_name);

                let (new_class_type, generate_roku_thumb) =
                    classify_media(original_media_class, &file_extension, &file_metadata.name);

                let media_id = Uuid::now_v7();
                let media_json = json!({ "Added": Utc::now().to_string() });
                if let Err(error) = mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_insert(
                    &sqlx_pool_rw,
                    media_id,
                    new_class_type,
                    &file_metadata.name,
                    None,
                    json!({}),
                    media_json,
                )
                .await
                {
                    if let Err(err) = mk_logging_loki_push(json!({
                        "level": "error",
                        "message": "media insert failed",
                        "module": module_path!(),
                        "payload": {"path": file_metadata.name, "error": error.to_string()},
                    }))
                    .await
                    {
                        eprintln!("mkmediascanner loki push failed ({err})");
                    }
                    continue;
                }

                let needs_ffprobe =
                    !MEDIA_EXTENSION_SKIP_FFMPEG.contains(&file_extension.as_str()) && is_media;
                if needs_ffprobe {
                    if let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                        rabbit_channel.clone(),
                        "mktranscode",
                        json!({
                            "Type": "FFMPEG",
                            "Subtype": "Probe",
                            "Media UUID": media_id,
                            "Media Path": file_metadata.name,
                        })
                        .to_string(),
                    )
                    .await
                    {
                        if let Err(err) = mk_logging_loki_push(json!({
                            "level": "error",
                            "message": "ffprobe publish failed",
                            "module": module_path!(),
                            "payload": {"media_id": media_id, "error": error.to_string()},
                        }))
                        .await
                        {
                            eprintln!("mkmediascanner loki push failed ({err})");
                        }
                    }
                    if generate_roku_thumb && original_media_class != DLMediaType::MUSIC {
                        if let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
                            rabbit_channel.clone(),
                            "mktranscode",
                            json!({
                                "Type": "Roku",
                                "Media UUID": media_id,
                                "Media Path": file_metadata.name,
                            })
                            .to_string(),
                        )
                        .await
                        {
                            if let Err(err) = mk_logging_loki_push(json!({
                                "level": "error",
                                "message": "roku publish failed",
                                "module": module_path!(),
                                "payload": {"media_id": media_id, "error": error.to_string()},
                            }))
                            .await
                            {
                                eprintln!("mkmediascanner loki push failed ({err})");
                            }
                        }
                    }
                }

                if save_dl_record {
                    if let Err(error) = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_insert(
                        &sqlx_pool_rw,
                        "Z".to_string(),
                        new_class_type,
                        media_id,
                        None,
                        "Search".to_string(),
                        Some(&file_metadata.name),
                    )
                    .await
                    {
                        if let Err(err) = mk_logging_loki_push(json!({
                            "level": "error",
                            "message": "download queue insert failed",
                            "module": module_path!(),
                            "payload": {"media_id": media_id, "error": error.to_string()},
                        }))
                        .await
                        {
                            eprintln!("mkmediascanner loki push failed ({err})");
                        }
                    }
                }
            }

            let _ = mk_lib_database::mk_lib_database_library::mk_lib_database_library_path_status_update(
                &sqlx_pool_rw,
                row_data.mm_media_dir_guid,
                json!({"Status": "File scan complete", "Pct": 100}),
            )
            .await;

            if total_files > 0 {
                let _ = mk_lib_database::mk_lib_database_notification::mk_lib_database_notification_insert(
                    &sqlx_pool_rw,
                    format!(
                        "{} file(s) added from {}",
                        total_files.to_formatted_string(&Locale::en),
                        row_data.mm_media_dir_path
                    ),
                    true,
                )
                .await;
            }
        }

        if let Some(deliver) = msg.deliver {
            if let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                &rabbit_channel,
                deliver.delivery_tag(),
            )
            .await
            {
                if let Err(err) = mk_logging_loki_push(json!({
                    "level": "error",
                    "message": "rabbitmq_ack failed",
                    "module": module_path!(),
                    "payload": {"error": error.to_string()},
                }))
                .await
                {
                    eprintln!("mkmediascanner loki push failed ({err})");
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_share() -> DBShareList {
        DBShareList {
            mm_network_share_guid: Uuid::nil(),
            mm_network_share_ip: "192.168.1.100".parse().unwrap(),
            mm_network_share_path: "/shared/media".to_string(),
            mm_network_share_comment: "Test share".to_string(),
            mm_network_share_version: 1,
            mm_share_auth_user: Some("user".to_string()),
            mm_share_auth_password: Some("pass".to_string()),
            mm_network_share_workgroup: Some("WORKGROUP".to_string()),
        }
    }

    #[test]
    fn test_mk_nfs_uri_no_path() {
        let share = test_share();
        assert_eq!(mk_nfs_uri(&share, ""), "nfs://192.168.1.100/shared/media");
    }

    #[test]
    fn test_mk_nfs_uri_with_path() {
        let share = test_share();
        assert_eq!(
            mk_nfs_uri(&share, "/movies"),
            "nfs://192.168.1.100/shared/media/movies"
        );
    }

    #[test]
    fn test_mk_nfs_uri_leading_slash_stripped() {
        let share = test_share();
        assert_eq!(
            mk_nfs_uri(&share, "//movies"),
            "nfs://192.168.1.100/shared/media/movies"
        );
    }

    #[test]
    fn test_mk_nfs_uri_path_with_trailing_slash() {
        let share = test_share();
        assert_eq!(
            mk_nfs_uri(&share, "/movies/"),
            "nfs://192.168.1.100/shared/media/movies/"
        );
    }

    #[test]
    fn test_mk_smb_share_uri_simple() {
        let share = test_share();
        assert_eq!(mk_smb_share_uri(&share), Some("//192.168.1.100/shared"));
    }

    #[test]
    fn test_mk_smb_share_uri_unc_path() {
        let mut share = test_share();
        share.mm_network_share_path = "\\\\server\\share\\sub".to_string();
        assert_eq!(mk_smb_share_uri(&share), Some("//192.168.1.100/share"));
    }

    #[test]
    fn test_mk_smb_share_uri_ip_path() {
        let mut share = test_share();
        share.mm_network_share_path = "//192.168.1.10/media".to_string();
        assert_eq!(mk_smb_share_uri(&share), Some("//192.168.1.100/media"));
    }

    #[test]
    fn test_smb_path_is_safe_valid() {
        assert!(smb_path_is_safe("movies"));
        assert!(smb_path_is_safe("movies/action"));
        assert!(smb_path_is_safe("movies/action"));
        assert!(smb_path_is_safe("movie.mp4"));
    }

    #[test]
    fn test_smb_path_is_safe_semicolon() {
        assert!(!smb_path_is_safe("movies; rm -rf"));
    }

    #[test]
    fn test_smb_path_is_safe_quotes() {
        assert!(!smb_path_is_safe("movies\"test"));
    }

    #[test]
    fn test_smb_path_is_safe_newline() {
        assert!(!smb_path_is_safe("movies\ntest"));
    }

    #[test]
    fn test_smb_path_is_safe_carriage_return() {
        assert!(!smb_path_is_safe("movies\rtest"));
    }

    #[test]
    fn test_smb_path_is_safe_backslash() {
        assert!(!smb_path_is_safe("movies\\test"));
    }

    #[test]
    fn test_is_stacked_non_first_cd() {
        assert!(!is_stacked_non_first("Movie -cd1"));
        assert!(is_stacked_non_first("Movie -cd2"));
        assert!(is_stacked_non_first("Movie -cd10"));
    }

    #[test]
    fn test_is_stacked_non_first_part() {
        assert!(!is_stacked_non_first("Movie -part1"));
        assert!(is_stacked_non_first("Movie -part2"));
        assert!(is_stacked_non_first("Movie -part10"));
    }

    #[test]
    fn test_is_stacked_non_first_dvd() {
        assert!(!is_stacked_non_first("Movie -dvd1"));
        assert!(is_stacked_non_first("Movie -dvd2"));
    }

    #[test]
    fn test_is_stacked_non_first_pt() {
        assert!(!is_stacked_non_first("Movie -pt1"));
        assert!(is_stacked_non_first("Movie -pt2"));
    }

    #[test]
    fn test_is_stacked_non_first_disk() {
        assert!(!is_stacked_non_first("Movie -disk1"));
        assert!(is_stacked_non_first("Movie -disk2"));
    }

    #[test]
    fn test_is_stacked_non_first_disc() {
        assert!(!is_stacked_non_first("Movie -disc1"));
        assert!(is_stacked_non_first("Movie -disc2"));
    }

    #[test]
    fn test_is_stacked_non_first_not_stacked() {
        assert!(!is_stacked_non_first("Movie.mkv"));
        assert!(!is_stacked_non_first("Movie Part 1.mkv"));
    }

    #[test]
    fn test_path_contains_segment_forward() {
        assert!(path_contains_segment("/movies/action/file.mkv", "movies"));
        assert!(path_contains_segment("/movies/action/file.mkv", "action"));
    }

    #[test]
    fn test_path_contains_segment_backslash() {
        assert!(path_contains_segment(
            "\\movies\\action\\file.mkv",
            "movies"
        ));
        assert!(path_contains_segment(
            "\\movies\\action\\file.mkv",
            "action"
        ));
    }

    #[test]
    fn test_path_contains_segment_not_found() {
        assert!(!path_contains_segment("/movies/action/file.mkv", "comedy"));
        assert!(!path_contains_segment("/movies/file.mkv", "action"));
    }

    #[test]
    fn test_path_contains_segment_partial_no_match() {
        assert!(!path_contains_segment("/movies/action/file.mkv", "movie"));
    }

    #[test]
    fn test_path_contains_prefix_forward() {
        assert!(path_contains_prefix("/movies/action/file.mkv", "movies"));
        assert!(path_contains_prefix("/movies/file.mkv", "movies"));
    }

    #[test]
    fn test_path_contains_prefix_backslash() {
        assert!(path_contains_prefix("\\movies\\action\\file.mkv", "movies"));
    }

    #[test]
    fn test_path_contains_prefix_not_found() {
        assert!(!path_contains_prefix("/movies/action/file.mkv", "comedy"));
    }

    #[test]
    fn test_path_contains_prefix_partial_no_match() {
        assert!(!path_contains_prefix("/movies/action/file.mkv", "movie"));
    }

    #[test]
    fn test_is_tv_class_true() {
        assert!(is_tv_class(DLMediaType::TV));
        assert!(is_tv_class(DLMediaType::TV_EPISODE));
        assert!(is_tv_class(DLMediaType::TV_SEASON));
    }

    #[test]
    fn test_is_tv_class_false() {
        assert!(!is_tv_class(DLMediaType::MOVIE));
        assert!(!is_tv_class(DLMediaType::GAME));
        assert!(!is_tv_class(DLMediaType::TV_SUBTITLE));
    }

    #[test]
    fn test_classify_media_game_iso() {
        let (class, _) = classify_media(DLMediaType::GAME, "iso", "/games/game.iso");
        assert_eq!(class, DLMediaType::GAME_ISO);
    }

    #[test]
    fn test_classify_media_game_chd() {
        let (class, _) = classify_media(DLMediaType::GAME, "chd", "/games/game.chd");
        assert_eq!(class, DLMediaType::GAME_CHD);
    }

    #[test]
    fn test_classify_media_game_rom() {
        let (class, _) = classify_media(DLMediaType::GAME, "nfs", "/games/game.nfs");
        assert_eq!(class, DLMediaType::GAME_ROM);
    }

    #[test]
    fn test_classify_media_game_no_roku_thumb() {
        let (_, roku) = classify_media(DLMediaType::GAME, "chd", "/games/game.chd");
        assert!(!roku);
    }

    #[test]
    fn test_classify_media_movie_subtitle() {
        let (class, _) = classify_media(DLMediaType::MOVIE, "srt", "/movies/movie.srt");
        assert_eq!(class, DLMediaType::MOVIE_SUBTITLE);
    }

    #[test]
    fn test_classify_media_tv_subtitle() {
        let (class, _) = classify_media(DLMediaType::TV_EPISODE, "srt", "/tv/episode.srt");
        assert_eq!(class, DLMediaType::TV_SUBTITLE);
    }

    #[test]
    fn test_classify_media_subtitle_unknown_class() {
        let (class, _) = classify_media(999, "srt", "/other/file.srt");
        assert_eq!(class, 999);
    }

    #[test]
    fn test_classify_media_subtitle_no_roku() {
        let (_, roku) = classify_media(DLMediaType::MOVIE, "srt", "/movies/movie.srt");
        assert!(!roku);
    }

    #[test]
    fn test_classify_media_trailer_movie() {
        let (class, _) = classify_media(DLMediaType::MOVIE, "mp4", "/movies/trailers/trailer.mp4");
        assert_eq!(class, DLMediaType::MOVIE_TRAILER);
    }

    #[test]
    fn test_classify_media_trailer_tv() {
        let (class, _) = classify_media(DLMediaType::TV_EPISODE, "mp4", "/tv/trailers/trailer.mp4");
        assert_eq!(class, DLMediaType::TV_TRAILER);
    }

    #[test]
    fn test_classify_media_trailer_generate_roku() {
        let (_, roku) = classify_media(DLMediaType::MOVIE, "mp4", "/movies/trailers/trailer.mp4");
        assert!(roku);
    }

    #[test]
    fn test_classify_media_theme_movie() {
        let (class, _) = classify_media(DLMediaType::MOVIE, "jpg", "/movies/theme.jpg");
        assert_eq!(class, DLMediaType::MOVIE_THEME);
    }

    #[test]
    fn test_classify_media_theme_tv() {
        let (class, _) = classify_media(DLMediaType::TV_SEASON, "jpg", "/tv/season/theme.jpg");
        assert_eq!(class, DLMediaType::TV_THEME);
    }

    #[test]
    fn test_classify_media_theme_backdrops() {
        let (class, _) = classify_media(DLMediaType::MOVIE, "jpg", "/movies/backdrops/theme.jpg");
        assert_eq!(class, DLMediaType::MOVIE_THEME);
    }

    #[test]
    fn test_classify_media_theme_generate_roku() {
        let (_, roku) = classify_media(DLMediaType::MOVIE, "jpg", "/movies/theme.jpg");
        assert!(roku);
    }

    #[test]
    fn test_classify_media_extras_movie() {
        let (class, _) = classify_media(DLMediaType::MOVIE, "mp4", "/movies/extras/featurette.mp4");
        assert_eq!(class, DLMediaType::MOVIE_EXTRAS);
    }

    #[test]
    fn test_classify_media_extras_tv() {
        let (class, _) =
            classify_media(DLMediaType::TV_EPISODE, "mp4", "/tv/season/extras/bts.mp4");
        assert_eq!(class, DLMediaType::TV_EXTRAS);
    }

    #[test]
    fn test_classify_media_extras_generate_roku() {
        let (_, roku) = classify_media(DLMediaType::MOVIE, "mp4", "/movies/extras/featurette.mp4");
        assert!(roku);
    }

    #[test]
    fn test_classify_media_passthrough() {
        let (class, roku) = classify_media(DLMediaType::MOVIE, "mp4", "/movies/movie.mp4");
        assert_eq!(class, DLMediaType::MOVIE);
        assert!(roku);
    }

    #[test]
    fn test_classify_media_unknown_class_passthrough() {
        let (class, roku) = classify_media(999, "mp4", "/other/file.mp4");
        assert_eq!(class, 999);
        assert!(roku);
    }
}
