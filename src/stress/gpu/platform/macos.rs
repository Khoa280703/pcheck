// macOS GPU stress testing and metrics
// Uses powermetrics, system_profiler, and SMC

use std::process::Command;

use super::super::{ThermalPressure, AppleGpuMetrics};

// Temporary struct for system_profiler data
#[derive(Debug, Clone, Default)]
struct AppleGpuInfo {
    pub gpu_cores: Option<u32>,
    pub metal_version: Option<String>,
}

/// Get Apple Silicon GPU metrics from powermetrics
/// Requires sudo (verbose mode only)
/// Returns None if not available or command fails
pub fn get_apple_gpu_metrics() -> Option<AppleGpuMetrics> {
    let output = Command::new("powermetrics")
        .args(&["--samplers", "gpu_power,thermal", "-i", "1000", "-n", "1"])
        .output();

    let content = match output {
        Ok(result) if result.status.success() => String::from_utf8_lossy(&result.stdout).to_string(),
        _ => return None,
    };

    let mut metrics = AppleGpuMetrics::default();

    // Parse "GPU HW active frequency: 338 MHz"
    for line in content.lines() {
        let line = line.trim();
        if let Some(freq_str) = line.strip_prefix("GPU HW active frequency:")
            .or_else(|| line.strip_prefix("GPU HW active frequency:")) {
            // Extract number before "MHz"
            let parts: Vec<&str> = freq_str.split_whitespace().collect();
            if !parts.is_empty() {
                if let Ok(freq) = parts[0].parse::<u32>() {
                    metrics.frequency_mhz = Some(freq);
                }
            }
        }

        // Parse "GPU Power: 3 mW" or "GPU Power: 1948 mW"
        if let Some(power_str) = line.strip_prefix("GPU Power:") {
            let parts: Vec<&str> = power_str.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(power) = parts[0].parse::<u32>() {
                    metrics.power_mw = Some(power);
                }
            }
        }

        // Parse "GPU HW active residency:   0.93%"
        if line.contains("GPU HW active residency:") {
            if let Some(residency_str) = line.split("residency:").nth(1) {
                let pct_str = residency_str.trim().trim_end_matches('%');
                if let Ok(pct) = pct_str.parse::<f32>() {
                    metrics.residency_pct = Some(pct);
                }
            }
        }

        // Parse thermal data - SoC temperature
        // Format: "GPU-0  CPU 0  die temperature: 58 C"
        if line.contains("die temperature:") {
            if let Some(temp_str) = line.split("temperature:").nth(1) {
                let temp_str = temp_str.trim().trim_end_matches('C').trim();
                if let Ok(temp) = temp_str.parse::<f32>() {
                    metrics.temperature_c = Some(temp);
                }
            }
        }

        // Parse "Thermal pressure: Nominal"
        if line.contains("Thermal pressure:") {
            if let Some(pressure_str) = line.split("pressure:").nth(1) {
                let pressure = pressure_str.trim();
                metrics.thermal_pressure = Some(match pressure {
                    "Nominal" => ThermalPressure::Nominal,
                    "Moderate" => ThermalPressure::Moderate,
                    "Heavy" => ThermalPressure::Heavy,
                    "Trapping" => ThermalPressure::Trapping,
                    "Sleeping" => ThermalPressure::Sleeping,
                    _ => ThermalPressure::Unknown,
                });
            }
        }
    }

    // Try to get static GPU info from system_profiler
    if let Ok(gpu_info) = get_apple_gpu_info() {
        metrics.gpu_cores = gpu_info.gpu_cores;
        metrics.metal_version = gpu_info.metal_version;
    }

    // Try to get temperature from SMC (more accurate)
    if let Ok(smc_temp) = get_smc_temperature() {
        metrics.smc_temperature_c = Some(smc_temp);
    }

    // Return Some if we got at least one metric
    if metrics.frequency_mhz.is_some() || metrics.power_mw.is_some() || metrics.residency_pct.is_some()
        || metrics.temperature_c.is_some() || metrics.thermal_pressure.is_some()
        || metrics.smc_temperature_c.is_some() || metrics.gpu_cores.is_some() {
        Some(metrics)
    } else {
        None
    }
}

/// Get Apple Silicon static GPU info from system_profiler
/// No sudo required
fn get_apple_gpu_info() -> Result<AppleGpuInfo, String> {
    let output = Command::new("system_profiler")
        .args(&["SPDisplaysDataType"])
        .output();

    let content = match output {
        Ok(result) if result.status.success() => String::from_utf8_lossy(&result.stdout).to_string(),
        Ok(_) => return Err("Command failed".to_string()),
        Err(e) => return Err(format!("Failed to run: {}", e)),
    };

    let mut info = AppleGpuInfo::default();

    for line in content.lines() {
        let line = line.trim();

        // Parse "Total Number of Cores: 10"
        if line.contains("Total Number of Cores:") {
            if let Some(cores_str) = line.split("Cores:").nth(1) {
                if let Ok(cores) = cores_str.trim().parse::<u32>() {
                    info.gpu_cores = Some(cores);
                }
            }
        }

        // Parse "Metal: Supported, Metal Family: 3320" or similar
        if line.contains("Metal") {
            // Extract Metal version/family info
            if let Some(metal_str) = line.split("Metal:").nth(1) {
                let metal = metal_str.trim().to_string();
                if !metal.is_empty() && metal != "Not Supported" {
                    info.metal_version = Some(metal);
                }
            }
        }
    }

    Ok(info)
}

/// Get temperature from Apple SMC (System Management Controller)
/// Direct hardware reading, more accurate than powermetrics
#[cfg(feature = "apple-smc")]
fn get_smc_temperature() -> Result<f32, String> {
    use smc::SMC;

    let smc = SMC::new().map_err(|e| format!("SMC init failed: {:?}", e))?;

    // Common SMC keys for GPU/SoC temperature on Apple Silicon
    let candidate_keys = vec![
        "Tg0D",  // GPU die temperature (common on M1/M2)
        "Tg0f",  // GPU frequency/temperature
        "Tg05",  // GPU temperature 5
        "Tg00",  // GPU temperature 0
        "Tg10",  // GPU temperature 10
        "Tg11",  // GPU temperature 11
        "S0D0",  // SoC die temperature
        "SocD",  // SoC die temperature (M3)
        "Tp0D",  // P-cluster die temperature
        "Tp05",  // P-cluster temperature 5
        "TW0b",  // Airflow temperature
    ];

    for key in candidate_keys {
        if let Ok(temp) = smc.read_key::<f32>(key.into()) {
            // Valid temperature range: 10°C to 120°C
            if temp > 10.0 && temp < 120.0 {
                return Ok(temp);
            }
        }
    }

    Err("No valid temperature key found".to_string())
}

/// Stub for SMC temperature when feature is not enabled
#[cfg(not(feature = "apple-smc"))]
fn get_smc_temperature() -> Result<f32, String> {
    Err("SMC feature not enabled".to_string())
}
