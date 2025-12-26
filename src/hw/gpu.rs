// GPU detection module
// Provides cross-platform GPU detection using platform-specific implementations

mod common;
mod platform;

pub use common::GpuInfo;

impl GpuInfo {
    /// Detect GPU using platform-specific commands
    pub fn new() -> Vec<Self> {
        #[cfg(target_os = "macos")]
        return platform::macos::detect_gpus();

        #[cfg(target_os = "windows")]
        return platform::windows::detect_gpus();

        #[cfg(target_os = "linux")]
        return platform::linux::detect_gpus();

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        return vec![Self {
            model: "Unknown platform".to_string(),
            vram_gb: None,
            gpu_type: GpuType::Unknown,
        }];
    }
}
