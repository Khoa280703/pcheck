// Background CPU usage monitoring
// Runs a separate thread to continuously refresh CPU usage data

use sysinfo::{System, RefreshKind, CpuRefreshKind};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

/// Shared CPU usage data updated by background thread
#[derive(Debug, Clone)]
pub struct CpuUsageMonitor {
    pub per_core_usage: HashMap<usize, f32>,
    pub total_usage: f32,
    pub core_count: usize,
}

impl Default for CpuUsageMonitor {
    fn default() -> Self {
        Self {
            per_core_usage: HashMap::new(),
            total_usage: 0.0,
            core_count: 0,
        }
    }
}

/// Background monitor handle
pub struct CpuMonitorHandle {
    running: Arc<AtomicBool>,
    data: Arc<Mutex<CpuUsageMonitor>>,
    _thread: Option<thread::JoinHandle<()>>,
}

impl CpuMonitorHandle {
    /// Start a new background CPU monitor
    pub fn start() -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let data = Arc::new(Mutex::new(CpuUsageMonitor::default()));

        // Initialize first reading with proper 2-call + delay for accurate usage
        {
            let mut sys = System::new_with_specifics(
                RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
            );
            sys.refresh_cpu_usage();
            thread::sleep(Duration::from_millis(100));
            sys.refresh_cpu_usage();

            let mut monitor = data.lock().unwrap();
            monitor.core_count = sys.cpus().len();
            let mut total = 0.0;
            for (i, cpu) in sys.cpus().iter().enumerate() {
                let usage = cpu.cpu_usage();
                monitor.per_core_usage.insert(i, usage);
                total += usage;
            }
            monitor.total_usage = if sys.cpus().len() > 0 {
                total / sys.cpus().len() as f32
            } else {
                0.0
            };
        }

        // Clone for thread
        let running_clone = Arc::clone(&running);
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut sys = System::new_with_specifics(
                RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
            );

            while running_clone.load(Ordering::Relaxed) {
                sys.refresh_cpu_usage();
                thread::sleep(Duration::from_millis(100));
                sys.refresh_cpu_usage();

                // Update shared data
                if let Ok(mut monitor) = data_clone.try_lock() {
                    let mut total = 0.0;
                    for (i, cpu) in sys.cpus().iter().enumerate() {
                        let usage = cpu.cpu_usage();
                        monitor.per_core_usage.insert(i, usage);
                        total += usage;
                    }
                    monitor.total_usage = if sys.cpus().len() > 0 {
                        total / sys.cpus().len() as f32
                    } else {
                        0.0
                    };
                }
            }
        });

        Self {
            running,
            data,
            _thread: Some(handle),
        }
    }

    /// Get per-core usage
    pub fn get_per_core_usage(&self) -> HashMap<usize, f32> {
        self.data.lock().unwrap().per_core_usage.clone()
    }
}

// Keep monitor running, drop handle without stopping
impl Drop for CpuMonitorHandle {
    fn drop(&mut self) {
        // Stop the background thread when monitor is dropped
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self._thread.take() {
            let _ = handle.join();
        }
    }
}
