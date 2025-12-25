// pchecker - Cross-platform hardware detection CLI tool
// https://github.com/Khoa280703/pcheck

mod platform;
mod hw;
mod fmt;
mod lang;
mod prompt;

use std::time::Instant;
use hw::{CpuInfo, RamInfo, DiskInfo, GpuInfo};
use lang::Text;
use fmt::{print_header_with_text, print_section, print_footer_with_text};

fn main() {
    // Step 1: Select language
    let lang = prompt::select_language();
    let text = Text::new(lang);

    let start_time = Instant::now();

    // Step 2: Print header with translated text
    print_header_with_text("v0.1.0", text.header());

    // Step 3: Detect platform
    let platform = platform::detect();
    print_section("💻", text.system(), &platform.to_string());

    // Step 4: Detect hardware
    println!("⏳ {}", text.detecting());

    // Detect CPU
    let cpu = CpuInfo::new();
    let cpu_display = format!("{} ({} {})", cpu.model, cpu.cores, text.cores());
    print_section("🧠", text.cpu(), &cpu_display);

    // Detect GPU
    let gpus = GpuInfo::new();
    if let Some(gpu) = gpus.first() {
        print_section("🎮", text.gpu(), &gpu.display());
    } else {
        print_section("🎮", text.gpu(), text.no_gpu());
    }

    // Detect RAM
    let ram = RamInfo::new();
    let ram_display = format!("{:.1} GB ({:.1} GB {})", ram.total_gb, ram.used_gb, text.ram_free());
    print_section("💾", text.ram(), &ram_display);

    // Detect Disk (show first one)
    let disks = DiskInfo::new();
    if let Some(disk) = disks.first() {
        print_section("💿", text.disk(), &disk.display());
    }

    // Step 5: Print footer with translated text
    print_footer_with_text(start_time, text.done_in());
}
