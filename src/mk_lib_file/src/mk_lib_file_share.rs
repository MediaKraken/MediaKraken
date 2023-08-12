use std::error::Error;
use std::fs;
use std::process::{Command, Stdio};

pub async fn mk_file_share_mount(
    host_ip: &String,
    host_path: &String,
    share_guid: &uuid::Uuid,
    share_username: &String,
    share_password: &String,
    share_version_old: bool,
) -> Result<(), Box<dyn Error>> {
    /*
     # due to old ass synology
     mount -t cifs -o rw,guest,vers=1.0  //192.168.1.122/testshare /mnt
     ,username=user_name,password=password
    */
    let mut share_options = vec![
        format!("{}/{}", host_ip, host_path.replace("\\", "/")),
        format!("/mediakraken/mnt/{}", share_guid),
    ];
    if share_version_old {
        share_options.push("-o rw,guest,vers=1.0".to_string());
    }
    if share_username != "" {
        share_options.push(format!("-o username={},password={}", share_username, share_password));
    }
    fs::create_dir_all(format!("/mediakraken/mnt/{}", share_guid))?;
    let output = Command::new("mount")
        .args(&share_options)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let _stdout = String::from_utf8(output.stdout).unwrap();
    Ok(())
}
