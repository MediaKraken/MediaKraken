// https://docs.rs/ipnet/latest/ipnet/#

use ipnet::Ipv4Subnets;
use mk_lib_file::mk_lib_file;
use mk_lib_logging::mk_lib_logging;
use quickxml_to_serde::{xml_string_to_json, Config, JsonArray, JsonType, NullValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use stdext::function_name;

// nmap -sU -sS -p U:137,T:139 --script smb-enum-shares 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null
// nmap -sS -sV -p 111,2049 --script nfs-showmount 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null

// nmap -sV --script=broadcast-upnp-info 192.168.1.1
// way faster
// nmap -sU -p 1900 --script=upnp-info 192.168.1.1

pub struct NMAPShareList {
    pub mm_share_type: String,
    pub mm_share_ip: String,
    pub mm_share_path: String,
    pub mm_share_comment: String,
}

pub async fn mk_network_share_scan(
    subnet_prefix: String,
) -> Result<Vec<NMAPShareList>, Box<dyn std::error::Error>> {
    let conf = Config::new_with_defaults();
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
                        let share_data = NMAPShareList {
                            mm_share_type: "smb".to_string(),
                            mm_share_ip: format!("{:?}", ip_address.1).parse().unwrap(),
                            mm_share_path: v[share_ndx]["@key"].to_string(),
                            mm_share_comment: v[share_ndx]["elem"][1]["#text"].to_string(),
                        };
                        vec_share.push(share_data);
                    }
                }
            }
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
        let file_data = mk_lib_file::mk_read_file_data("scan.xml").await.unwrap();
        if !file_data.contains("(0 hosts up)") && file_data.contains("table key=") {
            let nmap_json = xml_string_to_json(file_data.to_string(), &conf).unwrap();
            println!("json: {:?}", nmap_json);
            for val in nmap_json["nmaprun"]["host"].as_object().unwrap() {
                let (key, v) = val;
                println!("Key: {:?}", key);
                if key == "table" {
                    println!("value: {}", v);
                    for share_ndx in 0..v.as_array().unwrap().len() {
                        println!("num: {}", v.as_array().unwrap().len());
                        println!("path: {}", v[share_ndx]["@key"]);
                        println!("elem: {}", v[share_ndx]["elem"]);
                        println!("elem: {}", v[share_ndx]["elem"][1]["#text"]);
                        let share_data = NMAPShareList {
                            mm_share_type: "nfs".to_string(),
                            mm_share_ip: format!("{:?}", ip_address.1).parse().unwrap(),
                            mm_share_path: v[share_ndx]["@key"].to_string(),
                            mm_share_comment: v[share_ndx]["elem"][1]["#text"].to_string(),
                        };
                        //println!("Share NFS: {:?}", share_data);
                        vec_share.push(share_data);
                    }
                }
            }
        }
    }
    //println!("Vec: {:?}", vec_share);
    Ok(vec_share)
}
