use async_compression::tokio::bufread::GzipDecoder;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client as S3Client, config::Builder as S3ConfigBuilder};
use futures::StreamExt;
use mk_lib_file::mk_lib_file_s3_garage::mk_lib_file_s3_garage_add;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{Pool, Postgres};
use std::io::Read;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::task::JoinSet;

const BASE_URL: &str = "https://openlibrary.org/data/";

#[derive(Debug, Deserialize)]
struct BulkImageCoverLoadOptions {
    #[serde(rename = "ArchiveUrl")]
    archive_url: String,
    #[serde(rename = "Bucket")]
    bucket: Option<String>,
    #[serde(rename = "S3Prefix", default = "default_cover_prefix")]
    s3_prefix: String,
    #[serde(rename = "AddJson", default)]
    add_json: bool,
}

fn default_cover_prefix() -> String {
    "openlibrary/covers".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Database using your library
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120)
            .await
            .map_err(|e| format!("failed to open database pool: {e}"))?;

    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    // 2. Initialize RabbitMQ using your library
    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkopenlibrarynetfetchbulk").await?;

    let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
        "mkopenlibrarynetfetchbulk",
        &rabbit_channel,
    )
    .await?;

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
        if json_message["Type"].as_str() == Some("START_BULK_IMAGE_COVER_LOAD")
            || json_message["BulkImageCoverLoad"].is_object()
        {
            match serde_json::from_value::<BulkImageCoverLoadOptions>(
                json_message["BulkImageCoverLoad"].clone(),
            ) {
                Ok(options) => {
                    tokio::spawn(async move {
                        if let Err(e) = run_bulk_cover_archive_upload(options).await {
                            eprintln!("❌ Bulk cover archive upload failed: {}", e);
                        }
                    });
                }
                Err(err) => {
                    eprintln!(
                        "⚠️ Skipping cover archive upload due to invalid BulkImageCoverLoad payload: {}",
                        err
                    );
                }
            }
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

async fn run_bulk_cover_archive_upload(options: BulkImageCoverLoadOptions) -> anyhow::Result<()> {
    let endpoint = std::env::var("MK_GARAGE_S3_ENDPOINT")
        .or_else(|_| std::env::var("AWS_ENDPOINT_URL"))
        .map_err(|_| anyhow::anyhow!("missing MK_GARAGE_S3_ENDPOINT (or AWS_ENDPOINT_URL)"))?;

    let bucket = options
        .bucket
        .or_else(|| std::env::var("MK_GARAGE_S3_BUCKET").ok())
        .ok_or_else(|| {
            anyhow::anyhow!("missing BulkImageCoverLoad.Bucket and MK_GARAGE_S3_BUCKET")
        })?;

    let shared = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_config = S3ConfigBuilder::from(&shared)
        .endpoint_url(endpoint)
        .force_path_style(true)
        .build();
    let s3_client = S3Client::from_conf(s3_config);

    let response = reqwest::get(&options.archive_url)
        .await?
        .error_for_status()?;
    let archive_bytes = response.bytes().await?;

    let s3_prefix = options.s3_prefix.trim_matches('/').to_string();
    let upload_s3_prefix = s3_prefix.clone();
    let add_json = options.add_json;
    let archive_url = options.archive_url.clone();
    let upload_bucket = bucket.clone();

    let uploaded = tokio::task::spawn_blocking(move || -> anyhow::Result<usize> {
        let mut uploaded_count: usize = 0;
        let rt = tokio::runtime::Handle::current();
        let cursor = std::io::Cursor::new(archive_bytes);
        let decoder = flate2::read::GzDecoder::new(cursor);
        let mut archive = tar::Archive::new(decoder);

        for item in archive.entries()? {
            let mut entry = item?;
            let entry_path = entry.path()?.display().to_string();
            let Some(file_name) = std::path::Path::new(&entry_path).file_name() else {
                continue;
            };
            let file_name = file_name.to_string_lossy().to_string();

            let content_type = match std::path::Path::new(&file_name)
                .extension()
                .and_then(|v| v.to_str())
                .map(|ext| ext.to_ascii_lowercase())
                .as_deref()
            {
                Some("jpg" | "jpeg") => "image/jpeg",
                Some("png") => "image/png",
                Some("webp") => "image/webp",
                _ => continue,
            };

            let mut payload = Vec::new();
            entry.read_to_end(&mut payload)?;
            let payload_len = payload.len();
            let object_key = format!("{}/{}", upload_s3_prefix, file_name);

            rt.block_on(mk_lib_file_s3_garage_add(
                &s3_client,
                &upload_bucket,
                &object_key,
                payload,
                Some(content_type),
            ))
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            uploaded_count += 1;

            if add_json {
                let json_key = format!("{object_key}.json");
                let metadata = json!({
                    "source_archive_url": archive_url,
                    "source_archive_path": entry_path,
                    "object_key": object_key,
                    "size_bytes": payload_len,
                    "content_type": content_type
                });
                rt.block_on(mk_lib_file_s3_garage_add(
                    &s3_client,
                    &upload_bucket,
                    &json_key,
                    serde_json::to_vec(&metadata)?,
                    Some("application/json"),
                ))
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
        }
        Ok(uploaded_count)
    })
    .await??;

    println!(
        "✅ Uploaded {} cover images to s3://{}/{}",
        uploaded, bucket, s3_prefix
    );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bulk_image_cover_load_options_all_fields() {
        let json = r#"{"ArchiveUrl":"http://example.com/archive.tar.gz","Bucket":"my-bucket","S3Prefix":"custom/prefix","AddJson":true}"#;
        let options: BulkImageCoverLoadOptions = serde_json::from_str(json).unwrap();
        assert_eq!(options.archive_url, "http://example.com/archive.tar.gz");
        assert_eq!(options.bucket, Some("my-bucket".to_string()));
        assert_eq!(options.s3_prefix, "custom/prefix");
        assert!(options.add_json);
    }

    #[test]
    fn test_bulk_image_cover_load_options_minimal() {
        let json = r#"{"ArchiveUrl":"http://example.com/archive.tar.gz"}"#;
        let options: BulkImageCoverLoadOptions = serde_json::from_str(json).unwrap();
        assert_eq!(options.archive_url, "http://example.com/archive.tar.gz");
        assert!(options.bucket.is_none());
        assert_eq!(options.s3_prefix, "default_cover_prefix");
        assert!(!options.add_json);
    }

    #[test]
    fn test_bulk_image_cover_load_options_null_bucket() {
        let json = r#"{"ArchiveUrl":"http://example.com/archive.tar.gz","Bucket":null}"#;
        let options: BulkImageCoverLoadOptions = serde_json::from_str(json).unwrap();
        assert!(options.bucket.is_none());
    }

    #[test]
    fn test_bulk_image_cover_load_options_default_s3_prefix() {
        let json = r#"{"ArchiveUrl":"http://example.com/archive.tar.gz","S3Prefix":""}"#;
        let options: BulkImageCoverLoadOptions = serde_json::from_str(json).unwrap();
        assert_eq!(options.s3_prefix, "");
    }

    #[test]
    fn test_bulk_image_cover_load_options_add_json_false() {
        let json = r#"{"ArchiveUrl":"http://example.com/archive.tar.gz","AddJson":false}"#;
        let options: BulkImageCoverLoadOptions = serde_json::from_str(json).unwrap();
        assert!(!options.add_json);
    }

    #[test]
    fn test_bulk_image_cover_load_options_serialization_roundtrip() {
        let options = BulkImageCoverLoadOptions {
            archive_url: "http://test.com/archive.tar.gz".to_string(),
            bucket: Some("test-bucket".to_string()),
            s3_prefix: "test/prefix".to_string(),
            add_json: true,
        };
        let serialized = serde_json::to_string(&options).unwrap();
        let deserialized: BulkImageCoverLoadOptions = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.archive_url, options.archive_url);
        assert_eq!(deserialized.bucket, options.bucket);
        assert_eq!(deserialized.s3_prefix, options.s3_prefix);
        assert_eq!(deserialized.add_json, options.add_json);
    }

    #[test]
    fn test_default_cover_prefix_function() {
        assert_eq!(default_cover_prefix(), "openlibrary/covers");
    }

    #[test]
    fn test_base_url_constant() {
        assert_eq!(BASE_URL, "https://openlibrary.org/data/");
    }

    #[test]
    fn test_bulk_image_cover_load_options_debug() {
        let options = BulkImageCoverLoadOptions {
            archive_url: "http://example.com/archive.tar.gz".to_string(),
            bucket: None,
            s3_prefix: "prefix".to_string(),
            add_json: false,
        };
        let debug_str = format!("{:?}", options);
        assert!(debug_str.contains("BulkImageCoverLoadOptions"));
        assert!(debug_str.contains("archive_url"));
    }
}
