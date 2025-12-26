// Linux GPU stress testing and metrics
// Uses sysfs for thermal information

use super::super::AppleGpuMetrics;

/// Get GPU metrics on Linux
/// Currently not implemented - returns None
pub fn get_apple_gpu_metrics() -> Option<AppleGpuMetrics> {
    // Linux doesn't have powermetrics/system_profiler
    // Could be extended with sysfs thermal queries in the future
    None
}
