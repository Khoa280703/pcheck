// Torture test - Full system stress test (Boss Fight)
// Runs CPU, GPU, RAM, Disk simultaneously to detect PSU/thermal issues

use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread;

use crate::lang::{Language, Text};

pub mod tests;
pub mod dashboard;

use tests::{CpuTortureTest, RamTortureTest, DiskTortureTest, GpuTortureTest};
use tests::cpu::CpuPartialResult;
use tests::ram::RamPartialResult;
use tests::disk::DiskPartialResult;
use tests::gpu::GpuPartialResult;
use dashboard::render_torture_dashboard;

pub struct TortureConfig {
    pub duration_secs: u64,
    pub _verbose: bool,
    pub language: Language,
}

pub struct TortureResult {
    pub _duration_actual_secs: u64,
    pub _cpu_result: Option<CpuPartialResult>,
    pub _ram_result: Option<RamPartialResult>,
    pub _disk_result: Option<DiskPartialResult>,
    pub _gpu_result: Option<GpuPartialResult>,
    pub _survived: bool,
}

/// Run full system torture test
/// Shows warning, waits for confirmation, then runs all tests simultaneously
pub fn run_torture_test(config: TortureConfig) -> TortureResult {
    let text = Text::new(config.language);

    // Warning message
    println!();
    println!("============================================================");
    println!("🔥 {} - v0.3.0", text.torture_test());
    println!("============================================================");
    println!();
    println!("⚠️  {}", text.torture_warning());
    println!();
    println!("• {}", text.torture_warning_psu());
    println!("• {}", text.torture_warning_thermal());
    println!("• {}", text.torture_warning_fans());
    println!();
    println!("{} {} {} {}", text.torture_duration(), config.duration_secs, text.seconds(), text.torture_cancel_info());
    println!();

    // Confirm prompt
    print!("{} [Y/n]: ", text.torture_confirm());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();
    if input == "n" || input == "no" {
        println!("❌ {}", text.torture_cancelled());
        return TortureResult {
            _duration_actual_secs: 0,
            _cpu_result: None,
            _ram_result: None,
            _disk_result: None,
            _gpu_result: None,
            _survived: false,
        };
    }

    println!();
    println!("🔥 {}...", text.torture_starting());
    println!();

    // Give user a moment to prepare
    thread::sleep(Duration::from_secs(2));

    // Run the test
    

    run_torture_test_internal(config)
}

/// Internal torture test implementation
fn run_torture_test_internal(config: TortureConfig) -> TortureResult {
    let start = Instant::now();
    let duration = Duration::from_secs(config.duration_secs);
    let cycle_duration = Duration::from_millis(100); // 100ms per cycle

    // Create tests
    let mut cpu_test = CpuTortureTest::new(config.duration_secs);
    let mut ram_test = RamTortureTest::new();
    let mut disk_test = DiskTortureTest::new();
    let mut gpu_test = GpuTortureTest::new(config.duration_secs);

    let mut max_cpu_temp: Option<f32> = None;
    let mut max_gpu_temp: Option<f32> = None;

    // Main loop: round-robin through all tests
    let cycle_count = 4; // CPU, RAM, Disk, GPU
    let chunk_ms = 100 / cycle_count; // 25ms per test per cycle
    let mut first_render = true;
    let mut cycle_counter = 0;

    while start.elapsed() < duration {
        let cycle_start = Instant::now();

        // Run each test for its chunk
        cpu_test.run_chunk(chunk_ms);
        ram_test.run_chunk(chunk_ms);
        disk_test.run_chunk(chunk_ms);
        gpu_test.run_chunk(chunk_ms);

        // Collect metrics and update dashboard
        let cpu_metrics = cpu_test.get_metrics();
        let ram_metrics = ram_test.get_metrics();
        let disk_metrics = disk_test.get_metrics();
        let gpu_metrics = gpu_test.get_metrics();

        // Track max temperatures
        if let Some(temp) = cpu_metrics.temp_c {
            max_cpu_temp = Some(max_cpu_temp.unwrap_or(0.0).max(temp));
        }
        if let Some(temp) = gpu_metrics.temp_c {
            max_gpu_temp = Some(max_gpu_temp.unwrap_or(0.0).max(temp));
        }

        // Render dashboard every 10 cycles (1Hz) to match CPU test behavior
        cycle_counter += 1;
        if cycle_counter % 10 == 0 {
            render_torture_dashboard(
                start.elapsed(),
                duration,
                &cpu_metrics,
                &ram_metrics,
                &disk_metrics,
                &gpu_metrics,
                &Text::new(config.language),
                first_render,
            );
            first_render = false;
        }

        // Sleep for remaining cycle time
        let elapsed = cycle_start.elapsed();
        if elapsed < cycle_duration {
            thread::sleep(cycle_duration - elapsed);
        }
    }

    // Stop all tests
    cpu_test.stop();
    ram_test.stop();
    disk_test.stop();
    gpu_test.stop();

    // Collect results
    let cpu_result = cpu_test.get_result();
    let ram_result = ram_test.get_result();
    let disk_result = disk_test.get_result();
    let gpu_result = gpu_test.get_result();

    let actual_duration = start.elapsed().as_secs();

    // Print summary
    print_torture_summary(
        actual_duration,
        &cpu_result,
        &ram_result,
        &disk_result,
        &gpu_result,
        max_cpu_temp,
        max_gpu_temp,
        &Text::new(config.language),
    );

    TortureResult {
        _duration_actual_secs: actual_duration,
        _cpu_result: Some(cpu_result),
        _ram_result: Some(ram_result),
        _disk_result: Some(disk_result),
        _gpu_result: Some(gpu_result),
        _survived: true,
    }
}

