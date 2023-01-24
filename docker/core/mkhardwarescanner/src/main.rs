#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::error::Error;
//use onvif::discovery;
use futures_util::{pin_mut, stream::StreamExt};
use stdext::function_name;
use serde_json::json;
use tokio::time::{sleep, Duration};

#[path = "mk_lib_hardware_chromecast.rs"]
mod mk_lib_hardware_chromecast;

#[path = "mk_lib_hardware_phue.rs"]
mod mk_lib_hardware_phue;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "mk_lib_network_dlna.rs"]
mod mk_lib_network_dlna;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"}))
            .await
            .unwrap();
    }

    // media_devices = []
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({"HWScan": "Before Chromcast"}),
        )
        .await
        .unwrap();
    }

    // chromecast discover
    mk_lib_hardware_chromecast::mk_hardware_chromecast_discover()
        .await
        .unwrap();
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({"HWScan": "After Chromcast"}),
        )
        .await
        .unwrap();
    }

    // crestron device discover
    // # crestron_devices = common_hardware_crestron.com_hardware_crestron_discover()
    // # if crestron_devices != None:
    // #     for crestron in crestron_devices:
    // #         common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text= {'crestron out': crestron})
    // #         media_devices.append({'Crestron': crestron})
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({"HWScan": "After Crestron"}),
        )
        .await
        .unwrap();
    }

    // dlna devices
    mk_lib_network_dlna::mk_lib_network_dlna_discover().await;
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({"HWScan": "After DLNA"}))
            .await
            .unwrap();
    }
    // hdhomerun tuner discovery
    // tuner_api = common_hardware_hdhomerun_py.CommonHardwareHDHomeRunPY()
    // tuner_api.com_hdhomerun_discover()
    // for row_tuner in tuner_api.com_hdhomerun_list():
    //     print(row_tuner, flush=True)
    // # tuner_api = common_hardware_hdhomerun.CommonHardwareHDHomeRun()
    // # tuner_api.com_hdhomerun_discover()
    // # for row_tuner in tuner_api.com_hdhomerun_list():
    // #     common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text= {
    // #         'hdhomerun out': common_string.com_string_ip_int_to_ascii(row_tuner.get_device_ip())})
    // #     media_devices.append({'HDHomeRun': {'Model': row_tuner.get_var(item='/sys/model'),
    // #                                         'HWModel': row_tuner.get_var(item='/sys/hwmodel'),
    // #                                         'Name': row_tuner.get_name(),
    // #                                         'ID': str(hex(row_tuner.get_device_id())),
    // #                                         'IP': common_string.com_string_ip_int_to_ascii(
    // #                                             row_tuner.get_device_ip()),
    // #                                         'Firmware': row_tuner.get_version(),
    // #                                         'Active': True,
    // #                                         'Channels': {}}})
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({"HWScan": "After HDHomerun"}),
        )
        .await
        .unwrap();
    }

    // onvif cameras discover
    //use futures_util::stream::StreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;
    // discovery::discover(std::time::Duration::from_secs(1))
    //     .await
    //     .unwrap()
    //     .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| async move {
    //         println!("Onvif Device found: {:?}", addr);
    //     })
    //     .await;
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({"HWScan": "After Onvif"}))
            .await
            .unwrap();
    }

    // phillips hue hub discover
    mk_lib_hardware_phue::mk_hardware_phue_discover()
        .await
        .unwrap();
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({"HWScan": "After PHue"}))
            .await
            .unwrap();
    }
    // roku discover
    // for roku in common_hardware_roku_network.com_roku_network_discovery():
    //     common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
    //                                                          message_text={'roku out': roku})
    //     media_devices.append({'Roku': roku})
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({"HWScan": "After ROKU"}))
            .await
            .unwrap();
    }
    // soco discover
    // soco_devices = common_hardware_soco.com_hardware_soco_discover()
    // if soco_devices != None:
    //     for soco in soco_devices:
    //         common_logging_elasticsearch_httpx.com_es_httpx_post(
    //             message_type='info',
    //             message_text={'soco out': soco})
    //         media_devices.append({'Soco': soco})
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({"HWScan": "After SOCO"}))
            .await
            .unwrap();
    }
    // sonos discover
    // let mut devices = sonor::discover(std::time::Duration::from_secs(5)).await?;
    // while let Some(device) = devices.try_next().await? {
    //     let name = device.name().await?;
    //     println!("Sonos Discovered {}", name);
    // }
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({"HWScan": "After Sonos"}))
            .await
            .unwrap();
    }

    // common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
    //                                                      message_text={'devices': media_devices})
    //
    // common_file.com_file_save_data(file_name='/mediakraken/devices/device_scan.txt',
    //                                data_block=media_devices,
    //                                as_pickle=True,
    //                                with_timestamp=False,
    //                                file_ext=None)
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk("info", json!({"STOP": "STOP"}))
            .await
            .unwrap();
    }
    Ok(())
}
