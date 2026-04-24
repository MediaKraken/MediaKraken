use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client as S3Client, config::Builder as S3ConfigBuilder};
use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

const POSTER_BUCKETS: &[&str] = &[
    "movie_poster",
    "tv_poster",
    "person_poster",
    "crew_poster",
    "cast_poster",
    "book_cover",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to local garage s3 and ensure required buckets exist before
    // other containers try to use them.
    let endpoint = env::var("MK_GARAGE_S3_ENDPOINT")
        .or_else(|_| env::var("AWS_ENDPOINT_URL"))
        .unwrap();
    let shared = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_config = S3ConfigBuilder::from(&shared)
        .endpoint_url(endpoint)
        .force_path_style(true)
        .build();
    let s3_client = S3Client::from_conf(s3_config);

    for bucket in POSTER_BUCKETS {
        match s3_client.create_bucket().bucket(*bucket).send().await {
            Ok(_) => println!("Created bucket: {}", bucket),
            Err(err) => {
                let service_err = err.into_service_error();
                if service_err.is_bucket_already_owned_by_you()
                    || service_err.is_bucket_already_exists()
                {
                    println!("Bucket already exists: {}", bucket);
                } else {
                    return Err(Box::new(service_err));
                }
            }
        }
    }

    // connect to db and do a version check and upgrade if needed
    let (sqlx_pool_rw, sqlx_pool_ro) = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
        .await
        .unwrap();
    // see if db exists
    let db_exists = mk_lib_database::mk_lib_database_postgresql::mk_lib_database_table_exists(
        &sqlx_pool_ro,
        "mm_version",
    )
    .await
    .unwrap();
    if db_exists == false {
        let db_pass = env::var("POSTGRES_PASSWORD").unwrap();
        unsafe {
            env::set_var("PGPASSWORD", &db_pass);
        }
        let output = Command::new("psql")
            .args([
                "-h",
                "pgcluster-with-metrics-rw.cnpg-system",
                "-U",
                env::var("POSTGRES_USER").unwrap().as_str(),
                "-d",
                "mkdatabase",
                "-f",
                "/scripts/create_schema.sql",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        let stdout: String = String::from_utf8(output.stdout).unwrap();
        println!("stdout: {}", stdout);
        let stderr: String = String::from_utf8(output.stderr).unwrap();
        println!("stderr: {}", stderr);
    }
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_rw, true)
        .await
        .unwrap();
    Ok(())
}
