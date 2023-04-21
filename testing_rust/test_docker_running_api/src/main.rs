use docker_api::Docker;

#[tokio::main]
async fn main() {
    let docker = Docker::unix("/var/run/docker.sock");
    println!("docker containers that are running");
    let result = docker.containers().list(&Default::default()).await;
    match result {
        Ok(images) => {
            for i in images {
//                if i.names[0] == "/mkstack_reactor" {
                    println!(
                        "{:?} {:?} {:?} {:?}",
                        i.id,
                        i.created,
                        //i.labels,  // the actual labels in the dockerfile
                        i.ports,
                        i.names  );
//                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
