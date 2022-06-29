use std::net::UdpSocket;
use std::thread;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:9999").unwrap();
    let buf = [1u8; 15000];
    let mut count = 1473;
    socket.send_to(&buf[0..count], "234,8,8,8:8888").unwrap();
    thread::sleep_ms(1000);
}
