use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use mk_lib_rabbitmq;
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use tokio::sync::Notify;

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
        .await;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkstack_rabbitmq", "mkmusicbrainz")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkmusicbrainz", &rabbit_channel)
            .await
            .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                #[cfg(debug_assertions)]
                {
                    mk_lib_logging::mk_logging_post_elk(
                        std::module_path!(),
                        json!({ "msg body": json_message }),
                    )
                    .await
                    .unwrap();
                }
                let db_pass = fs::read_to_string("/run/secrets/db_password").unwrap();
                // extensions, collations, types
                let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                    .args([
                        "-h",
                        "mkstack_database",
                        "-U",
                        "postgres",
                        "-f",
                        "/scripts/Extensions.sql",
                    ])
                    .stdout(Stdio::piped())
                    .output()
                    .unwrap();
                let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                    .args([
                        "-h",
                        "mkstack_database",
                        "-U",
                        "postgres",
                        "-f",
                        "/scripts/CreateCollations.sql",
                    ])
                    .stdout(Stdio::piped())
                    .output()
                    .unwrap();

                let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                    .args([
                        "-h",
                        "mkstack_database",
                        "-U",
                        "postgres",
                        "-f",
                        "/scripts/CreateTypes.sql",
                    ])
                    .stdout(Stdio::piped())
                    .output()
                    .unwrap();

                // create tables
                let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                    .args([
                        "-h",
                        "mkstack_database",
                        "-U",
                        "postgres",
                        "-f",
                        "/scripts/CreateTables.sql",
                    ])
                    .stdout(Stdio::piped())
                    .output()
                    .unwrap();
                let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                    .args([
                        "-h",
                        "mkstack_database",
                        "-U",
                        "postgres",
                        "-f",
                        "/scripts/caa/CreateTables.sql",
                    ])
                    .stdout(Stdio::piped())
                    .output()
                    .unwrap();

                // import dump tables
                let pg_tables =
                    mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_size(
                        &sqlx_pool,
                    )
                    .await
                    .unwrap();
                for row_data in pg_tables.iter() {
                    // loop through tables and see if dump files exist
                    if Path::new(&format!("/home/spoot/mbdump/{}", row_data.table_name)).exists() {
                        let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                            .args(["-h", "mkstack_database", "-U", "postgres"])
                            .stdout(Stdio::piped())
                            .output()
                            .unwrap();
                        // PGPASSWORD=pass1234 psql -h mkstack_database -U postgres
                        // PGPASSWORD=pass1234 psql -d postgres -U postgres -c "\copy usa from /Users/EDB1/Downloads/usa.csv delimiter E'\t' csv;"
                    }
                }

                // creat keys and indexes
                let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                    .args([
                        "-h",
                        "mkstack_database",
                        "-U",
                        "postgres",
                        "-f",
                        "/scripts/CreatePrimaryKeys.sql",
                    ])
                    .stdout(Stdio::piped())
                    .output()
                    .unwrap();
                let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                    .args([
                        "-h",
                        "mkstack_database",
                        "-U",
                        "postgres",
                        "-f",
                        "/scripts/caa/CreatePrimaryKeys.sql",
                    ])
                    .stdout(Stdio::piped())
                    .output()
                    .unwrap();
                let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                    .args([
                        "-h",
                        "mkstack_database",
                        "-U",
                        "postgres",
                        "-f",
                        "/scripts/CreateIndexes.sql",
                    ])
                    .stdout(Stdio::piped())
                    .output()
                    .unwrap();
                let _output = Command::new(format!("PGPASSWORD={} psql", db_pass))
                    .args([
                        "-h",
                        "mkstack_database",
                        "-U",
                        "postgres",
                        "-f",
                        "/scripts/caa/CreateIndexes.sql",
                    ])
                    .stdout(Stdio::piped())
                    .output()
                    .unwrap();
   
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
