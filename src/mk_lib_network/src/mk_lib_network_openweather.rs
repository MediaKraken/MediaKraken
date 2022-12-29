#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://openweathermap.org/api
// https://crates.io/crates/openweathermap

use openweathermap::weather;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn network_openweather_current(city: String, country: String, api_key: String) {
    match &weather(format!("{},{}", city, country), "metric", "en", api_key) {
        Ok(current) => println!(
            "Today's weather in {} is {}",
            current.name.as_str(),
            current.weather[0].main.as_str()
        ),
        Err(e) => println!("Could not fetch weather because: {}", e),
    }
}
