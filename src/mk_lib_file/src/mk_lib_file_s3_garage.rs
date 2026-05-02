// https://github.com/awslabs/aws-sdk-rust
// Operations against a local Garage S3 cluster.

use aws_config::BehaviorVersion;
use aws_sdk_s3::config::{Builder as S3ConfigBuilder, Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use std::error::Error;

/// Build an S3 client targeting a local Garage cluster.
///
/// `endpoint` is the Garage S3 API endpoint (e.g. `http://garage-api.garage:3900`).
/// `region` is the Garage region name (e.g. `garage`).
pub async fn mk_lib_file_s3_garage_client(
    access_key_id: &str,
    secret_access_key: &str,
) -> Result<S3Client, Box<dyn Error>> {
    let credentials = Credentials::new(
        std::env::var("ACCESS_KEY_ID").to_string(),
        std::env::var("ACCESS_SECRET_KEY").to_string(),
        None,
        None,
        "mk_lib_file_s3_garage",
    );
    let shared = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_config = S3ConfigBuilder::from(&shared)
        .endpoint_url("http://garage-api.garage:3900".to_string())
        .region(Region::new("garage".to_string()))
        .credentials_provider(credentials)
        .force_path_style(true)
        .build();
    Ok(S3Client::from_conf(s3_config))
}

/// Add a new object to the bucket.
pub async fn mk_lib_file_s3_garage_add(
    client: &S3Client,
    bucket: &str,
    key: &str,
    body: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(body))
        .send()
        .await?;
    Ok(())
}

/// Delete an object from the bucket.
pub async fn mk_lib_file_s3_garage_delete(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<(), Box<dyn Error>> {
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    Ok(())
}

/// Update an existing object by overwriting it.
/// S3 has no in-place update; PUT replaces the object at `key`.
pub async fn mk_lib_file_s3_garage_update(
    client: &S3Client,
    bucket: &str,
    key: &str,
    body: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(body))
        .send()
        .await?;
    Ok(())
}

/// Load (GET) an object from the bucket and return the bytes.
pub async fn mk_lib_file_s3_garage_load(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let response = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    let bytes = response.body.collect().await?.into_bytes();
    Ok(bytes.to_vec())
}

/// Object listing entry returned by [`mk_lib_file_s3_garage_list`].
#[derive(Debug, Clone)]
pub struct GarageObject {
    pub key: String,
    pub size: u64,
    pub last_modified: Option<i64>,
}

/// List objects in the bucket, optionally filtered by prefix.
/// Pages through results until all objects are collected.
pub async fn mk_lib_file_s3_garage_list(
    client: &S3Client,
    bucket: &str,
    prefix: Option<&str>,
) -> Result<Vec<GarageObject>, Box<dyn Error>> {
    let mut results: Vec<GarageObject> = Vec::new();
    let mut continuation_token: Option<String> = None;
    loop {
        let mut request = client.list_objects_v2().bucket(bucket);
        if let Some(p) = prefix {
            request = request.prefix(p);
        }
        if let Some(token) = continuation_token.as_deref() {
            request = request.continuation_token(token);
        }
        let page = request.send().await?;
        for object in page.contents() {
            let Some(key) = object.key() else {
                continue;
            };
            let size = object
                .size()
                .and_then(|s| u64::try_from(s).ok())
                .unwrap_or(0);
            let last_modified = object.last_modified().map(|ts| ts.secs());
            results.push(GarageObject {
                key: key.to_string(),
                size,
                last_modified,
            });
        }
        if page.is_truncated().unwrap_or(false) {
            continuation_token = page.next_continuation_token().map(|s| s.to_string());
            if continuation_token.is_none() {
                break;
            }
        } else {
            break;
        }
    }
    Ok(results)
}
