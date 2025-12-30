// Deep Disk Information Trait
// Defines interface for disk health details

use crate::hw::deep::common::DiskHealth;

/// Trait for deep disk information
pub trait DeepDiskInfo {
    /// Get disk firmware version
    fn get_firmware(&self) -> Option<String>;

    /// Get total bytes written (TBW)
    fn get_tbw(&self) -> Option<f64>;

    /// Get power-on hours
    fn get_power_hours(&self) -> Option<u64>;

    /// Get complete disk health information
    fn get_disk_health(&self) -> Option<DiskHealth>;
}
