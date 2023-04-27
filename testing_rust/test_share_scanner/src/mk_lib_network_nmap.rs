#![cfg_attr(debug_assertions, allow(dead_code))]
// https://docs.rs/ipnet/latest/ipnet/#
// ipnet=2.5.1

use ipnet::Ipv4Subnets;
use serde::{Deserialize, Serialize};

// nmap -sU -sS -p U:137,T:139 --script smb-enum-shares 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null
// nmap -sS -sV -p 111,2049 --script nfs-showmount 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null

use crate::mk_lib_file;

#[derive(Debug, Deserialize, Serialize)]
pub struct NMAPShareList {
    pub mm_share_type: String,
    pub mm_share_xml: String,
}

pub async fn mk_network_share_scan(
    subnet_prefix: String,
) -> Result<Vec<NMAPShareList>, Box<dyn std::error::Error>> {
    let subnets = Ipv4Subnets::new(
        format!("{}.1", subnet_prefix).parse().unwrap(),
        format!("{}.255", subnet_prefix).parse().unwrap(),
        32,
    );
    println!("Subnets: {:?}", subnets);
    let mut vec_share = Vec::new();
    for ip_address in subnets.enumerate() {
        println!("IP Addr: {:?}", ip_address.1);
        // scan for smb
        std::process::Command::new("nmap")
            .arg("-sU")
            .arg("-sS")
            .arg("-p")
            .arg("U:137,T:139")
            .arg("--script")
            .arg("smb-enum-shares")
            .arg(format!("{:?}", ip_address.1))
            .arg("-oX")
            .arg("scan.xml")
            .output()
            .unwrap();
        let share_data = NMAPShareList {
            mm_share_type: "smb".to_string(),
            mm_share_xml: mk_lib_file::mk_read_file_data("scan.xml").await.unwrap(),
        };
        if !share_data.mm_share_xml.contains("(0 hosts up)") {
            vec_share.push(share_data);
        }
        // scan for nfs
        std::process::Command::new("nmap")
            .arg("-sS")
            .arg("-sV")
            .arg("-p")
            .arg("111,2049")
            .arg("--script")
            .arg("nfs-showmount")
            .arg(format!("{:?}", ip_address.1))
            .arg("-oX")
            .arg("scan.xml")
            .output()
            .unwrap();
        let share_data = NMAPShareList {
            mm_share_type: "nfs".to_string(),
            mm_share_xml: mk_lib_file::mk_read_file_data("scan.xml").await.unwrap(),
        };
        if !share_data.mm_share_xml.contains("(0 hosts up)") {
            vec_share.push(share_data);
        }
    }
    Ok(vec_share)
}
