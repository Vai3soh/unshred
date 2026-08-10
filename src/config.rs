use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnshredConfig {
    pub bind_address: String,
    pub num_fec_workers: Option<u8>,
    pub num_batch_workers: Option<u8>,
    /// If non-empty, only UDP datagrams from these source IPs are accepted.
    /// Empty means accept from any source.
    #[serde(default)]
    pub allowed_sources: Vec<IpAddr>,
}

impl Default for UnshredConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:8001".to_string(),
            num_fec_workers: None,
            num_batch_workers: None,
            allowed_sources: Vec::new(),
        }
    }
}
