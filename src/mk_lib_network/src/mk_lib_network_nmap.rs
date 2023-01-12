#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
// https://docs.rs/ipnet/latest/ipnet/#
// ipnet=2.5.1

// nmap -sU -sS -p U:137,T:139 --script smb-enum-shares 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null
// nmap -sS -sV -p 111,2049 --script nfs-showmount 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn mk_network_share_scan(
    subnet_prefix: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let subnets = Ipv4Subnets::new(
        format!("{}.1", subnet_prefix).parse().unwrap(),
        format!("{}.255", subnet_prefix).parse().unwrap(),
        24,
    );
    for ip_address in subnets.iter() {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({ "ip_address": ip_address }),
            )
            .await.unwrap();
        }
        // scan for smb
        std::process::Command::new("nmap")
            .arg("-sU")
            .arg("-sS")
            .arg("-p")
            .arg("U:137,T:139")
            .arg("--script")
            .arg("smb-enum-shares")
            .arg(ip_address)
            .arg("-oX")
            .arg("scan.xml")
            .arg("1>/dev/null")
            .arg("2>/dev/null")
            .spawn()
            .unwrap();
        // scan for nfs
        std::process::Command::new("nmap")
            .arg("-sS")
            .arg("-sV")
            .arg("-p")
            .arg("111,2049")
            .arg("--script")
            .arg("nfs-showmount")
            .arg(ip_address)
            .arg("-oX")
            .arg("scan.xml")
            .arg("1>/dev/null")
            .arg("2>/dev/null")
            .spawn()
            .unwrap();
    }
    Ok(res)
}
