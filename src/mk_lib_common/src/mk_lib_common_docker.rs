#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use shiplift::Docker;

pub async fn mk_common_docker_info() -> Result<(String), std::io::Error> {
    let docker = Docker::new();
    match docker.containers().list(&Default::default()).await {
        Ok(containers) => {
            for c in containers {
                println!("container -> {:#?}", c)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(stdout)
}
