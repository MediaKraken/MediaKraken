use mini_telnet::Telnet;
use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use std::time::Duration;
use stdext::function_name;

pub async fn telnet_connect(
    telnet_prompt: String,
    telnet_ip: String,
    telnet_port: String,
    telnet_user: String,
    telnet_password: String,
) -> Result<Telnet, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
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
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    telnet_instance
        .normal_execute(&telnet_command)
        .await
        .unwrap();
}

pub async fn telnet_execute(mut telnet_instance: Telnet, telnet_command: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    telnet_instance.execute(&telnet_command).await.unwrap();
}
