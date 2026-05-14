
pub fn classify_smbclient_browse_error(
    stdout_output: &str,
    stderr_output: &str,
) -> (u16, &'static str) {
    let combined_output = format!("{stdout_output}\n{stderr_output}").to_ascii_lowercase();

    if combined_output.contains("nt_status_access_denied")
        || combined_output.contains("nt_status_logon_failure")
        || combined_output.contains("nt_status_account_disabled")
        || combined_output.contains("nt_status_no_logon_servers")
        || combined_output.contains("session setup failed")
        || combined_output.contains("access denied")
        || combined_output.contains("permission denied")
    {
        return (
            403, // StatusCode::FORBIDDEN,
            "Share is reachable but access was denied",
        );
    }

    if combined_output.contains("nt_status_object_path_not_found")
        || combined_output.contains("nt_status_object_name_not_found")
        || combined_output.contains("nt_status_bad_network_name")
        || combined_output.contains("no such file")
        || combined_output.contains("cannot chdir")
    {
        return (
            404, // StatusCode::NOT_FOUND, 
            "Share path was not found");
    }

    if combined_output.contains("nt_status_bad_network_path")
        || combined_output.contains("nt_status_network_name_deleted")
        || combined_output.contains("nt_status_io_timeout")
        || combined_output.contains("connection to")
        || combined_output.contains("connection refused")
        || combined_output.contains("could not resolve")
        || combined_output.contains("host is down")
        || combined_output.contains("name or service not known")
        || combined_output.contains("timed out")
    {
        return (502, // StatusCode::BAD_GATEWAY, 
            "Unable to reach share");
    }

    (502, "Failed to list share directories")
}

// Matches the 24-byte ASCII output of samba's `time_to_asc()` /
// `asctime`-style format: "Day Mon DD HH:MM:SS YYYY".
pub fn is_smb_ls_date(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 24 {
        return false;
    }
    bytes[3] == b' '
        && bytes[7] == b' '
        && bytes[10] == b' '
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b' '
        && bytes[0..3].iter().all(|c| c.is_ascii_alphabetic())
        && bytes[4..7].iter().all(|c| c.is_ascii_alphabetic())
        && (bytes[8] == b' ' || bytes[8].is_ascii_digit())
        && bytes[9].is_ascii_digit()
        && bytes[11..13].iter().all(|c| c.is_ascii_digit())
        && bytes[14..16].iter().all(|c| c.is_ascii_digit())
        && bytes[17..19].iter().all(|c| c.is_ascii_digit())
        && bytes[20..24].iter().all(|c| c.is_ascii_digit())
}

pub fn mk_file_smb_client_tree_smbclient(
    share_to_mount: &mk_lib_database::mk_lib_database_network_share::DBShareList,
    uri: &str,
) -> Result<Vec<FileMetadata>, Box<dyn Error>> {
    // smbclient's `-c` argument is a script: commands are separated by `;`
    // and paths are quoted with `"`. A uri containing either character could
    // inject additional smbclient commands, so reject it up front.
    let disallowed = [';', '"', '\n', '\r', '\\'];
    if uri.contains(disallowed) {
        return Err(format!("smbclient path contains disallowed characters: {uri:?}").into());
    }
    if share_to_mount.mm_network_share_path.contains(disallowed)
    {
        return Err("smbclient share host or path contains disallowed characters".into());
    }
    let mut smb_command = Command::new("smbclient");
    let share_uri = format!(
        "//{}/{}",
        share_to_mount.mm_network_share_ip, share_to_mount.mm_network_share_path
    );
    let mut smb_commands: Vec<String> =
        vec![String::from("recurse ON"), String::from("prompt OFF")];
    let cleaned_uri = uri.trim_start_matches('/');
    if !cleaned_uri.is_empty() {
        smb_commands.push(format!("cd \"{}\"", cleaned_uri));
    }
    smb_commands.push(String::from("ls"));
    smb_command
        .arg(share_uri)
        .arg("-g")
        .arg("-c")
        .arg(smb_commands.join(";"));
    if let Some(workgroup) = share_to_mount.mm_network_share_workgroup.as_deref() {
        if !workgroup.is_empty() {
            smb_command.arg("-W").arg(workgroup);
        }
    }
    let user_opt = share_to_mount
        .mm_share_auth_user
        .as_deref()
        .filter(|u| !u.is_empty());
    if let Some(user) = user_opt {
        // The `-U user%pass` form exposes the password to anything that can
        // read this process's argv (ps, /proc). Reject `%` in the credentials
        // so a malicious password can't escape the user field; a follow-up
        // should move to an auth file via `-A` to keep the secret off argv.
        let pass = share_to_mount
            .mm_share_auth_password
            .as_deref()
            .unwrap_or_default();
        if user.contains('%') || pass.contains('%') || user.contains('\n') || pass.contains('\n') {
            return Err("smb credentials contain disallowed characters".into());
        }
        smb_command.arg("-U").arg(format!("{}%{}", user, pass));
    } else {
        smb_command.arg("-N");
    }
    let smb_output = smb_command.output()?;
    if !smb_output.status.success() {
        return Err(format!(
            "smbclient failed with status {:?}",
            smb_output.status.code()
        )
        .into());
    }
    let stdout_data = String::from_utf8(smb_output.stdout)?;
    let mut file_list: Vec<FileMetadata> = vec![];
    for line in stdout_data.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 2 {
            continue;
        }
        if parts[0] != "D" && parts[0] != "F" {
            continue;
        }
        if parts[1] == "." || parts[1] == ".." {
            continue;
        }
        let path_value = if parts[1].starts_with('/') {
            parts[1].to_string()
        } else if uri.ends_with('/') {
            format!("{}{}", uri, parts[1])
        } else {
            format!("{}/{}", uri, parts[1])
        };
        file_list.push(FileMetadata {
            name: path_value,
            directory: parts[0] == "D",
        });
    }
    Ok(file_list)
}

