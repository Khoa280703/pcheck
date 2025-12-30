// Deep GPU Information Trait
// Defines interface for GPU driver and PCIe link details

use crate::hw::deep::common::{GpuDriver, PcieLink};

/// Trait for deep GPU information
pub trait DeepGpuInfo {
    /// Get GPU driver version
    #[allow(dead_code)]  // Reserved for future GPU driver features
    fn get_driver_version(&self) -> Option<String>;

    /// Get Metal version (macOS) / Driver version (Linux/Windows)
    fn get_metal_version(&self) -> Option<String>;

    /// Get PCIe link information
    #[allow(dead_code)]  // Reserved for future PCIe features
    fn get_pcie_link(&self) -> Option<PcieLink>;

    /// Get complete GPU driver information
    fn get_gpu_driver(&self) -> Option<GpuDriver>;
}
