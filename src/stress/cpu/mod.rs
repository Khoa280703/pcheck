// CPU health check module
// Tests CPU by running intensive calculations on all cores

mod platform;

use std::thread;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::io::{self, Write};
use std::collections::HashMap;

use super::HealthStatus;
use crate::sensors::{CpuTemp, CpuFrequency, get_cpu_temp, get_cpu_frequency, CpuMonitorHandle, get_all_sensors};
use crate::fmt::{RESET, CYAN, temp_color, temp_status, format_large_number, progress_bar};

pub struct CpuTestConfig {
    pub duration_secs: u64,
    pub thread_count: Option<usize>,
    pub verbose: bool,
    // AI commentary callbacks (optional, for real-time comments)
    pub on_comment: Option<Box<dyn Fn(&str) + Send>>,
}

impl Default for CpuTestConfig {
    fn default() -> Self {
        Self {
            duration_secs: 60,
            thread_count: None,
            verbose: false,
            on_comment: None,
        }
    }
}

pub struct CpuTestResult {
    // Hardware info
    pub cpu_model: String,
    pub cpu_cores: usize,
    // Test metrics
    pub operations: u64,
    pub ops_per_second: f64,
    pub avg_op_time_ms: f64,
    pub variance_pct: f64,
    pub temperature: Option<CpuTemp>,
    pub frequency_start: CpuFrequency,
    pub frequency_end: CpuFrequency,
    pub freq_drop_pct: f64,
    pub health: HealthStatus,
}

/// CPU workload intensity: number of primes to calculate per iteration
/// Higher values = more CPU-intensive test
const CPU_PRIME_WORKLOAD: usize = 10000;

