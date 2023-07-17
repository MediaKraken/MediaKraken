// https://github.com/torbencarstens/sonos_discovery
// https://github.com/jakobhellermann/sonor

use sonos_discovery::Discover;
use std::net::IpAddr;

pub async fn mk_lib_hardware_sonos_discover() {
    let discovery: Discover = Discover::new().unwrap();
    // fn start(self, timeout: Option<u32>, device_count: Option<usize>)
    // timeout default: 5 | device_count: u32::MAX
    // Checks that {discovered_devices} < {device_count} && {elapsed_time} < {timeout}
    // Waits until 3 devices are found, or 5seconds have elapsed
    let sonos_ips: Vec<IpAddr> = discovery.start(None, Some(3)).unwrap();
    for sonos_ip in sonos_ips {
        println!("{}", sonos_ip);
    }
}
