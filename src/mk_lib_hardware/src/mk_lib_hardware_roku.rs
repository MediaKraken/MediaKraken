// https://github.com/RoseSecurity/Abusing-Roku-APIs
// http://sdkdocs.roku.com/display/sdkdoc/External+Control+Guide

use crate::mk_lib_hardware_ssdp;
use mk_lib_network::mk_lib_network;
use serde_json::json;
use url::Url;

pub async fn mk_lib_hardware_roku_discover() -> Result<Vec<Url>, Box<dyn std::error::Error>> {
    let responses = mk_lib_hardware_ssdp::mk_lib_hardware_ssdp_search("roku:ecp").await?;
    let urls = responses
        .iter()
        .filter_map(|resp| Url::parse(resp.location()).ok())
        .collect();
    Ok(urls)
}

pub async fn mk_lib_hardware_roku_command(
    roku_addr: String,
    roku_port: i8,
    roku_command: String,
    roku_command_seconds: i8,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let roku_url = format!("http://{}:{}/", roku_addr, roku_port);
    let _request_url: String = String::new();
    let mut request_json: serde_json::Value = json!({});
    if roku_command_seconds > 0 {
        /*
                        urllib.request.urlopen(
                    roku_addr + ':' + roku_port + '/keydown/' + roku_command)
                time.sleep(roku_command_seconds)
                response = urllib.request.urlopen(
                    roku_addr + ':' + roku_port + '/keyup/' + roku_command)
        */
    } else {
        request_json = mk_lib_network::mk_data_from_url_to_json(format!(
            "{}keypress/{}",
            roku_url, roku_command
        ))
        .await?;
    }
    Ok(request_json)
}

pub async fn mk_lib_hardware_roku_app_list(
    roku_addr: String,
    roku_port: i8,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let request_json: serde_json::Value =
        mk_lib_network::mk_data_from_url_to_json(format!("{}:{}/query/apps", roku_addr, roku_port))
            .await?;
    Ok(request_json)
}

pub async fn mk_lib_hardware_roku_app_launch(
    roku_addr: String,
    roku_port: i16,
    roku_app_id: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let request_json: serde_json::Value = mk_lib_network::mk_data_from_url_to_json(format!(
        "{}:{}/launch/{}",
        roku_addr, roku_port, roku_app_id
    ))
    .await?;
    Ok(request_json)
}

pub async fn mk_lib_hardware_roku_icon_save(
    roku_addr: String,
    roku_port: i16,
    roku_app_id: String,
    file_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    mk_lib_network::mk_download_file_from_url(
        format!("{}:{}/query/icon/{}", roku_addr, roku_port, roku_app_id),
        &file_path,
    )
    .await?;
    Ok(())
}

pub async fn mk_lib_hardware_roku_touch_sreen(
    roku_addr: String,
    roku_port: i16,
    x_pos: u16,
    y_pos: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    mk_lib_network::mk_data_from_url(format!(
        "{}:{}/input?touch.0.x={}.0&touch.0.y={}.0&touch.0.op=down",
        roku_addr, roku_port, x_pos, y_pos
    ))
    .await?;
    Ok(())
}