/// Run CPU health check
/// Spawns threads equal to logical CPU cores and runs intensive calculations
pub fn run_stress_test(config: CpuTestConfig, cpu_model: String, cpu_cores: usize) -> CpuTestResult {
    let thread_count = config.thread_count.unwrap_or(cpu_cores);
    let running = Arc::new(AtomicBool::new(true));

    // Start background CPU usage monitor
    let monitor = CpuMonitorHandle::start();

    // Capture start frequency
    let frequency_start = get_cpu_frequency();

    // Shared counter for total operations (for progress display)
    let total_ops = Arc::new(AtomicU64::new(0));

    // Clone callback for use in loop
    let comment_callback = config.on_comment;

    // Spawn worker threads
    let threads: Vec<_> = (0..thread_count)
        .map(|_| {
            let running = Arc::clone(&running);
            let total_ops = Arc::clone(&total_ops);
            thread::spawn(move || {
                let mut ops = 0u64;
                let mut times = Vec::new();

                while running.load(Ordering::Relaxed) {
                    let start = Instant::now();

                    // CPU-intensive work: calculate primes
                    calculate_primes(CPU_PRIME_WORKLOAD);

                    let elapsed = start.elapsed().as_micros() as f64;
                    times.push(elapsed);
                    ops += 1;
                    total_ops.fetch_add(1, Ordering::Relaxed);
                }

                (ops, times)
            })
        })
        .collect();

    // Run for specified duration with progress updates
    for elapsed in 0..config.duration_secs {
        thread::sleep(Duration::from_secs(1));

        // Get current stats for progress display
        let ops = total_ops.load(Ordering::Relaxed);
        let temp = get_cpu_temp();
        let freq = get_cpu_frequency();
        let cpu_usage = monitor.get_per_core_usage();

        // AI commentary based on temperature (every 10 seconds or at start)
        if let Some(ref callback) = comment_callback {
            if elapsed == 0 || elapsed % 10 == 0 {
                if let Some(ref t) = temp {
                    if t.current > 80.0 {
                        callback(&format!("CPU temperature at {:.0}°C - running hot", t.current));
                    } else if t.current > 60.0 {
                        callback(&format!("CPU temperature at {:.0}°C - warming up nicely", t.current));
                    }
                }
            }
        }

        // Print progress box (overwrites previous)
        // Track if first iteration to avoid moving cursor up before first print
        let is_first = elapsed == 0;
        print_cpu_progress_box(
            elapsed + 1,
            config.duration_secs,
            ops,
            &temp,
            &freq,
            &cpu_usage,
            is_first,
            config.verbose,
        );
    }

    // Stop test
    running.store(false, Ordering::Relaxed);

    // Clear the progress lines before showing results
    // Normal mode: 1 line, Verbose mode: varies based on core count
    let lines_to_clear = if config.verbose {
        // Main line + per-core rows + sensor section (max 4 sensors + 1 header + 1 blank)
        let freq = get_cpu_frequency();
        use platform::cores_per_row_verbose;
        let core_rows = freq.cores.div_ceil(cores_per_row_verbose());
        1 + core_rows + 6 // +6 for sensor section (blank + header + max 4 sensors)
    } else {
        1 // Normal mode: only 1 line
    };

    for _ in 0..lines_to_clear {
        print!("\r\x1b[2K");  // Clear line
        print!("\x1b[1A");     // Move up
    }
    print!("\r\x1b[2K");  // Clear first line
    print!("\x1b[0m");     // Reset all colors
    io::stdout().flush().unwrap();

    // Capture end frequency
    let frequency_end = get_cpu_frequency();

    // Get temperature after stress test
    let temperature = get_cpu_temp();

    // Collect results from all threads
    let mut all_ops = 0u64;
    let mut all_times: Vec<f64> = Vec::new();
    let mut completed = true;

    for t in threads {
        if let Ok((ops, times)) = t.join() {
            all_ops += ops;
            all_times.extend(times);
        } else {
            // Thread panicked = CPU crash
            completed = false;
        }
    }

    // Calculate metrics
    let ops_per_second = all_ops as f64 / config.duration_secs as f64;
    let avg_time = if all_times.is_empty() {
        0.0
    } else {
        all_times.iter().sum::<f64>() / all_times.len() as f64
    };
    let variance = calculate_variance(&all_times, avg_time);

    // Calculate frequency drop percentage
    let freq_drop_pct = if frequency_start.current_mhz > 0 {
        ((frequency_start.current_mhz.saturating_sub(frequency_end.current_mhz)) as f64
            / frequency_start.current_mhz as f64) * 100.0
    } else {
        0.0
    };

    // Determine health status
    let health = evaluate_cpu_health(
        completed,
        variance,
        temperature.as_ref(),
        freq_drop_pct,
    );

    CpuTestResult {
        cpu_model,
        cpu_cores,
        operations: all_ops,
        ops_per_second,
        avg_op_time_ms: avg_time / 1000.0,
        variance_pct: variance,
        temperature,
        frequency_start,
        frequency_end,
        freq_drop_pct,
        health,
    }
}

/// Print the animated progress box for CPU test
/// Shows multi-line per-core display with platform-specific formatting
fn print_cpu_progress_box(
    elapsed: u64,
    total: u64,
    ops: u64,
    temp: &Option<CpuTemp>,
    freq: &CpuFrequency,
    cpu_usage: &HashMap<usize, f32>,
    is_first: bool,
    verbose: bool,
) {
    let percent = ((elapsed * 100) / total) as u8;
    let bar = progress_bar(percent, 14);

    // Get temperature values
    let temp_val = temp.as_ref().map(|t| t.current).unwrap_or(0.0);
    let temp_str = if let Some(t) = temp {
        format!("{:.0}°C", t.current)
    } else {
        "N/A".to_string()
    };
    let temp_color_code = if temp_val > 0.0 { temp_color(temp_val) } else { RESET };
    let temp_status_text = if temp_val > 0.0 { temp_status(temp_val) } else { "" };

    // Format operations
    let ops_str = format_large_number(ops);

    // Build per-core rows based on platform
    let cores = freq.cores;
    let per_core_rows = build_per_core_display(freq, cpu_usage, cores, verbose);

    if verbose {
        // === VERBOSE MODE ===
        // Calculate lines to move back (main line + core rows)
        let line_count = per_core_rows.len() + 1;

        // Move cursor up to overwrite previous output (not on first iteration)
        if !is_first {
            for _ in 0..line_count {
                print!("\x1b[1A"); // Move up one line
            }
        }

        // Main progress line
        let temp_display = format!("{}{}{} ({}{})", temp_color_code, temp_str, RESET, temp_color_code, temp_status_text);
        println!("⏳ CPU: [{}] {}% | {} ops | {} | {:.2} GHz",
              bar, percent, ops_str, temp_display, freq.current_ghz);

        // Per-core rows with detailed format
        for row in &per_core_rows {
            println!("{}", row);
        }

        // Sensor list section (update every 5 seconds to avoid flicker)
        if elapsed.is_multiple_of(5) || elapsed == total {
            let sensors = get_all_sensors();
            if !sensors.is_empty() {
                println!();
                println!("🌡️  Sensors:");
                for sensor in sensors.iter().take(8) { // Limit to 8 sensors
                    let s_temp = sensor.temp;
                    let s_color = temp_color(s_temp);
                    println!("   • {}{}{}: {}{:.1}°C{}",
                        CYAN, sensor.label, RESET, s_color, s_temp, RESET);
                }
            }
        }
    } else {
        // === NORMAL MODE ===
        // Use \r to return to start of line, then print (no cursor-up needed)
        let temp_display = format!("{}{}{} ({}{})", temp_color_code, temp_str, RESET, temp_color_code, temp_status_text);
        print!("\r⏳ CPU: [{}] {}% | {} ops | {} | {:.2} GHz",
              bar, percent, ops_str, temp_display, freq.current_ghz);
    }

    io::stdout().flush().unwrap();
}

