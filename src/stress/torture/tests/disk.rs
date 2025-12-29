// Disk torture test wrapper
// Performs continuous disk I/O in chunks

use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Disk torture test state
pub struct DiskTortureTest {
    file: Option<File>,
    test_path: String,
    chunk_index: usize,
    total_chunks: usize,
    errors: Arc<AtomicU64>,
    bytes_written: Arc<AtomicU64>,
    bytes_read: Arc<AtomicU64>,
    io_time_ns: Arc<AtomicU64>, // Actual I/O time in NANOSECONDS for precision
    test_size_mb: u64,
    phase: DiskPhase,
    stop_requested: bool,
}

/// Disk test phases
enum DiskPhase {
    CreateFile,
    Write,
    Read,
    Done,
}

/// Partial result collected during torture test
#[derive(Debug)]
pub struct DiskPartialResult {
    pub write_speed_mb_s: f64,
    pub read_speed_mb_s: f64,
    pub healthy: bool,
    pub status: Option<String>,
}

/// Test metrics
pub struct TestMetrics {
    pub load_pct: f32,
    pub _temp_c: Option<f32>,
    pub _errors: u64,
    pub _progress_pct: f32,
    pub write_speed_mb_s: f64,
    pub read_speed_mb_s: f64,
    pub _status_msg: String,
}

impl DiskTortureTest {
    /// Create new Disk torture test
    pub fn new() -> Self {
        // Get temp directory
        let test_path = std::env::temp_dir()
            .join("pchecker_torture_disk.tmp")
            .to_string_lossy()
            .to_string();

        Self {
            file: None,
            test_path,
            chunk_index: 0,
            total_chunks: 0,
            errors: Arc::new(AtomicU64::new(0)),
            bytes_written: Arc::new(AtomicU64::new(0)),
            bytes_read: Arc::new(AtomicU64::new(0)),
            io_time_ns: Arc::new(AtomicU64::new(0)),
            test_size_mb: 10, // Small size for torture mode
            phase: DiskPhase::CreateFile,
            stop_requested: false,
        }
    }

    /// Run a chunk of Disk work
    pub fn run_chunk(&mut self, _chunk_ms: u64) {
        match self.phase {
            DiskPhase::CreateFile => {
                self.create_file();
            }
            DiskPhase::Write => {
                self.write_chunk();
            }
            DiskPhase::Read => {
                self.read_chunk();
            }
            DiskPhase::Done => {
                // Test complete, do nothing
            }
        }
    }

