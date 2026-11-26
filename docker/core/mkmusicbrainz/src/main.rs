use serde_json::Value;
use std::env;
use std::error::Error;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use tokio::sync::Notify;

// The MusicBrainz fullexport index exposes a `LATEST` pointer file whose
// contents are the newest `YYYYMMDD-HHMMSS` directory name.
const MUSICBRAINZ_FULLEXPORT_BASE: &str =
    "https://data.metabrainz.org/pub/musicbrainz/data/fullexport";
// The fullexport archives contain data files only (no SQL); the schema DDL is
// fetched from the musicbrainz-server repository at runtime so it always
// matches the dump (their README asks for a compatible server version).
const MUSICBRAINZ_DDL_BASE: &str =
    "https://raw.githubusercontent.com/metabrainz/musicbrainz-server/master/admin/sql";
// The dumps and their extracted tables live on the mounted MusicBrainz volume.
const MB_DUMP_DIR: &str = "/mediakraken/mbdump";
// All four archives carry the same top-level `mbdump/` prefix and their table
// sets are disjoint, so extracting all of them merges into a single data dir.
const MB_DUMP_DATA_DIR: &str = "/mediakraken/mbdump/mbdump";
// Fetched DDL scripts (scratch, re-fetched on every run).
const MB_DDL_DIR: &str = "/tmp/mbddl";
// psql talks to the cluster writer directly (the sqlx pools use pgbouncer).
const PG_HOST_RW: &str = "pgcluster-with-metrics-rw.cnpg-system";
const PG_DATABASE: &str = "mkdatabase";

// The four archives that make up a full export.
const DUMP_ARCHIVES: [&str; 4] = [
    "mbdump.tar.bz2",
    "mbdump-derived.tar.bz2",
    "mbdump-cdstubs.tar.bz2",
    "mbdump-cover-art-archive.tar.bz2",
];

// DDL scripts to fetch, relative to musicbrainz-server's admin/sql/.
const DDL_SCRIPTS: [&str; 14] = [
    "Extensions.sql",
    "CreateCollations.sql",
    "CreateTypes.sql",
    "CreateTables.sql",
    "CreateFunctions.sql",
    "CreateSearchConfiguration.sql",
    "CreatePrimaryKeys.sql",
    "CreateIndexes.sql",
    "CreateSearchIndexes.sql",
    "SetSequences.sql",
    "caa/CreateTables.sql",
    "caa/CreatePrimaryKeys.sql",
    "caa/CreateIndexes.sql",
    "caa/SetSequences.sql",
];

// PGOPTIONS values must not contain spaces (the string is parsed like a
// command line).
const SEARCH_PATH_MUSICBRAINZ: &str = "musicbrainz,public";
const SEARCH_PATH_CAA: &str = "cover_art_archive,public";

/// Validate a fullexport directory name (`YYYYMMDD-HHMMSS`) before it is used
/// to build a download URL, so a malformed `LATEST` value can't inject a path.
fn validate_dump_timestamp(timestamp: &str) -> Result<(), String> {
    let ok = timestamp.len() == 15
        && timestamp.chars().take(8).all(|c| c.is_ascii_digit())
        && timestamp.as_bytes()[8] == b'-'
        && timestamp.chars().skip(9).all(|c| c.is_ascii_digit());
    if !ok {
        return Err(format!("Invalid fullexport timestamp: {timestamp}"));
    }
    Ok(())
}

/// A safe SQL identifier (schema/table/role name) that can be interpolated
/// into SQL or a file name without quoting.
fn is_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('_' | 'a'..='z' | 'A'..='Z'))
        && chars.all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
        })
}

/// A dump file is either `table` (core/derived/cdstubs, which go into the
/// musicbrainz schema) or `schema.table` (cover-art-archive files, e.g.
/// `cover_art_archive.cover_art`).
fn split_dump_name(name: &str) -> Option<(&str, &str)> {
    let (schema, table) = match name.split_once('.') {
        Some(parts) => parts,
        None => ("musicbrainz", name),
    };
    if is_identifier(schema) && is_identifier(table) {
        Some((schema, table))
    } else {
        None
    }
}

/// True if `marker_path` exists and contains exactly `timestamp`. Markers are
/// written only after a stage (download/extraction) succeeds for a given
/// fullexport, so a match means the stage can be skipped on re-runs.
async fn marker_matches(marker_path: &str, timestamp: &str) -> bool {
    match tokio::fs::read_to_string(marker_path).await {
        Ok(contents) => contents.trim() == timestamp,
        Err(_) => false,
    }
}

