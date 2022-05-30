#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://openweathermap.org/api
// https://crates.io/crates/openweathermap

use openweathermap::weather;

pub async fn network_openweather_current() {
    match &weather("Berlin,DE", "metric", "en", "<APIKEY>") {
        Ok(current) => println!(
            "Today's weather in {} is {}",
            current.name.as_str(),
            current.weather[0].main.as_str()
        ),
        Err(e) => println!("Could not fetch weather because: {}", e),
    }
}
