// CPU torture test wrapper
// Runs CPU-intensive calculations in chunks during round-robin execution

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use crate::sensors::{get_cpu_temp, get_cpu_frequency};

/// CPU torture test state
pub struct CpuTortureTest {
    running: Arc<AtomicBool>,
    operations: Arc<AtomicU64>,
    thread_handles: Vec<thread::JoinHandle<()>>,
    _start_time: Instant,
    _target_duration_secs: u64,
    _cpu_cores: usize,
    completed: bool,
}

/// Partial result collected during torture test
#[derive(Debug)]
pub struct CpuPartialResult {
    pub operations: u64,
    pub temp_c: Option<f32>,
    pub freq_ghz: f64,
    pub healthy: bool,
    pub status: Option<String>,
}

impl CpuTortureTest {
    /// Create new CPU torture test
    pub fn new(duration_secs: u64) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let operations = Arc::new(AtomicU64::new(0));
        let cpu_cores = num_cpus::get();

        // Spawn worker threads
        let thread_handles: Vec<_> = (0..cpu_cores)
            .map(|_| {
                let running = Arc::clone(&running);
                let operations = Arc::clone(&operations);
                thread::spawn(move || {
                    while running.load(Ordering::Relaxed) {
                        // CPU-intensive work: calculate primes
                        calculate_primes(100);
                        operations.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();

        Self {
            running,
            operations,
            thread_handles,
            _start_time: Instant::now(),
            _target_duration_secs: duration_secs,
            _cpu_cores: cpu_cores,
            completed: false,
        }
    }

    /// Run a chunk of CPU work (just waits - workers are running in background)
    pub fn run_chunk(&mut self, _chunk_ms: u64) {
        // Worker threads are running in background
        // This method is a no-op for CPU since threads run continuously
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> TestMetrics {
        let ops = self.operations.load(Ordering::Relaxed);
        let temp = get_cpu_temp();
        let freq = get_cpu_frequency();

        TestMetrics {
            load_pct: 100.0, // Always 100% when running
            temp_c: temp.as_ref().map(|t| t.current),
            _freq_mhz: Some(freq.current_mhz),
            freq_ghz: freq.current_ghz,
            _operations: ops,
            _errors: 0,
            _status_msg: format!("{} ops", format_large_number(ops)),
        }
    }

    /// Stop the CPU test and collect results
    pub fn stop(&mut self) {
        if self.completed {
            return;
        }

        self.running.store(false, Ordering::Relaxed);

        // Wait for all threads to finish
        for handle in self.thread_handles.drain(..) {
            let _ = handle.join();
        }

        self.completed = true;
    }

    /// Get final result
    pub fn get_result(&self) -> CpuPartialResult {
        let ops = self.operations.load(Ordering::Relaxed);
        let temp = get_cpu_temp();
        let freq = get_cpu_frequency();

        let temp_c = temp.as_ref().map(|t| t.current);
        let freq_ghz = freq.current_ghz;

        // Simple health check
        let (healthy, status) = if let Some(t) = temp_c {
            if t > 95.0 {
                (false, Some(format!("Overheating: {:.1}°C", t)))
            } else if t > 85.0 {
                (false, Some(format!("Running hot: {:.1}°C", t)))
            } else {
                (true, Some("OK".to_string()))
            }
        } else {
            (true, Some("OK (no temp data)".to_string()))
        };

        CpuPartialResult {
            operations: ops,
            temp_c,
            freq_ghz,
            healthy,
            status,
        }
    }
}

/// Metrics collected during test
#[derive(Debug)]
pub struct TestMetrics {
    pub load_pct: f32,
    pub temp_c: Option<f32>,
    pub _freq_mhz: Option<u64>,
    pub freq_ghz: f64,
    pub _operations: u64,
    pub _errors: u64,
    pub _status_msg: String,
}

/// Calculate n prime numbers (CPU-intensive work)
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

/// Format large number with thousands separator
fn format_large_number(n: u64) -> String {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*c);
    }
    result
}
