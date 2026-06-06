use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceJson {
    pub id: u32,
    pub name: String,
    pub value: f64,
}

#[cfg(test)]
mod tests {
    use super::DeviceJson;
    use serde_json;

    #[test]
    fn device_json_serialize_roundtrip() {
        let original = DeviceJson {
            id: 42,
            name: "Test Device".to_string(),
            value: 3.14,
        };
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: DeviceJson = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.value, deserialized.value);
    }

    #[test]
    fn device_json_deserialize_from_json() {
        let json_str = r#"{"id":1,"name":"Receiver","value":75.5}"#;
        let device: DeviceJson = serde_json::from_str(json_str).unwrap();
        assert_eq!(device.id, 1);
        assert_eq!(device.name, "Receiver");
        assert_eq!(device.value, 75.5);
    }

    #[test]
    fn device_json_serialize_to_json() {
        let device = DeviceJson {
            id: 2,
            name: "Amplifier".to_string(),
            value: 0.0,
        };
        let json_value = serde_json::to_value(&device).unwrap();
        assert_eq!(json_value["id"], 2);
        assert_eq!(json_value["name"], "Amplifier");
        assert_eq!(json_value["value"], 0.0);
    }
}

pub async fn mk_hardware_main_command(
    _machine_brand: String,
    _machine_type: String,
    _machine_model: String,
    _command_type: String,
    _command_value: String,
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
