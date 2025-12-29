// Torture test wrappers for individual stress tests
// Each wrapper maintains state for chunked execution during round-robin scheduling

pub mod cpu;
pub mod ram;
pub mod disk;
pub mod gpu;

pub use cpu::CpuTortureTest;
pub use ram::RamTortureTest;
pub use disk::DiskTortureTest;
pub use gpu::GpuTortureTest;

