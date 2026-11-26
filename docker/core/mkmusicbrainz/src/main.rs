use serde_json::Value;
use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};
use tokio::io::AsyncWriteExt;
use tokio::sync::Notify;

// MusicBrainz full export. The fullexport index exposes a `LATEST` pointer file
// whose contents are the newest `YYYYMMDD-HHMMSS` directory name.
const MUSICBRAINZ_FULLEXPORT_BASE: &str =
    "https://data.metabrainz.org/pub/musicbrainz/data/fullexport";
// The dump and its extracted tables live on the mounted MusicBrainz volume.
const MB_DUMP_DIR: &str = "/mediakraken/mbdump";
const MB_DUMP_TAR: &str = "/mediakraken/mbdump/mbdump.tar.bz2";
const MB_DUMP_TAR_PARTIAL: &str = "/mediakraken/mbdump/mbdump.tar.bz2.partial";

fn validate_table_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Table name cannot be empty".to_string());
    }
    // Allow alphanumeric, underscore, dot (for schema.table)
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
    {
        return Err(format!("Table name contains invalid characters: {}", name));
    }
    Ok(())
}

/// Validate a fullexport directory name (`YYYYMMDD-HHMMSS`) before it is used
/// to build a download URL, so a malformed `LATEST` value can't inject a path.
fn validate_dump_timestamp(timestamp: &str) -> Result<(), String> {
    let ok = timestamp.len() == 15
        && timestamp.chars().take(8).all(|c| c.is_ascii_digit())
        && timestamp.as_bytes()[8] == b'-'
        && timestamp.chars().skip(9).all(|c| c.is_ascii_digit());
    if !ok {
        return Err(format!("Invalid fullexport timestamp: {}", timestamp));
    }
    Ok(())
}

/// Download the latest MusicBrainz full export and stage its table files for import.
///
/// 1. Reads the `LATEST` pointer to find the newest `YYYYMMDD-HHMMSS` directory.
/// 2. Streams `mbdump.tar.bz2` to the mounted volume (partial file, then rename).
/// 3. Extracts the `mbdump/` table files into the volume, stripping the `mbdump/`
///    prefix so they land at `/mediakraken/mbdump/{table}`.
///
/// Returns the resolved timestamp for logging.
async fn fetch_latest_mbdump() -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    // 1. Resolve the latest fullexport timestamp.
    let latest_url = format!("{}/LATEST", MUSICBRAINZ_FULLEXPORT_BASE);
    let latest_body = client.get(&latest_url).send().await?.text().await?;
    let timestamp = latest_body.trim().to_string();
    validate_dump_timestamp(&timestamp)?;
    println!("Latest MusicBrainz fullexport: {}", timestamp);

    // 2. Stream the dump tarball to the volume.
    let dump_url = format!(
        "{}/{}/mbdump.tar.bz2",
        MUSICBRAINZ_FULLEXPORT_BASE, timestamp
    );
    let mut response = client.get(&dump_url).send().await?;
    if !response.status().is_success() {
        return Err(format!(
            "mbdump download failed with HTTP status: {}",
            response.status()
        )
        .into());
    }

    let mut file = tokio::fs::File::create(MB_DUMP_TAR_PARTIAL).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    drop(file);
    tokio::fs::rename(MB_DUMP_TAR_PARTIAL, MB_DUMP_TAR).await?;

    // 3. Extract the mbdump/ table files, stripping the leading mbdump/ component.
    let output = Command::new("tar")
        .args([
            "-xjf",
            MB_DUMP_TAR,
            "-C",
            MB_DUMP_DIR,
            "--strip-components=1",
            "mbdump",
        ])
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("mbdump extraction failed: {}", stderr).into());
    }

    Ok(timestamp)
}

