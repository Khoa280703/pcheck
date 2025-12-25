// pchecker - Cross-platform hardware detection CLI tool
// https://github.com/Khoa280703/pcheck

mod platform;
mod hw;
mod fmt;

use std::time::Instant;
use hw::{CpuInfo, RamInfo, DiskInfo, GpuInfo};
use fmt::{print_header, print_section, print_footer};

fn main() {
    let start_time = Instant::now();

    print_header("v0.1.0");

    // Detect platform
    let platform = platform::detect();
    print_section("💻", "SYSTEM", &platform.to_string());

    println!("⏳ Detecting hardware...");

    // Detect CPU
    let cpu = CpuInfo::new();
    print_section("🧠", "CPU", &cpu.display());

    // Detect GPU
    let gpus = GpuInfo::new();
    if let Some(gpu) = gpus.first() {
        print_section("🎮", "GPU", &gpu.display());
    } else {
        print_section("🎮", "GPU", "No GPU detected");
    }

    // Detect RAM
    let ram = RamInfo::new();
    print_section("💾", "RAM", &ram.display());

    // Detect Disk (show first one)
    let disks = DiskInfo::new();
    if let Some(disk) = disks.first() {
        print_section("💿", "DISK", &disk.display());
    }

    print_footer(start_time);
}
