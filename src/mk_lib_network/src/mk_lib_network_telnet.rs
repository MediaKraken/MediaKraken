use mini_telnet::Telnet;
use std::time::Duration;

pub async fn telnet_connect(
    telnet_prompt: String,
    telnet_ip: String,
    telnet_port: String,
    telnet_user: String,
    telnet_password: String,
) -> Result<Telnet, Box<dyn std::error::Error>> {
    let mut telnet_instance = Telnet::builder()
        .prompt(telnet_prompt)
        .login_prompt("login: ", "Password: ")
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(5))
        .connect(&format!("{}:{}", telnet_ip, telnet_port))
        .await?;
    telnet_instance
        .login(&telnet_user, &telnet_password)
        .await
        .unwrap();
    Ok(telnet_instance)
}

pub async fn telnet_execute_normal(mut telnet_instance: Telnet, telnet_command: String) {
    telnet_instance
        .normal_execute(&telnet_command)
        .await
        .unwrap();
}

pub async fn telnet_execute(mut telnet_instance: Telnet, telnet_command: String) {
    telnet_instance.execute(&telnet_command).await.unwrap();
}
