//! A list of utilities for Eludris.

use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_timestamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Couldn't determine unix timestamp")
        .as_secs() as u32
}
