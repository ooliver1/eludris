use serde::{Deserialize, Serialize};

/// The instance info payload
#[cfg(not(feature = "logic"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub instance_name: String,
    pub description: Option<String>,
    pub version: String,
    pub message_limit: usize,
    pub oprish_url: String,
    pub pandemonium_url: String,
    pub effis_url: String,
    pub file_size: u64,
    pub attachment_file_size: u64,
}

/// The instance info payload
#[cfg(feature = "logic")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceInfo<'a> {
    pub instance_name: String,
    pub description: Option<String>,
    pub version: &'a str,
    pub message_limit: usize,
    pub oprish_url: &'a str,
    pub pandemonium_url: &'a str,
    pub effis_url: &'a str,
    pub file_size: u64,
    pub attachment_file_size: u64,
}
