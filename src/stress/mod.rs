// Stress test module
// Provides CPU, RAM, Disk, and GPU health testing functionality

pub mod cpu;
pub mod ram;
pub mod disk;
pub mod gpu;
pub mod gpu_compute;

/// Health status after hardware test
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    IssuesDetected(Vec<String>),
    Failed(String),
}

pub use cpu::{CpuTestConfig, CpuTestResult, run_stress_test as run_cpu_test};
pub use ram::{RamTestConfig, RamTestResult, run_stress_test as run_ram_test};
pub use disk::{DiskTestConfig, DiskTestResult, run_stress_test as run_disk_test};
pub use gpu::{GpuTestConfig, GpuTestResult, run_stress_test as run_gpu_test};
