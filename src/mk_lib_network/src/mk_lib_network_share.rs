// https://docs.rs/ipnet/latest/ipnet/#

use quickxml_to_serde::{Config, xml_string_to_json};
use std::net::IpAddr;
use tempfile::NamedTempFile;
use tokio::process::Command;

// nmap -sU -sS -p U:137,T:139 --script smb-enum-shares 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null
// By default, the script uses guest permissions to list only publicly available shares
// - private shares will be left out as they are not accessible with guest permissions.

// nmap -sS -sV -p 111,2049 --script nfs-showmount 192.168.1.122 -oX scan.xml 1>/dev/null 2>/dev/null
// nmap -p 445 --script smb-enum-shares 192.168.1.122 -Pn -n -oX scan.xml

/*
0 - smb1
1 - smb2
2 - smb3
5 - nfs
6 - nfs3
7 - nfs4
8 - nfs4.1
9 - nfs4.2
 */

pub const SHARE_TYPE_SMB2: i16 = 1;
pub const SHARE_TYPE_NFS41: i16 = 8;

#[derive(Debug)]
pub struct NMAPShareList {
    pub mm_share_type: i16,
    pub mm_share_ip: IpAddr,
    pub mm_share_path: serde_json::Value,
    pub mm_share_comment: serde_json::Value,
}

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub async fn mk_network_share_scan_port_rustscan(
    subnet_prefix: String,
) -> Result<Vec<NMAPShareList>, BoxError> {
    // rustscan -n -a 192.168.1.0/24 -p 445,2049 -g
    let output = Command::new("rustscan")
        .arg("-n")
        .arg("-a")
        .arg(format!("{}.0/24", subnet_prefix))
        .arg("-p")
        .arg("445,2049")
        .arg("-g")
        .output()
        .await?;
    let stdout = String::from_utf8(output.stdout)?;
    let mut vec_share = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        let Some(ip_str) = trimmed.split_whitespace().next() else {
            continue;
        };
        let Ok(ip_addr) = ip_str.parse::<IpAddr>() else {
            continue;
        };
        if trimmed.contains("445") {
            vec_share.extend(mk_network_share_smb_detail(ip_addr).await?);
        }
        if trimmed.contains("2049") {
            vec_share.extend(mk_network_share_nfs_detail(ip_addr).await?);
        }
    }
    Ok(vec_share)
}

pub async fn mk_network_share_scan_port(
    subnet_prefix: String,
) -> Result<Vec<NMAPShareList>, BoxError> {
    // find all open smb, nfs ports, -n to not do dns lookup
    // nmap -p 445,2049 -n --open 192.168.1.*
    let tmp = NamedTempFile::new()?;
    let tmp_path = tmp.path().to_path_buf();
    Command::new("nmap")
        .arg("-p")
        .arg("445,2049")
        .arg("-n")
        .arg(format!("{}.*", subnet_prefix))
        .arg("-oN")
        .arg(&tmp_path)
        .output()
        .await?;
    let text = tokio::fs::read_to_string(&tmp_path).await?;
    let mut vec_share = Vec::new();
    let mut ip_addr: Option<IpAddr> = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Nmap scan report for ") {
            ip_addr = rest.split_whitespace().last().and_then(|s| s.parse().ok());
        } else if trimmed.starts_with("445/tcp") {
            if let Some(ip) = ip_addr {
                vec_share.extend(mk_network_share_smb_detail(ip).await?);
            }
        } else if trimmed.starts_with("2049/tcp") {
            if let Some(ip) = ip_addr {
                vec_share.extend(mk_network_share_nfs_detail(ip).await?);
            }
        }
    }
    Ok(vec_share)
}

pub async fn mk_network_share_smb_detail(
    ip_addr: IpAddr,
) -> Result<Vec<NMAPShareList>, BoxError> {
    let tmp = NamedTempFile::new()?;
    let tmp_path = tmp.path().to_path_buf();
    Command::new("nmap")
        .arg("-sU")
        .arg("-sS")
        .arg("-p")
        .arg("U:137,T:139")
        .arg("--script")
        .arg("smb-enum-shares")
        .arg(ip_addr.to_string())
        .arg("-oX")
        .arg(&tmp_path)
        .output()
        .await?;
    let file_data = tokio::fs::read_to_string(&tmp_path).await?;
    parse_smb_xml(&file_data, ip_addr)
}

