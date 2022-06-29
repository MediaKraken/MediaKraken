#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::net::UdpSocket;
use std::net::Ipv4Addr;

fn main() {
    let mut socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
    let mut buf = [0u8; 65535];
    let multi_addr = Ipv4Addr::new(234, 8, 8, 8);
    let inter = Ipv4Addr::new(0,0,0,0);
    socket.join_multicast_v4(&multi_addr,&inter);
    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from {:?}", amt, src);
    }
}
