use reqwest::Client;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize)]
struct KubernetesCnpgBackupList {
    #[serde(default)]
    items: Vec<KubernetesCnpgBackup>,
}

#[derive(Debug, Deserialize)]
struct KubernetesCnpgBackup {
    metadata: KubernetesCnpgBackupMetadata,
    #[serde(default)]
    spec: KubernetesCnpgBackupSpec,
    #[serde(default)]
    status: KubernetesCnpgBackupStatus,
}

#[derive(Debug, Deserialize)]
struct KubernetesCnpgBackupMetadata {
    name: String,
    #[serde(default)]
    namespace: String,
    #[serde(default, rename = "creationTimestamp")]
    creation_timestamp: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct KubernetesCnpgBackupSpec {
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    cluster: Option<KubernetesCnpgBackupCluster>,
}

#[derive(Debug, Deserialize)]
struct KubernetesCnpgBackupCluster {
    #[serde(default)]
    name: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct KubernetesCnpgBackupStatus {
    #[serde(default)]
    phase: Option<String>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default, rename = "startedAt")]
    started_at: Option<String>,
    #[serde(default, rename = "stoppedAt")]
    stopped_at: Option<String>,
    #[serde(default, rename = "backupId")]
    backup_id: Option<String>,
    #[serde(default, rename = "destinationPath")]
    destination_path: Option<String>,
}

/// A CloudNativePG backup as reported by the Kubernetes API
/// (apis/postgresql.cnpg.io/v1/namespaces/{namespace}/backups).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CnpgBackup {
    pub name: String,
    pub namespace: String,
    pub creation_timestamp: Option<String>,
    pub cluster_name: Option<String>,
    pub method: Option<String>,
    pub phase: Option<String>,
    pub error: Option<String>,
    pub started_at: Option<String>,
    pub stopped_at: Option<String>,
    pub backup_id: Option<String>,
    pub destination_path: Option<String>,
}

/// List the CloudNativePG backups in the given namespace.
pub async fn mk_lib_network_kubernetes_cnpg_backups(
    namespace: &str,
) -> Result<Vec<CnpgBackup>, Box<dyn std::error::Error>> {
    let token = fs::read_to_string(K8S_SERVICE_ACCOUNT_TOKEN_PATH)?;
    let ca_cert_pem = fs::read(K8S_SERVICE_ACCOUNT_CA_PATH)?;
    let ca_cert = reqwest::Certificate::from_pem(&ca_cert_pem)?;

    let client = Client::builder().add_root_certificate(ca_cert).build()?;
    let response = client
        .get(format!(
            "{K8S_API_SERVER}/apis/postgresql.cnpg.io/v1/namespaces/{namespace}/backups"
        ))
        .bearer_auth(token.trim())
        .send()
        .await?
        .error_for_status()?;

    let list = response.json::<KubernetesCnpgBackupList>().await?;

    let backups = list
        .items
        .into_iter()
        .map(|backup| CnpgBackup {
            name: backup.metadata.name,
            namespace: backup.metadata.namespace,
            creation_timestamp: backup.metadata.creation_timestamp,
            cluster_name: backup.spec.cluster.and_then(|cluster| cluster.name),
            method: backup.spec.method,
            phase: backup.status.phase,
            error: backup.status.error,
            started_at: backup.status.started_at,
            stopped_at: backup.status.stopped_at,
            backup_id: backup.status.backup_id,
            destination_path: backup.status.destination_path,
        })
        .collect();

    Ok(backups)
}
