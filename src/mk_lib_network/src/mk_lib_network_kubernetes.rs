use reqwest::Client;
use serde::Deserialize;
use std::fs;

const K8S_SERVICE_ACCOUNT_TOKEN_PATH: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";
const K8S_SERVICE_ACCOUNT_CA_PATH: &str = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt";
const K8S_API_SERVER: &str = "https://kubernetes.default.svc";

#[derive(Debug, Deserialize)]
struct KubernetesNodeList {
    items: Vec<KubernetesNode>,
}

#[derive(Debug, Deserialize)]
struct KubernetesNode {
    metadata: KubernetesNodeMetadata,
    status: KubernetesNodeStatus,
}

#[derive(Debug, Deserialize)]
struct KubernetesNodeMetadata {
    #[serde(default)]
    labels: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct KubernetesNodeStatus {
    #[serde(default)]
    addresses: Vec<KubernetesNodeAddress>,
}

#[derive(Debug, Deserialize)]
struct KubernetesNodeAddress {
    #[serde(rename = "type")]
    address_type: String,
    address: String,
}

fn is_worker_node(labels: &std::collections::HashMap<String, String>) -> bool {
    !labels.contains_key("node-role.kubernetes.io/control-plane")
        && !labels.contains_key("node-role.kubernetes.io/master")
}

fn find_external_ip(addresses: &[KubernetesNodeAddress]) -> Option<String> {
    addresses
        .iter()
        .find(|address| address.address_type == "ExternalIP")
        .map(|address| address.address.clone())
}

pub async fn mk_lib_network_kubernetes_worker_node_ips()
-> Result<Vec<String>, Box<dyn std::error::Error>> {
    let token = fs::read_to_string(K8S_SERVICE_ACCOUNT_TOKEN_PATH)?;
    let ca_cert_pem = fs::read(K8S_SERVICE_ACCOUNT_CA_PATH)?;
    let ca_cert = reqwest::Certificate::from_pem(&ca_cert_pem)?;

    let client = Client::builder().add_root_certificate(ca_cert).build()?;
    let response = client
        .get(format!("{K8S_API_SERVER}/api/v1/nodes"))
        .bearer_auth(token.trim())
        .send()
        .await?
        .error_for_status()?;

    let nodes = response.json::<KubernetesNodeList>().await?;

    let worker_node_ips = nodes
        .items
        .into_iter()
        .filter(|node| is_worker_node(&node.metadata.labels))
        .filter_map(|node| find_external_ip(&node.status.addresses))
        .collect();

    Ok(worker_node_ips)
}
