// Disk detection module

use sysinfo::Disks;
use std::collections::HashSet;

#[derive(Clone)]
pub struct DiskInfo {
    pub name: String,
    pub total_gb: f64,
    pub used_gb: f64,
    pub available_gb: f64,
    pub mount_point: String,
}

impl DiskInfo {
    pub fn new() -> Vec<Self> {
        let disks = Disks::new_with_refreshed_list();

        // Deduplicate by (name + total_space) combination
        // On macOS APFS, / and /System/Volumes/Data are same physical disk
        let mut seen: HashSet<(String, u64)> = HashSet::new();
        let mut result = Vec::new();

        for disk in disks.iter() {
            let total_bytes = disk.total_space();
            let name = disk.name().to_string_lossy().to_string();
            let key = (name.clone(), total_bytes);

            if seen.insert(key) {
                // First time seeing this disk
                let available_bytes = disk.available_space();
                let used_bytes = total_bytes.saturating_sub(available_bytes);

                result.push(Self {
                    name,
                    total_gb: total_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
                    used_gb: used_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
                    available_gb: available_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                });
            }
        }

        result
    }

    pub fn display(&self) -> String {
        format!("{} {:.0} GB", self.name, self.total_gb)
    }
}
