#![cfg_attr(debug_assertions, allow(dead_code))]
// https://docs.rs/ipnet/latest/ipnet/#
// ipnet=2.5.1

use ipnet::Ipv4Subnets;
use serde::{Deserialize, Serialize};
use serde_json::json;
use stdext::function_name;

// nmap -sU -sS -p U:137,T:139 --script smb-enum-shares 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null
// nmap -sS -sV -p 111,2049 --script nfs-showmount 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null

#[path = "mk_lib_file.rs"]
mod mk_lib_file;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

#[derive(Debug, Deserialize, Serialize)]
pub struct NMAPShareList {
    pub mm_share_type: String,
    pub mm_share_xml: String,
}

pub async fn mk_network_share_scan(
    subnet_prefix: String,
) -> Result<Vec<NMAPShareList>, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let subnets = Ipv4Subnets::new(
        format!("{}.1", subnet_prefix).parse().unwrap(),
        format!("{}.255", subnet_prefix).parse().unwrap(),
        24,
    );
    let mut vec_share = Vec::new();
    for ip_address in subnets.enumerate() {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({ "ip_address": format!("{:?}", ip_address) }),
            )
            .await
            .unwrap();
        }
        // scan for smb
        std::process::Command::new("nmap")
            .arg("-sU")
            .arg("-sS")
            .arg("-p")
            .arg("U:137,T:139")
            .arg("--script")
            .arg("smb-enum-shares")
            .arg(format!("{:?}", ip_address))
            .arg("-oX")
            .arg("scan.xml")
            .arg("1>/dev/null")
            .arg("2>/dev/null")
            .spawn()
            .unwrap();
        let share_data = NMAPShareList {
            mm_share_type: "smb".to_string(),
            mm_share_xml: mk_lib_file::mk_read_file_data("scan.xml").unwrap(),
        };
        vec_share.push(share_data);
        // scan for nfs
        std::process::Command::new("nmap")
            .arg("-sS")
            .arg("-sV")
            .arg("-p")
            .arg("111,2049")
            .arg("--script")
            .arg("nfs-showmount")
            .arg(format!("{:?}", ip_address))
            .arg("-oX")
            .arg("scan.xml")
            .arg("1>/dev/null")
            .arg("2>/dev/null")
            .spawn()
            .unwrap();
        let share_data = NMAPShareList {
            mm_share_type: "nfs".to_string(),
            mm_share_xml: mk_lib_file::mk_read_file_data("scan.xml").unwrap(),
        };
        vec_share.push(share_data);
    }
    Ok(vec_share)
}
