// https://docs.rs/ipnet/latest/ipnet/#

//use ipnet::Ipv4Subnets;
use mk_lib_file::mk_lib_file;
use quickxml_to_serde::{xml_string_to_json, Config};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process::{Command, Stdio};

// nmap -sU -sS -p U:137,T:139 --script smb-enum-shares 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null
// By default, the script uses guest permissions to list only publicly available shares
// - private shares will be left out as they are not accessible with guest permissions.

// nmap -sS -sV -p 111,2049 --script nfs-showmount 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null
// nmap -p 445 --script smb-enum-shares 192.168.1.122 -Pn -n -oX scan.xml

#[derive(Debug)]
pub struct NMAPShareList {
    pub mm_share_type: String,
    pub mm_share_ip: std::net::IpAddr,
    pub mm_share_path: serde_json::Value,
    pub mm_share_comment: serde_json::Value,
}

pub async fn mk_network_share_scan_port_rustscan(
    subnet_prefix: String,
) -> Result<Vec<NMAPShareList>, Box<dyn std::error::Error>> {
    // rustscan -n -a 192.168.1.0/24 -p 445,2049 -g
    let output = std::process::Command::new("rustscan")
        .arg("-n")
        .arg("-a")
        .arg(format!("{}.0/24", subnet_prefix))
        .arg("-p")
        .arg("445,2049")
        .arg("-g")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut vec_share = Vec::new();
    let mut ip_addr = String::new();
    for line in stdout.split("\n") {
        let text_line = &line.trim().to_string();
        ip_addr = text_line.split(" ").next().unwrap().to_string();
        if text_line.contains("445") == true {
            // smb share
            vec_share.extend(
                mk_network_share_smb_detail(ip_addr.parse().unwrap())
                    .await
                    .unwrap(),
            );
        }
        if text_line.contains("2049") == true {
            // nfs share
            vec_share.extend(
                mk_network_share_nfs_detail(ip_addr.parse().unwrap())
                    .await
                    .unwrap(),
            );
        }
    }
    Ok(vec_share)
}

pub async fn mk_network_share_scan_port(
    subnet_prefix: String,
) -> Result<Vec<NMAPShareList>, Box<dyn std::error::Error>> {
    // find all open smb, nfs ports, -n to not do dns lookup
    // nmap -p 445,2049 -n --open 192.168.1.*
    std::process::Command::new("nmap")
        .arg("-p")
        .arg("445,2049")
        .arg("-n")
        .arg(format!("{}.*", subnet_prefix))
        .arg("-oN")
        .arg("port.txt")
        .output()
        .unwrap();
    let mut vec_share = Vec::new();
    let file = File::open(&"port.txt").unwrap();
    let reader = BufReader::new(file);
    let mut ip_addr = String::new();
    for line in reader.lines() {
        let text_line = &line.unwrap().trim().to_string();
        if text_line.starts_with("Nmap scan report for") == true {
            ip_addr = text_line.split(" ").last().unwrap().to_string();
        } else if text_line.starts_with("445/tcp") == true {
            // smb share
            vec_share.extend(
                mk_network_share_smb_detail(ip_addr.parse().unwrap())
                    .await
                    .unwrap(),
            );
        } else if text_line.starts_with("2049/tcp") == true {
            // nfs share
            vec_share.extend(
                mk_network_share_nfs_detail(ip_addr.parse().unwrap())
                    .await
                    .unwrap(),
            );
        }
    }
    Ok(vec_share)
}

