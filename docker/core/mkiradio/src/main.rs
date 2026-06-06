use radiobrowser::{RadioBrowserAPI, StationOrder};
use serde_json::Value;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to DB and check version.
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(4, 120).await?;

    let _ = mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(
        &sqlx_pool_ro,
        false,
    )
    .await;

    // Connect to RabbitMQ.
    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkiradio").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkiradio", &rabbit_channel).await?;

    while let Some(msg) = rabbit_consumer.recv().await {
        let delivery_tag = msg.deliver.as_ref().map(|d| d.delivery_tag());

        let Some(payload) = msg.content.as_ref() else {
            eprintln!("Received message with no payload.");
            if let Some(tag) = delivery_tag {
                let _ = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(&rabbit_channel, tag).await;
            }
            continue;
        };

        let json_message: Value = match serde_json::from_slice(payload) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("Invalid JSON payload: {err}");
                if let Some(tag) = delivery_tag {
                    let _ =
                        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(&rabbit_channel, tag).await;
                }
                continue;
            }
        };

        println!(" [x] Received {:?}", json_message);

        if json_message.get("Type").and_then(Value::as_str) == Some("radiobrowser") {
            // Add retry logic for RadioBrowser API calls
            let mut retries = 0;
            const MAX_RETRIES: u32 = 3;
            let mut api_call_success = false;

            while !api_call_success && retries < MAX_RETRIES {
                match RadioBrowserAPI::new().await {
                    Ok(api) => {
                        match api
                            .get_stations()
                            .order(StationOrder::Clickcount)
                            .reverse(true)
                            .send()
                            .await
                        {
                            Ok(stations) => {
                                api_call_success = true;
                                if stations.is_empty() {
                                    println!("No stations found.");
                                } else {
                                    let mut upsert_count = 0usize;

                                    for (idx, station) in stations.iter().enumerate() {
                                        println!("{}. {}", idx + 1, station.name);
                                        println!("   Station UUID: {}", station.stationuuid);

                                        if !station.country.is_empty() {
                                            println!("   Country: {}", station.country);
                                        }

                                        if !station.language.is_empty() {
                                            println!("   Language: {}", station.language);
                                        }

                                        if !station.tags.is_empty() {
                                            println!("   Tags: {}", station.tags);
                                        }

                                        println!("   Stream URL: {}", station.url_resolved);
                                        println!();

                                        match mk_lib_database::database_media::mk_lib_database_media_iradio::mk_lib_database_media_iradio_upsert(
                                            &sqlx_pool_rw,
                                            station.stationuuid.as_ref(),
                                            station.name.as_ref(),
                                            station.url.as_ref(),
                                            Some(station.country.as_str()),
                                            Some(station.language.as_str()),
                                            Some(station.tags.as_str()),
                                            station.url_resolved.as_ref(),
                                        )
                                        .await
                                        {
                                            Ok(_) => {
                                                upsert_count += 1;
                                            }
                                            Err(err) => {
                                                eprintln!(
                                                    "Failed to upsert station '{}' ({}): {}",
                                                    station.name,
                                                    station.stationuuid,
                                                    err
                                                );
                                            }
                                        }
                                    }

                                    println!("Upserted {} stations into mm_radio.", upsert_count);
                                }
                            }
                            Err(err) => {
                                retries += 1;
                                eprintln!(
                                    "RadioBrowser API call failed (attempt {retries}/{MAX_RETRIES}): {err}"
                                );
                                if retries < MAX_RETRIES {
                                    tokio::time::sleep(Duration::from_secs(5)).await;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        retries += 1;
                        eprintln!(
                            "Failed to initialize RadioBrowser API (attempt {retries}/{MAX_RETRIES}): {err}"
                        );
                        if retries < MAX_RETRIES {
                            tokio::time::sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }

            if !api_call_success {
                eprintln!(
                    "RadioBrowser API calls failed after {MAX_RETRIES} attempts, skipping this message"
                );
            }
        }

        // Always acknowledge the message
        if let Some(tag) = delivery_tag
            && let Err(err) =
                mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(&rabbit_channel, tag).await
            {
                eprintln!("Failed to acknowledge message: {err}");
            }
    }

    Ok(())
}
