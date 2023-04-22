#![cfg_attr(debug_assertions, allow(dead_code))]

use pnet::datalink;
use serde_json::json;
use shiplift::Docker;
use socket2::{Domain, Protocol, Socket, Type};
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Duration;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

fn new_socket(addr: &SocketAddr) -> io::Result<Socket> {
    let domain = if addr.is_ipv4() {
        Domain::IPV4
    } else {
        Domain::IPV6
    };
    let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
    // we're going to use read timeouts so that we don't hang waiting for packets
    socket.set_read_timeout(Some(Duration::from_millis(100)))?;
    Ok(socket)
}

#[cfg(windows)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    let addr = match *addr {
        SocketAddr::V4(addr) => SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), addr.port()),
        SocketAddr::V6(addr) => {
            SocketAddr::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(), addr.port())
        }
    };
    socket.bind(&socket2::SockAddr::from(addr))
}

#[cfg(unix)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    socket.bind(&socket2::SockAddr::from(*addr))
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        // start logging
        mk_lib_logging::mk_logging_post_elk("info", json!({"START": "START"})).await.unwrap();
    }

    let mut mediakraken_ip: String = "127.0.0.1".to_string();
    // loop through interfaces
    for iface in datalink::interfaces() {
        // Debian 10/11, CentOS 6/7, CentOS 8
        if iface.name == "ens18" || iface.name == "eth0" || iface.name == "ens192" {
            for source_ip in iface.ips.iter() {
                if source_ip.is_ipv4() {
                    #[cfg(debug_assertions)]
                    {
                        mk_lib_logging::mk_logging_post_elk(
                            "info",
                            json!({ "source_ip": source_ip }),
                        )
                        .await
                        .unwrap();
                    }
                    let source_ip = iface
                        .ips
                        .iter()
                        .find(|ip| ip.is_ipv4())
                        .map(|ip| match ip.ip() {
                            IpAddr::V4(ip) => ip,
                            _ => unreachable!(),
                        })
                        .unwrap();
                    mediakraken_ip = source_ip.to_string();
                    #[cfg(debug_assertions)]
                    {
                        mk_lib_logging::mk_logging_post_elk(
                            "info",
                            json!({ "mediakraken_ip": mediakraken_ip }),
                        )
                        .await
                        .unwrap();
                    }
                    break;
                }
            }
        }
    }
    let mut host_port: u64 = 8903;
    // Grab public port that the web app is running on
    let docker = Docker::new();
    let result = docker.containers().list(&Default::default()).await;
    match result {
        Ok(images) => {
            for i in images {
                if i.names[0] == "/mkstack_webapp" {
                    host_port = i.ports[0].private_port;
                    break;
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    let response = format!("{}:{}", mediakraken_ip, host_port);
    let socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
    let mut buf = [0u8; 65535];
    let multi_addr = Ipv4Addr::new(234, 2, 2, 2);
    let inter = Ipv4Addr::new(0, 0, 0, 0);
    socket.join_multicast_v4(&multi_addr, &inter);
    loop {
        let (amt, remote_addr) = socket.recv_from(&mut buf).unwrap();
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                "info",
                json!({"amt": amt, "remote_addr": remote_addr}),
            )
            .await
            .unwrap();
        }
        // create a socket to send the response
        let responder =
            UdpSocket::from(new_socket(&remote_addr).expect("failed to create responder"));

        // we send the response that was set at the method beginning
        responder
            .send_to(response.as_bytes(), &remote_addr)
            .expect("failed to respond");
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                "info",
                json!({"response": response, "remote_addr": remote_addr}),
            )
            .await
            .unwrap();
        }
    }
}
