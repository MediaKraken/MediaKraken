use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use serde_json::{json, Value};

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database/src/mk_lib_database.rs"]
mod mk_lib_database;
#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_network/src/mk_lib_network.rs"]
mod mk_lib_network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    let db_client = &mk_lib_database::mk_lib_database_open().await?;
    // grab the manufacturer's from Global Cache
    let fetch_result = mk_lib_network::mk_data_from_url(
        "https://irdb.globalcache.com:8081/api/brands".to_string()).await.unwrap();
    print!("{:?}", &fetch_result);

    let v: Vec<Value> = serde_json::from_str(&fetch_result)?;
    for item in &v {
        println!("{:?}\n", item);
    }
    //         let result = mk_lib_database_metadata::mk_lib_database_metadata_exists_movie(db_client,
    //                                                                                      metadata_struct.id).await.unwrap();
    // }
    Ok(())
}