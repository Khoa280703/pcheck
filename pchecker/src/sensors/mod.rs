// Sensors module - CPU temperature and frequency monitoring
// Uses sysinfo crate for cross-platform support

pub mod temp;
pub mod frequency;
pub mod monitor;

pub use temp::{CpuTemp, get_cpu_temp, get_all_sensors};
pub use frequency::{CpuFrequency, get_cpu_frequency};
pub use monitor::CpuMonitorHandle;