/// Print torture test summary
fn print_torture_summary(
    duration: u64,
    cpu: &CpuPartialResult,
    ram: &RamPartialResult,
    disk: &DiskPartialResult,
    gpu: &GpuPartialResult,
    _max_cpu_temp: Option<f32>,
    _max_gpu_temp: Option<f32>,
    text: &Text,
) {
    println!("============================================================");
    println!("📊 {} - v0.3.0", text.torture_summary());
    println!("============================================================");
    println!();
    println!("{}: {}s", text.torture_duration(), duration);
    println!();

    // CPU result
    println!("🧠 {}", text.cpu());
    println!("   {} {} | {} {:.1}°C | {} {:.2} GHz",
        text.operations(), cpu.operations,
        text.temperature(), cpu.temp_c.unwrap_or(0.0),
        text.frequency(), cpu.freq_ghz
    );
    if let Some(ref msg) = cpu.status {
        println!("   {} {}", if cpu.healthy { "✅" } else { "❌" }, msg);
    }
    println!();

    // RAM result
    println!("💾 {}", text.ram());
    println!("   {} {:.1} GB | {} {}",
        text.tested_gb(), ram.tested_gb,
        text.errors_detected(), ram.errors
    );
    if let Some(ref msg) = ram.status {
        println!("   {} {}", if ram.healthy { "✅" } else { "❌" }, msg);
    }
    println!();

    // Disk result
    println!("💿 {}", text.disk());
    println!("   {} {:.3} MB/s | {} {:.3} MB/s",
        text.write_speed(), disk.write_speed_mb_s,
        text.read_speed(), disk.read_speed_mb_s
    );
    if let Some(ref msg) = disk.status {
        println!("   {} {}", if disk.healthy { "✅" } else { "❌" }, msg);
    }
    println!();

    // GPU result
    println!("🎮 {}", text.gpu());
    if let Some(temp) = gpu.temp_c {
        println!("   {} {:.1}°C", text.temperature(), temp);
    }
    if let Some(ref msg) = gpu.status {
        println!("   {} {}", if gpu.healthy { "✅" } else { "❌" }, msg);
    }
    println!();

    // Overall verdict
    let all_healthy = cpu.healthy && ram.healthy && disk.healthy && gpu.healthy;
    println!("============================================================");
    if all_healthy {
        println!("✅ {}", text.torture_passed());
    } else {
        println!("❌ {}", text.torture_failed());
    }
    println!("============================================================");
}
