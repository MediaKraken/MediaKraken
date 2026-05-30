use std::error::Error;
use std::net::UdpSocket;

/*
firewalld can't be running! Or allow multicast in firewalld

firewall-cmd --permanent --direct --add-rule ipv4 filter INPUT 0 -m pkttype --pkt-type multicast -j ACCEPT

firewall-cmd --permanent --direct --add-rule ipv6 filter INPUT 0 -m pkttype --pkt-type multicast -j ACCEPT

firewall-cmd --reload
 */

pub async fn mk_lib_network_find_mediakraken_server() -> Result<String, Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:9999")
        .map_err(|e| format!("failed to bind udp socket: {e}"))?;
    let buf = [1u8; 15000];
    let count = 1473;
    socket.send_to(&buf[0..count], "234.2.2.2:8888")
        .map_err(|e| format!("failed to send discovery packet: {e}"))?;

    let mut buf = [0u8; 64];
    match socket.recv_from(&mut buf) {
        Ok((len, _remote_addr)) => {
            let data = &buf[..len];
            let response = String::from_utf8_lossy(data);
            Ok(response.to_string())
        }
        Err(err) => {
            eprintln!("client: had a problem: {}", err);
            Ok("Invalid".to_string())
        }
    }
}
