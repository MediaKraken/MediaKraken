// Garage admin API client.
// Reference: https://garagehq.deuxfleurs.fr/api-docs/admin-v1.html
//
// The admin API is separate from the S3 API and is bound to its own port
// (`api_bind_addr` under `[admin]` in garage.toml, default 3903). It uses a
// bearer token (`admin_token`) for authentication.

use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

/// Client for the Garage admin API.
#[derive(Debug, Clone)]
pub struct GarageAdminClient {
    base_url: String,
    token: String,
    http: Client,
}

/// Aggregated free/total bytes across all storage nodes in the cluster.
///
/// Note: these are *physical* bytes on the underlying disks. Logical capacity
/// for the cluster is roughly `data_total / replication_factor`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ClusterFreeSpace {
    pub data_available: u64,
    pub data_total: u64,
    pub metadata_available: u64,
    pub metadata_total: u64,
}

/// Per-bucket usage as reported by the admin API.
#[derive(Debug, Clone, Copy, Default)]
pub struct BucketUsage {
    pub bytes: u64,
    pub objects: u64,
    pub unfinished_uploads: u64,
}

/// How to look up a bucket on the admin API.
#[derive(Debug, Clone)]
pub enum BucketRef<'a> {
    Id(&'a str),
    GlobalAlias(&'a str),
}

#[derive(Debug, Deserialize)]
struct ClusterStatus {
    #[serde(default)]
    nodes: Vec<NodeStatus>,
}

#[derive(Debug, Deserialize)]
struct NodeStatus {
    #[serde(default, rename = "dataPartition")]
    data_partition: Option<Partition>,
    #[serde(default, rename = "metadataPartition")]
    metadata_partition: Option<Partition>,
}

#[derive(Debug, Deserialize)]
struct Partition {
    available: u64,
    total: u64,
}

#[derive(Debug, Deserialize)]
struct BucketInfo {
    #[serde(default)]
    bytes: u64,
    #[serde(default)]
    objects: u64,
    #[serde(default, rename = "unfinishedUploads")]
    unfinished_uploads: u64,
}

impl GarageAdminClient {
    /// Build a client.
    ///
    /// `base_url` is the admin API root, e.g. `http://garage-admin.garage:3903`.
    /// `token` is the `admin_token` from `garage.toml`.
    pub fn new(
        base_url: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let http = Client::builder().build()?;
        let base = base_url.into().trim_end_matches('/').to_string();
        Ok(Self {
            base_url: base,
            token: token.into(),
            http,
        })
    }

    /// Sum free/total bytes across all storage nodes.
    pub async fn cluster_free_space(&self) -> Result<ClusterFreeSpace, Box<dyn Error>> {
        let url = format!("{}/v1/status", self.base_url);
        let status: ClusterStatus = self
            .http
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let mut out = ClusterFreeSpace::default();
        for node in status.nodes {
            if let Some(p) = node.data_partition {
                out.data_available = out.data_available.saturating_add(p.available);
                out.data_total = out.data_total.saturating_add(p.total);
            }
            if let Some(p) = node.metadata_partition {
                out.metadata_available = out.metadata_available.saturating_add(p.available);
                out.metadata_total = out.metadata_total.saturating_add(p.total);
            }
        }
        Ok(out)
    }

    /// Fetch bytes/objects/unfinished-uploads for a single bucket.
    pub async fn bucket_usage(&self, bucket: BucketRef<'_>) -> Result<BucketUsage, Box<dyn Error>> {
        let url = format!("{}/v1/bucket", self.base_url);
        let query: [(&str, &str); 1] = match bucket {
            BucketRef::Id(id) => [("id", id)],
            BucketRef::GlobalAlias(alias) => [("globalAlias", alias)],
        };
        let info: BucketInfo = self
            .http
            .get(&url)
            .bearer_auth(&self.token)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(BucketUsage {
            bytes: info.bytes,
            objects: info.objects,
            unfinished_uploads: info.unfinished_uploads,
        })
    }
}
