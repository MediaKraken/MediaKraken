// https://github.com/1148118271/ssh-rs/tree/main

use ssh_rs::ssh;

pub async fn mk_network_ssh_command(
    username: &str,
    password: &str,
    host_ip: &str,
    host_port: u16,
    command_string: &str,
) {
    let mut session = ssh::create_session()
        .username(username)
        .password(password)
        //.private_key_path("./id_rsa")
        .connect(format!("{}:{}", host_ip, host_port))
        .unwrap()
        .run_local();
    let exec = session.open_exec().unwrap();
    let vec: Vec<u8> = exec.send_command(command_string).unwrap();
    println!("{}", String::from_utf8(vec).unwrap());
    session.close();
}

pub async fn mk_network_ssh_upload(
    username: &str,
    password: &str,
    host_ip: &str,
    host_port: u16,
    local_file: &str,
    remote_file: &str,
) {
    let mut session = ssh::create_session()
        .username(username)
        .password(password)
        //.private_key_path("./id_rsa")
        .connect(format!("{}:{}", host_ip, host_port))
        .unwrap()
        .run_local();
    let scp = session.open_scp().unwrap();
    scp.upload(local_file, remote_file).unwrap();
}

pub async fn mk_network_ssh_download(
    username: &str,
    password: &str,
    host_ip: &str,
    host_port: u16,
    local_file: &str,
    remote_file: &str,
) {
    let mut session = ssh::create_session()
        .username(username)
        .password(password)
        //.private_key_path("./id_rsa")
        .connect(format!("{}:{}", host_ip, host_port))
        .unwrap()
        .run_local();
    let scp = session.open_scp().unwrap();
    scp.download(local_file, remote_file).unwrap();
}
