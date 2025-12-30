// Common structs for deep hardware information
// Shared across all platforms

use serde::{Serialize, Deserialize};
use crate::hw::deep::{cpu::DeepCpuInfo, ram::DeepRamInfo, disk::DeepDiskInfo, gpu::DeepGpuInfo};

/// CPU cache information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub l1_kb: Option<u32>,
    pub l2_kb: Option<u32>,
    pub l3_kb: Option<u32>,
}

/// CPU instruction sets / features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionSets {
    pub features: Vec<String>,
}

/// RAM DIMM slot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimmSlot {
    pub id: usize,
    pub bank: String,
    pub size_gb: f64,
    #[serde(rename = "type")]
    pub type_: String,
    pub speed_mhz: Option<u32>,
    pub manufacturer: Option<String>,
    pub part_number: Option<String>,
}

/// Disk health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskHealth {
    pub status: String,
    pub firmware: Option<String>,
    pub tbw: Option<f64>,
    pub hours: Option<u64>,
    pub percentage_used: Option<u8>,
}

/// GPU driver information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDriver {
    pub version: Option<String>,
    pub metal: Option<String>,
}

/// PCIe link information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]  // Reserved for future PCIe features
pub struct PcieLink {
    pub link_speed: String,
    pub generation: u8,
}

/// Platform probe type
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]  // Linux/Windows/Unknown reserved for future platforms
pub enum PlatformProbe {
    MacOs,
    Linux,
    Windows,
    Unknown,
}

/// Platform selector - returns appropriate probe for current platform
pub fn get_platform_probe() -> PlatformProbe {
    #[cfg(target_os = "macos")]
    return PlatformProbe::MacOs;

    #[cfg(target_os = "linux")]
    return PlatformProbe::Linux;

    #[cfg(target_os = "windows")]
    return PlatformProbe::Windows;

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    return PlatformProbe::Unknown;
}

// Delegate trait methods to platform-specific implementations
impl PlatformProbe {
    pub fn get_cache_info(&self) -> Option<CacheInfo> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_cache_info()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    pub fn get_instruction_sets(&self) -> Option<InstructionSets> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_instruction_sets()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    pub fn get_tdp(&self, model: &str) -> Option<u32> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_tdp(model)
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    pub fn get_dimm_slots(&self) -> Vec<DimmSlot> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_dimm_slots()
        }
        #[cfg(not(target_os = "macos"))]
        { vec![] }
    }

    #[allow(dead_code)]  // Reserved for future disk health features
    pub fn get_firmware(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_firmware()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    #[allow(dead_code)]  // Reserved for future disk health features
    pub fn get_tbw(&self) -> Option<f64> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_tbw()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    #[allow(dead_code)]  // Reserved for future disk health features
    pub fn get_power_hours(&self) -> Option<u64> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_power_hours()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    pub fn get_disk_health(&self) -> Option<DiskHealth> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_disk_health()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    #[allow(dead_code)]  // Reserved for future GPU driver features
    pub fn get_driver_version(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_driver_version()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    #[allow(dead_code)]  // Reserved for future Metal version parsing
    pub fn get_metal_version(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_metal_version()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    #[allow(dead_code)]  // Reserved for future PCIe features
    pub fn get_pcie_link(&self) -> Option<PcieLink> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_pcie_link()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }

    pub fn get_gpu_driver(&self) -> Option<GpuDriver> {
        #[cfg(target_os = "macos")]
        {
            use crate::hw::deep::platform::macos::MacOsDeepProbe;
            let probe = MacOsDeepProbe;
            probe.get_gpu_driver()
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }
}
