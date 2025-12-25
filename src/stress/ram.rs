// RAM health check module
// Tests RAM by allocating memory and verifying write/read patterns

use std::time::Instant;
use std::io::{self, Write};
use sysinfo::System;

use super::HealthStatus;

pub struct RamTestConfig {
    pub max_gb: Option<f64>,
}

impl Default for RamTestConfig {
    fn default() -> Self {
        Self {
            max_gb: None,
        }
    }
}

pub struct RamTestResult {
    // Hardware info
    pub ram_total_gb: f64,
    // Test metrics
    pub tested_gb: f64,
    pub write_speed_gb_s: f64,
    pub read_speed_gb_s: f64,
    pub errors: u64,
    pub health: HealthStatus,
}

/// Run RAM health check
/// Allocates memory, writes patterns, reads back to verify
pub fn run_stress_test(config: RamTestConfig, ram_total_gb: f64) -> RamTestResult {
    // Get available memory
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let available_gb = (sys.total_memory() - sys.used_memory()) as f64 / 1024.0 / 1024.0 / 1024.0;

    // Use 80% of available RAM, or specified amount
    let test_gb = config.max_gb.unwrap_or_else(|| {
        (available_gb * 0.8).min(total_gb * 0.8)
    });

    // Limit to max 16GB to prevent OOM on systems with lots of RAM
    let test_gb = test_gb.min(16.0);

    print!("⏳ Đang kiểm tra RAM... Đang cấp phát {:.1} GB...", test_gb);
    io::stdout().flush().unwrap();

    // Allocate buffer
    let element_count = (test_gb * 1024.0 * 1024.0 * 1024.0 / 8.0) as usize;
    let mut buffer: Vec<u64> = vec![0; element_count];

    let _start = Instant::now();

    // Write test: fill buffer with pattern
    print!("\r⏳ Đang kiểm tra RAM... Đang ghi dữ liệu...");
    io::stdout().flush().unwrap();

    let write_start = Instant::now();
    let pattern = 0xAA55_AA55_AA55_AA55_u64;
    let chunk_size = 1024 * 1024;
    let total_chunks = (element_count + chunk_size - 1) / chunk_size;

    for (i, chunk) in buffer.chunks_mut(chunk_size).enumerate() {
        for val in chunk.iter_mut() {
            *val = pattern;
        }
        // Show progress every 100 chunks
        if (i + 1) % 100 == 0 || i + 1 == total_chunks {
            let progress = ((i + 1) * 100 / total_chunks) as u8;
            print!("\r⏳ Đang kiểm tra RAM... Đang ghi dữ liệu... {}%", progress);
            io::stdout().flush().unwrap();
        }
    }

    let write_time = write_start.elapsed();
    let write_speed = if write_time.as_secs_f64() > 0.0 {
        test_gb / write_time.as_secs_f64()
    } else {
        0.0
    };

    // Read + verify test
    print!("\r⏳ Đang kiểm tra RAM... Đang xác thực dữ liệu...");
    io::stdout().flush().unwrap();

    let mut errors = 0u64;
    let read_start = Instant::now();

    for (i, chunk) in buffer.chunks(chunk_size).enumerate() {
        for &val in chunk.iter() {
            if val != pattern {
                errors += 1;
            }
        }
        // Show progress every 100 chunks
        if (i + 1) % 100 == 0 || i + 1 == total_chunks {
            let progress = ((i + 1) * 100 / total_chunks) as u8;
            print!("\r⏳ Đang kiểm tra RAM... Đang xác thực dữ liệu... {}%", progress);
            io::stdout().flush().unwrap();
        }
    }

    let read_time = read_start.elapsed();
    let read_speed = if read_time.as_secs_f64() > 0.0 {
        test_gb / read_time.as_secs_f64()
    } else {
        0.0
    };

    println!(); // New line after progress

    // Evaluate health
    let health = evaluate_ram_health(test_gb, write_speed, read_speed, errors);

    RamTestResult {
        ram_total_gb,
        tested_gb: test_gb,
        write_speed_gb_s: write_speed,
        read_speed_gb_s: read_speed,
        errors,
        health,
    }
}

/// Evaluate RAM health based on test results
fn evaluate_ram_health(test_gb: f64, write: f64, read: f64, errors: u64) -> HealthStatus {
    // Critical: any memory errors = BAD RAM
    if errors > 0 {
        return HealthStatus::Failed(format!(
            "Memory errors detected ({} errors) - BAD RAM",
            errors
        ));
    }

    // Check if allocation worked at all
    if test_gb < 0.1 {
        return HealthStatus::Failed("Memory allocation failed".to_string());
    }

    // Only very low speed indicates actual fault (< 0.3 GB/s)
    // Speed variations are normal depending on RAM type, generation, system load
    if write < 0.3 {
        return HealthStatus::Failed(format!(
            "Extremely low write speed ({:.1} GB/s) - faulty RAM or wrong slot",
            write
        ));
    }

    if read < 0.3 {
        return HealthStatus::Failed(format!(
            "Extremely low read speed ({:.1} GB/s) - faulty RAM or wrong slot",
            read
        ));
    }

    // Otherwise healthy
    HealthStatus::Healthy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_test_small() {
        let config = RamTestConfig {
            max_gb: Some(0.1), // Only test 100MB
        };
        let result = run_stress_test(config, 16.0);

        assert!(result.tested_gb > 0.0);
        assert!(matches!(result.health, HealthStatus::Healthy));
        assert_eq!(result.errors, 0); // Should have no errors on healthy RAM
    }

    #[test]
    fn test_evaluate_ram_health() {
        // Healthy RAM - normal speeds
        assert!(matches!(
            evaluate_ram_health(8.0, 15.0, 20.0, 0),
            HealthStatus::Healthy
        ));

        // Healthy - slow but working (>0.3 GB/s)
        assert!(matches!(
            evaluate_ram_health(8.0, 0.5, 0.5, 0),
            HealthStatus::Healthy
        ));

        // Failed - memory errors
        assert!(matches!(
            evaluate_ram_health(8.0, 15.0, 20.0, 1),
            HealthStatus::Failed(_)
        ));

        // Failed - extremely slow (<0.3 GB/s)
        assert!(matches!(
            evaluate_ram_health(8.0, 0.2, 0.2, 0),
            HealthStatus::Failed(_)
        ));
    }
}
