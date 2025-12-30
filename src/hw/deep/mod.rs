// Deep Hardware Information Module
// Provides low-level hardware details via platform-specific probes

pub mod common;
pub mod cpu;
pub mod ram;
pub mod disk;
pub mod gpu;

pub mod platform;

// Re-export common types
#[allow(unused_imports)]  // Types reserved for future deep info features
pub use common::{
    CacheInfo, InstructionSets, DimmSlot, DiskHealth, GpuDriver, PcieLink,
    get_platform_probe, PlatformProbe,
};

// Re-export traits
pub use cpu::{DeepCpuInfo, estimate_tdp_from_model};
pub use ram::DeepRamInfo;
pub use disk::DeepDiskInfo;
pub use gpu::DeepGpuInfo;
