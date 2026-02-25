use async_compression::tokio::bufread::GzipDecoder;
use futures::StreamExt;
use serde_json::Value;
use sqlx::{Pool, Postgres};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::task::JoinSet;

const BASE_URL: &str = "https://openlibrary.org/data/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Database using your library
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120)
            .await
            .unwrap();

    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await
        .unwrap();

    // 2. Initialize RabbitMQ using your library
    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkopenlibrarynetfetchbulk")
            .await
            .unwrap();

    let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
        "mkopenlibrarynetfetchbulk",
        &rabbit_channel,
    )
    .await
    .unwrap();

    println!("📥 Worker online. Waiting for 'START_BULK_LOAD' signal...");

    // 3. The Message Loop
    while let Some(msg) = rabbit_consumer.recv().await {
        let Some(payload) = msg.content else { continue };

        let json_message: Value = match serde_json::from_slice(&payload) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                continue;
            }
        };

        if json_message["Type"].as_str() == Some("START_BULK_LOAD") {
            let pool = sqlx_pool_rw.clone();

            // Spawn the import task so the RabbitMQ consumer remains responsive
            tokio::spawn(async move {
                if let Err(e) = run_bulk_import(pool).await {
                    eprintln!("❌ Bulk import failed: {}", e);
                }
            });
        }

        // Always Ack to keep the queue moving
        if let Some(deliver) = msg.deliver {
            let _ = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                &rabbit_channel,
                deliver.delivery_tag(),
            )
            .await;
        }
    }

    Ok(())
}

async fn run_bulk_import(pool: Pool<Postgres>) -> anyhow::Result<()> {
    // We define this as a vector of borrowed strings (static)
    let jobs = vec![
        ("tmp_mm_openlib_author", "ol_dump_authors_latest.txt.gz"),
        ("tmp_mm_openlib_work", "ol_dump_works_latest.txt.gz"),
        ("tmp_mm_openlib_edition", "ol_dump_editions_latest.txt.gz"),
    ];

    println!("🚀 Starting Stream-Download & Import...");
    let mut import_set = JoinSet::new();

    // 1. USE &jobs TO BORROW
    for (table, filename) in &jobs {
        let p = pool.clone();
        let t = table.to_string(); // Clone for 'static task
        let url = format!("{}{}", BASE_URL, filename);

        import_set.spawn(async move { download_and_import(p, &t, &url).await });
    }

    while let Some(res) = import_set.join_next().await {
        res??;
    }

    println!("✅ Ingestion Complete. Starting Parallel Indexing...");
    let mut index_set = JoinSet::new();
    // 2. USE &jobs TO BORROW
    for (table, _) in &jobs {
        let p = pool.clone();
        let t = table.to_string(); // Clone for 'static task
        index_set.spawn(async move {
            let mut conn = p.acquire().await?;
            sqlx::query("SET maintenance_work_mem = '1GB'")
                .execute(&mut *conn)
                .await?;

            let sql = format!("CREATE INDEX IF NOT EXISTS {}_key_idx ON {} (key);", t, t);
            sqlx::query(&sql).execute(&mut *conn).await?;
            Ok::<(), anyhow::Error>(())
        });
    }

    while let Some(res) = index_set.join_next().await {
        res??;
    }

    println!("🔒 Converting tables to LOGGED for durability...");

    for (table, _) in &jobs {
        let mut conn = pool.acquire().await?;

        println!("Logging table {}...", table);

        sqlx::query("SET lock_timeout = '30s'")
            .execute(&mut *conn)
            .await?;

        let sql = format!("ALTER TABLE {} SET LOGGED;", table);
        sqlx::query(&sql).execute(&mut *conn).await?;
    }

    println!("🔄 Performing final table swap...");
    // 4. ALREADY USING &jobs HERE - THIS WAS CORRECT
    for (tmp_table, _) in &jobs {
        let final_name = tmp_table.replace("tmp_", "");
        let mut conn = pool.acquire().await?;

        let swap_sql = format!(
            "DROP TABLE IF EXISTS {final_name} CASCADE; 
             ALTER TABLE {tmp_table} RENAME TO {final_name};"
        );

        match sqlx::query(&swap_sql).execute(&mut *conn).await {
            Ok(_) => println!("✅ Swapped {tmp_table} -> {final_name}"),
            Err(e) => eprintln!("❌ Failed to swap {tmp_table}: {e}"),
        }

        let old_idx = format!("{tmp_table}_key_idx");
        let new_idx = format!("{final_name}_key_idx");
        let _ = sqlx::query(&format!(
            "ALTER INDEX IF EXISTS {old_idx} RENAME TO {new_idx};"
        ))
        .execute(&mut *conn)
        .await;
    }

    println!("🏆 Migration successful! The new data is now live.");
    Ok(())
}

async fn download_and_import(pool: Pool<Postgres>, table: &str, url: &str) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    // Use UNLOGGED for raw speed during initial load
    sqlx::query(&format!(
        "CREATE UNLOGGED TABLE IF NOT EXISTS {} (type text, key text, revision int, last_modified timestamp, data jsonb);",
        table
    )).execute(&mut *conn).await?;

    let response = reqwest::get(url).await?.error_for_status()?;

    // Map reqwest error to std::io::Error for the StreamReader
    let stream = response
        .bytes_stream()
        .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));

    let reader = tokio_util::io::StreamReader::new(stream);
    let decoder = GzipDecoder::new(BufReader::new(reader));
    let mut lines = BufReader::new(decoder).lines();

    let mut writer = conn
        .copy_in_raw(&format!(
            "COPY {} FROM STDIN WITH (FORMAT csv, DELIMITER E'\\t', QUOTE E'\\b')",
            table
        ))
        .await?;

    let mut count = 0;
    while let Some(mut line) = lines.next_line().await? {
        // next_line() strips the newline, so we add it back for the CSV parser
        line.push('\n');

        // Use .send() for sqlx 0.8 compatibility
        writer.send(line.as_bytes()).await?;

        count += 1;
        if count % 500_000 == 0 {
            println!("- {}: {} rows...", table, count);
        }
    }

    writer.finish().await?;
    println!("⭐ Finished {}: {} total rows.", table, count);
    Ok(())
}
