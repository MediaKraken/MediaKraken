// https://redocly.github.io/redoc/?url=https://dl.dropbox.com/s/bnkx6ov4g5z4bxe/GCapi_generated3.yaml?dl=1&nocors#tag/API-Entry
use mk_lib_network;

pub async fn mk_hardware_global_cache_api(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api?expand=all", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

pub async fn mk_hardware_global_cache_api_host(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host?expand=all", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

pub async fn mk_hardware_global_cache_api_host_id(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/id?expand=all", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

pub async fn mk_hardware_global_cache_api_host_id_unit(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/id/unit?expand=all", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

pub async fn mk_hardware_global_cache_api_leds(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/LEDs?expand=all", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

pub async fn mk_hardware_global_cache_api_leds_detail(
    device_ip: String,
    led_number: u8
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/LEDs/{}?expand=all", device_ip, led_number).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

// TODO POST set http://192.168.1.70/api/host/LEDs/{ledNum}

pub async fn mk_hardware_global_cache_api_host_config(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/config?expand=all", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

// TODO POST set http://192.168.1.70/api/host/config

pub async fn mk_hardware_global_cache_api_host_config_network(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/config/network?expand=all", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

// TODO POST set http://192.168.1.70/api/host/config/network

pub async fn mk_hardware_global_cache_api_host_storage(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/storage", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

// TODO POST http://192.168.1.70/api/host/storage/format

// TODO POST http://192.168.1.70/api/host/storage/validate/{filePath}

pub async fn mk_hardware_global_cache_api_host_storage_files(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/storage/files", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

pub async fn mk_hardware_global_cache_api_host_storage_file_retrieve(
    device_ip: String,
    file_path: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/storage/files/{}", device_ip, file_path).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

// TODO Post http://192.168.1.70/api/host/storage/files/{filePath}

// TODO DELETE http://192.168.1.70/api/host/storage/files/{filePath}

pub async fn mk_hardware_global_cache_api_host_modules(
    device_ip: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/modules?expand=all", device_ip).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

pub async fn mk_hardware_global_cache_api_host_module_detail(
    device_ip: String,
    module_number: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(
        format!("http://{}/api/host/modules/{}", device_ip, module_number).to_string(),
    )
    .await
    .unwrap();
    Ok(json_data)
}

// TODO put http://192.168.1.70/api/host/modules/{modNum}

// TODO at serial section
