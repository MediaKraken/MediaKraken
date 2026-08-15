// https://github.com/1148118271/ssh-rs/tree/main

pub async fn mk_network_ssh_command(
    username: &str,
    password: &str,
    host_ip: &str,
    host_port: u16,
    command_string: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ssh::create_session()
        .username(username)
        .password(password)
        .connect(format!("{}:{}", host_ip, host_port))
        .map_err(|e| format!("ssh connect failed: {e}"))?
        .run_local();
    let exec = session.open_exec().map_err(|e| format!("open exec failed: {e}"))?;
    let vec: Vec<u8> = exec.send_command(command_string).map_err(|e| format!("send command failed: {e}"))?;
    println!("{}", String::from_utf8(vec).map_err(|e| format!("invalid utf8: {e}"))?);
    session.close();
    Ok(())
}

pub async fn mk_network_ssh_upload(
    username: &str,
    password: &str,
    host_ip: &str,
    host_port: u16,
    local_file: &str,
    remote_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ssh::create_session()
        .username(username)
        .password(password)
        .connect(format!("{}:{}", host_ip, host_port))
        .map_err(|e| format!("ssh connect failed: {e}"))?
        .run_local();
    let scp = session.open_scp().map_err(|e| format!("open scp failed: {e}"))?;
    scp.upload(local_file, remote_file).map_err(|e| format!("scp upload failed: {e}"))?;
    session.close();
    Ok(())
}

pub async fn mk_network_ssh_download(
    username: &str,
    password: &str,
    host_ip: &str,
    host_port: u16,
    local_file: &str,
    remote_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ssh::create_session()
        .username(username)
        .password(password)
        .connect(format!("{}:{}", host_ip, host_port))
        .map_err(|e| format!("ssh connect failed: {e}"))?
        .run_local();
    let scp = session.open_scp().map_err(|e| format!("open scp failed: {e}"))?;
    scp.download(local_file, remote_file).map_err(|e| format!("scp download failed: {e}"))?;
    session.close();
    Ok(())
}
