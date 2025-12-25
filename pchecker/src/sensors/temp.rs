// CPU temperature reading using sysinfo Components
// Works on Linux, Windows (WMI), and macOS (x86 + Apple Silicon)

use sysinfo::Components;

/// CPU temperature reading
#[derive(Debug, Clone)]
pub struct CpuTemp {
    pub current: f32,
}

/// Get CPU temperature from sysinfo Components
/// On Apple Silicon, reads from PMU tdie components (CPU die temp)
/// Returns None if temperature not available
pub fn get_cpu_temp() -> Option<CpuTemp> {
    let components = Components::new_with_refreshed_list();

    // Try to find CPU temperature from components
    // Look for tdie (CPU die) or other CPU-related sensors
    let mut cpu_temps: Vec<f32> = Vec::new();

    for comp in components.iter() {
        let label = comp.label();
        let temp = comp.temperature()?;

        // Filter out invalid readings (negative temps on Apple Silicon)
        if temp < -1000.0 || temp > 150.0 {
            continue;
        }

        // Look for CPU-related temperature sensors
        let label_lower = label.to_lowercase();
        if label_lower.contains("cpu")
            || label_lower.contains("tdie")
            || label_lower.contains("core")
            || label_lower.contains("package")
            || label_lower.contains("tcal")  // Apple Silicon calibration
        {
            cpu_temps.push(temp);
        }
    }

    if cpu_temps.is_empty() {
        return None;
    }

    // Use the highest temperature from CPU sensors
    let temp = cpu_temps
        .into_iter()
        .reduce(|a, b| a.max(b))
        .unwrap();

    Some(CpuTemp { current: temp })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu_temp() {
        let temp = get_cpu_temp();
        if let Some(t) = temp {
            println!("CPU: {}°C", t.current);
            assert!(t.current > -100.0 && t.current < 150.0);
        }
        // May be None on some systems
    }
}
