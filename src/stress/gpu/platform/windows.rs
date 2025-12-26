// Windows GPU stress testing and metrics
// Uses WMI for thermal information

use super::super::AppleGpuMetrics;

/// Get GPU metrics on Windows
/// Currently not implemented - returns None
pub fn get_apple_gpu_metrics() -> Option<AppleGpuMetrics> {
    // Windows doesn't have powermetrics/system_profiler
    // Could be extended with WMI thermal queries in the future
    None
}
