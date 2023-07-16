// https://github.com/awslabs/aws-sdk-rust

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Region, Client};
use std::error::Error;

pub async fn mk_lib_file_s3_amazon_connect(
    region_name: &str,
) -> Result<aws_sdk_s3::Client, Box<dyn Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new(region_name.to_owned()));
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    Ok(client)
}

// pub async fn mk_lib_file_s3_amazon_bucket_delete(
//     bucket: s3::Bucket,
//     bucket_name: &str,
//     s3_path: &str,
// ) -> Result<(), S3Error> {
//     let response_data = bucket.delete_object(s3_path).await?;
//     assert_eq!(response_data.status_code(), 204);
//     Ok(())
// }

// pub async fn mk_lib_file_s3_amazon_bucket_get(
//     bucket: s3::Bucket,
//     bucket_name: &str,
//     s3_path: &str,
// ) -> Result<(), S3Error> {
//     let response_data = bucket.get_object(s3_path).await?;
//     assert_eq!(response_data.status_code(), 200);
//     assert_eq!(test, response_data.as_slice());
//     let response_data = bucket
//         .get_object_range(s3_path, 100, Some(1000))
//         .await
//         .unwrap();
//     assert_eq!(response_data.status_code(), 206);
//     let (head_object_result, code) = bucket.head_object(s3_path).await?;
//     assert_eq!(code, 200);
//     assert_eq!(
//         head_object_result.content_type.unwrap_or_default(),
//         "application/octet-stream".to_owned()
//     );
//     Ok(())
// }

// pub async fn mk_lib_file_s3_amazon_bucket_put(
//     bucket: s3::Bucket,
//     bucket_name: &str,
//     s3_path: &str,
// ) -> Result<(), S3Error> {
//     let response_data = bucket.put_object(s3_path, test).await?;
//     assert_eq!(response_data.status_code(), 200);
//     Ok(())
// }