pub async fn mk_network_share_smb_detail(
    ip_addr: std::net::IpAddr,
) -> Result<Vec<NMAPShareList>, Box<dyn std::error::Error>> {
    let conf = Config::new_with_defaults();
    let mut vec_share = Vec::new();
    std::process::Command::new("nmap")
        .arg("-sU")
        .arg("-sS")
        .arg("-p")
        .arg("U:137,T:139")
        .arg("--script")
        .arg("smb-enum-shares")
        .arg(format!("{:?}", ip_addr))
        .arg("-oX")
        .arg("scan.xml")
        .output()
        .unwrap();
    let file_data = mk_lib_file::mk_read_file_data("scan.xml").await.unwrap();
    if !file_data.contains("(0 hosts up)") && file_data.contains("table key=") {
        let nmap_json = xml_string_to_json(file_data.to_string(), &conf).unwrap();
        for val in nmap_json["nmaprun"]["host"]["hostscript"]["script"]
            .as_object()
            .unwrap()
        {
            let (key, v) = val;
            if key == "table" {
                for share_ndx in 0..v.as_array().unwrap().len() {
                    if v[share_ndx]["@key"].to_string().contains("$") {
                    } else {
                        // println!("path: {}", v[share_ndx]["@key"]);
                        // println!("elem: {}", v[share_ndx]["elem"]);
                        // println!("elem: {}", v[share_ndx]["elem"][1]["#text"]);
                        let share_data = NMAPShareList {
                            mm_share_type: "smb".to_string(),
                            mm_share_ip: format!("{:?}", ip_addr).parse().unwrap(),
                            mm_share_path: v[share_ndx]["@key"].clone(),
                            mm_share_comment: v[share_ndx]["elem"][1]["#text"].clone(),
                        };
                        vec_share.push(share_data);
                    }
                }
            }
        }
    }
    Ok(vec_share)
}

pub async fn mk_network_share_nfs_detail(
    ip_addr: std::net::IpAddr,
) -> Result<Vec<NMAPShareList>, Box<dyn std::error::Error>> {
    let conf = Config::new_with_defaults();
    let mut vec_share = Vec::new();
    std::process::Command::new("nmap")
        .arg("-sS")
        .arg("-sV")
        .arg("-p")
        .arg("111,2049")
        .arg("--script")
        .arg("nfs-showmount")
        .arg(format!("{:?}", ip_addr))
        .arg("-oX")
        .arg("scan.xml")
        .output()
        .unwrap();
    let file_data = mk_lib_file::mk_read_file_data("scan.xml").await.unwrap();
    if !file_data.contains("(0 hosts up)") && file_data.contains("table key=") {
        let nmap_json = xml_string_to_json(file_data.to_string(), &conf).unwrap();
        for val in nmap_json["nmaprun"]["host"].as_object().unwrap() {
            let (key, v) = val;
            if key == "table" {
                for share_ndx in 0..v.as_array().unwrap().len() {
                    // println!("num: {}", v.as_array().unwrap().len());
                    // println!("path: {}", v[share_ndx]["@key"]);
                    // println!("elem: {}", v[share_ndx]["elem"]);
                    // println!("elem: {}", v[share_ndx]["elem"][1]["#text"]);
                    let share_data = NMAPShareList {
                        mm_share_type: "nfs".to_string(),
                        mm_share_ip: format!("{:?}", ip_addr).parse().unwrap(),
                        mm_share_path: v[share_ndx]["@key"].clone(),
                        mm_share_comment: v[share_ndx]["elem"][1]["#text"].clone(),
                    };
                    vec_share.push(share_data);
                }
            }
        }
    }
    Ok(vec_share)
}

// use scan port above as it'll be way faster!
// pub async fn mk_network_share_scan(
//     subnet_prefix: String,
// ) -> Result<Vec<NMAPShareList>, Box<dyn std::error::Error>> {
//     let subnets = Ipv4Subnets::new(
//         format!("{}.1", subnet_prefix).parse().unwrap(),
//         format!("{}.255", subnet_prefix).parse().unwrap(),
//         32,
//     );
//     println!("Subnets: {:?}", subnets);
//     let mut vec_share = Vec::new();
//     for ip_address in subnets.enumerate() {
//         println!("IP Addr: {:?}", ip_address.1);
//         // scan for smb
//         vec_share.extend(mk_network_share_smb_detail(ip_address.1).await.unwrap());
//         // scan for nfs
//         vec_share.extend(mk_network_share_nfs_detail(ip_address.1).await.unwrap());
//     }
//     //println!("Vec: {:?}", vec_share);
//     Ok(vec_share)
// }
