use pnet::datalink;
use shiplift::Docker;
use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

const DEFAULT_BIND: &str = "0.0.0.0:8888";
const DEFAULT_MULTICAST: &str = "234.2.2.2";
const DEFAULT_WEBAPP_PORT: u64 = 8903;
const DEFAULT_WEBAPP_NAME: &str = "/mkstack-webapp";
const MAX_DOCKER_RETRIES: u32 = 3;
const DOCKER_RETRY_DELAY: Duration = Duration::from_secs(5);

fn detect_local_ipv4() -> Option<Ipv4Addr> {
    let iface_filter = env::var("MEDIAKRAKEN_IFACE").ok();
    for iface in datalink::interfaces() {
        if iface.is_loopback() || !iface.is_up() {
            continue;
        }
        if let Some(ref name) = iface_filter
            && &iface.name != name
        {
            continue;
        }
        for net in &iface.ips {
            if let IpAddr::V4(ip) = net.ip()
                && !ip.is_loopback()
                && !ip.is_unspecified()
            {
                return Some(ip);
            }
        }
    }
    None
}

async fn lookup_webapp_port_with_retry(docker: &Docker, name: &str) -> Option<u64> {
    let mut retries = 0;

    loop {
        match timeout(
            Duration::from_secs(10),
            docker.containers().list(&Default::default()),
        )
        .await
        {
            Ok(Ok(list)) => {
                return list
                    .into_iter()
                    .find(|c| c.names.iter().any(|n| n == name))
                    .and_then(|c| c.ports.first().map(|p| p.private_port));
            }
            Ok(Err(e)) => {
                retries += 1;
                eprintln!("docker list error (attempt {retries}/{MAX_DOCKER_RETRIES}): {e}");
                if retries >= MAX_DOCKER_RETRIES {
                    return None;
                }
                tokio::time::sleep(DOCKER_RETRY_DELAY).await;
            }
            Err(_) => {
                retries += 1;
                eprintln!("docker list timeout (attempt {retries}/{MAX_DOCKER_RETRIES})");
                if retries >= MAX_DOCKER_RETRIES {
                    return None;
                }
                tokio::time::sleep(DOCKER_RETRY_DELAY).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let bind_addr: SocketAddr = env::var("MEDIAKRAKEN_BIND")
        .unwrap_or_else(|_| DEFAULT_BIND.to_string())
        .parse()
        .map_err(|_| format!("invalid MEDIAKRAKEN_BIND: {}", env::var("MEDIAKRAKEN_BIND").unwrap_or_default()))?;
    let multi_addr: Ipv4Addr = env::var("MEDIAKRAKEN_MULTICAST")
        .unwrap_or_else(|_| DEFAULT_MULTICAST.to_string())
        .parse()
        .map_err(|_| format!("invalid MEDIAKRAKEN_MULTICAST: {}", env::var("MEDIAKRAKEN_MULTICAST").unwrap_or_default()))?;
    let webapp_name =
        env::var("MEDIAKRAKEN_WEBAPP_NAME").unwrap_or_else(|_| DEFAULT_WEBAPP_NAME.to_string());

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
    let mut cached_port = lookup_webapp_port_with_retry(&docker, &webapp_name)
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

        // Attempt to refresh port with retry logic
        if let Some(port) = lookup_webapp_port_with_retry(&docker, &webapp_name).await {
            cached_port = port;
        } else {
            eprintln!("Failed to get webapp port, keeping cached port {cached_port}");
        }

        let response = format!("{}:{}", local_ip, cached_port);
        if let Err(e) = socket.send_to(response.as_bytes(), remote_addr).await {
            eprintln!("send_to {remote_addr} error: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bind_constant() {
        assert_eq!(DEFAULT_BIND, "0.0.0.0:8888");
    }

    #[test]
    fn test_default_multicast_constant() {
        assert_eq!(DEFAULT_MULTICAST, "234.2.2.2");
    }

    #[test]
    fn test_default_webapp_port_constant() {
        assert_eq!(DEFAULT_WEBAPP_PORT, 8903);
    }

    #[test]
    fn test_default_webapp_name_constant() {
        assert_eq!(DEFAULT_WEBAPP_NAME, "/mkstack-webapp");
    }

    #[test]
    fn test_max_docker_retries_constant() {
        assert_eq!(MAX_DOCKER_RETRIES, 3);
    }

    #[test]
    fn test_docker_retry_delay_constant() {
        assert_eq!(DOCKER_RETRY_DELAY, Duration::from_secs(5));
    }

    #[test]
    fn test_detect_local_ipv4_returns_some_or_none() {
        let result = detect_local_ipv4();
        if let Some(ip) = result {
            assert!(!ip.is_loopback());
            assert!(!ip.is_unspecified());
            assert!(ip.is_ipv4());
        }
    }

    #[test]
    fn test_detect_local_ipv4_consistent() {
        let result1 = detect_local_ipv4();
        let result2 = detect_local_ipv4();
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_constants_no_overlap() {
        assert_ne!(DEFAULT_BIND, DEFAULT_MULTICAST);
        assert_ne!(DEFAULT_WEBAPP_NAME, DEFAULT_BIND);
    }
}