/// Record that a stage completed for `timestamp`.
async fn write_marker(
    marker_path: &str,
    timestamp: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tokio::fs::write(marker_path, timestamp).await?;
    Ok(())
}

/// Download one archive for `timestamp`, skipping it when a previous run
/// already fetched it (marker and file must both be present).
async fn download_archive(
    client: &reqwest::Client,
    timestamp: &str,
    archive: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let tar_path = format!("{MB_DUMP_DIR}/{archive}");
    let marker_path = format!("{MB_DUMP_DIR}/{archive}.latest");
    if marker_matches(&marker_path, timestamp).await
        && tokio::fs::try_exists(&tar_path).await.unwrap_or(false)
    {
        println!("{archive} already present for {timestamp}, skipping download");
        return Ok(());
    }

    println!("Downloading {archive} for {timestamp}...");
    let url = format!("{MUSICBRAINZ_FULLEXPORT_BASE}/{timestamp}/{archive}");
    let mut response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(format!(
            "{archive} download failed with HTTP status: {}",
            response.status()
        )
        .into());
    }

    // Stream to a partial file, then rename into place, so an interrupted
    // download can't leave a truncated tarball behind.
    let partial_path = format!("{MB_DUMP_DIR}/{archive}.partial");
    let mut file = tokio::fs::File::create(&partial_path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    drop(file);
    tokio::fs::rename(&partial_path, &tar_path).await?;
    write_marker(&marker_path, timestamp).await?;
    println!("Downloaded {archive} for {timestamp}");
    Ok(())
}

/// Extract one archive into MB_DUMP_DIR (skipped when already extracted for
/// `timestamp`).
///
/// Deliberately no `--strip-components`: every archive carries the `mbdump/`
/// top-level prefix, and extracting without stripping merges the four table
/// sets into one data dir (the per-archive header files TIMESTAMP/README/...
/// land at the volume root and are harmless).
///
/// Deliberately no member list: the real archives have no bare `mbdump/`
/// directory member, and busybox tar then reports "mbdump: not found in
/// archive" (non-zero exit) even though every file was extracted.
async fn extract_archive(
    timestamp: &str,
    archive: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let marker_path = format!("{MB_DUMP_DIR}/{archive}.extracted");
    if marker_matches(&marker_path, timestamp).await {
        println!("{archive} already extracted for {timestamp}, skipping extraction");
        return Ok(());
    }

    let tar_path = format!("{MB_DUMP_DIR}/{archive}");
    let size_mb = tokio::fs::metadata(&tar_path).await?.len() / (1024 * 1024);
    println!("Extracting {archive} ({size_mb} MB) into {MB_DUMP_DIR}... this can take a while");
    let output = tokio::process::Command::new("tar")
        .args(["-xjf", &tar_path, "-C", MB_DUMP_DIR])
        .output()
        .await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("extraction of {archive} failed: {stderr}").into());
    }
    write_marker(&marker_path, timestamp).await?;
    println!("Extracted {archive} for {timestamp}");
    Ok(())
}

/// Fetch the schema DDL from musicbrainz-server master into MB_DDL_DIR.
async fn fetch_ddl_scripts(client: &reqwest::Client) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Start clean so a half-fetched run can never leave stale scripts behind.
    match tokio::fs::remove_dir_all(MB_DDL_DIR).await {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }
    for script in DDL_SCRIPTS {
        let url = format!("{MUSICBRAINZ_DDL_BASE}/{script}");
        let response = client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(format!(
                "fetching {script} failed with HTTP status: {}",
                response.status()
            )
            .into());
        }
        let body = response.bytes().await?;
        let path = format!("{MB_DDL_DIR}/{script}");
        if let Some(parent) = Path::new(&path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&path, body).await?;
    }
    println!(
        "Fetched {} DDL scripts into {MB_DDL_DIR}",
        DDL_SCRIPTS.len()
    );
    Ok(())
}

fn ddl_path(script: &str) -> String {
    format!("{MB_DDL_DIR}/{script}")
}

