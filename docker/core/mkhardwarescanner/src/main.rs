#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde_json::json;
use std::error::Error;
//use onvif::discovery;
use huelib::resource::sensor;
use huelib::{bridge, Bridge};
use tokio::time::{Duration, sleep};
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Error, Record, RecordKind};
use std::{net::IpAddr, time::Duration};

const CHROMECAST_SERVICE_NAME: &'static str = "_googlecast._tcp.local";

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // start logging
    const LOGGING_INDEX_NAME: &str = "mkhardwarescanner";
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"START": "START"}),
                                        LOGGING_INDEX_NAME).await;


// media_devices = []

    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "Before Chromcast"}),
                                        LOGGING_INDEX_NAME).await;

    // chromecast discover
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))?.listen();
    pin_mut!(stream);
    while let Some(Ok(response)) = stream.next().await {
        let addr = response.records()
                           .filter_map(self::to_ip_addr)
                           .next();
        if let Some(addr) = addr {
            println!("found cast device at {}", addr);
        } else {
            println!("cast device does not advertise address");
        }
    }

// for chromecast_ip, model_name, friendly_name \
//         in common_hardware_chromecast.com_hard_chrome_discover():
//     common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
//                                                          message_text={
//                                                              'chromecast out': chromecast_ip})
//     media_devices.append({'IP': chromecast_ip,
//                           'Model': model_name,
//                           'Name': friendly_name})
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "After Chromcast"}),
                                        LOGGING_INDEX_NAME).await;

    // crestron device discover
// # crestron_devices = common_hardware_crestron.com_hardware_crestron_discover()
// # if crestron_devices != None:
// #     for crestron in crestron_devices:
// #         common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text= {'crestron out': crestron})
// #         media_devices.append({'Crestron': crestron})
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "After Crestron"}),
                                        LOGGING_INDEX_NAME).await;

// dlna devices
// // TODO looks like debugging shows up if run from this program
// # for dlna_devices in common_network_dlna.com_net_dlna_discover():
// #     if dlna_devices.find('No compatible devices found.') != -1:
// #         break
// #     media_devices.append({'DLNA': dlna_devices})
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "After DLNA"}),
                                        LOGGING_INDEX_NAME).await;

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
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "After HDHomerun"}),
                                        LOGGING_INDEX_NAME).await;

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
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "After Onvif"}),
                                        LOGGING_INDEX_NAME).await;

    // phillips hue hub discover
    let hub_ip_addresses = bridge::discover_nupnp().unwrap();
    for bridge_ip in hub_ip_addresses {
        println!("{}", bridge_ip);
        // Register a new user.
        let username = bridge::register_user(bridge_ip, "huelib-rs example").unwrap();
        let bridge = Bridge::new(bridge_ip, username);
        let lights = bridge.get_all_lights().unwrap();
        println!("{:?}", lights);
    }
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "After PHue"}),
                                        LOGGING_INDEX_NAME).await;

// roku discover
// for roku in common_hardware_roku_network.com_roku_network_discovery():
//     common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
//                                                          message_text={'roku out': roku})
//     media_devices.append({'Roku': roku})
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "After ROKU"}),
                                        LOGGING_INDEX_NAME).await;

// soco discover
// soco_devices = common_hardware_soco.com_hardware_soco_discover()
// if soco_devices != None:
//     for soco in soco_devices:
//         common_logging_elasticsearch_httpx.com_es_httpx_post(
//             message_type='info',
//             message_text={'soco out': soco})
//         media_devices.append({'Soco': soco})
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "After SOCO"}),
                                        LOGGING_INDEX_NAME).await;

    // sonos discover
    // let mut devices = sonor::discover(std::time::Duration::from_secs(5)).await?;
    // while let Some(device) = devices.try_next().await? {
    //     let name = device.name().await?;
    //     println!("Sonos Discovered {}", name);
    // }
    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"HWScan": "After Sonos"}),
                                        LOGGING_INDEX_NAME).await;

// common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
//                                                      message_text={'devices': media_devices})
//
// common_file.com_file_save_data(file_name='/mediakraken/devices/device_scan.txt',
//                                data_block=media_devices,
//                                as_pickle=True,
//                                with_timestamp=False,
//                                file_ext=None)

    mk_lib_logging::mk_logging_post_elk("info",
                                        json!({"STOP": "STOP"}),
                                        LOGGING_INDEX_NAME).await;
    Ok(())
}