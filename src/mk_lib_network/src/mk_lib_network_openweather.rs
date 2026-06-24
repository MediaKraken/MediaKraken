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
    let weather = reading.weather.first()
        .ok_or(ClientError::Other("no weather data available".to_string()))?;
    Ok((reading.main.temp, weather.description.clone()))
}
