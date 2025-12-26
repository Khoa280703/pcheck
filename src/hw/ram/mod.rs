// RAM detection module

use sysinfo::System;

pub struct RamInfo {
    pub total_gb: f64,
    pub used_gb: f64,
}

impl RamInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_memory();

        let total = sys.total_memory();
        let used = sys.used_memory();

        Self {
            total_gb: total as f64 / 1024.0 / 1024.0 / 1024.0,
            used_gb: used as f64 / 1024.0 / 1024.0 / 1024.0,
        }
    }
}