async fn process_musicbrainz_dump(db_pass: String) -> Result<(), Box<dyn Error>> {
    // extensions, collations, types
    let _output = Command::new("psql")
        .env("PGPASSWORD", &db_pass)
        .args([
            "-h",
            "pgcluster-with-metrics-rw.cnpg-system",
            "-U",
            "postgres",
            "-f",
            "/scripts/Extensions.sql",
        ])
        .stdout(Stdio::piped())
        .output()?;
    let _output = Command::new("psql")
        .env("PGPASSWORD", &db_pass)
        .args([
            "-h",
            "pgcluster-with-metrics-rw.cnpg-system",
            "-U",
            "postgres",
            "-f",
            "/scripts/CreateCollations.sql",
        ])
        .stdout(Stdio::piped())
        .output()?;

    let _output = Command::new("psql")
        .env("PGPASSWORD", &db_pass)
        .args([
            "-h",
            "pgcluster-with-metrics-rw.cnpg-system",
            "-U",
            "postgres",
            "-f",
            "/scripts/CreateTypes.sql",
        ])
        .stdout(Stdio::piped())
        .output()?;

    // create tables
    let _output = Command::new("psql")
        .env("PGPASSWORD", &db_pass)
        .args([
            "-h",
            "pgcluster-with-metrics-rw.cnpg-system",
            "-U",
            "postgres",
            "-f",
            "/scripts/CreateTables.sql",
        ])
        .stdout(Stdio::piped())
        .output()?;

    // no db dumps for caa
    // let _output = Command::new("psql")
    //     .args([
    //         "-h",
    //         "pgcluster-with-metrics-rw.cnpg-system",
    //         "-U",
    //         "postgres",
    //         "-f",
    //         "/scripts/caa/CreateTables.sql",
    //     ])
    //     .stdout(Stdio::piped())
    //     .output()
    //     ?;

    Ok(())
}

async fn import_dump_tables(
    sqlx_pool_rw: &sqlx::PgPool,
    db_pass: &str,
) -> Result<(), Box<dyn Error>> {
    let pg_tables =
        mk_lib_database::mk_lib_database_postgresql::mk_lib_database_tables(sqlx_pool_rw).await?;
    for row_data in pg_tables.iter() {
        // loop through tables and see if dump files exist
        let table_name = row_data.table_name.replace("public.", "");
        if let Err(e) = validate_table_name(&table_name) {
            eprintln!("Skipping invalid table name: {}", e);
            continue;
        }
        println!("Table: {}", table_name);
        // TODO remove all l_* tables?   don't know what these are for
        // TODO remove all event* tables
        // TODO remove all link* tables
        if Path::new(&format!("/mediakraken/mbdump/{}", table_name)).exists() {
            println!("Table Found");
            // Clear existing rows first so re-imports (Update) don't duplicate data.
            // The MusicBrainz schema has no enforced FK constraints, so a plain
            // TRUNCATE is safe regardless of table order.
            let truncate_sql = format!("TRUNCATE {};", table_name);
            let output = Command::new("psql")
                .env("PGPASSWORD", db_pass)
                .args([
                    "-h",
                    "pgcluster-with-metrics-rw.cnpg-system",
                    "-d",
                    "postgres",
                    "-U",
                    "postgres",
                    "-c",
                    &truncate_sql,
                ])
                .output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.is_empty() {
                println!("Output: {}", stdout);
            }
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                eprintln!("Error: {}", stderr);
            }
            let copy_sql = format!(
                "\\copy {} from '/mediakraken/mbdump/{}';",
                table_name, table_name
            );
            if let Some(invalid) = copy_sql
                .chars()
                .find(|c| !c.is_alphanumeric() && !c.is_ascii_punctuation() && !c.is_whitespace())
            {
                eprintln!("Invalid character in SQL command: {}", invalid);
                continue;
            }
            let output = Command::new("psql")
                .env("PGPASSWORD", db_pass)
                .args([
                    "-h",
                    "pgcluster-with-metrics-rw.cnpg-system",
                    "-d",
                    "postgres",
                    "-U",
                    "postgres",
                    "-c",
                    &copy_sql,
                ])
                .output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.is_empty() {
                println!("Output: {}", stdout);
            }
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                eprintln!("Error: {}", stderr);
            }
        } else {
            // remove the table unless it start with mm_
            // the cascade should dump the sequences
            let sub = &table_name[..3];
            if sub != "mm_" {
                let drop_sql = format!("DROP TABLE {} CASCADE;", table_name);
                let output = Command::new("psql")
                    .env("PGPASSWORD", db_pass)
                    .args([
                        "-h",
                        "pgcluster-with-metrics-rw.cnpg-system",
                        "-d",
                        "postgres",
                        "-U",
                        "postgres",
                        "-c",
                        &drop_sql,
                    ])
                    .output()?;
                let stdout = String::from_utf8_lossy(&output.stdout);
                if !stdout.is_empty() {
                    println!("Output: {}", stdout);
                }
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.is_empty() {
                    eprintln!("Error: {}", stderr);
                }
            }
        }
    }
    Ok(())
}