/// Build per-core display rows
/// Platform-specific: macOS shows usage %, Win/Linux shows usage %@frequency
/// Uses real-time CPU usage from background monitor
/// Verbose mode: Shows detailed bar chart with usage + frequency
fn build_per_core_display(
    _freq: &CpuFrequency,
    cpu_usage: &HashMap<usize, f32>,
    cores: usize,
    verbose: bool,
) -> Vec<String> {
    use platform::{cores_per_row_verbose, cores_per_row_normal, format_core_display_verbose, format_core_display_normal};

    let mut rows = Vec::new();

    if verbose {
        // === VERBOSE MODE: Detailed format with bars ===
        let cores_per_row = cores_per_row_verbose();

        for chunk_start in (0..cores).step_by(cores_per_row) {
            let chunk_end = (chunk_start + cores_per_row).min(cores);
            let mut row = String::new();

            for i in chunk_start..chunk_end {
                let display_usage = cpu_usage.get(&i).copied().unwrap_or(0.0);
                let usage_int = display_usage as u32;

                // Create usage bar (10 chars wide)
                let bar_filled = (usage_int * 10 / 100).min(10) as usize;

                // Format: C00: [████░░░░░░] 95% @4.2GHz
                #[cfg(target_os = "macos")]
                let core_str = format_core_display_verbose(i, usage_int, bar_filled);

                #[cfg(not(target_os = "macos"))]
                let core_str = {
                    let base_str = format_core_display_verbose(i, usage_int, bar_filled);
                    let core_mhz = freq.per_core_mhz.get(&i).copied().unwrap_or(0);
                    let core_ghz = core_mhz as f64 / 1000.0;
                    format!("{} @{:.1}GHz", base_str, core_ghz)
                };

                row.push_str(&core_str);
                if i < chunk_end - 1 {
                    row.push(' '); // Space between cores
                }
            }
            rows.push(row);
        }
    } else {
        // === NORMAL MODE: Compact format ===
        let cores_per_row = cores_per_row_normal();

        for chunk_start in (0..cores).step_by(cores_per_row) {
            let chunk_end = (chunk_start + cores_per_row).min(cores);
            let mut row = String::new();
            for i in chunk_start..chunk_end {
                let display_usage = cpu_usage.get(&i).copied().unwrap_or(0.0);
                row.push_str(&format_core_display_normal(i, display_usage));
            }
            rows.push(row.trim().to_string());
        }
    }

    rows
}

