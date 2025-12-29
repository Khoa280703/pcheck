// Dashboard renderer for torture test
// Shows real-time progress on 5 lines (in-place updates)

use std::io::Write;
use std::time::Duration;
use crate::lang::Text;
use crate::fmt::{RESET, temp_color};

/// ANSI escape sequence to move cursor up 5 lines (to overwrite 5-line dashboard)
const MOVE_UP_LINES: &str = "\x1b[5A";

/// Render the torture test progress dashboard.
///
/// Displays 5 lines of output that update in-place:
/// - Line 1: Progress header [XX% | elapsed/total]
/// - Line 2: CPU metrics (load, temp, frequency)
/// - Line 3: GPU metrics (load, temp)
/// - Line 4: RAM metrics (load, errors)
/// - Line 5: Disk metrics (load, write speed, read speed)
///
/// # Terminal Requirements
/// - Supports ANSI escape sequences (most modern terminals)
/// - Width >= 60 characters recommended
///
/// # Arguments
/// * `first_render` - if true, print without moving cursor; if false, move up first
pub fn render_torture_dashboard(
    elapsed: Duration,
    total: Duration,
    cpu: &crate::stress::torture::tests::cpu::TestMetrics,
    ram: &crate::stress::torture::tests::ram::TestMetrics,
    disk: &crate::stress::torture::tests::disk::TestMetrics,
    gpu: &crate::stress::torture::tests::gpu::TestMetrics,
    text: &Text,
    first_render: bool,
) {
    let elapsed_secs = elapsed.as_secs();
    let total_secs = total.as_secs();
    let progress_pct = ((elapsed_secs as f32 / total_secs as f32) * 100.0).min(100.0);

    // Build temperature strings with color
    let cpu_temp_str = if let Some(temp) = cpu.temp_c {
        format!("{}{}°C{}", temp_color(temp), temp as i32, RESET)
    } else {
        text.torture_na().to_string()
    };

    let gpu_temp_str = if let Some(temp) = gpu.temp_c {
        format!("{}{}°C{}", temp_color(temp), temp as i32, RESET)
    } else {
        text.torture_na().to_string()
    };

    // Build component strings
    let cpu_line = format!("{}:  {}% {} | {} | {:.2}GHz",
        text.torture_cpu(),
        cpu.load_pct as i32,
        text.torture_load(),
        cpu_temp_str,
        cpu.freq_ghz,
    );

    let gpu_line = format!("{}:  {}% {}  | {}",
        text.torture_gpu(),
        gpu.load_pct as i32,
        text.torture_load(),
        gpu_temp_str,
    );

    let ram_line = format!("{}:  {}% {}  | {} {}",
        text.torture_ram(),
        ram.load_pct as i32,
        text.torture_load(),
        ram.errors,
        text.torture_errors(),
    );

    let disk_line = format!("{}: {}% {}  | {:.3} {}/{} | {:.3} {}/{}",
        text.torture_disk(),
        disk.load_pct as i32,
        text.torture_load(),
        disk.write_speed_mb_s,
        text.torture_mb_s(),
        text.torture_write(),
        disk.read_speed_mb_s,
        text.torture_mb_s(),
        text.torture_read(),
    );

    // Move cursor up to overwrite previous output (5 lines)
    if !first_render {
        print!("{}", MOVE_UP_LINES);
    }

    // Print all 5 lines
    println!("[{}% | {}/{}s]", progress_pct as i32, elapsed_secs, total_secs);
    println!("{}", cpu_line);
    println!("{}", gpu_line);
    println!("{}", ram_line);
    println!("{}", disk_line);

    std::io::stdout().flush().unwrap();
}
