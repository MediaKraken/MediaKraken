use mk_lib_database;
use mk_lib_rabbitmq;
use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};
use tokio::sync::Notify;

fn validate_table_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Table name cannot be empty".to_string());
    }
    // Allow alphanumeric, underscore, dot (for schema.table)
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.') {
        return Err(format!("Table name contains invalid characters: {}", name));
    }
    Ok(())
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
        mk_lib_database::mk_lib_database_postgresql::mk_lib_database_tables(
            sqlx_pool_rw,
        )
        .await?;
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
            let copy_sql = format!(
                "\\copy {} from '/mediakraken/mbdump/{}';",
                table_name, table_name
            );
            if let Some(invalid) = copy_sql.chars().find(|c| !c.is_alphanumeric() && !c.is_ascii_punctuation() && !c.is_whitespace()) {
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
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    let _ = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
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
            if let Some(_payload) = msg.content {
                let db_pass = match env::var("POSTGRES_PASSWORD") {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Failed to get POSTGRES_PASSWORD: {}", e);
                        continue;
                    }
                };
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
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.unwrap().delivery_tag(),
                )
                .await;
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
