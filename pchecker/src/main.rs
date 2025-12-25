// pchecker - Cross-platform hardware detection & health check CLI tool
// https://github.com/Khoa280703/pcheck

mod platform;
mod hw;
mod fmt;
mod lang;
mod prompt;
mod stress;
mod sensors;

use std::time::Instant;
use std::io::{self, Write};
use clap::Parser;
use hw::{CpuInfo, RamInfo, DiskInfo, GpuInfo};
use lang::{Text, Language};
use fmt::{print_header_with_text, print_section, print_footer_with_text};
use stress::{CpuTestConfig, RamTestConfig, HealthStatus};

/// pchecker - Hardware detection and health check tool
#[derive(Parser, Debug)]
#[command(name = "pchecker")]
#[command(version = "0.2.0")]
#[command(about = "Hardware detection and health check tool", long_about = None)]
struct Args {
    /// Run health check mode (CPU + RAM)
    #[arg(short, long)]
    stress: bool,

    /// Health check duration in seconds (default: 60 for CPU, 30 for RAM)
    #[arg(short, long, default_value = "60")]
    duration: u64,

    /// Quick health check (15 seconds)
    #[arg(long, conflicts_with = "duration")]
    quick: bool,
}

fn main() {
    let args = Args::parse();

    // Determine duration
    let duration = if args.quick {
        15
    } else {
        args.duration
    };

    // Select language first
    let lang = select_language_standalone();
    let text = Text::new(lang);

    if args.stress {
        run_health_check_mode(duration, &text);
    } else {
        run_info_mode(&text);
    }
}

/// Standalone language selection
fn select_language_standalone() -> Language {
    println!();
    println!("============================================================");
    println!("🤖 PCHECKER v0.2.0");
    println!("============================================================");
    println!();
    println!("Chọn ngôn ngữ / Select language:");
    println!();
    println!("  [1] Tiếng Việt");
    println!("  [2] English");
    println!();

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("Your choice [1-2]: ");
        io::stdout().flush().unwrap();

        input.clear();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "1" | "vi" | "VI" | "vietnamese" => return Language::Vietnamese,
                    "2" | "en" | "EN" | "english" => return Language::English,
                    _ => {
                        println!("⚠️  Invalid choice. Please select 1 or 2.");
                        continue;
                    }
                }
            }
            Err(_) => return Language::Vietnamese,
        }
    }
}

/// Run info-only mode (v0.1.0 behavior)
fn run_info_mode(text: &Text) {
    let start_time = Instant::now();

    // Print header
    print_header_with_text("v0.2.0", text.header());

    // Detect platform
    let platform = platform::detect();
    print_section("💻", text.system(), &platform.to_string());

    // Detect hardware
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

    // Print footer
    print_footer_with_text(start_time, text.done_in());
}

/// Run health check mode (v0.2.0 feature)
fn run_health_check_mode(duration: u64, text: &Text) {
    let start_time = Instant::now();

    println!();
    println!("============================================================");
    println!("🧪 PCHECKER {} - v0.2.0", text.health_check());
    println!("============================================================");
    println!();

    let mut all_healthy = true;
    let mut all_issues: Vec<String> = Vec::new();
    let mut critical_issues: Vec<String> = Vec::new();

    // CPU Test
    println!("⏳ {} ({}s)", text.testing_cpu(), duration);
    io::stdout().flush().unwrap();

    let cpu_config = CpuTestConfig {
        duration_secs: duration,
        thread_count: None,
    };
    let cpu_result = stress::run_cpu_test(cpu_config);

    let (cpu_healthy, cpu_issues) = print_cpu_result(&cpu_result, text);
    if !cpu_healthy {
        all_healthy = false;
        if matches!(cpu_result.health, HealthStatus::Failed(_)) {
            if let HealthStatus::Failed(ref msg) = cpu_result.health {
                critical_issues.push(format!("CPU: {}", msg));
            }
        }
    }
    all_issues.extend(cpu_issues);
    println!();

    // RAM Test
    let ram_duration = (duration / 2).max(10);
    println!("⏳ {} (~{}s)", text.testing_ram(), ram_duration);
    io::stdout().flush().unwrap();

    let ram_config = RamTestConfig {
        max_gb: None,
    };
    let ram_result = stress::run_ram_test(ram_config);

    let (ram_healthy, ram_issues) = print_ram_result(&ram_result, text);
    if !ram_healthy {
        all_healthy = false;
        if matches!(ram_result.health, HealthStatus::Failed(_)) {
            if let HealthStatus::Failed(ref msg) = ram_result.health {
                critical_issues.push(format!("RAM: {}", msg));
            }
        }
    }
    all_issues.extend(ram_issues);
    println!();

    // Overall summary
    println!("============================================================");
    if !critical_issues.is_empty() {
        println!("❌ {}", text.critical_issues());
        for issue in &critical_issues {
            println!("   • {}", issue);
        }
    } else if !all_issues.is_empty() {
        println!("⚠️  {}", text.issues_detected());
        for issue in &all_issues {
            println!("   • {}", issue);
        }
    }
    println!("{}", text.summary());
    if all_healthy && critical_issues.is_empty() {
        println!("✅ {}", text.hardware_good());
    } else if critical_issues.is_empty() {
        println!("⚠️  {}", text.hardware_some_issues());
    } else {
        println!("❌ {}", text.hardware_not_recommended());
    }
    println!("============================================================");
    println!();
    println!("{} {:.2}s", text.done_in(), start_time.elapsed().as_secs_f64());
    println!("============================================================");
}

