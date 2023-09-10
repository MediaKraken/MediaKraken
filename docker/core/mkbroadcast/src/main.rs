use pnet::datalink::{self};
use std::io;
use std::net::IpAddr;
use std::str;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut mediakraken_ip: String = "127.0.0.1".to_string();
    // loop through interfaces
    for iface in datalink::interfaces() {
        // Debian 10/11, CentOS 6/7, CentOS 8
        if iface.name == "ens18" || iface.name == "eth0" || iface.name == "ens192" {
            for source_ip in iface.ips.iter() {
                if source_ip.is_ipv4() {
                    // #[cfg(debug_assertions)]
                    // {
                    //     mk_lib_logging::mk_logging_post_elk(
                    //         std::module_path!(),
                    //         json!({ "source_ip": source_ip }),
                    //     )
                    //     .await
                    //     .unwrap();
                    // }
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
                    // #[cfg(debug_assertions)]
                    // {
                    //     mk_lib_logging::mk_logging_post_elk(
                    //         std::module_path!(),
                    //         json!({ "mediakraken_ip": mediakraken_ip }),
                    //     )
                    //     .await
                    //     .unwrap();
                    // }
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

    // Begin the broadcast receive loop
    let sock = UdpSocket::bind("0.0.0.0:9101").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        if len == 25 {
            let net_string = match str::from_utf8(&buf[..25]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            if net_string == "who is MediaKrakenServer?" {
                // #[cfg(debug_assertions)]
                // {
                //     mk_lib_logging::mk_logging_post_elk(
                //         std::module_path!(),
                //         json!({"bytes received": len, "addr": addr, "net_string": net_string}),
                //     )
                //     .await
                //     .unwrap();
                //     mk_lib_logging::mk_logging_post_elk(
                //         std::module_path!(),
                //         json!({ "host_port": host_port }),
                //     )
                //     .await
                //     .unwrap();
                // }
                let mk_address = format!("{}:{}", mediakraken_ip, host_port);
                // #[cfg(debug_assertions)]
                // {
                //     mk_lib_logging::mk_logging_post_elk(
                //         std::module_path!(),
                //         json!({ "mk_address": mk_address }),
                //     )
                //     .await
                //     .unwrap();
                // }
                let _len = sock.send_to(&mk_address.into_bytes(), addr).await?;
                // #[cfg(debug_assertions)]
                // {
                //     mk_lib_logging::mk_logging_post_elk(
                //         std::module_path!(),
                //         json!({ "bytes sent": len }),
                //     )
                //     .await
                //     .unwrap();
                // }
            }
        }
    }
}
