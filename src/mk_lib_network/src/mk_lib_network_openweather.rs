#![cfg_attr(debug_assertions, allow(dead_code))]

// https://openweathermap.org/api
// https://crates.io/crates/openweathermap

use openweathermap::weather;
use serde_json::json;
use stdext::function_name;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn network_openweather_current(city: String, country: String, api_key: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    match &weather(format!("{},{}", city, country), "metric", "en", api_key) {
        Ok(current) => println!(
            "Today's weather in {} is {}",
            current.name.as_str(),
            current.weather[0].main.as_str()
        ),
        Err(e) => eprintln!("Could not fetch weather because: {}", e),
    }
}
