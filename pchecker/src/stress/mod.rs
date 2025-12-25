// Stress test module
// Provides CPU and RAM health testing functionality

pub mod cpu;
pub mod ram;

/// Health status after hardware test
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    IssuesDetected(Vec<String>),
    Failed(String),
}

pub use cpu::{CpuTestConfig, CpuTestResult, run_stress_test as run_cpu_test};
pub use ram::{RamTestConfig, RamTestResult, run_stress_test as run_ram_test};