/// Spawn psql against the cluster writer. Either `file` or `command` (or
/// both) may be given; the caller decides whether a non-zero exit is fatal.
async fn run_psql(
    user: &str,
    pass: &str,
    search_path: Option<&str>,
    file: Option<&str>,
    command: Option<&str>,
) -> Result<std::process::Output, Box<dyn Error + Send + Sync>> {
    let mut cmd = tokio::process::Command::new("psql");
    cmd.env("PGPASSWORD", pass);
    if let Some(search_path) = search_path {
        cmd.env("PGOPTIONS", format!("-c search_path={search_path}"));
    }
    let mut args = vec![
        "-h".to_string(),
        PG_HOST_RW.to_string(),
        "-U".to_string(),
        user.to_string(),
        "-d".to_string(),
        PG_DATABASE.to_string(),
    ];
    if let Some(file) = file {
        args.extend(["-f".to_string(), file.to_string()]);
    }
    if let Some(command) = command {
        args.extend(["-c".to_string(), command.to_string()]);
    }
    Ok(cmd.args(args).output().await?)
}

/// Checked psql invocation: a non-zero exit becomes an error with stderr.
async fn checked_psql(
    user: &str,
    pass: &str,
    search_path: Option<&str>,
    file: Option<&str>,
    command: Option<&str>,
    label: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let output = run_psql(user, pass, search_path, file, command).await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{label} failed: {}", stderr.trim()).into());
    }
    Ok(())
}

/// Run a DDL script file as a fatal stage. The scripts manage their own
/// ON_ERROR_STOP, so a non-zero exit means the schema is broken and the run
/// must not continue into the data import.
async fn run_psql_script(
    user: &str,
    pass: &str,
    search_path: Option<&str>,
    file: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    checked_psql(
        user,
        pass,
        search_path,
        Some(file),
        None,
        &format!("psql -f {file}"),
    )
    .await?;
    println!("Ran {file}");
    Ok(())
}

/// (Re)create the musicbrainz and cover_art_archive schemas, owned by the app
/// user, and run the pre-import DDL (extensions, collations, types, tables,
/// functions, search configuration).
///
/// The drop-up-front keeps the run idempotent: the fetched DDL is not
/// re-runnable (no IF NOT EXISTS on types/collations), and the import
/// rebuilds everything from scratch on every Initial/Update anyway.
async fn setup_schemas(
    admin_user: &str,
    admin_pass: &str,
    db_user: &str,
    db_pass: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_identifier(db_user) {
        return Err(format!("POSTGRES_USER {db_user:?} is not a safe identifier").into());
    }
    // The app user owns both schemas (a superuser created them with
    // AUTHORIZATION), so it can drop them. No-op on a first run.
    checked_psql(
        db_user,
        db_pass,
        None,
        None,
        Some("DROP SCHEMA IF EXISTS musicbrainz CASCADE; DROP SCHEMA IF EXISTS cover_art_archive CASCADE;"),
        "dropping schemas",
    )
    .await?;
    // Creating schemas for another role requires a superuser; AUTHORIZATION
    // hands ownership to the app user.
    let create_sql = format!(
        "CREATE SCHEMA IF NOT EXISTS musicbrainz AUTHORIZATION {db_user}; CREATE SCHEMA IF NOT EXISTS cover_art_archive AUTHORIZATION {db_user};"
    );
    checked_psql(
        admin_user,
        admin_pass,
        None,
        None,
        Some(create_sql.as_str()),
        "creating schemas",
    )
    .await?;

    // cube/earthdistance/unaccent + helper functions need a superuser.
    run_psql_script(
        admin_user,
        admin_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("Extensions.sql"),
    )
    .await?;
    // Everything below runs as the app user.
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("CreateCollations.sql"),
    )
    .await?;
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("CreateTypes.sql"),
    )
    .await?;
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("CreateTables.sql"),
    )
    .await?;
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_CAA),
        &ddl_path("caa/CreateTables.sql"),
    )
    .await?;
    // Includes mb_simple_tsvector/mb_lower, which the search indexes use.
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("CreateFunctions.sql"),
    )
    .await?;
    // The GIN search indexes reference the mb_simple text search
    // configuration, so it must exist before they are built.
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("CreateSearchConfiguration.sql"),
    )
    .await?;
    Ok(())
}

