// Disk detection module

use sysinfo::Disks;

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

        disks
            .iter()
            .map(|disk| {
                let total_bytes = disk.total_space();
                let available_bytes = disk.available_space();
                let used_bytes = total_bytes.saturating_sub(available_bytes);

                Self {
                    name: disk.name().to_string_lossy().to_string(),
                    total_gb: total_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
                    used_gb: used_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
                    available_gb: available_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                }
            })
            .collect()
    }

    pub fn display(&self) -> String {
        format!("{} {:.0} GB", self.name, self.total_gb)
    }
}
