use mk_lib_database;
use mk_lib_network;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use tokio::sync::Notify;

#[derive(Debug, Serialize, Deserialize)]
struct ApiBrands {
    #[serde(rename = "$id")]
    brand_id: String,
    #[serde(rename = "Name")]
    brand_name: String,
    #[serde(rename = "Links")]
    brand_link: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiBrandsTypes {
    #[serde(rename = "$id")]
    brand_id: String,
    #[serde(rename = "Brand")]
    brand_name: String,
    #[serde(rename = "Type")]
    brand_type: String,
    #[serde(rename = "Links")]
    brand_link: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiBrandsTypeModels {
    #[serde(rename = "$id")]
    brand_id: String,
    #[serde(rename = "ID")]
    brand_model_id: String,
    #[serde(rename = "Brand")]
    brand_name: String,
    #[serde(rename = "Type")]
    brand_type: String,
    #[serde(rename = "Name")]
    brand_model: String,
    #[serde(rename = "Notes")]
    brand_notes: String,
    #[serde(rename = "Links")]
    brand_link: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiBrandsTypeCodeset {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkglobalcache")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkglobalcache", &rabbit_channel)
            .await
            .unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                // grab the manufacturer's from Global Cache
                let fetch_brand_result: Vec<ApiBrands> = serde_json::from_str(
                    &mk_lib_network::mk_lib_network::mk_data_from_url(
                        "https://irdb.globalcache.com:8081/api/brands/".to_string(),
                    )
                    .await
                    .unwrap(),
                )
                .unwrap();
                // loop through all brands
                for brand_item in fetch_brand_result.iter() {
                    #[cfg(debug_assertions)]
                    {
                        println!("{:?}\n", brand_item);
                    }
                    let _result =
        mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_manufacturer_upsert(
                &sqlx_pool,
                brand_item.brand_name.replace("\"", ""),
                brand_item
                    .brand_id
                    .replace("\"", "")
                    .parse::<i32>()
                    .unwrap(),
            )
            .await;
                    // fetch types for the manufacturer (dvd, cd, etc)
                    let fetch_result_type: Vec<ApiBrandsTypes> = serde_json::from_str(
                        &mk_lib_network::mk_lib_network::mk_data_from_url(
                            format!(
                                "https://irdb.globalcache.com:8081/api/brands/{}/types",
                                brand_item
                                    .brand_name
                                    .replace("\"", "")
                                    .replace(":", "xcolx")
                                    .replace("&", "xampx")
                                    .replace("+", "xaddx")
                                    .replace(" ", "%20")
                                    .replace("/", "xfslx"),
                            )
                            .to_string(),
                        )
                        .await
                        .unwrap(),
                    )
                    .unwrap();
                    for item_type in fetch_result_type.iter() {
                        #[cfg(debug_assertions)]
                        {
                            println!("item_type: {:?}\n", item_type);
                        }
                        let _result = mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_type_upsert(
                &sqlx_pool,
                item_type.brand_type.replace("\"", ""),
            )
            .await;
                        let fetch_model_type: Vec<ApiBrandsTypeModels> = serde_json::from_str(
                            &mk_lib_network::mk_lib_network::mk_data_from_url(
                                format!(
                        "https://irdb.globalcache.com:8081/api/brands/{}/types/{}/models",
                        item_type
                            .brand_name
                            .replace("\"", "")
                            .replace(":", "xcolx")
                            .replace("&", "xampx")
                            .replace("+", "xaddx")
                            .replace(" ", "%20")
                            .replace("/", "xfslx"),
                        item_type
                            .brand_type
                            .replace("\"", "")
                            .replace("&", "%26")
                            .replace("+", "xaddx")
                            .replace(" ", "%20")
                            .replace("/", "xfslx"),
                        // .replace("Receiver/Preamp", "ReceiverxfslxPreamp")
                        // .replace("TV/DVD/VCR", "TVxfslxDVDxfslxVCR")
                        // .replace("TV/DVD", "TVxfslxDVD")
                        // .replace("TV/VCR", "TVxfslxVCR")
                        // .replace("DVD/VCR", "DVDxfslxVCR")
                    )
                                .to_string(),
                            )
                            .await
                            .unwrap(),
                        )
                        .unwrap();
                        // loop through all the models
                        for item_model in fetch_model_type.iter() {
                            #[cfg(debug_assertions)]
                            {
                                println!("model_item: {:?}\n", item_model);
                            }
                            let device_count =
                mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_model_device_count_by_type(
                        &sqlx_pool,
                        item_model.brand_name.replace("\"", ""),
                        item_model.brand_type.replace("\"", ""),
                        item_model.brand_model.replace("\"", ""),
                    )
                    .await
                    .unwrap();
                            if device_count == 0 {
                                let _result =
                    mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_model_insert(
                            &sqlx_pool,
                            item_model.brand_name.replace("\"", ""),
                            item_model.brand_type.replace("\"", ""),
                            item_model.brand_model.replace("\"", ""),
                        )
                        .await;
                                /*
                                                let fetch_codeset: Vec<ApiBrandsTypeCodeset> = serde_json::from_str(
                                                    &mk_lib_network::mk_data_from_url(
                                                        format!(
                                                            "https://irdb.globalcache.com:8081/api/codesets/{}",
                                                            item_model.brand_model_id.replace("\"", "").replace("&", "%26")
                                                        )
                                                        .to_string(),
                                                    )
                                                    .await
                                                    .unwrap(),
                                                )
                                                .unwrap();
                                                for item_codeset in fetch_codeset.iter() {
                                                        #[cfg(debug_assertions)]
                                {
                                                    println!("item_codeset: {?}\n", item_codeset);}
                                                }
                                                */
                            }
                        }
                    }
                }
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.unwrap().delivery_tag(),
                )
                .await;
            }
        }
    });
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
