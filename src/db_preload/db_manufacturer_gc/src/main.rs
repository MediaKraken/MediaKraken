use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use serde_json::{json, Value};

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database/src/mk_lib_database.rs"]
mod mk_lib_database;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_database/src/mk_lib_database_hardware_device.rs"]
mod mk_lib_database_hardware_device;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_file/src/mk_lib_file.rs"]
mod mk_lib_file;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_network/src/mk_lib_network.rs"]
mod mk_lib_network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // open the database
    let db_client = &mk_lib_database::mk_lib_database_open().await?;
    // grab the manufacturer's from Global Cache
    // let fetch_result = mk_lib_network::mk_data_from_url(
    //     "https://irdb.globalcache.com:8081/api/brands/".to_string()).await;
    let fetch_result = mk_lib_file::mk_read_file_data(
        &"/home/spoot/MediaKraken/src/db_preload/db_manufacturer_gc/src/data.txt".to_string()).unwrap();
    //print!("{:?}", fetch_result);

    let v: Vec<Value> = serde_json::from_str(&fetch_result)?;
    for item in &v {
        println!("{:?}\n", item);
        println!("{:?} {:?}\n", item["$id"], item["Name"]);
        let result = mk_lib_database_hardware_device::mk_lib_database_hardware_manufacturer_upsert(db_client,
                                                                                     item["Name"].to_string(),
                                                                                                   item["$id"].to_string().parse::<i16>().unwrap()).await;
    }
    Ok(())
}