async fn create_keys_indexes(db_pass: &str) -> Result<(), Box<dyn Error>> {
    // creat keys and indexes
    let _output = Command::new("psql")
        .env("PGPASSWORD", db_pass)
        .args([
            "-h",
            "pgcluster-with-metrics-rw.cnpg-system",
            "-U",
            "postgres",
            "-f",
            "/scripts/CreatePrimaryKeys.sql",
        ])
        .stdout(Stdio::piped())
        .output()?;
    // no db dumps for caa
    // let _output = Command::new("psql")
    //     .args([
    //         "-h",
    //         "pgcluster-with-metrics-rw.cnpg-system",
    //         "-U",
    //         "postgres",
    //         "-f",
    //         "/scripts/caa/CreatePrimaryKeys.sql",
    //     ])
    //     .stdout(Stdio::piped())
    //     .output()
    //     ?;
    let _output = Command::new("psql")
        .env("PGPASSWORD", db_pass)
        .args([
            "-h",
            "pgcluster-with-metrics-rw.cnpg-system",
            "-U",
            "postgres",
            "-f",
            "/scripts/CreateIndexes.sql",
        ])
        .stdout(Stdio::piped())
        .output()?;
    // no db dumps for caa
    // let _output = Command::new("psql")
    //     .args([
    //         "-h",
    //         "pgcluster-with-metrics-rw.cnpg-system",
    //         "-U",
    //         "postgres",
    //         "-f",
    //         "/scripts/caa/CreateIndexes.sql",
    //     ])
    //     .stdout(Stdio::piped())
    //     .output()
    //     ?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
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
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkmusicbrainz").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkmusicbrainz", &rabbit_channel)
            .await?;

    let _db_pass = env::var("POSTGRES_PASSWORD")?;

    let pool = sqlx_pool_rw.clone();
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
            // anything else is discarded to avoid a poison-message redelivery loop.
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

            let db_pass = match env::var("POSTGRES_PASSWORD") {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to get POSTGRES_PASSWORD: {}", e);
                    continue;
                }
            };

            println!("MusicBrainz command: {}", command);
            if let Err(e) = fetch_latest_mbdump().await {
                eprintln!("Error in fetch_latest_mbdump: {}", e);
                continue;
            }
            if let Err(e) = process_musicbrainz_dump(db_pass.clone()).await {
                eprintln!("Error in process_musicbrainz_dump: {}", e);
                continue;
            }
            if let Err(e) = import_dump_tables(&pool, &db_pass).await {
                eprintln!("Error in import_dump_tables: {}", e);
                continue;
            }
            if let Err(e) = create_keys_indexes(&db_pass).await {
                eprintln!("Error in create_keys_indexes: {}", e);
                continue;
            }

            // # ??  why  RunSQLScript($DB, 'CreateSearchIndexes.sql', 'Creating search indexes ...');
            let _result =
                mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(&rabbit_channel, delivery_tag).await;
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
    fn test_validate_table_name_valid_simple() {
        assert!(validate_table_name("artists").is_ok());
    }

    #[test]
    fn test_validate_table_name_valid_underscore() {
        assert!(validate_table_name("my_table").is_ok());
    }

    #[test]
    fn test_validate_table_name_valid_schema_table() {
        assert!(validate_table_name("public.artists").is_ok());
    }

    #[test]
    fn test_validate_table_name_valid_numbers() {
        assert!(validate_table_name("table123").is_ok());
    }

    #[test]
    fn test_validate_table_name_empty() {
        let result = validate_table_name("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_table_name_special_chars() {
        assert!(validate_table_name("table-name").is_err());
    }

    #[test]
    fn test_validate_table_name_space() {
        assert!(validate_table_name("table name").is_err());
    }

    #[test]
    fn test_validate_table_name_sql_injection() {
        assert!(validate_table_name("table; DROP TABLE artists").is_err());
    }

    #[test]
    fn test_validate_table_name_slash() {
        assert!(validate_table_name("table/name").is_err());
    }

    #[test]
    fn test_validate_table_name_backslash() {
        assert!(validate_table_name("table\\name").is_err());
    }

    #[test]
    fn test_validate_table_name_quotes() {
        assert!(validate_table_name("table'name").is_err());
    }

    #[test]
    fn test_validate_table_name_double_dot() {
        assert!(validate_table_name("schema..table").is_ok());
    }

    #[test]
    fn test_validate_table_name_leading_dot() {
        assert!(validate_table_name(".table").is_ok());
    }

    #[test]
    fn test_validate_table_name_trailing_dot() {
        assert!(validate_table_name("table.").is_ok());
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
