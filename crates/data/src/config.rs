// Echidna - Data

use crate::*;

pub struct Configuration {
    pub sample_size: usize,
    pub samples_per_heartbeat: u32,
    pub heartbeat_nsec: u64,
}
