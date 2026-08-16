// https://openweathermap.org/api
// https://openweathermap.org/current#multi for currently supported languages

use openweathermap_client::models::{City, UnitSystem};
use openweathermap_client::{Client, ClientOptions, error::ClientError};

pub async fn network_openweather_current(
    city: &str,
    country: &str,
    api_key: String,
    temp_type: UnitSystem,
) -> Result<(f64, String), ClientError> {
    let options = ClientOptions {
        units: temp_type,
        language: "en".to_string(),
        api_key,
    };
    let client = Client::new(options)?;
    let reading = client.fetch_weather(&City::new(city, country)).await?;
    let weather = match reading.weather.first() {
        Some(w) => w,
        None => return Err(ClientError::InvalidOptionsError(
            openweathermap_client::error::InvalidOptionsError { message: "no weather data available".to_string() },
        )),
    };
    Ok((reading.main.temp, weather.description.clone()))
}
