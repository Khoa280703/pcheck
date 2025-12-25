// CPU detection module

use sysinfo::{System, CpuRefreshKind, RefreshKind};

pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
}

impl CpuInfo {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::everything().with_cpu(CpuRefreshKind::everything())
        );
        sys.refresh_cpu_all();

        let cpus = sys.cpus();
        let first_cpu = cpus.first();

        Self {
            model: first_cpu
                .map(|c| c.brand().to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
            cores: cpus.len(),
        }
    }
}
