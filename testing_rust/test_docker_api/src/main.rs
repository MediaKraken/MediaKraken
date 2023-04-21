use docker_api::{Docker};
use docker_api::api::service::Services;
use docker_api::opts::ServiceListOpts;

#[tokio::main]
async fn main() {
    let docker = Docker::unix("/var/run/docker.sock");
    println!("docker images");
    let result = docker.images().list(&Default::default()).await;
    match result {
        Ok(images) => {
            for i in images {
                println!(
                    "{} {} {:?} {}",
                    i.id,
                    i.created, // unix timestamp
                    i.repo_tags.join(","),
                    i.labels
                        .into_iter()
                        .map(|(k, v)| format!(" - {k}={v}"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    //   i.repo_tags.unwrap_or_else(|| vec!["none".into()])
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("docker volumes");
    match docker.volumes().list(&Default::default()).await {
        Ok(volumes) => {
            //for v in volumes {
            println!("volume -> {:#?}", volumes)
            //}
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("docker info");
    match docker.info().await {
        Ok(info) => println!("info {:?}", info),
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("docker services");
    match docker
        .services()
        .list(&ServiceListOpts::builder().build())
        .await
    {
        Ok(services) => {
            for s in services {
                println!("service -> {:#?}", s)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
