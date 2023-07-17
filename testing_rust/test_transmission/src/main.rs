use mk_lib_network;

#[tokio::main]
async fn main() {
    let trans_client = mk_lib_network::mk_lib_network_transmission::mk_network_transmission_login().await.unwrap();

    let res = mk_lib_network::mk_lib_network_transmission::mk_network_transmission_list_torrents(trans_client).await.unwrap();
    println!("{:?}", res);
    //let _res = mk_lib_network::mk_lib_network_transmission::mk_network_transmission_close(trans_client).await;
}
