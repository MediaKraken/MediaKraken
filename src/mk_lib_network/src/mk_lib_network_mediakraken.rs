#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::net::UdpSocket;

/*
firewalld can't be running! Or allow multicast in firewalld

firewall-cmd --permanent --direct --add-rule ipv4 filter INPUT 0 -m pkttype --pkt-type multicast -j ACCEPT

firewall-cmd --permanent --direct --add-rule ipv6 filter INPUT 0 -m pkttype --pkt-type multicast -j ACCEPT

firewall-cmd --reload
 */

pub async fn mk_lib_network_find_mediakraken_server() -> Result<String, std::error::Error> {
    let socket = UdpSocket::bind("0.0.0.0:9999").unwrap();
    let buf = [1u8; 15000];
    let mut count = 1473;
    socket.send_to(&buf[0..count], "234.2.2.2:8888").unwrap();
    println!("before recv");

    let mut buf = [0u8; 64];
    match socket.recv_from(&mut buf) {
        Ok((len, remote_addr)) => {
            let data = &buf[..len];
            let response = String::from_utf8_lossy(data);
            println!("{} - client: got data: {}", remote_addr, response);
            return response;
        }
        Err(err) => {
            println!("client: had a problem: {}", err);
            return "Invalid";
        }
    }
}
