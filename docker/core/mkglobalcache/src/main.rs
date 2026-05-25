use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::signal;

const IRDB_BASE_URL: &str = "https://irdb.globalcache.com:8081/api";

fn sanitize(value: &str) -> String {
    value.replace('"', "")
}

fn encode_brand_path(value: &str) -> String {
    value
        .replace(':', "xcolx")
        .replace('&', "xampx")
        .replace('+', "xaddx")
        .replace(' ', "%20")
        .replace('/', "xfslx")
}

fn encode_type_path(value: &str) -> String {
    value
        .replace('&', "%26")
        .replace('+', "xaddx")
        .replace(' ', "%20")
        .replace('/', "xfslx")
}

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

async fn fetch_json<T: for<'de> Deserialize<'de>>(url: String) -> Result<T, Box<dyn Error>> {
    let body = mk_lib_network::mk_lib_network::mk_data_from_url(url).await?;
    Ok(serde_json::from_str(&body)?)
}

async fn refresh_catalog(
    sqlx_pool_rw: &sqlx::PgPool,
    sqlx_pool_ro: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    let brands: Vec<ApiBrands> = fetch_json(format!("{IRDB_BASE_URL}/brands/")).await?;

    for brand_item in brands.iter() {
        let brand_name = sanitize(&brand_item.brand_name);
        let brand_id: i32 = match sanitize(&brand_item.brand_id).parse() {
            Ok(id) => id,
            Err(error) => {
                eprintln!(
                    "mkglobalcache: skipping brand '{brand_name}', non-numeric id '{}' ({error})",
                    brand_item.brand_id
                );
                continue;
            }
        };
        let brand_name_path = encode_brand_path(&brand_name);

        if let Err(error) =
            mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_manufacturer_upsert(
                sqlx_pool_rw,
                brand_name.clone(),
                brand_id,
            )
            .await
        {
            eprintln!("mkglobalcache: manufacturer upsert failed for '{brand_name}' ({error})");
            continue;
        }

        let types: Vec<ApiBrandsTypes> = match fetch_json(format!(
            "{IRDB_BASE_URL}/brands/{brand_name_path}/types"
        ))
        .await
        {
            Ok(value) => value,
            Err(error) => {
                eprintln!("mkglobalcache: types fetch failed for '{brand_name}' ({error})");
                continue;
            }
        };

        for item_type in types.iter() {
            let type_name = sanitize(&item_type.brand_type);
            let type_name_path = encode_type_path(&type_name);

            if let Err(error) =
                mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_type_upsert(
                    sqlx_pool_rw,
                    type_name.clone(),
                )
                .await
            {
                eprintln!("mkglobalcache: type upsert failed for '{type_name}' ({error})");
                continue;
            }

            let models: Vec<ApiBrandsTypeModels> = match fetch_json(format!(
                "{IRDB_BASE_URL}/brands/{brand_name_path}/types/{type_name_path}/models"
            ))
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    eprintln!(
                        "mkglobalcache: models fetch failed for '{brand_name}'/'{type_name}' ({error})"
                    );
                    continue;
                }
            };

            for item_model in models.iter() {
                let model_name = sanitize(&item_model.brand_model);

                let device_count = match mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_model_device_count_by_type(
                    sqlx_pool_ro,
                    brand_name.clone(),
                    type_name.clone(),
                    model_name.clone(),
                )
                .await
                {
                    Ok(count) => count,
                    Err(error) => {
                        eprintln!(
                            "mkglobalcache: model count failed for '{brand_name}'/'{type_name}'/'{model_name}' ({error})"
                        );
                        continue;
                    }
                };

                if device_count == 0 {
                    if let Err(error) =
                        mk_lib_database::mk_lib_database_hardware_device::mk_lib_database_hardware_model_insert(
                            sqlx_pool_rw,
                            brand_name.clone(),
                            type_name.clone(),
                            model_name.clone(),
                        )
                        .await
                    {
                        eprintln!(
                            "mkglobalcache: model insert failed for '{brand_name}'/'{type_name}'/'{model_name}' ({error})"
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkglobalcache").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkglobalcache", &rabbit_channel)
            .await?;

    loop {
        tokio::select! {
            _ = shutdown_signal() => {
                eprintln!("mkglobalcache: shutdown signal received");
                return Ok(());
            }
            msg = rabbit_consumer.recv() => {
                let Some(msg) = msg else {
                    eprintln!("mkglobalcache: rabbit consumer closed");
                    return Ok(());
                };

                if msg.content.is_some() {
                    let mut retry_count = 0;
                    const MAX_RETRIES: u32 = 3;
                    let mut success = false;

                    while !success && retry_count < MAX_RETRIES {
                        match refresh_catalog(&sqlx_pool_rw, &sqlx_pool_ro).await {
                            Ok(_) => {
                                success = true;
                                eprintln!("mkglobalcache: catalog refresh succeeded");
                            }
                            Err(error) => {
                                retry_count += 1;
                                eprintln!("mkglobalcache: catalog refresh failed ({error}), retry {retry_count}/{MAX_RETRIES}");
                                if retry_count < MAX_RETRIES {
                                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                }
                            }
                        }
                    }

                    if !success {
                        eprintln!("mkglobalcache: catalog refresh failed after {MAX_RETRIES} retries, continuing with next message");
                    }
                }

                if let Some(deliver) = msg.deliver {
                    if let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                        &rabbit_channel,
                        deliver.delivery_tag(),
                    )
                    .await
                    {
                        eprintln!("mkglobalcache: rabbit ack failed ({error})");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_strips_double_quotes() {
        assert_eq!(sanitize("\"Sony\""), "Sony");
        assert_eq!(sanitize("Son\"y"), "Sony");
        assert_eq!(sanitize("plain"), "plain");
    }

    #[test]
    fn encode_brand_path_substitutes_special_characters() {
        assert_eq!(encode_brand_path("A&B"), "AxampxB");
        assert_eq!(encode_brand_path("A+B"), "AxaddxB");
        assert_eq!(encode_brand_path("A B"), "A%20B");
        assert_eq!(encode_brand_path("A/B"), "AxfslxB");
        assert_eq!(encode_brand_path("A:B"), "AxcolxB");
        assert_eq!(
            encode_brand_path("Pioneer: A/V & More"),
            "Pioneerxcolx%20AxfslxV%20xampx%20More"
        );
    }

    #[test]
    fn encode_type_path_substitutes_special_characters() {
        assert_eq!(encode_type_path("A&B"), "A%26B");
        assert_eq!(encode_type_path("A+B"), "AxaddxB");
        assert_eq!(encode_type_path("A B"), "A%20B");
        assert_eq!(encode_type_path("A/B"), "AxfslxB");
        assert_eq!(encode_type_path("A:B"), "A:B");
    }

    #[test]
    fn encode_type_path_preserves_plain_names() {
        assert_eq!(encode_type_path("DVD"), "DVD");
        assert_eq!(encode_type_path("TV/DVD"), "TVxfslxDVD");
        assert_eq!(encode_type_path("Receiver/Preamp"), "ReceiverxfslxPreamp");
    }

    #[test]
    fn api_brands_deserializes_expected_payload() {
        let payload = r#"[{"$id":"1","Name":"Sony","Links":[]}]"#;
        let parsed: Vec<ApiBrands> = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].brand_id, "1");
        assert_eq!(parsed[0].brand_name, "Sony");
    }
}
