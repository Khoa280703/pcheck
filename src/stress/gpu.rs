// GPU health check module
// Tests GPU by checking temperature and running thermal-based evaluation

mod platform;

use std::time::Duration;
use std::thread;
use sysinfo::Components;

use super::HealthStatus;
use super::gpu_compute::run_gpu_compute_stress_sync;

/// GPU temperature reading
#[derive(Debug, Clone)]
pub struct GpuTemp {
    pub current: f32,
}

/// Thermal pressure level from powermetrics
#[derive(Debug, Clone, PartialEq)]
pub enum ThermalPressure {
    Nominal,
    Moderate,
    Heavy,
    Trapping,
    Sleeping,
    Unknown,
}

/// Apple Silicon GPU metrics from powermetrics and SMC
#[derive(Debug, Clone, Default)]
pub struct AppleGpuMetrics {
    // From powermetrics
    pub frequency_mhz: Option<u32>,
    pub power_mw: Option<u32>,
    pub residency_pct: Option<f32>,
    pub temperature_c: Option<f32>,
    pub thermal_pressure: Option<ThermalPressure>,
    // From SMC (more accurate temperature)
    pub smc_temperature_c: Option<f32>,
    // Static info from system_profiler
    pub gpu_cores: Option<u32>,
    pub metal_version: Option<String>,
}

/// Get GPU temperature from sysinfo Components
/// Looks for GPU-related temperature sensors
/// Returns None if temperature not available
pub fn get_gpu_temp() -> Option<GpuTemp> {
    let components = Components::new_with_refreshed_list();

    // Try to find GPU temperature from components
    let mut gpu_temps: Vec<f32> = Vec::new();

    for comp in components.iter() {
        let label = comp.label();
        let temp = comp.temperature()?;

        // Filter out invalid readings
        if temp < -1000.0 || temp > 150.0 {
            continue;
        }

        // Look for GPU-related temperature sensors
        let label_lower = label.to_lowercase();
        if label_lower.contains("gpu")
            || label_lower.contains("graphic")
            || label_lower.contains("nvidia")
            || label_lower.contains("amd")
            || label_lower.contains("radeon")
            || label_lower.contains("intel")
            || label_lower.contains("gt")
            || label_lower.contains("pch")  // Platform Controller Hub (can include GPU)
        {
            gpu_temps.push(temp);
        }
    }

    if gpu_temps.is_empty() {
        return None;
    }

    // Use the highest temperature from GPU sensors
    let temp = gpu_temps
        .into_iter()
        .reduce(|a, b| a.max(b))
        .unwrap();

    Some(GpuTemp { current: temp })
}

pub struct GpuTestConfig {
    pub duration_secs: u64,
    pub verbose: bool,
}

impl Default for GpuTestConfig {
    fn default() -> Self {
        Self {
            duration_secs: 30,
            verbose: false,
        }
    }
}

pub struct GpuTestResult {
    // Hardware info
    pub gpu_model: String,
    pub gpu_type: String,
    pub vram_gb: Option<f64>,
    // Test metrics
    #[allow(dead_code)]
    pub temperature_start: Option<GpuTemp>,
    pub temperature_end: Option<GpuTemp>,
    pub temperature_max: Option<f32>,
    pub is_apple_silicon: bool,
    pub apple_gpu_metrics: Option<AppleGpuMetrics>,
    pub health: HealthStatus,
}

