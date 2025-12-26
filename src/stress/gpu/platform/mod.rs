// Platform-specific GPU stress testing modules

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

// Re-export platform functions for use in parent module
#[cfg(target_os = "macos")]
pub use macos::get_apple_gpu_metrics;

// For non-macOS, provide stub implementation
#[cfg(not(target_os = "macos"))]
pub use self::stub::get_apple_gpu_metrics;

// Stub implementations for non-macOS platforms
mod stub {
    use crate::stress::gpu::AppleGpuMetrics;

    #[allow(dead_code)]
    pub fn get_apple_gpu_metrics() -> Option<AppleGpuMetrics> {
        None
    }
}
