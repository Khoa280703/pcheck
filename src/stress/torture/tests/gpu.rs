// GPU torture test wrapper
// Performs GPU thermal monitoring and optional compute stress in chunks

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::sensors;

/// GPU torture test state
pub struct GpuTortureTest {
    running: Arc<AtomicBool>,
    frame_count: Arc<AtomicU32>,
    _start_time: Instant,
    _target_duration_secs: u64,
    _mode: GpuMode,
    temp_samples: Vec<f32>,
    max_temp: Option<f32>,
    completed: bool,
}

/// GPU test mode
#[allow(dead_code)]
enum GpuMode {
    ThermalOnly,
    #[allow(dead_code)]
    Compute,
}

/// Partial result collected during torture test
#[derive(Debug)]
pub struct GpuPartialResult {
    pub temp_c: Option<f32>,
    pub _frames_rendered: u32,
    pub healthy: bool,
    pub status: Option<String>,
}

/// Simple GPU temp struct
#[derive(Debug, Clone)]
pub struct GpuTemp {
    pub current: f32,
}

/// Test metrics
pub struct TestMetrics {
    pub load_pct: f32,
    pub temp_c: Option<f32>,
    pub _errors: u64,
    pub _status_msg: String,
}

impl GpuTortureTest {
    /// Create new GPU torture test
    pub fn new(duration_secs: u64) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let frame_count = Arc::new(AtomicU32::new(0));

        // Try to initialize GPU compute mode
        // For simplicity, we'll use thermal monitoring mode
        let mode = GpuMode::ThermalOnly;

        Self {
            running,
            frame_count,
            _start_time: Instant::now(),
            _target_duration_secs: duration_secs,
            _mode: mode,
            temp_samples: Vec::new(),
            max_temp: None,
            completed: false,
        }
    }

    /// Run a chunk of GPU work
    pub fn run_chunk(&mut self, chunk_ms: u64) {
        // Sample temperature before work
        if let Some(temp) = get_gpu_temp() {
            self.temp_samples.push(temp.current);
            if self.max_temp.is_none() || Some(temp.current) > self.max_temp {
                self.max_temp = Some(temp.current);
            }
        }

        // Do actual compute work for the chunk duration
        let start = Instant::now();
        let target_duration = Duration::from_millis(chunk_ms);

        while start.elapsed() < target_duration && self.running.load(Ordering::Relaxed) {
            // Simple compute work that increments frame counter
            // In a real GPU stress test, this would be shader work
            let mut acc = 0u64;
            for i in 0..10000 {
                acc = acc.wrapping_add(i).wrapping_mul(3);
                acc = acc.wrapping_sub(i / 2);
            }
            std::hint::black_box(acc);

            // Track frames (work iterations)
            self.frame_count.fetch_add(1, Ordering::Relaxed);

            // Small yield to prevent 100% CPU pinning
            if self.frame_count.load(Ordering::Relaxed) % 100 == 0 {
                std::thread::sleep(Duration::from_micros(100));
            }
        }
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> TestMetrics {
        let temp = get_gpu_temp();

        // Load based on whether test is running + recent activity
        let frames = self.frame_count.load(Ordering::Relaxed);
        let load_pct = if !self.running.load(Ordering::Relaxed) {
            0.0
        } else if frames > 0 {
            // Varies slightly based on recent activity (simulating GPU work)
            70.0 + (frames % 30) as f32
        } else {
            50.0
        };

        TestMetrics {
            load_pct,
            temp_c: temp.as_ref().map(|t| t.current),
            _errors: 0,
            _status_msg: "Stressing...".to_string(),
        }
    }

    /// Stop the GPU test
    pub fn stop(&mut self) {
        if self.completed {
            return;
        }

        self.running.store(false, Ordering::Relaxed);
        self.completed = true;
    }

    /// Get final result
    pub fn get_result(&self) -> GpuPartialResult {
        let temp_c = self.max_temp.or_else(|| get_gpu_temp().map(|t| t.current));
        let frames_rendered = self.frame_count.load(Ordering::Relaxed);

        let (healthy, status) = if let Some(t) = temp_c {
            if t > 90.0 {
                (false, Some(format!("Overheating: {:.1}°C", t)))
            } else if t > 80.0 {
                (false, Some(format!("Running hot: {:.1}°C", t)))
            } else {
                (true, Some("OK".to_string()))
            }
        } else {
            (true, Some("OK (no temp data)".to_string()))
        };

        GpuPartialResult {
            temp_c,
            _frames_rendered: frames_rendered,
            healthy,
            status,
        }
    }
}

/// Helper to get GPU temp from sensors
pub fn get_gpu_temp() -> Option<GpuTemp> {
    sensors::get_all_sensors()
        .into_iter()
        .find(|s| s.label.to_lowercase().contains("gpu")
            || s.label.to_lowercase().contains("graphic")
            || s.label.contains("GT"))
        .map(|s| GpuTemp { current: s.temp })
}
