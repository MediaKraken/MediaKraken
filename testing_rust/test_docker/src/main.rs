use mk_lib_common::mk_lib_common_docker;

#[tokio::main]
async fn main() {
    let template_data = mk_lib_common_docker::mk_common_docker_info().await.unwrap();
    println!("Docker Info");
    println!("{:?}", template_data.operating_system);
    println!("{:?}", template_data.kernel_version);
    println!("{:?}", template_data.server_version);
    let what = template_data.swarm.unwrap().node_addr.unwrap();
    println!("{:?}", what);
    // let swarm_info = template_data.swarm.unwrap();

    // println!("{:?}", swarm_info.node_addr);
    // println!("{:?}", swarm_info.nodes); // don't need as iter below
    //                                     //println!("{:?}", template_data.swarm.unwrap());

    println!("\nSwarm Inspect");
    let docker_data = mk_lib_common_docker::mk_common_docker_swarm_inspect()
        .await
        .unwrap();
    let token = docker_data.join_tokens.unwrap();
    println!("{:?}", token.manager);
    println!("{:?}", token.worker);

    println!("\nSwarm Nodes");
    let node_data = mk_lib_common_docker::mk_common_docker_swarm_nodes()
        .await
        .unwrap();
    for ndx in node_data.iter() {
        println!("{:?}", ndx.created_at);
        println!("{:?}", ndx.description);
        println!("{:?}", ndx.id);
        println!("{:?}", ndx.manager_status);
        println!("{:?}", ndx.spec);
        println!("{:?}", ndx.status);
        println!("{:?}", ndx.updated_at);
        println!("{:?}", ndx.version);
        //println!("{:?}", node_data.get(ndx).unwrap().id);
    }
}
