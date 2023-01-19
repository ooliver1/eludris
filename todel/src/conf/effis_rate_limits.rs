use serde::{Deserialize, Deserializer, Serialize};
use ubyte::ByteUnit;

use super::RateLimitConf;

/// Effis rate limit data config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffisRateLimits {
    #[serde(default = "assets_default")]
    pub assets: EffisRateLimitConf,
    #[serde(default = "attachments_default")]
    pub attachments: EffisRateLimitConf,
    #[serde(default = "fetch_file_default")]
    pub fetch_file: RateLimitConf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffisRateLimitConf {
    pub reset_after: u32,
    pub limit: u32,
    #[serde(deserialize_with = "deserialize_file_size")]
    pub file_size_limit: u64,
}

fn assets_default() -> EffisRateLimitConf {
    EffisRateLimitConf {
        reset_after: 60,
        limit: 5,
        file_size_limit: 30_000_000, // 30MB
    }
}

fn attachments_default() -> EffisRateLimitConf {
    EffisRateLimitConf {
        reset_after: 180,
        limit: 20,
        file_size_limit: 500_000_000, // 500MB
    }
}

fn fetch_file_default() -> RateLimitConf {
    RateLimitConf {
        reset_after: 60,
        limit: 30,
    }
}

impl Default for EffisRateLimits {
    fn default() -> Self {
        Self {
            assets: assets_default(),
            attachments: attachments_default(),
            fetch_file: fetch_file_default(),
        }
    }
}

pub(crate) fn deserialize_file_size<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(ByteUnit::deserialize(deserializer)?.as_u64())
}
