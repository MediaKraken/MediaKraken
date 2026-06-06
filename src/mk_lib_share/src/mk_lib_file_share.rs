use mk_lib_database;
use std::error::Error;
use std::process::Stdio;
use tokio::fs;
use tokio::process::Command;

fn validate_mount_option_value(value: &str, field: &str) -> Result<(), Box<dyn Error>> {
    // `-o key=value,key=value,...` is comma-separated, so a value containing
    // `,` or NUL could inject additional mount flags. Refuse those here.
    if value.contains(',') || value.contains('\0') || value.contains('\n') {
        return Err(
            format!("mount option {field:?} contains disallowed characters: {value:?}").into(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mount_option_value_valid() {
        assert!(validate_mount_option_value("myuser", "username").is_ok());
    }

    #[test]
    fn test_validate_mount_option_value_comma_rejected() {
        let result = validate_mount_option_value("user,name", "username");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_mount_option_value_nul_rejected() {
        let result = validate_mount_option_value("user\0name", "username");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_mount_option_value_newline_rejected() {
        let result = validate_mount_option_value("user\nname", "username");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_mount_option_value_empty_valid() {
        assert!(validate_mount_option_value("", "username").is_ok());
    }

    #[test]
    fn test_validate_mount_option_value_special_chars_valid() {
        assert!(validate_mount_option_value("user-name_123.test", "username").is_ok());
    }
}

pub async fn mk_file_share_mount(
    host_ip: &str,
    host_path: &str,
    share_guid: &uuid::Uuid,
    share_username: &Option<String>,
    share_password: &Option<String>,
    share_version_old: bool,
) -> Result<(), Box<dyn Error>> {
    // Example target: mount -t cifs -o rw,guest,vers=1.0 //host/share /mnt
    let source = format!("//{}/{}", host_ip, host_path.replace('\\', "/"));
    let target = format!("/mediakraken/mnt/{share_guid}");
    fs::create_dir_all(&target).await?;

    let mut options: Vec<String> = Vec::new();
    if share_version_old {
        options.push("rw".to_string());
        options.push("guest".to_string());
        options.push("vers=1.0".to_string());
    }
    if let Some(user) = share_username.as_deref() {
        validate_mount_option_value(user, "username")?;
        options.push(format!("username={user}"));
        if let Some(pass) = share_password.as_deref() {
            validate_mount_option_value(pass, "password")?;
            options.push(format!("password={pass}"));
        }
    }

    let mut cmd = Command::new("mount");
    if !options.is_empty() {
        cmd.arg("-o").arg(options.join(","));
    }
    cmd.arg(&source).arg(&target);

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;
    if !output.status.success() {
        return Err(format!(
            "mount {source} -> {target} failed: status={:?} stderr={}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(())
}

pub async fn mk_file_share_mount_all(
    shares_to_mount: Vec<mk_lib_database::mk_lib_database_network_share::DBShareList>,
) -> Result<(), Box<dyn Error>> {
    for share_info in shares_to_mount.iter() {
        // `mm_network_share_version` is an i16 in the DB; treat `1` as the
        // "old SMBv1" flag since that's what the original behaviour encoded.
        let share_version_old = share_info.mm_network_share_version == 1;
        if let Err(e) = mk_file_share_mount(
            &share_info.mm_network_share_ip.to_string(),
            &share_info.mm_network_share_path,
            &share_info.mm_network_share_guid,
            &share_info.mm_share_auth_user,
            &share_info.mm_share_auth_password,
            share_version_old,
        )
        .await
        {
            eprintln!(
                "failed to mount share {}: {e}",
                share_info.mm_network_share_guid
            );
        }
    }
    Ok(())
}
