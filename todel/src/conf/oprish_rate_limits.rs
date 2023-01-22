use serde::{Deserialize, Serialize};

use super::RateLimitConf;

/// Oprish rate limit config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OprishRateLimits {
    #[serde(default = "info_default")]
    pub info: RateLimitConf,
    #[serde(default = "message_create_default")]
    pub message_create: RateLimitConf,
    #[serde(default = "rate_limits_default")]
    pub rate_limits: RateLimitConf,
}

impl Default for OprishRateLimits {
    fn default() -> Self {
        Self {
            info: info_default(),
            message_create: message_create_default(),
            rate_limits: rate_limits_default(),
        }
    }
}

fn info_default() -> RateLimitConf {
    RateLimitConf {
        reset_after: 5,
        limit: 2,
    }
}

fn message_create_default() -> RateLimitConf {
    RateLimitConf {
        reset_after: 5,
        limit: 10,
    }
}

fn rate_limits_default() -> RateLimitConf {
    RateLimitConf {
        reset_after: 5,
        limit: 2,
    }
}
