use serde::{Deserialize, Serialize};

/// The message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub author: String,
    pub content: String,
}