/// COPY every dump file into its schema-qualified table. A failing table is
/// logged and skipped (the dump has no FK constraints, so one bad table does
/// not poison the rest); the list of failures is returned for the final
/// summary.
async fn import_dump_data(
    db_user: &str,
    db_pass: &str,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let mut names = Vec::new();
    let mut dir = tokio::fs::read_dir(MB_DUMP_DATA_DIR).await?;
    while let Some(entry) = dir.next_entry().await? {
        if entry
            .file_type()
            .await
            .map(|ft| ft.is_file())
            .unwrap_or(false)
        {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();
    if names.is_empty() {
        return Err(format!("no dump files found in {MB_DUMP_DATA_DIR}").into());
    }
    println!(
        "Importing {} dump files from {MB_DUMP_DATA_DIR}",
        names.len()
    );

    let mut failed = Vec::new();
    for name in &names {
        let Some((schema, table)) = split_dump_name(name) else {
            eprintln!("Skipping unexpected dump file: {name}");
            continue;
        };
        let path = format!("{MB_DUMP_DATA_DIR}/{name}");
        let target = format!("{schema}.{table}");
        let copy_sql = format!("\\copy {target} FROM '{path}'");
        let output = run_psql(db_user, db_pass, None, None, Some(copy_sql.as_str())).await?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Copied {target} ({})", stdout.trim());
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("FAILED copying {target}: {}", stderr.trim());
            failed.push(name.clone());
        }
    }
    Ok(failed)
}

/// Primary keys, plain indexes, sequences and the GIN search indexes. Runs
/// after the data copy; the search index build is by far the slowest step.
async fn create_keys_indexes_sequences(
    db_user: &str,
    db_pass: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("CreatePrimaryKeys.sql"),
    )
    .await?;
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("CreateIndexes.sql"),
    )
    .await?;
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_CAA),
        &ddl_path("caa/CreatePrimaryKeys.sql"),
    )
    .await?;
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_CAA),
        &ddl_path("caa/CreateIndexes.sql"),
    )
    .await?;
    // Keep the SERIAL sequences above the dumped ids so future INSERTs
    // cannot collide with them.
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("SetSequences.sql"),
    )
    .await?;
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_CAA),
        &ddl_path("caa/SetSequences.sql"),
    )
    .await?;
    run_psql_script(
        db_user,
        db_pass,
        Some(SEARCH_PATH_MUSICBRAINZ),
        &ddl_path("CreateSearchIndexes.sql"),
    )
    .await?;
    Ok(())
}