/// Run GPU health check
/// Uses wgpu compute shader for actual GPU load when available
/// Falls back to thermal monitoring only if compute is not available
pub fn run_stress_test(
    config: GpuTestConfig,
    gpu_model: String,
    gpu_type: String,
    vram_gb: Option<f64>,
) -> GpuTestResult {
    // Check if this is Apple Silicon (integrated GPU)
    let is_apple_silicon = gpu_type.contains("Integrated")
        && (gpu_model.contains("M1") || gpu_model.contains("M2")
            || gpu_model.contains("M3") || gpu_model.contains("M4"));

    // Get start temperature
    let temperature_start = get_gpu_temp();
    let mut temperature_max = temperature_start.as_ref().map(|t| t.current);

    // Try to run GPU compute stress test
    let compute_result = run_gpu_compute_stress_sync(config.duration_secs, true);

    if compute_result.is_err() {
        // Compute test failed - show error and fall back to thermal
        println!("   ⚠️  GPU compute unavailable");
        println!("   Falling back to thermal monitoring...");
    }

    // Verbose mode: additional thermal monitoring with platform-specific metrics
    use platform::*;
    let apple_gpu_metrics = if config.verbose && is_apple_silicon {
        // Try to get Apple GPU metrics from platform implementation
        get_apple_gpu_metrics()
    } else {
        None
    };

    // If compute failed, run thermal monitoring loop
    if compute_result.is_err() {
        for elapsed in 0..config.duration_secs {
            thread::sleep(Duration::from_secs(1));

            // Get current temperature
            if let Some(temp) = get_gpu_temp() {
                if temperature_max.is_none() || temp.current > temperature_max.unwrap() {
                    temperature_max = Some(temp.current);
                }

                // Print progress - same format as CPU
                let percent = ((elapsed + 1) * 100 / config.duration_secs) as u8;
                let filled = (percent as usize * 14 / 100).min(14);
                let empty = 14 - filled;
                let bar = format!("{}{}{}{}",
                    "\x1b[32m", "█".repeat(filled), "\x1b[90m", "░".repeat(empty));
                print!("\r⏳ GPU: [{}\x1b[0m] {}% | {:.1}°C",
                       bar, percent, temp.current);
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            } else if is_apple_silicon {
                // Apple Silicon: show platform metrics if available
                let percent = ((elapsed + 1) * 100 / config.duration_secs) as u8;
                let filled = (percent as usize * 14 / 100).min(14);
                let empty = 14 - filled;
                let bar = format!("{}{}{}{}",
                    "\x1b[32m", "█".repeat(filled), "\x1b[90m", "░".repeat(empty));

                // Refresh metrics every 5 seconds
                if elapsed % 5 == 0 || elapsed == 0 {
                    // Try to get fresh metrics
                    let fresh_metrics = get_apple_gpu_metrics();
                    if let Some(ref m) = fresh_metrics {
                        // Priority: SMC temp > powermetrics temp > thermal pressure > unknown
                        let temp_str = m.smc_temperature_c
                            .or(m.temperature_c)
                            .map(|t| format!("{:.0}°C", t))
                            .unwrap_or_else(|| {
                                m.thermal_pressure.as_ref().map_or("?".to_string(), |p| {
                                    match p {
                                        ThermalPressure::Nominal => "✅ Nominal".to_string(),
                                        ThermalPressure::Moderate => "⚠️ Moderate".to_string(),
                                        ThermalPressure::Heavy => "❌ Heavy".to_string(),
                                        ThermalPressure::Trapping => "🔥 Trapping".to_string(),
                                        ThermalPressure::Sleeping => "💤 Sleeping".to_string(),
                                        ThermalPressure::Unknown => "?".to_string(),
                                    }
                                })
                            });

                        // Update temperature_max from best available source
                        let best_temp = m.smc_temperature_c.or(m.temperature_c);
                        if let Some(t) = best_temp {
                            if temperature_max.is_none() || t > temperature_max.unwrap() {
                                temperature_max = Some(t);
                            }
                        }

                        let freq_str = m.frequency_mhz.map_or("?".to_string(), |f| format!("{}MHz", f));
                        let power_str = m.power_mw.map_or("?".to_string(), |p| format!("{}mW", p));
                        let usage_str = m.residency_pct.map_or("?".to_string(), |r| format!("{:.0}%", r));

                        // Build display string with available info
                        let mut parts = vec![temp_str, freq_str, power_str, usage_str];

                        // Add GPU cores if available
                        if let Some(cores) = m.gpu_cores {
                            parts.push(format!("{} cores", cores));
                        }

                        print!("\r⏳ GPU: [{}\x1b[0m] {}% | {}",
                               bar, percent, parts.join(" | "));
                        use std::io::Write;
                        std::io::stdout().flush().unwrap();
                    } else {
                        print!("\r⏳ GPU: [{}\x1b[0m] {}% | SoC (needs sudo)",
                               bar, percent);
                        use std::io::Write;
                        std::io::stdout().flush().unwrap();
                    }
                } else {
                    // Don't re-query powermetrics every second (too slow)
                    // Just update progress bar
                    print!("\r⏳ GPU: [{}\x1b[0m] {}%", bar, percent);
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                }
            } else {
                // No temperature sensor available
                let percent = ((elapsed + 1) * 100 / config.duration_secs) as u8;
                let filled = (percent as usize * 14 / 100).min(14);
                let empty = 14 - filled;
                let bar = format!("{}{}{}{}",
                    "\x1b[32m", "█".repeat(filled), "\x1b[90m", "░".repeat(empty));
                print!("\r⏳ GPU: [{}\x1b[0m] {}% | N/A (no sensor)",
                       bar, percent);
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
        }
        // Clear progress line after thermal monitoring (reset color first)
        print!("\x1b[0m\r\x1b[2K");  // Reset color, then clear line
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }

    // Get end temperature
    let temperature_end = get_gpu_temp();

    // Update max with end temp
    if let Some(ref end) = temperature_end {
        if temperature_max.is_none() || end.current > temperature_max.unwrap() {
            temperature_max = Some(end.current);
        }
    }

    // Determine health status based on temperature
    let health = evaluate_gpu_health(temperature_max, is_apple_silicon);

    GpuTestResult {
        gpu_model,
        gpu_type,
        vram_gb,
        temperature_start,
        temperature_end,
        temperature_max,
        is_apple_silicon,
        apple_gpu_metrics,
        health,
    }
}

/// Evaluate GPU health based on temperature
/// Thresholds: Warning ≥85°C, FAIL >95°C
/// Apple Silicon integrated GPUs: No separate GPU sensor (SoC thermal)
fn evaluate_gpu_health(max_temp: Option<f32>, is_apple_silicon: bool) -> HealthStatus {
    let mut issues = Vec::new();

    if let Some(temp) = max_temp {
        if temp > 95.0 {
            return HealthStatus::Failed(format!(
                "GPU overheating ({:.1}°C) - cooling system failure or defective GPU",
                temp
            ));
        } else if temp > 85.0 {
            issues.push(format!("GPU running hot ({:.1}°C) - check cooling system", temp));
        }
    } else if is_apple_silicon {
        // Apple Silicon: No separate GPU temp sensor is expected
        // Don't add warning - GPU shares SoC thermal with CPU
        return HealthStatus::Healthy;
    } else {
        // Other platforms: No temperature sensor is a problem
        issues.push("GPU temperature sensor not available - unable to verify thermal status".to_string());
    }

    if !issues.is_empty() {
        HealthStatus::IssuesDetected(issues)
    } else {
        HealthStatus::Healthy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_gpu_temp() {
        let temp = get_gpu_temp();
        if let Some(t) = temp {
            println!("GPU: {}°C", t.current);
            assert!(t.current > -100.0 && t.current < 150.0);
        }
        // May be None on systems without GPU sensors
    }

    #[test]
    fn test_evaluate_gpu_health() {
        // Healthy - normal temperature
        assert!(matches!(evaluate_gpu_health(Some(70.0), false), HealthStatus::Healthy));
        assert!(matches!(evaluate_gpu_health(Some(80.0), false), HealthStatus::Healthy));

        // Issues detected - hot temp (>85°C)
        assert!(matches!(evaluate_gpu_health(Some(90.0), false), HealthStatus::IssuesDetected(_)));

        // Failed - overheating (>95°C)
        assert!(matches!(evaluate_gpu_health(Some(100.0), false), HealthStatus::Failed(_)));

        // No sensor - issues detected (non-Apple Silicon)
        assert!(matches!(evaluate_gpu_health(None, false), HealthStatus::IssuesDetected(_)));

        // No sensor - healthy on Apple Silicon (expected behavior)
        assert!(matches!(evaluate_gpu_health(None, true), HealthStatus::Healthy));
    }

    #[test]
    fn test_gpu_test_short() {
        let config = GpuTestConfig {
            duration_secs: 1,
            verbose: false,
        };
        let result = run_stress_test(
            config,
            "Test GPU".to_string(),
            "Integrated".to_string(),
            Some(8.0),
        );

        assert_eq!(result.gpu_model, "Test GPU");
        assert_eq!(result.gpu_type, "Integrated");
        assert_eq!(result.vram_gb, Some(8.0));
    }
}
