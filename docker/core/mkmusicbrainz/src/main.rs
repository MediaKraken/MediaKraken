use mk_lib_database;
use mk_lib_rabbitmq;
use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkmusicbrainz")
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
                let db_pass = fs::read_to_string("/run/secrets/db_password").unwrap();
                env::set_var("PGPASSWORD", &db_pass);
                // extensions, collations, types
                let _output = Command::new("psql")
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
                let _output = Command::new("psql")
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

                let _output = Command::new("psql")
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
                let _output = Command::new("psql")
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

                // no db dumps for caa
                // let _output = Command::new("psql")
                //     .args([
                //         "-h",
                //         "mkstack_database",
                //         "-U",
                //         "postgres",
                //         "-f",
                //         "/scripts/caa/CreateTables.sql",
                //     ])
                //     .stdout(Stdio::piped())
                //     .output()
                //     .unwrap();

                // import dump tables
                let pg_tables =
                    mk_lib_database::mk_lib_database_postgresql::mk_lib_database_tables(&sqlx_pool)
                        .await
                        .unwrap();
                for row_data in pg_tables.iter() {
                    // loop through tables and see if dump files exist
                    let table_name = row_data.table_name.replace("public.", "");
                    println!("Table: {}", table_name);
                    // TODO remove all l_* tables?   don't know what these are for
                    // TODO remove all event* tables
                    // TODO remove all link* tables
                    if Path::new(&format!("/mediakraken/mbdump/{}", table_name)).exists() {
                        println!("Table Found");
                        let output = Command::new("psql")
                            .args([
                                "-h",
                                "mkstack_database",
                                "-d",
                                "postgres",
                                "-U",
                                "postgres",
                                "-c",
                                &format!(
                                    "\\copy {} from '/mediakraken/mbdump/{}';",
                                    table_name, table_name
                                ),
                            ])
                            .output()
                            .expect("failed to execute process");
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        println!("Output: {:?}", stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("Output Error: {:?}", stderr);
                    } else {
                        // remove the table unless it start with mm_
                        // the cascade should dump the sequences
                        let sub = &table_name[..3];
                        if sub != "mm_" {
                            let output = Command::new("psql")
                                .args([
                                    "-h",
                                    "mkstack_database",
                                    "-d",
                                    "postgres",
                                    "-U",
                                    "postgres",
                                    "-c",
                                    &format!("DROP TABLE {} CASCADE;", table_name),
                                ])
                                .output()
                                .expect("failed to execute process");
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            println!("Output: {:?}", stdout);
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            println!("Output Error: {:?}", stderr);
                        }
                    }
                }

                // creat keys and indexes
                let _output = Command::new("psql")
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
                // no db dumps for caa
                // let _output = Command::new("psql")
                //     .args([
                //         "-h",
                //         "mkstack_database",
                //         "-U",
                //         "postgres",
                //         "-f",
                //         "/scripts/caa/CreatePrimaryKeys.sql",
                //     ])
                //     .stdout(Stdio::piped())
                //     .output()
                //     .unwrap();
                let _output = Command::new("psql")
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
                // no db dumps for caa
                // let _output = Command::new("psql")
                //     .args([
                //         "-h",
                //         "mkstack_database",
                //         "-U",
                //         "postgres",
                //         "-f",
                //         "/scripts/caa/CreateIndexes.sql",
                //     ])
                //     .stdout(Stdio::piped())
                //     .output()
                //     .unwrap();

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
