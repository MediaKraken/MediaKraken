use crate::mk_lib_hardware_alexa;
use crate::mk_lib_hardware_ampro;
use crate::mk_lib_hardware_appletv;
use crate::mk_lib_hardware_chromecast;
use crate::mk_lib_hardware_crestron;
use crate::mk_lib_hardware_firetv;
use crate::mk_lib_hardware_hdhomerun;
use crate::mk_lib_hardware_lg;
use crate::mk_lib_hardware_marantz;
use crate::mk_lib_hardware_onkyo;
use crate::mk_lib_hardware_phue;
use crate::mk_lib_hardware_pioneer;
use crate::mk_lib_hardware_roku;
use crate::mk_lib_hardware_samsung;
use crate::mk_lib_hardware_tivo;
use crate::mk_lib_hardware_yamaha;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceJson {
    pub id: u32,
    pub name: String,
    pub value: f64,
}
//let mut device_json_array: Vec<DeviceJson> = Vec::new();

pub async fn mk_hardware_main_command(machine_brand: String, machine_type: String, 
    machine_model: String, command_type: String, command_value: String
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // TODO pull from the web api?
    // TODO loop through the json files and find brand, type, model
    // TODO then find the command and execute it via connection type
    // match machine_brand.as_str() {
    //     "alexa" => {
    //         //return mk_lib_hardware_alexa::mk_hardware_chromecast_discover().await;
    //     }
    //     "ampro" => {
    //         //return mk_lib_hardware_ampro::mk_hardware_chromecast_discover().await;
    //     }
    //     "appletv" => {
    //         //return mk_lib_hardware_appletv::mk_hardware_chromecast_discover().await;
    //     }
    //     "arduino" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "chromecast" => {
    //         return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "crestron" => {
    //         //return mk_lib_hardware_creston::mk_hardware_chromecast_discover().await;
    //     }
    //     "firetv" => {
    //         //return mk_lib_hardware_firetv::mk_hardware_chromecast_discover().await;
    //     }
    //     "hdhomerun" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "lenbrook" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "lg" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "marantz" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "onkyo" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "phue" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "pioneer" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "roku" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "samsung" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "tivo" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     "yamaha" => {
    //         //return mk_lib_hardware_chromecast::mk_hardware_chromecast_discover().await;
    //     }
    //     _ => {
    //         return Err(Box::new(std::io::Error::new(
    //             std::io::ErrorKind::Other,
    //             "Unsupported machine type",
    //         )));
    //     }
    // }
    Ok(json!(null))
}