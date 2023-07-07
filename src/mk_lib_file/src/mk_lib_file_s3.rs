use awscreds::Credentials;
use awsregion::Region;
use s3::error::S3Error;
use s3::Bucket;

pub async fn mk_lib_file_s3_connect_amazon(
    bucket_name: &str,
    region_name: &str,
    minio_host: &str,
) -> Result<s3::Bucket, S3Error> {
    let bucket = Bucket::new(
        bucket_name,
        region_name.parse()?,
        // Credentials are collected from environment, config, profile or instance metadata
        Credentials::default()?,
    )?;
    Ok(bucket)
}

pub async fn mk_lib_file_s3_connect_google(
    bucket_name: &str,
    region_name: &str,
    endpoint_url: &str,
    minio_host: &str,
) -> Result<s3::Bucket, S3Error> {
    let bucket = Bucket::new(
        bucket_name,
        Region::Custom {
            region: region_name.to_owned(),
            endpoint: endpoint_url.to_owned(),
        },
        Credentials::default()?,
    )?
    .with_path_style();
    Ok(bucket)
}

pub async fn mk_lib_file_s3_connect_minio(
    bucket_name: &str,
    minio_host: &str,
) -> Result<s3::Bucket, S3Error> {
    let bucket = Bucket::new(
        bucket_name,
        Region::Custom {
            region: "eu-central-1".to_owned(),
            endpoint: format!("http://{}:9000", minio_host).to_owned(),
        },
        Credentials::default()?,
    )?
    .with_path_style();
    Ok(bucket)
}

pub async fn mk_lib_file_s3_bucket_delete(bucket_name: &str, minio_host: &str) {
    let response_data = bucket.delete_object(s3_path).await?;
    assert_eq!(response_data.status_code(), 204);
}

pub async fn mk_lib_file_s3_bucket_get(bucket_name: &str, minio_host: &str) {
    let response_data = bucket.get_object(s3_path).await?;
    assert_eq!(response_data.status_code(), 200);
    assert_eq!(test, response_data.as_slice());
    let response_data = bucket
        .get_object_range(s3_path, 100, Some(1000))
        .await
        .unwrap();
    assert_eq!(response_data.status_code(), 206);
    let (head_object_result, code) = bucket.head_object(s3_path).await?;
    assert_eq!(code, 200);
    assert_eq!(
        head_object_result.content_type.unwrap_or_default(),
        "application/octet-stream".to_owned()
    );
}

pub async fn mk_lib_file_s3_bucket_put(bucket_name: &str, minio_host: &str) {
    let response_data = bucket.put_object(s3_path, test).await?;
    assert_eq!(response_data.status_code(), 200);
}