/// One full Initial/Update run: fetch the four archives for the current
/// LATEST, rebuild the schemas from the freshly fetched DDL, and COPY all of
/// the data in.
async fn run_musicbrainz_import(
    db_user: &str,
    db_pass: &str,
    admin_user: &str,
    admin_pass: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    // 1. Resolve the latest fullexport timestamp.
    let latest_url = format!("{MUSICBRAINZ_FULLEXPORT_BASE}/LATEST");
    let latest_body = client.get(&latest_url).send().await?.text().await?;
    let timestamp = latest_body.trim().to_string();
    validate_dump_timestamp(&timestamp)?;
    println!("Latest MusicBrainz fullexport: {timestamp}");

    // 2. Download and extract all four archives (per-archive markers keep
    //    re-runs from re-doing finished stages).
    for archive in DUMP_ARCHIVES {
        download_archive(&client, &timestamp, archive).await?;
    }
    for archive in DUMP_ARCHIVES {
        extract_archive(&timestamp, archive).await?;
    }

    // 3. Fetch the DDL for this run.
    fetch_ddl_scripts(&client).await?;

    // 4. (Re)build the schema.
    setup_schemas(admin_user, admin_pass, db_user, db_pass).await?;

    // 5. Import the data.
    let failed = import_dump_data(db_user, db_pass).await?;

    // 6. Keys, indexes, sequences, search indexes.
    create_keys_indexes_sequences(db_user, db_pass).await?;

    if failed.is_empty() {
        println!("MusicBrainz import completed for {timestamp}");
    } else {
        eprintln!(
            "MusicBrainz import for {timestamp} finished with {} failed table(s): {failed:?}",
            failed.len()
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check (the rw pool is unused for now:
    // all database work in this service goes through psql)
    let (_sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120)
            .await
            .map_err(|e| format!("failed to open database pool: {e}"))?;
    let _ = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(
        &sqlx_pool_ro,
        false,
    )
    .await;

    // Fail fast when the database credentials are missing/misconfigured.
    let db_user = env::var("POSTGRES_USER")?;
    let db_pass = env::var("POSTGRES_PASSWORD")?;
    let admin_user = env::var("POSTGRES_USER_ADMIN")?;
    let admin_pass = env::var("POSTGRES_PASSWORD_ADMIN")?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkmusicbrainz").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkmusicbrainz", &rabbit_channel)
            .await?;

    let db_user_rabbit = db_user.clone();
    let db_pass_rabbit = db_pass.clone();
    let admin_user_rabbit = admin_user.clone();
    let admin_pass_rabbit = admin_pass.clone();
    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            let delivery_tag = match msg.deliver {
                Some(deliver) => deliver.delivery_tag(),
                None => {
                    eprintln!("Rabbit message missing delivery metadata");
                    continue;
                }
            };

            let Some(payload) = msg.content else {
                // Empty message: discard (ack) so the broker doesn't redeliver it.
                let _ =
                    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(&rabbit_channel, delivery_tag)
                        .await;
                continue;
            };

            let json_message: Value = match serde_json::from_slice(&payload) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("Invalid JSON message, discarding: {error}");
                    let _ = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                        &rabbit_channel,
                        delivery_tag,
                    )
                    .await;
                    continue;
                }
            };

            // Commands are dispatched on the `Type` field, matching the other
            // MediaKraken consumers. Only "Initial" and "Update" are accepted;
            // anything else is discarded to avoid a poison-message redelivery
            // loop.
            let command = json_message
                .get("Type")
                .and_then(Value::as_str)
                .unwrap_or("");
            if command != "Initial" && command != "Update" {
                eprintln!(
                    "Unknown command {:?}, discarding (expected \"Initial\" or \"Update\")",
                    command
                );
                let _ =
                    mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(&rabbit_channel, delivery_tag)
                        .await;
                continue;
            }

            println!("MusicBrainz command: {command}");
            match run_musicbrainz_import(
                &db_user_rabbit,
                &db_pass_rabbit,
                &admin_user_rabbit,
                &admin_pass_rabbit,
            )
            .await
            {
                Ok(()) => {
                    let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                        &rabbit_channel,
                        delivery_tag,
                    )
                    .await;
                }
                Err(e) => {
                    // Leave the message unacked so a pod restart redelivers it;
                    // the pipeline is idempotent (per-stage markers + full
                    // schema rebuild), so a retry is safe.
                    eprintln!("MusicBrainz import run failed: {e}");
                }
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_identifier_valid() {
        assert!(is_identifier("musicbrainz"));
        assert!(is_identifier("_private"));
        assert!(is_identifier("cover_art_archive"));
        assert!(is_identifier("Table123"));
    }

    #[test]
    fn test_is_identifier_invalid() {
        assert!(!is_identifier(""));
        assert!(!is_identifier("123abc"));
        assert!(!is_identifier("with space"));
        assert!(!is_identifier("with.dot"));
        assert!(!is_identifier("with-dash"));
        assert!(!is_identifier("with$sign"));
    }

    #[test]
    fn test_split_dump_name_plain_table() {
        assert_eq!(split_dump_name("artist"), Some(("musicbrainz", "artist")));
    }

    #[test]
    fn test_split_dump_name_schema_table() {
        assert_eq!(
            split_dump_name("cover_art_archive.cover_art"),
            Some(("cover_art_archive", "cover_art"))
        );
    }

    #[test]
    fn test_split_dump_name_invalid() {
        assert_eq!(split_dump_name(""), None);
        assert_eq!(split_dump_name("cover_art_archive."), None);
        assert_eq!(split_dump_name(".cover_art"), None);
        assert_eq!(split_dump_name("schema..table"), None);
        assert_eq!(split_dump_name("cover_art_archive/x"), None);
        assert_eq!(
            split_dump_name("cover_art_archive.table; DROP TABLE x"),
            None
        );
    }

    #[test]
    fn test_validate_dump_timestamp_valid() {
        assert!(validate_dump_timestamp("20261125-002239").is_ok());
    }

    #[test]
    fn test_validate_dump_timestamp_wrong_separator() {
        assert!(validate_dump_timestamp("20261125_002239").is_err());
    }

    #[test]
    fn test_validate_dump_timestamp_too_short() {
        assert!(validate_dump_timestamp("2026112-002239").is_err());
    }

    #[test]
    fn test_validate_dump_timestamp_too_long() {
        assert!(validate_dump_timestamp("20261125-0022390").is_err());
    }

    #[test]
    fn test_validate_dump_timestamp_trailing_slash() {
        assert!(validate_dump_timestamp("20261125-002239/").is_err());
    }

    #[test]
    fn test_validate_dump_timestamp_non_numeric() {
        assert!(validate_dump_timestamp("202611a5-002239").is_err());
    }

    #[test]
    fn test_validate_dump_timestamp_path_traversal() {
        assert!(validate_dump_timestamp("../20261125-00223").is_err());
    }
}
