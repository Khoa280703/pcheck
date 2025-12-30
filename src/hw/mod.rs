// Hardware detection modules

pub mod cpu;
pub mod ram;
pub mod disk;
pub mod gpu;
pub mod deep;

pub use cpu::CpuInfo;
pub use ram::RamInfo;
pub use disk::DiskInfo;
pub use gpu::GpuInfo;
