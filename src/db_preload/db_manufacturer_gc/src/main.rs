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
#[path = "../../../../src/mk_lib_database/src/mk_lib_database_version.rs"]
mod mk_lib_database_version;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_file/src/mk_lib_file.rs"]
mod mk_lib_file;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_network/src/mk_lib_network.rs"]
mod mk_lib_network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database_open_pool().await.unwrap();
    mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool,
                                                           false).await;
    // grab the manufacturer's from Global Cache
    let fetch_result = mk_lib_network::mk_data_from_url_to_json(
        "https://irdb.globalcache.com:8081/api/brands/".to_string()).await.unwrap();
    print!("{:?}", fetch_result);

    let v: Vec<Value> = serde_json::from_str(&fetch_result)?;
    for item in &v {
        println!("here 100");
        // println!("{:?}\n", item);
        // println!("{:?} {:?}\n", item["$id"].to_string().replace("\"", ""), item["Name"].to_string().replace("\"", ""));
        let result = mk_lib_database_hardware_device::mk_lib_database_hardware_manufacturer_upsert(&sqlx_pool,
                                                                                     item["Name"].to_string().to_string().replace("\"", ""),
                                                                                                   item["$id"].to_string().replace("\"", "").parse::<i32>().unwrap()).await?;
        // fetch types for the manufacturer
        //let fetch_result_type = mk_lib_network::mk_data_from_url(format!("https://irdb.globalcache.com:8081/api/brands/{:?}/types",
        println!("here 200");
        let fetch_result_type = mk_lib_network::mk_data_from_url_to_json("https://irdb.globalcache.com:8081/api/brands/10Moons/types".to_string()).await?;
        println!("here 300 {:?}", fetch_result_type);
        // let v_type: Vec<Value> = serde_json::from_str(&fetch_result_type)?;
        // println!("here 400");
        // for item_type in &v_type {
        //     println!("item_type: {:?}\n", item_type);
        // }
        break;
    }
    Ok(())
}