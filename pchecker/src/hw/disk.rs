// Disk detection module

use sysinfo::Disks;

pub struct DiskInfo {
    pub name: String,
    pub total_gb: f64,
    pub kind: String,
}

impl DiskInfo {
    pub fn new() -> Vec<Self> {
        let disks = Disks::new_with_refreshed_list();

        disks
            .iter()
            .map(|disk| {
                let total_bytes = disk.total_space();
                Self {
                    name: disk.name().to_string_lossy().to_string(),
                    total_gb: total_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
                    kind: format!("{:?}", disk.kind()),
                }
            })
            .collect()
    }

    pub fn display(&self) -> String {
        format!("{} {:.0} GB", self.name, self.total_gb)
    }
}
