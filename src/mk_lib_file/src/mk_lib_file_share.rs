use std::error::Error;
use std::process::{Command, Stdio};

pub async fn mk_file_share_mount(mount_info: &serde_json::Value) -> Result<(), Box<dyn Error>> {
    /*
    mount //192.168.1.122/testshare /mnt/cifs
    # due to old ass syn
    mount -t cifs -o rw,guest,vers=1.0  //192.168.1.122/testshare /mnt
    ,username=user_name,password=password 
   */
    let output = Command::new("mount")
        .args([
            "-hide_banner",
            "-show_format",
            "json",
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let _stdout = String::from_utf8(output.stdout).unwrap();
    Ok(())
}
