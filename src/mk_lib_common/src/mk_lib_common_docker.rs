// https://github.com/vv9k/docker-api-rs
// docker-api = { version = "0.12.1", features = ["swarm"] }

extern crate tokio;
use futures::StreamExt;
use mk_lib_logging::mk_lib_logging;

use docker_api::opts::ContainerListOpts;
use docker_api::opts::LogsOpts;
use docker_api::opts::ServiceListOpts;
use docker_api::{Docker, Result};
use serde_json::json;
use stdext::function_name;

pub async fn mk_common_docker_container_inspect(id: String) -> Result<Vec<String>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut logs_list: Vec<String> = Vec::new();
    match docker.containers().get(&id).inspect().await {
        Ok(container) => println!("{:#?}", container),
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}

pub async fn mk_common_docker_container_list() -> Result<Vec<String>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut container_list: Vec<String> = Vec::new();
    let opts = ContainerListOpts::builder().all(true).build();
    match docker.containers().list(&opts).await {
        Ok(containers) => {
            containers.into_iter().for_each(|container| {
                println!(
                    "{}\t{}\t{:?}\t{}\t{}",
                    &container.id.unwrap_or_default()[..12],
                    container.image.unwrap_or_default(),
                    container.state,
                    container.status.unwrap_or_default(),
                    container.names.map(|n| n[0].to_owned()).unwrap_or_default()
                );
            });
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(container_list)
}

pub async fn mk_common_docker_container_logs(id: String) -> Result<Vec<String>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut logs_list: Vec<String> = Vec::new();
    let container = docker.containers().get(&id);
    let logs_stream = container.logs(&LogsOpts::builder().stdout(true).stderr(true).build());
    let logs: Vec<_> = logs_stream
        .map(|chunk| match chunk {
            Ok(chunk) => chunk.to_vec(),
            Err(e) => {
                eprintln!("Error: {e}");
                vec![]
            }
        })
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    print!("{}", String::from_utf8_lossy(&logs));
    Ok(logs_list)
}

pub async fn mk_common_docker_container_stats(id: String) -> Result<Vec<String>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut stats_list: Vec<String> = Vec::new();
    while let Some(result) = docker.containers().get(&id).stats().next().await {
        match result {
            Ok(stat) => println!("{:?}", stat),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(stats_list)
}

pub async fn mk_common_docker_service_inspect(service: String) -> Result<Vec<String>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut logs_list: Vec<String> = Vec::new();
    match docker.services().get(&service).inspect().await {
        Ok(service) => println!("{:#?}", service),
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}

pub async fn mk_common_docker_service_list() -> Result<Vec<String>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut logs_list: Vec<String> = Vec::new();
    match docker
        .services()
        .list(&ServiceListOpts::builder().status(true).build())
        .await
    {
        Ok(services) => {
            for s in services {
                #[cfg(debug_assertions)]
                {
                    mk_lib_logging::mk_logging_post_elk(
                        std::module_path!(),
                        json!({ "service": s }),
                    )
                    .await
                    .unwrap();
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(logs_list)
}

pub async fn mk_common_docker_service_logs(service: String) -> Result<Vec<String>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut logs_list: Vec<String> = Vec::new();
    let service = docker.services().get(&service);
    let logs_stream = service.logs(&LogsOpts::builder().stdout(true).stderr(true).build());
    let logs: Vec<_> = logs_stream
        .map(|chunk| match chunk {
            Ok(chunk) => chunk.to_vec(),
            Err(e) => {
                eprintln!("Error: {}", e);
                vec![]
            }
        })
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    print!("{}", String::from_utf8_lossy(&logs));
    Ok(logs_list)
}

pub async fn mk_common_docker_volume_inspect(volume: String) -> Result<Vec<String>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut logs_list: Vec<String> = Vec::new();
    match docker.volumes().get(&volume).inspect().await {
        Ok(info) => println!("{:#?}", info),
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}

pub async fn mk_common_docker_volume_list() -> Result<Vec<String>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut logs_list: Vec<String> = Vec::new();
    match docker.volumes().list(&Default::default()).await {
        Ok(volumes) => {
            for v in volumes.volumes {
                #[cfg(debug_assertions)]
                {
                    mk_lib_logging::mk_logging_post_elk(
                        std::module_path!(),
                        json!({ "volume": v }),
                    )
                    .await
                    .unwrap();
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}

pub async fn mk_common_docker_info() -> Result<serde_json::Value> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let docker = Docker::unix("/var/run/docker.sock");
    let mut logs_list: serde_json::Value = serde_json::json!({});
    match docker.info().await {
        Ok(info) => {
            #[cfg(debug_assertions)]
            {
                mk_lib_logging::mk_logging_post_elk(
                    std::module_path!(),
                    json!({ "info": &format!("{:#?}", info) }),
                )
                .await
                .unwrap();
            }
            logs_list = json!(&format!("{:#?}", info));
            //logs_list = serde_json::from_str(&format!("{:#?}", info)).unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_common_docker_container_inspect() {
        let test_results = mk_common_docker_container_inspect("mkstack_example".to_string())
            .await
            .unwrap();
        println!("Cont Inspect: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_container_list() {
        let test_results = mk_common_docker_container_list().await.unwrap();
        println!("Cont List: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_container_logs() {
        let test_results = mk_common_docker_container_logs("mkstack_example".to_string())
            .await
            .unwrap();
        println!("Cont Logs: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_container_stats() {
        let test_results = mk_common_docker_container_stats("mkstack_example".to_string())
            .await
            .unwrap();
        println!("Cont Stats: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_service_inspect() {
        let test_results = mk_common_docker_service_inspect("mkstack_example".to_string())
            .await
            .unwrap();
        println!("Service Inspect: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_service_list() {
        let test_results = mk_common_docker_service_list().await.unwrap();
        println!("Service List: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_service_logs() {
        let test_results = mk_common_docker_service_logs("mkstack_example".to_string())
            .await
            .unwrap();
        println!("Service Logs: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_volume_inspect() {
        let test_results = mk_common_docker_volume_inspect("mkstack_example".to_string())
            .await
            .unwrap();
        println!("Volume Inspect: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_volume_list() {
        let test_results = mk_common_docker_volume_list().await.unwrap();
        println!("Volume List: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_info() {
        let test_results = mk_common_docker_info().await.unwrap();
        println!("Info: {:?}", test_results);
    }
}
