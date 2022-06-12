//! A list of utilities for Eludris.

use std::time::{SystemTime, UNIX_EPOCH};

/// A function that returns the current UNIX timestamp with eludris' custom epoch.
pub fn now_timestamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Couldn't determine unix timestamp")
        .as_secs() as u32
        - 1_650_000_000
}
