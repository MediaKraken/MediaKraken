use std::error::Error;
use std::process::{Command, Stdio};
use std::fs;

pub async fn mk_file_share_mount(
    host_ip: &String,
    host_path: &String,
    share_guid: &uuid::Uuid,
    share_username: &String,
    share_password: &String,
    share_version_old: &bool,
) -> Result<(), Box<dyn Error>> {
    /*
     # due to old ass synology
     mount -t cifs -o rw,guest,vers=1.0  //192.168.1.122/testshare /mnt
     ,username=user_name,password=password
    */
    fs::create_dir_all(format!("/mediakraken/mnt/{}", share_guid))?;
    let output = Command::new("mount")
        .args([
            format!("//{}/{}", host_ip, host_path),
            format!("/mediakraken/mnt/{}", share_guid),
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let _stdout = String::from_utf8(output.stdout).unwrap();
    Ok(())
}