/// Evaluate CPU health based on test results
fn evaluate_cpu_health(
    completed: bool,
    variance: f64,
    temperature: Option<&crate::sensors::CpuTemp>,
    freq_drop_pct: f64,
) -> HealthStatus {
    let mut issues = Vec::new();

    // Critical: test crashed = actual hardware fault
    if !completed {
        return HealthStatus::Failed("CPU crashed during test - FAULTY HARDWARE".to_string());
    }

    // Check temperature - from Check.md: > 95°C = FAIL
    if let Some(temp) = temperature {
        if temp.current > 95.0 {
            return HealthStatus::Failed(format!(
                "CPU overheating ({:.1}°C) - cooling system failure",
                temp.current
            ));
        } else if temp.current > 85.0 {
            issues.push(format!("CPU running hot ({:.1}°C) - check cooling", temp.current));
        }
    }

    // Frequency throttling warning (>10% drop)
    if freq_drop_pct > 10.0 {
        issues.push(format!("CPU throttled by {:.1}% - possible thermal or power limit", freq_drop_pct));
    }

    // Only extreme variance (>200%) suggests possible CPU fault
    if variance > 200.0 {
        return HealthStatus::Failed(format!(
            "Extreme instability detected (variance: {:.1}%) - possible CPU fault",
            variance
        ));
    }

    // Return issues or healthy
    if !issues.is_empty() {
        HealthStatus::IssuesDetected(issues)
    } else {
        HealthStatus::Healthy
    }
}

/// Calculate first n prime numbers (CPU-intensive work)
fn calculate_primes(n: usize) -> usize {
    let mut count = 0;
    let mut num = 2;
    while count < n {
        if is_prime(num) {
            count += 1;
        }
        num += 1;
    }
    count
}

/// Check if a number is prime
fn is_prime(n: usize) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n.is_multiple_of(2) { return false; }
    let sqrt = (n as f64).sqrt() as usize;
    for i in (3..=sqrt).step_by(2) {
        if n.is_multiple_of(i) { return false; }
    }
    true
}

/// Calculate coefficient of variation (std dev / mean * 100)
fn calculate_variance(times: &[f64], mean: f64) -> f64 {
    if times.is_empty() || mean == 0.0 {
        return 0.0;
    }

    let variance = times.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / times.len() as f64;

    let std_dev = variance.sqrt();
    std_dev / mean * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_test_short() {
        let config = CpuTestConfig {
            duration_secs: 1,
            thread_count: Some(2),
            verbose: false,
            on_comment: None,
        };
        let result = run_stress_test(config, "Test CPU".to_string(), 2);

        assert!(result.operations > 0);
        assert!(matches!(result.health, HealthStatus::Healthy));
    }

    #[test]
    fn test_prime_calculation() {
        assert_eq!(calculate_primes(1), 1);
        assert_eq!(calculate_primes(3), 3);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(!is_prime(4));
        assert!(!is_prime(9));
    }

    #[test]
    fn test_evaluate_cpu_health() {
        use crate::sensors::CpuTemp;

        // Healthy - normal variance
        assert!(matches!(evaluate_cpu_health(true, 10.0, None, 0.0), HealthStatus::Healthy));
        assert!(matches!(evaluate_cpu_health(true, 50.0, None, 0.0), HealthStatus::Healthy));
        assert!(matches!(evaluate_cpu_health(true, 150.0, None, 0.0), HealthStatus::Healthy));

        // Issues detected - hot temp (>85°C)
        let hot_temp = CpuTemp { current: 90.0 };
        assert!(matches!(evaluate_cpu_health(true, 10.0, Some(&hot_temp), 0.0), HealthStatus::IssuesDetected(_)));

        // Issues detected - throttling (>10%)
        assert!(matches!(evaluate_cpu_health(true, 10.0, None, 15.0), HealthStatus::IssuesDetected(_)));

        // Failed - variance too high (>200%)
        assert!(matches!(evaluate_cpu_health(true, 250.0, None, 0.0), HealthStatus::Failed(_)));

        // Failed - crashed
        assert!(matches!(evaluate_cpu_health(false, 0.0, None, 0.0), HealthStatus::Failed(_)));

        // Failed - overheating (>95°C)
        let overheat_temp = CpuTemp { current: 100.0 };
        assert!(matches!(evaluate_cpu_health(true, 10.0, Some(&overheat_temp), 0.0), HealthStatus::Failed(_)));
    }
}
