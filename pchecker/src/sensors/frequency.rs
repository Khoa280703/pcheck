// CPU frequency reading using sysinfo
// Cross-platform support for Linux, macOS, Windows

use sysinfo::{System, RefreshKind, CpuRefreshKind};

/// CPU frequency reading
#[derive(Debug, Clone)]
pub struct CpuFrequency {
    pub current_mhz: u64,
    pub current_ghz: f64,
    pub cores: usize,
}

/// Get current CPU frequency from sysinfo
pub fn get_cpu_frequency() -> CpuFrequency {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
    );
    sys.refresh_cpu_usage();

    let cpus = sys.cpus();
    let cores = cpus.len();

    if cores == 0 {
        return CpuFrequency {
            current_mhz: 0,
            current_ghz: 0.0,
            cores: 0,
        };
    }

    // Get current frequency (average of all cores)
    let total_mhz: u64 = cpus.iter().map(|c| c.frequency()).sum();
    let current_mhz = total_mhz / cores as u64;
    let current_ghz = current_mhz as f64 / 1000.0;

    CpuFrequency {
        current_mhz,
        current_ghz,
        cores,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu_frequency() {
        let freq = get_cpu_frequency();
        println!("CPU: {} cores, {} MHz ({:.2} GHz)",
            freq.cores, freq.current_mhz, freq.current_ghz);
        assert!(freq.cores > 0);
        assert!(freq.current_mhz > 0);
    }
}
