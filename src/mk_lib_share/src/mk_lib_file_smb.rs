
fn classify_smbclient_browse_error(
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
fn is_smb_ls_date(s: &str) -> bool {
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