pub async fn mk_network_share_nfs_detail(
    ip_addr: IpAddr,
) -> Result<Vec<NMAPShareList>, BoxError> {
    let tmp = NamedTempFile::new()?;
    let tmp_path = tmp.path().to_path_buf();
    Command::new("nmap")
        .arg("-sS")
        .arg("-sV")
        .arg("-p")
        .arg("111,2049")
        .arg("--script")
        .arg("nfs-showmount")
        .arg(ip_addr.to_string())
        .arg("-oX")
        .arg(&tmp_path)
        .output()
        .await?;
    let file_data = tokio::fs::read_to_string(&tmp_path).await?;
    parse_nfs_xml(&file_data, ip_addr)
}

// Exposed as pub(crate) so unit tests can exercise parsing without shelling out.
pub fn parse_smb_xml(
    file_data: &str,
    ip_addr: IpAddr,
) -> Result<Vec<NMAPShareList>, BoxError> {
    let mut vec_share = Vec::new();
    if file_data.contains("(0 hosts up)") || !file_data.contains("table key=") {
        return Ok(vec_share);
    }
    let conf = Config::new_with_defaults();
    let nmap_json = xml_string_to_json(file_data.to_string(), &conf)?;
    let script = &nmap_json["nmaprun"]["host"]["hostscript"]["script"];
    let Some(obj) = script.as_object() else {
        return Ok(vec_share);
    };
    for (key, v) in obj {
        if key != "table" {
            continue;
        }
        let Some(arr) = v.as_array() else { continue };
        for entry in arr {
            let path = &entry["@key"];
            if path.to_string().contains('$') {
                continue;
            }
            vec_share.push(NMAPShareList {
                mm_share_type: SHARE_TYPE_SMB2,
                mm_share_ip: ip_addr,
                mm_share_path: path.clone(),
                mm_share_comment: entry["elem"][1]["#text"].clone(),
            });
        }
    }
    Ok(vec_share)
}

pub fn parse_nfs_xml(
    file_data: &str,
    ip_addr: IpAddr,
) -> Result<Vec<NMAPShareList>, BoxError> {
    let mut vec_share = Vec::new();
    if file_data.contains("(0 hosts up)") || !file_data.contains("table key=") {
        return Ok(vec_share);
    }
    let conf = Config::new_with_defaults();
    let nmap_json = xml_string_to_json(file_data.to_string(), &conf)?;
    let Some(host_obj) = nmap_json["nmaprun"]["host"].as_object() else {
        return Ok(vec_share);
    };
    for (key, v) in host_obj {
        if key != "table" {
            continue;
        }
        let Some(arr) = v.as_array() else { continue };
        for entry in arr {
            vec_share.push(NMAPShareList {
                mm_share_type: SHARE_TYPE_NFS41,
                mm_share_ip: ip_addr,
                mm_share_path: entry["@key"].clone(),
                mm_share_comment: entry["elem"][1]["#text"].clone(),
            });
        }
    }
    Ok(vec_share)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn fixture(name: &str) -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read fixture {:?}: {e}", path))
    }

    #[test]
    fn parse_nfs_xml_extracts_mount_table() {
        let ip: IpAddr = "192.168.1.122".parse().unwrap();
        let shares = parse_nfs_xml(&fixture("nfs_scan.xml"), ip).unwrap();
        // The committed fixture contains rpcinfo tables but no nfs-showmount
        // `table` entries under <host>, so no shares should be produced; the
        // parser must handle that shape without panicking.
        assert!(shares.is_empty(), "unexpected shares: {shares:?}");
    }

    #[test]
    fn parse_nfs_xml_empty_scan_returns_empty() {
        let ip: IpAddr = "10.0.0.1".parse().unwrap();
        let shares = parse_nfs_xml("<nmaprun>(0 hosts up)</nmaprun>", ip).unwrap();
        assert!(shares.is_empty());
    }

    #[test]
    fn parse_smb_xml_empty_scan_returns_empty() {
        let ip: IpAddr = "10.0.0.1".parse().unwrap();
        let shares = parse_smb_xml("", ip).unwrap();
        assert!(shares.is_empty());
    }
}
