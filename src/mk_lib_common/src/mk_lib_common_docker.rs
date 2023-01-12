#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/vv9k/docker-api-rs
// docker-api = { version = "0.12.1", features = ["swarm"] }

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde_json::json;
use crate::rocket::futures::FutureExt;
use crate::rocket::futures::StreamExt;
use docker_api::api::ContainerListOpts;
use docker_api::api::LogsOpts;
use docker_api::api::ServiceListOpts;
use docker_api::{conn::TtyChunk, Docker, Result};

#[cfg(unix)]
pub fn new_docker() -> Result<Docker> {
    Ok(Docker::unix("/var/run/docker.sock"))
}

#[cfg(not(unix))]
pub fn new_docker() -> Result<Docker> {
    Docker::new("tcp://127.0.0.1:8080")
}

pub async fn mk_common_docker_container_inspect(id: String) -> Result<Vec<String>> {
    let docker = new_docker()?;
    let mut logs_list: Vec<String> = Vec::new();
    match docker.containers().get(&id).inspect().await {
        Ok(container) => println!("{:#?}", container),
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}

pub async fn mk_common_docker_container_list() -> Result<Vec<String>> {
    let docker = new_docker()?;
    let mut container_list: Vec<String> = Vec::new();
    let opts = ContainerListOpts::builder().all(true).build();
    match docker.containers().list(&opts).await {
        Ok(containers) => {
            containers.into_iter().for_each(|container| {
                println!(
                    "{}\t{}\t{:?}\t{}\t{}",
                    &container.id[..12],
                    container.image,
                    container.state,
                    container.status,
                    container.names[0]
                );
            });
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(container_list)
}

pub async fn mk_common_docker_container_logs(id: String) -> Result<Vec<String>> {
    let docker = new_docker()?;
    let mut logs_list: Vec<String> = Vec::new();
    let container = docker.containers().get(&id);
    let logs_stream = container.logs(&LogsOpts::builder().stdout(true).stderr(true).build());
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

pub async fn mk_common_docker_container_stats(id: String) -> Result<Vec<String>> {
    let docker = new_docker()?;
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
    let docker = new_docker()?;
    let mut logs_list: Vec<String> = Vec::new();
    match docker.services().get(&service).inspect().await {
        Ok(service) => println!("{:#?}", service),
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}

pub async fn mk_common_docker_service_list() -> Result<Vec<String>> {
    let docker = new_docker()?;
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
                    .await.unwrap();
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(logs_list)
}

pub async fn mk_common_docker_service_logs(service: String) -> Result<Vec<String>> {
    let docker = new_docker()?;
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
    let docker = new_docker()?;
    let mut logs_list: Vec<String> = Vec::new();
    match docker.volumes().get(&volume).inspect().await {
        Ok(info) => println!("{:#?}", info),
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}

pub async fn mk_common_docker_volume_list() -> Result<Vec<String>> {
    let docker = new_docker()?;
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
                    .await.unwrap();
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}

pub async fn mk_common_docker_info() -> Result<serde_json::Value> {
    let docker = new_docker()?;
    let mut logs_list: serde_json::Value = serde_json::json!({});
    match docker.info().await {
        Ok(info) => {
            #[cfg(debug_assertions)]
            {
                mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "info": info }))
                    .await.unwrap();
            }
            logs_list = serde_json::from_str(&format!("{:#?}", info)).unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(logs_list)
}