fn print_cpu_result(result: &stress::CpuTestResult, text: &Text) -> (bool, Vec<String>) {
    let ops_str = format_number(result.operations);
    let ops_sec_str = format!("{:.0}", result.ops_per_second);
    let time_str = format!("{:.3}ms", result.avg_op_time_ms);
    let var_str = format!("{:.1}%", result.variance_pct);

    // Temperature display
    let temp_str = if let Some(temp) = &result.temperature {
        format!("{:.1}°C", temp.current)
    } else {
        "N/A".to_string()
    };

    // Frequency display
    let freq_start_str = format!("{:.2} GHz", result.frequency_start.current_ghz);
    let freq_end_str = format!("{:.2} GHz", result.frequency_end.current_ghz);
    let freq_drop_str = if result.freq_drop_pct > 1.0 {
        format!("(-{:.0}%)", result.freq_drop_pct)
    } else {
        String::new()
    };

    let (status_icon, healthy, issues) = match &result.health {
        HealthStatus::Healthy => ("✅", true, vec![]),
        HealthStatus::IssuesDetected(issues) => ("⚠️", false, issues.clone()),
        HealthStatus::Failed(msg) => ("❌", false, vec![msg.clone()]),
    };

    // Calculate header padding
    let header_text = text.cpu_health_check();
    let header_len = header_text.chars().count();
    let header_padding = 52 - header_len - 4; // 4 for emoji + spaces

    println!("┌──────────────────────────────────────────────────────────┐");
    println!("│ 🧠 {} {:>width$} │", header_text, status_icon, width = header_padding + 2);
    println!("├──────────────────────────────────────────────────────────┤");
    println!("{}", table_row(text.operations(), &ops_str));
    println!("{}", table_row(text.ops_per_sec(), &ops_sec_str));
    println!("{}", table_row(text.avg_op_time(), &time_str));
    println!("{}", table_row(text.variance(), &var_str));
    println!("{}", table_row("temperature", &temp_str));

    // Frequency row is special (has arrow + optional drop)
    let freq_label = "frequency";
    let freq_value = if freq_drop_str.is_empty() {
        format!("{} -> {}", freq_start_str, freq_end_str)
    } else {
        format!("{} -> {} {}", freq_start_str, freq_end_str, freq_drop_str)
    };
    println!("{}", table_row(freq_label, &freq_value));
    println!("└──────────────────────────────────────────────────────────┘");

    (healthy, issues)
}

// Simple number formatter with thousands separator
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }
    result
}

/// Format a table row with proper alignment
/// Box width is 52 chars internally (between │ borders)
/// Format: │ label: value │ where value is right-aligned
fn table_row(label: &str, value: &str) -> String {
    const BOX_WIDTH: usize = 52;  // Internal width between borders
    let label_len = label.chars().count();
    let value_len = value.chars().count();

    // Calculate padding: BOX_WIDTH - label_len - ": " - value_len
    let padding = BOX_WIDTH.saturating_sub(label_len + 2 + value_len);
    format!("│ {}: {:>padding$} │", label, value, padding = value_len + padding)
}

fn print_ram_result(result: &stress::RamTestResult, text: &Text) -> (bool, Vec<String>) {
    let (status_icon, healthy, issues) = match &result.health {
        HealthStatus::Healthy => ("✅", true, vec![]),
        HealthStatus::IssuesDetected(issues) => ("⚠️", false, issues.clone()),
        HealthStatus::Failed(msg) => ("❌", false, vec![msg.clone()]),
    };

    // Calculate header padding
    let header_text = text.ram_health_check();
    let header_len = header_text.chars().count();
    let header_padding = 52 - header_len - 4; // 4 for emoji + spaces

    println!("┌──────────────────────────────────────────────────────────┐");
    println!("│ 💾 {} {:>width$} │", header_text, status_icon, width = header_padding + 2);
    println!("├──────────────────────────────────────────────────────────┤");
    println!("{}", table_row(text.tested_gb(), &format!("{:.1} GB", result.tested_gb)));
    println!("{}", table_row(text.write_speed(), &format!("{:.1} GB/s", result.write_speed_gb_s)));
    println!("{}", table_row(text.read_speed(), &format!("{:.1} GB/s", result.read_speed_gb_s)));
    println!("{}", table_row(text.errors_detected(), &format!("{}", result.errors)));
    println!("└──────────────────────────────────────────────────────────┘");

    (healthy, issues)
}
