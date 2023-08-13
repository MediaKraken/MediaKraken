use std::error::Error;
use std::fs;
use std::process::{Command, Stdio};
use mk_lib_database;

pub async fn mk_file_share_mount(
    host_ip: &String,
    host_path: &String,
    share_guid: &uuid::Uuid,
    share_username: &Option<String>,
    share_password: &Option<String>,
    share_version_old: Option<bool>,
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
    if share_version_old.is_some() {
        share_options.push("-o rw,guest,vers=1.0".to_string());
    }
    if share_username.is_some() {
        share_options.push(format!(
            "-o username={:?},password={:?}",
            share_username, share_password
        ));
    }
    fs::create_dir_all(format!("/mediakraken/mnt/{}", share_guid))?;
    let output = Command::new("mount")
        .args(&share_options)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("Mount out: {:?}", stdout);
    let stderr = String::from_utf8(output.stderr).unwrap();
    println!("Mount err: {:?}", stderr);
    Ok(())
}

pub async fn mk_file_share_mount_all(
    shares_to_mount: Vec<mk_lib_database::mk_lib_database_network_share::DBShareList>,
) -> Result<(), Box<dyn Error>> {
    for share_info in shares_to_mount.iter() {
        let _result = mk_file_share_mount(
            &share_info.mm_network_share_ip.to_string(),
            &share_info.mm_network_share_path,
            &share_info.mm_network_share_guid,
            &share_info.mm_network_share_user,
            &share_info.mm_network_share_password,
            share_info.mm_network_share_version,
        )
        .await;
    }
    Ok(())
}
