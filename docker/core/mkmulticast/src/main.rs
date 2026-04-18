use pnet::datalink;
use shiplift::Docker;
use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;

const DEFAULT_BIND: &str = "0.0.0.0:8888";
const DEFAULT_MULTICAST: &str = "234.2.2.2";
const DEFAULT_WEBAPP_PORT: u64 = 8903;
const DEFAULT_WEBAPP_NAME: &str = "/mkstack-webapp";

fn detect_local_ipv4() -> Option<Ipv4Addr> {
    let iface_filter = env::var("MEDIAKRAKEN_IFACE").ok();
    for iface in datalink::interfaces() {
        if iface.is_loopback() || !iface.is_up() {
            continue;
        }
        if let Some(ref name) = iface_filter {
            if &iface.name != name {
                continue;
            }
        }
        for net in &iface.ips {
            if let IpAddr::V4(ip) = net.ip() {
                if !ip.is_loopback() && !ip.is_unspecified() {
                    return Some(ip);
                }
            }
        }
    }
    None
}

async fn lookup_webapp_port(docker: &Docker, name: &str) -> Option<u64> {
    match docker.containers().list(&Default::default()).await {
        Ok(list) => list
            .into_iter()
            .find(|c| c.names.iter().any(|n| n == name))
            .and_then(|c| c.ports.first().map(|p| p.private_port)),
        Err(e) => {
            eprintln!("docker list error: {e}");
            None
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let bind_addr: SocketAddr = env::var("MEDIAKRAKEN_BIND")
        .unwrap_or_else(|_| DEFAULT_BIND.to_string())
        .parse()
        .expect("invalid MEDIAKRAKEN_BIND");
    let multi_addr: Ipv4Addr = env::var("MEDIAKRAKEN_MULTICAST")
        .unwrap_or_else(|_| DEFAULT_MULTICAST.to_string())
        .parse()
        .expect("invalid MEDIAKRAKEN_MULTICAST");
    let webapp_name = env::var("MEDIAKRAKEN_WEBAPP_NAME")
        .unwrap_or_else(|_| DEFAULT_WEBAPP_NAME.to_string());

    let local_ip = env::var("MEDIAKRAKEN_IP")
        .ok()
        .and_then(|s| s.parse::<Ipv4Addr>().ok())
        .or_else(detect_local_ipv4)
        .unwrap_or(Ipv4Addr::LOCALHOST);

    let socket = UdpSocket::bind(bind_addr).await?;
    if let Err(e) = socket.join_multicast_v4(multi_addr, Ipv4Addr::UNSPECIFIED) {
        eprintln!("failed to join multicast group {multi_addr}: {e}");
        return Err(e);
    }

    let docker = Docker::new();
    let mut cached_port = lookup_webapp_port(&docker, &webapp_name)
        .await
        .unwrap_or(DEFAULT_WEBAPP_PORT);

    println!(
        "mkmulticast listening on {bind_addr}, group {multi_addr}, responding {local_ip}:{cached_port}"
    );

    let mut buf = [0u8; 65535];
    loop {
        let (_amt, remote_addr) = match socket.recv_from(&mut buf).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("recv_from error: {e}");
                continue;
            }
        };

        if let Some(port) = lookup_webapp_port(&docker, &webapp_name).await {
            cached_port = port;
        }

        let response = format!("{}:{}", local_ip, cached_port);
        if let Err(e) = socket.send_to(response.as_bytes(), remote_addr).await {
            eprintln!("send_to {remote_addr} error: {e}");
        }
    }
}
