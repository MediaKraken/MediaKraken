// https://openweathermap.org/api
// https://openweathermap.org/current#multi for currently supported languages

use mk_lib_logging::mk_lib_logging;
use openweathermap_client::models::{City, UnitSystem};
use openweathermap_client::{error::ClientError, Client, ClientOptions};
use serde_json::json;
use stdext::function_name;

pub async fn network_openweather_current(
    city: &str,
    country: &str,
    api_key: String,
    temp_type: UnitSystem,
) -> Result<(f64, String), ClientError> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let options = ClientOptions {
        units: temp_type,
        language: "en".to_string(),
        api_key: api_key,
    };
    let client = Client::new(options)?;
    let reading = client.fetch_weather(&City::new(city, country)).await?;
    Ok((reading.main.temp, reading.weather[0].description.clone()))
}
