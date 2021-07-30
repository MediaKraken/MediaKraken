use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use serde_json::json;

#[cfg(debug_assertions)]
#[path = "../../src/mk_lib_database/src/mk_lib_database.rs"]
mod mk_lib_database;
#[cfg(debug_assertions)]
#[path = "../../src/mk_lib_network/src/mk_lib_network.rs"]
mod mk_lib_network;

#[derive(Serialize, Deserialize)]
struct Metadata {
    adult: bool,
    id: i32,
    original_title: String,
    popularity: f32,
    video: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    let db_client = &mk_lib_database::mk_lib_database_open().await?;
    // grab the manufacturer's from Global Cache
    let fetch_result = mk_lib_network::mk_data_from_url(
        "https://irdb.globalcache.com:8081/api/brands/").await;
    print!("{:?}", fetch_result);
    // for json_item in json_result.split('\n') {
    //         let metadata_struct: Metadata = serde_json::from_str(json_item)?;
    //         let result = mk_lib_database_metadata::mk_lib_database_metadata_exists_movie(db_client,
    //                                                                                      metadata_struct.id).await.unwrap();
    // }
    Ok(())
}