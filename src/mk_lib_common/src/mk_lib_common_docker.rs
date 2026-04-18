// https://github.com/vv9k/docker-api-rs
// docker-api = { version = "0.12.1", features = ["swarm"] }

extern crate tokio;
use docker_api::models::Node;
use docker_api::models::Swarm;
use docker_api::models::SystemInfo;
use docker_api::opts::ContainerListOpts;
use docker_api::opts::LogsOpts;
use docker_api::opts::ServiceListOpts;
use docker_api::{Docker, Result};
use futures::StreamExt;
// use docker_api::models::SwarmInfo;
// use docker_api::models::JoinTokens;

pub async fn mk_common_docker_container_inspect(id: String) -> Result<String> {
    let docker = Docker::unix("/var/run/docker.sock");
    let container = docker.containers().get(&id).inspect().await?;
    Ok(format!("{:#?}", container))
}

pub async fn mk_common_docker_container_list() -> Result<Vec<String>> {
    let docker = Docker::unix("/var/run/docker.sock");
    let opts = ContainerListOpts::builder().all(true).build();
    let containers = docker.containers().list(&opts).await?;
    let rows = containers
        .into_iter()
        .map(|container| {
            let id = container.id.unwrap_or_default();
            let id_short = id.get(..12).unwrap_or(id.as_str()).to_string();
            format!(
                "{}\t{}\t{:?}\t{}\t{}",
                id_short,
                container.image.unwrap_or_default(),
                container.state,
                container.status.unwrap_or_default(),
                container
                    .names
                    .and_then(|n| n.into_iter().next())
                    .unwrap_or_default()
            )
        })
        .collect();
    Ok(rows)
}

pub async fn mk_common_docker_container_logs(id: String) -> Result<String> {
    let docker = Docker::unix("/var/run/docker.sock");
    let container = docker.containers().get(&id);
    let mut logs_stream = container.logs(&LogsOpts::builder().stdout(true).stderr(true).build());
    let mut buffer: Vec<u8> = Vec::new();
    while let Some(chunk) = logs_stream.next().await {
        buffer.extend_from_slice(&chunk?);
    }
    Ok(String::from_utf8_lossy(&buffer).into_owned())
}

pub async fn mk_common_docker_container_stats(id: String) -> Result<Vec<String>> {
    let docker = Docker::unix("/var/run/docker.sock");
    let container = docker.containers().get(&id);
    let mut stats_stream = container.stats();
    let mut stats_list = Vec::new();
    while let Some(result) = stats_stream.next().await {
        stats_list.push(format!("{:?}", result?));
    }
    Ok(stats_list)
}

pub async fn mk_common_docker_service_inspect(service: String) -> Result<String> {
    let docker = Docker::unix("/var/run/docker.sock");
    let info = docker.services().get(&service).inspect().await?;
    Ok(format!("{:#?}", info))
}

pub async fn mk_common_docker_service_list() -> Result<Vec<String>> {
    let docker = Docker::unix("/var/run/docker.sock");
    let services = docker
        .services()
        .list(&ServiceListOpts::builder().status(true).build())
        .await?;
    Ok(services.into_iter().map(|s| format!("{:#?}", s)).collect())
}

pub async fn mk_common_docker_service_logs(service: String) -> Result<String> {
    let docker = Docker::unix("/var/run/docker.sock");
    let service = docker.services().get(&service);
    let mut logs_stream = service.logs(&LogsOpts::builder().stdout(true).stderr(true).build());
    let mut buffer: Vec<u8> = Vec::new();
    while let Some(chunk) = logs_stream.next().await {
        buffer.extend_from_slice(&chunk?);
    }
    Ok(String::from_utf8_lossy(&buffer).into_owned())
}

pub async fn mk_common_docker_volume_inspect(volume: String) -> Result<String> {
    let docker = Docker::unix("/var/run/docker.sock");
    let info = docker.volumes().get(&volume).inspect().await?;
    Ok(format!("{:#?}", info))
}

pub async fn mk_common_docker_volume_list() -> Result<Vec<String>> {
    let docker = Docker::unix("/var/run/docker.sock");
    let volumes = docker.volumes().list(&Default::default()).await?;
    Ok(volumes
        .volumes
        .into_iter()
        .map(|v| format!("{:#?}", v))
        .collect())
}

pub async fn mk_common_docker_info() -> Result<SystemInfo> {
    let docker = Docker::unix("/var/run/docker.sock");
    docker.info().await
}

pub async fn mk_common_docker_swarm_inspect() -> Result<Swarm> {
    let docker = Docker::unix("/var/run/docker.sock");
    docker.swarm().inspect().await
}

pub async fn mk_common_docker_swarm_nodes() -> Result<Vec<Node>> {
    let docker = Docker::unix("/var/run/docker.sock");
    docker.nodes().list(&Default::default()).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mk_common_docker_container_inspect() {
        let test_results = mk_common_docker_container_inspect("mkstack-example".to_string())
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
        let test_results = mk_common_docker_container_logs("mkstack-example".to_string())
            .await
            .unwrap();
        println!("Cont Logs: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_container_stats() {
        let test_results = mk_common_docker_container_stats("mkstack-example".to_string())
            .await
            .unwrap();
        println!("Cont Stats: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_service_inspect() {
        let test_results = mk_common_docker_service_inspect("mkstack-example".to_string())
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
        let test_results = mk_common_docker_service_logs("mkstack-example".to_string())
            .await
            .unwrap();
        println!("Service Logs: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_volume_inspect() {
        let test_results = mk_common_docker_volume_inspect("mkstack-example".to_string())
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

    #[tokio::test]
    async fn test_mk_common_docker_swarm_inspect() {
        let test_results = mk_common_docker_swarm_inspect().await.unwrap();
        println!("Swarm Inspect: {:?}", test_results);
    }

    #[tokio::test]
    async fn test_mk_common_docker_swarm_node() {
        let test_results = mk_common_docker_swarm_nodes().await.unwrap();
        println!("Swarm Node: {:?}", test_results);
    }
}