    /// Create test file
    fn create_file(&mut self) {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.test_path)
        {
            Ok(file) => {
                self.file = Some(file);
                self.chunk_index = 0;
                self.total_chunks = self.test_size_mb as usize; // 1MB chunks
                self.phase = DiskPhase::Write;
            }
            Err(_) => {
                self.phase = DiskPhase::Done;
            }
        }
    }

    /// Write a chunk of data
    fn write_chunk(&mut self) {
        let file = match &mut self.file {
            Some(f) => f,
            None => return,
        };

        let chunk_size = 1024 * 1024; // 1MB
        let data = vec![0xA5_u8; chunk_size];

        if self.chunk_index < self.total_chunks {
            // Time the I/O operation (nanoseconds for precision - fast SSD I/O can be < 1ms)
            let io_start = Instant::now();
            let result = file.write_all(&data);
            let io_elapsed = io_start.elapsed().as_nanos() as u64;
            self.io_time_ns.fetch_add(io_elapsed, Ordering::Relaxed);

            match result {
                Ok(_) => {
                    self.bytes_written.fetch_add(chunk_size as u64, Ordering::Relaxed);
                    self.chunk_index += 1;
                }
                Err(_) => {
                    self.errors.fetch_add(1, Ordering::Relaxed);
                    self.chunk_index += 1;
                }
            }

            // After writing all chunks, switch to read
            if self.chunk_index >= self.total_chunks {
                self.chunk_index = 0;
                let _ = file.rewind();
                self.phase = DiskPhase::Read;
            }
        }
    }

    /// Read a chunk of data
    fn read_chunk(&mut self) {
        let file = match &mut self.file {
            Some(f) => f,
            None => return,
        };

        let chunk_size = 1024 * 1024; // 1MB
        let mut buffer = vec![0u8; chunk_size];

        if self.chunk_index < self.total_chunks {
            // Time the I/O operation (nanoseconds for precision)
            let io_start = Instant::now();
            let result = file.read_exact(&mut buffer);
            let io_elapsed = io_start.elapsed().as_nanos() as u64;
            self.io_time_ns.fetch_add(io_elapsed, Ordering::Relaxed);

            match result {
                Ok(_) => {
                    // Verify pattern
                    let has_errors = buffer.iter().any(|&b| b != 0xA5);
                    if has_errors {
                        self.errors.fetch_add(1, Ordering::Relaxed);
                    }
                    self.bytes_read.fetch_add(chunk_size as u64, Ordering::Relaxed);
                    self.chunk_index += 1;
                }
                Err(_) => {
                    self.errors.fetch_add(1, Ordering::Relaxed);
                    self.chunk_index += 1;
                }
            }

            // After reading all chunks, start over
            if self.chunk_index >= self.total_chunks {
                self.chunk_index = 0;
                let _ = file.rewind();
                self.phase = DiskPhase::Write;
            }
        }
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> TestMetrics {
        let errors = self.errors.load(Ordering::Relaxed);
        // Use actual I/O time instead of elapsed time (nanoseconds for precision)
        let io_time_secs = self.io_time_ns.load(Ordering::Relaxed) as f64 / 1_000_000_000.0;

        let bytes_written_mb = self.bytes_written.load(Ordering::Relaxed) as f64 / 1024.0 / 1024.0;
        let bytes_read_mb = self.bytes_read.load(Ordering::Relaxed) as f64 / 1024.0 / 1024.0;

        // Calculate speed using actual I/O time
        let write_speed = if io_time_secs > 0.0 {
            bytes_written_mb / io_time_secs
        } else {
            0.0
        };

        let read_speed = if io_time_secs > 0.0 {
            bytes_read_mb / io_time_secs
        } else {
            0.0
        };

        let progress = if self.total_chunks > 0 {
            (self.chunk_index * 100 / self.total_chunks) as f32
        } else {
            0.0
        };

        let status = match self.phase {
            DiskPhase::CreateFile => "Creating file...".to_string(),
            DiskPhase::Write => format!("Writing {}%", progress as u8),
            DiskPhase::Read => format!("Reading {}%", progress as u8),
            DiskPhase::Done => "Complete".to_string(),
        };

        TestMetrics {
            // Dynamic load based on phase
            load_pct: if self.file.is_some() {
                match self.phase {
                    DiskPhase::CreateFile => 20.0,
                    DiskPhase::Write => 75.0,
                    DiskPhase::Read => 70.0,
                    DiskPhase::Done => 0.0,
                }
            } else {
                0.0
            },
            _temp_c: None,
            _errors: errors,
            _progress_pct: progress,
            write_speed_mb_s: write_speed,
            read_speed_mb_s: read_speed,
            _status_msg: status,
        }
    }

    /// Stop the Disk test
    pub fn stop(&mut self) {
        self.stop_requested = true;
        self.phase = DiskPhase::Done;

        // Clean up temp file
        let _ = std::fs::remove_file(&self.test_path);
    }

    /// Get final result
    pub fn get_result(&self) -> DiskPartialResult {
        // Use actual I/O time instead of elapsed time (nanoseconds for precision)
        let io_time_secs = self.io_time_ns.load(Ordering::Relaxed) as f64 / 1_000_000_000.0;

        let write_speed = if io_time_secs > 0.0 {
            (self.bytes_written.load(Ordering::Relaxed) as f64 / 1024.0 / 1024.0) / io_time_secs
        } else {
            0.0
        };

        let read_speed = if io_time_secs > 0.0 {
            (self.bytes_read.load(Ordering::Relaxed) as f64 / 1024.0 / 1024.0) / io_time_secs
        } else {
            0.0
        };

        let errors = self.errors.load(Ordering::Relaxed);

        let (healthy, status) = if write_speed < 1.0 {
            (false, Some(format!("Very slow write: {:.1} MB/s", write_speed)))
        } else if read_speed < 1.0 {
            (false, Some(format!("Very slow read: {:.1} MB/s", read_speed)))
        } else if errors > 0 {
            (false, Some(format!("{} I/O errors", errors)))
        } else {
            (true, Some("OK".to_string()))
        };

        DiskPartialResult {
            write_speed_mb_s: write_speed,
            read_speed_mb_s: read_speed,
            healthy,
            status,
        }
    }
}
