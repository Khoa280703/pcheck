// RAM torture test wrapper
// Performs continuous memory operations in chunks

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use sysinfo::System;

/// RAM torture test state
pub struct RamTortureTest {
    buffer: Option<Vec<u64>>,
    chunk_index: usize,
    total_chunks: usize,
    errors: Arc<AtomicU64>,
    _start_time: Instant,
    tested_gb: f64,
    phase: RamPhase,
    stop_requested: bool,
}

/// RAM test phases
enum RamPhase {
    Alloc,
    Write,
    Verify,
    Done,
}

/// Partial result collected during torture test
#[derive(Debug)]
pub struct RamPartialResult {
    pub tested_gb: f64,
    pub errors: u64,
    pub healthy: bool,
    pub status: Option<String>,
}

/// Test metrics
pub struct TestMetrics {
    pub load_pct: f32,
    pub _temp_c: Option<f32>,
    pub errors: u64,
    pub _progress_pct: f32,
    pub _status_msg: String,
}

impl RamTortureTest {
    /// Create new RAM torture test
    pub fn new() -> Self {
        Self {
            buffer: None,
            chunk_index: 0,
            total_chunks: 0,
            errors: Arc::new(AtomicU64::new(0)),
            _start_time: Instant::now(),
            tested_gb: 0.0,
            phase: RamPhase::Alloc,
            stop_requested: false,
        }
    }

    /// Run a chunk of RAM work
    pub fn run_chunk(&mut self, _chunk_ms: u64) {
        match self.phase {
            RamPhase::Alloc => {
                self.allocate_buffer();
            }
            RamPhase::Write => {
                self.write_chunk();
            }
            RamPhase::Verify => {
                self.verify_chunk();
            }
            RamPhase::Done => {
                // Test complete, do nothing
            }
        }
    }

    /// Allocate memory buffer
    fn allocate_buffer(&mut self) {
        // Get available memory
        let mut sys = System::new_all();
        sys.refresh_memory();

        let available_gb = (sys.total_memory() - sys.used_memory()) as f64
            / 1024.0 / 1024.0 / 1024.0;

        // Use smaller amount for torture mode to avoid OOM
        let test_gb = (available_gb * 0.5).min(4.0); // Max 4GB, 50% of available

        if test_gb < 0.1 {
            self.phase = RamPhase::Done;
            return;
        }

        let element_count = (test_gb * 1024.0 * 1024.0 * 1024.0 / 8.0) as usize;

        match vec![0u64; element_count] {
            buffer => {
                self.buffer = Some(buffer);
                self.tested_gb = test_gb;
                self.chunk_index = 0;
                self.total_chunks = element_count.div_ceil(1024 * 1024);
                self.phase = RamPhase::Write;
            }
        }
    }

    /// Write a chunk of data
    fn write_chunk(&mut self) {
        let buffer = match &mut self.buffer {
            Some(b) => b,
            None => return,
        };

        let chunk_size = 1024 * 1024; // 1MB chunks
        let pattern = 0xAA55_AA55_AA55_AA55_u64;

        // Write one chunk per call
        if self.chunk_index < self.total_chunks {
            let start = self.chunk_index * chunk_size;
            let end = (start + chunk_size).min(buffer.len());

            for i in start..end {
                buffer[i] = pattern;
            }

            self.chunk_index += 1;

            // After writing all chunks, switch to verify
            if self.chunk_index >= self.total_chunks {
                self.chunk_index = 0;
                self.phase = RamPhase::Verify;
            }
        }
    }

    /// Verify a chunk of data
    fn verify_chunk(&mut self) {
        let buffer = match &self.buffer {
            Some(b) => b,
            None => return,
        };

        let chunk_size = 1024 * 1024;
        let pattern = 0xAA55_AA55_AA55_AA55_u64;

        // Verify one chunk per call
        if self.chunk_index < self.total_chunks {
            let start = self.chunk_index * chunk_size;
            let end = (start + chunk_size).min(buffer.len());

            for i in start..end {
                if buffer[i] != pattern {
                    self.errors.fetch_add(1, Ordering::Relaxed);
                }
            }

            self.chunk_index += 1;

            // After verifying all chunks, start over
            if self.chunk_index >= self.total_chunks {
                self.chunk_index = 0;
                // Continue writing/verifying until stop
                self.phase = RamPhase::Write;
            }
        }
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> TestMetrics {
        let errors = self.errors.load(Ordering::Relaxed);
        let progress = if self.total_chunks > 0 {
            (self.chunk_index * 100 / self.total_chunks) as f32
        } else {
            0.0
        };

        let status = match self.phase {
            RamPhase::Alloc => "Allocating...".to_string(),
            RamPhase::Write => format!("Writing {}%", progress as u8),
            RamPhase::Verify => format!("Verifying {}%", progress as u8),
            RamPhase::Done => "Complete".to_string(),
        };

        TestMetrics {
            // Dynamic load based on phase and progress
            load_pct: if self.buffer.is_some() {
                match self.phase {
                    RamPhase::Alloc => 40.0,
                    RamPhase::Write => 90.0,
                    RamPhase::Verify => 85.0,
                    RamPhase::Done => 0.0,
                }
            } else {
                0.0
            },
            _temp_c: None,
            errors,
            _progress_pct: progress,
            _status_msg: status,
        }
    }

    /// Stop the RAM test
    pub fn stop(&mut self) {
        self.stop_requested = true;
        self.phase = RamPhase::Done;
    }

    /// Get final result
    pub fn get_result(&self) -> RamPartialResult {
        let errors = self.errors.load(Ordering::Relaxed);

        let (healthy, status) = if errors > 0 {
            (false, Some(format!("{} errors detected", errors)))
        } else if self.tested_gb < 0.1 {
            (false, Some("Allocation failed".to_string()))
        } else {
            (true, Some("OK".to_string()))
        };

        RamPartialResult {
            tested_gb: self.tested_gb,
            errors,
            healthy,
            status,
        }
    }
}
