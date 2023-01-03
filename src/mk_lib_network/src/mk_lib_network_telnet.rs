#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// mini-telnet = "0.1.8"

use mini_telnet::Telnet;
use std::time::Duration;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn telnet_connect(
    telnet_prompt: String,
    telnet_ip: String,
    telnet_port: String,
    telnet_user: String,
    telnet_password: String,
) {
    let mut telnet_instance = Telnet::builder()
        .prompt(telnet_prompt)
        .login_prompt("login: ", "Password: ")
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(5))
        .connect(format!("{}:{}", telnet_ip, telnet_port))
        .await?;
    telnet_instance
        .login(&telnet_user, &telnet_password)
        .await
        .unwrap();
}

pub async fn telnet_execute_normal(telnet_instance: Telnet, telnet_command: String) {
    telnet_instance
        .normal_execute(&telnet_command)
        .await
        .unwrap();
}

pub async fn telnet_execute(telnet_instance: Telnet, telnet_command: String) {
    telnet_instance.execute(&telnet_command).await.unwrap();
}
