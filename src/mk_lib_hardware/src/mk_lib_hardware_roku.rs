#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/RoseSecurity/Abusing-Roku-APIs
// http://sdkdocs.roku.com/display/sdkdoc/External+Control+Guide

use stdext::function_name;
use serde_json::json;

use crate::mk_lib_logging;

use crate::mk_lib_network;

pub async fn mk_lib_hardware_roku_discover() {

}

pub async fn mk_lib_hardware_roku_command(roku_addr: String,
                                            roku_port: i8,
                                            roku_command: String,
                                            roku_command_seconds i8) {
                                                #[cfg(debug_assertions)]
                                                {
                                                    mk_lib_logging::mk_logging_post_elk(
                                                        std::module_path!(),
                                                        json!({ "Function": function_name!() }),
                                                    )
                                                    .await
                                                    .unwrap();
                                                }
                                                let roku_url = format!("http://{}:{}/", roku_addr, roku_port);
    let mut request_url: String = String::new();
    let mut request_json: serde_json::json = json!({});
    if roku_command_seconds > 0 {
        /*
                urllib.request.urlopen(
            roku_addr + ':' + roku_port + '/keydown/' + roku_command)
        time.sleep(roku_command_seconds)
        response = urllib.request.urlopen(
            roku_addr + ':' + roku_port + '/keyup/' + roku_command)
 */
    }
    else {
        request_json = mk_lib_network::mk_data_from_url_to_json(
            format!("{}keypress/{}", roku_url, roku_command)).await.unwrap();
    }
    Ok(request_json)
}

pub async fn mk_lib_hardware_roku_app_list(roku_addr: String,
    roku_port: i8) {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({ "Function": function_name!() }),
            )
            .await
            .unwrap();
        }
            let request_json: serde_json::json = mk_lib_network::mk_data_from_url_to_json(
            format!("{}:{}/query/apps", roku_addr, roku_port)).await.unwrap();
    Ok(request_json)
}

pub async fn mk_lib_hardware_roku_app_launch(roku_addr: String,
    roku_port: i8, roku_app_id: String) {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({ "Function": function_name!() }),
            )
            .await
            .unwrap();
        }
            let request_json: serde_json::json = mk_lib_network::mk_data_from_url_to_json(
            format!("{}:{}/launch/{}", roku_addr, roku_port, roku_app_id)).await.unwrap();
    Ok(request_json)
}

/*
def com_roku_network_app_icon(roku_addr, roku_port, roku_app_id):
    """
    Grab app icon
    """
    return urllib.request.urlopen(roku_addr + ':' + roku_port + '/query/icon/' + roku_app_id)


def com_roku_network_touch(roku_addr, roku_port, x_pos, y_pos):
    """
    'Click' screen
    """
    return urllib.request.urlopen(roku_addr + ':' + roku_port + '/input?touch.0.x=' + str(x_pos)
                                  + '.0&touch.0.y=' + str(y_pos) + '.0&touch.0.op=down')
 */