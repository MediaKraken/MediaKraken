use mk_lib_common::mk_lib_common_docker;
use mk_lib_metadata;

#[tokio::main]
async fn main() {
    println!(
        "{:?}",
        mk_lib_metadata::metadata_provider::youtube::provider_youtube_trending("us")
            .await
            .unwrap()
    );
}
