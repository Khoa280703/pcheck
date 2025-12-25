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
use stress::{CpuTestConfig, RamTestConfig, DiskTestConfig, HealthStatus};

/// pchecker - Hardware detection and health check tool
#[derive(Parser, Debug)]
#[command(name = "pchecker")]
#[command(version = "0.3.0")]
#[command(about = "Hardware detection and health check tool", long_about = None)]
struct Args {
    /// Run health check mode (CPU + RAM + Disk)
    #[arg(short, long)]
    stress: bool,

    /// Run CPU stress test only
    #[arg(long)]
    cpu_stress: bool,

    /// Run RAM stress test only
    #[arg(long)]
    ram_stress: bool,

    /// Run Disk stress test only
    #[arg(long)]
    disk_stress: bool,

    /// Test all disks (default: only first/boot disk)
    #[arg(long, conflicts_with = "disk_index")]
    all_disks: bool,

    /// Test specific disk by index (0-based, use --list-disks to see available disks)
    #[arg(long, value_name = "INDEX")]
    disk_index: Option<usize>,

    /// List all available disks and exit
    #[arg(long)]
    list_disks: bool,

    /// Health check duration in seconds (default: 60 for CPU, 30 for RAM)
    #[arg(short, long, default_value = "60")]
    duration: u64,

    /// Quick health check (15 seconds)
    #[arg(long, conflicts_with = "duration")]
    quick: bool,

    /// Verbose output (show detailed per-core metrics and SMART data)
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // Select language first
    let lang = select_language_standalone();
    let text = Text::new(lang);

    // Handle --list-disks
    if args.list_disks {
        list_disks_mode(&text);
        return;
    }

    // Determine duration
    let duration = if args.quick {
        15
    } else {
        args.duration
    };

    // Determine which tests to run
    let run_cpu = args.stress || args.cpu_stress;
    let run_ram = args.stress || args.ram_stress;
    let run_disk = args.stress || args.disk_stress;
    let run_any_test = run_cpu || run_ram || run_disk;

    if run_any_test {
        run_health_check_mode(duration, args.verbose, &text, run_cpu, run_ram, run_disk, args.quick, args.all_disks, args.disk_index);
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

/// List all available disks
fn list_disks_mode(text: &Text) {
    println!();
    println!("============================================================");
    println!("💿 {} - pchecker v0.3.0", text.disk());
    println!("============================================================");
    println!();

    let disks = DiskInfo::new();

    if disks.is_empty() {
        println!("No disks detected.");
        return;
    }

    for (idx, disk) in disks.iter().enumerate() {
        println!("[{}] {}", idx, disk.name);
        println!("    Size: {:.0} GB", disk.total_gb);
        println!("    Used: {:.0} GB / {:.0} GB ({:.0}%)",
            disk.used_gb,
            disk.total_gb,
            (disk.used_gb / disk.total_gb * 100.0)
        );
        println!("    Available: {:.0} GB", disk.available_gb);
        println!("    Mount: {}", disk.mount_point);
        println!();
    }

    println!("Use --disk-stress --disk-index <N> to test specific disk");
    println!("Use --disk-stress --all-disks to test all disks");
}

/// Run health check mode (v0.3.0 feature)
fn run_health_check_mode(duration: u64, verbose: bool, text: &Text, run_cpu: bool, run_ram: bool, run_disk: bool, quick: bool, all_disks: bool, disk_index: Option<usize>) {
    let start_time = Instant::now();

    println!();
    println!("============================================================");
    println!("🧪 PCHECKER {} - v0.3.0", text.health_check());
    println!("============================================================");
    println!();

    let mut all_healthy = true;
    let mut all_issues: Vec<String> = Vec::new();
    let mut critical_issues: Vec<String> = Vec::new();

    // Detect hardware info first
    let cpu_info = CpuInfo::new();
    let ram_info = RamInfo::new();
    let disk_info_list = DiskInfo::new();

    // Determine which disks to test
    let disks_to_test: Vec<(usize, crate::hw::DiskInfo)> = if all_disks {
        disk_info_list.iter().enumerate().map(|(i, d)| (i, d.clone())).collect()
    } else if let Some(idx) = disk_index {
        if idx < disk_info_list.len() {
            vec![(idx, disk_info_list[idx].clone())]
        } else {
            eprintln!("❌ Invalid disk index: {}. Use --list-disks to see available disks.", idx);
            return;
        }
    } else {
        // Default: test first disk only
        disk_info_list.first().map(|d| vec![(0, d.clone())]).unwrap_or_default()
    };

    // CPU Test
    if run_cpu {
        println!("⏳ {} ({}s)", text.testing_cpu(), duration);
        io::stdout().flush().unwrap();

        let cpu_config = CpuTestConfig {
            duration_secs: duration,
            thread_count: None,
            verbose,
        };
        let cpu_result = stress::run_cpu_test(cpu_config, cpu_info.model.clone(), cpu_info.cores);

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
    }

    // RAM Test
    if run_ram {
        let ram_duration = (duration / 2).max(10);
        println!("⏳ {} (~{}s)", text.testing_ram(), ram_duration);
        io::stdout().flush().unwrap();

        let ram_config = RamTestConfig {
            max_gb: None,
        };
        let ram_result = stress::run_ram_test(ram_config, ram_info.total_gb);

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
    }

    // Disk Test
    if run_disk {
        // Quick mode: smaller test size (10MB), no seek test
        let disk_size_mb = if quick { 10 } else { 100 };
        let include_seek = !quick;

        for (idx, disk_info) in &disks_to_test {
            if disks_to_test.len() > 1 {
                println!("⏳ {} #{} (~{}s)", text.testing_disk(), idx, if quick { 5 } else { 30 });
            } else {
                println!("⏳ {} (~{}s)", text.testing_disk(), if quick { 5 } else { 30 });
            }
            io::stdout().flush().unwrap();

            let disk_config = DiskTestConfig {
                test_path: None,
                test_size_mb: disk_size_mb,
                include_seek_test: include_seek,
                verbose,
            };
            let disk_result = stress::run_disk_test(
                disk_config,
                disk_info.name.clone(),
                disk_info.total_gb,
                disk_info.used_gb,
                disk_info.available_gb,
                "APFS".to_string(),
                &disk_info.mount_point,
            );

            let (disk_healthy, disk_issues) = print_disk_result(&disk_result, text);
            if !disk_healthy {
                all_healthy = false;
                if matches!(disk_result.health, HealthStatus::Failed(_)) {
                    if let HealthStatus::Failed(ref msg) = disk_result.health {
                        if disks_to_test.len() > 1 {
                            critical_issues.push(format!("Disk #{} ({}): {}", idx, disk_info.name, msg));
                        } else {
                            critical_issues.push(format!("Disk: {}", msg));
                        }
                    }
                }
            }
            all_issues.extend(disk_issues);
            println!();
        }
    }

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
    // Hardware info
    println!("{}", table_row(text.cpu(), &result.cpu_model));
    println!("{}", table_row(text.cores_label(), &format!("{}", result.cpu_cores)));
    println!("{}", table_row(text.operations(), &ops_str));
    println!("{}", table_row(text.ops_per_sec(), &ops_sec_str));
    println!("{}", table_row(text.avg_op_time(), &time_str));
    println!("{}", table_row(text.variance(), &var_str));
    println!("{}", table_row(text.temperature(), &temp_str));

    // Frequency row is special (has arrow + optional drop)
    let freq_value = if freq_drop_str.is_empty() {
        format!("{} -> {}", freq_start_str, freq_end_str)
    } else {
        format!("{} -> {} {}", freq_start_str, freq_end_str, freq_drop_str)
    };
    println!("{}", table_row(text.frequency(), &freq_value));
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

/// Create a visual health bar (e.g., [██████░░░░] 60%)
/// Green (>70%), Yellow (30-70%), Red (<30%)
fn create_health_bar(percentage: u8) -> String {
    const BAR_WIDTH: usize = 10;
    let filled = (percentage as f64 / 100.0 * BAR_WIDTH as f64).round() as usize;
    let empty = BAR_WIDTH - filled;

    let (open, filled_char, empty_char, close) = if percentage >= 70 {
        ("[", "█", "░", "]")  // Green (normal)
    } else if percentage >= 30 {
        ("[", "▓", "░", "]")  // Yellow (warning)
    } else {
        ("[", "▒", "░", "]")  // Red (critical)
    };

    format!("{}{}{}{}{}", open, filled_char.repeat(filled), empty_char.repeat(empty), close, percentage)
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
    // Hardware info
    println!("{}", table_row(text.ram(), &format!("{:.1} GB", result.ram_total_gb)));
    println!("{}", table_row(text.tested_gb(), &format!("{:.1} GB", result.tested_gb)));
    println!("{}", table_row(text.write_speed(), &format!("{:.1} GB/s", result.write_speed_gb_s)));
    println!("{}", table_row(text.read_speed(), &format!("{:.1} GB/s", result.read_speed_gb_s)));
    println!("{}", table_row(text.errors_detected(), &format!("{}", result.errors)));
    println!("└──────────────────────────────────────────────────────────┘");

    (healthy, issues)
}

fn print_disk_result(result: &stress::DiskTestResult, text: &Text) -> (bool, Vec<String>) {
    let (status_icon, healthy, issues) = match &result.health {
        HealthStatus::Healthy => ("✅", true, vec![]),
        HealthStatus::IssuesDetected(issues) => ("⚠️", false, issues.clone()),
        HealthStatus::Failed(msg) => ("❌", false, vec![msg.clone()]),
    };

    // Calculate header padding
    let header_text = text.disk_health_check();
    let header_len = header_text.chars().count();
    let header_padding = 52 - header_len - 4; // 4 for emoji + spaces

    let disk_type = if result.is_ssd { "SSD" } else { "HDD" };
    let size_str = if result.disk_size_gb >= 1000.0 {
        format!("{:.1} TB", result.disk_size_gb / 1024.0)
    } else {
        format!("{:.0} GB", result.disk_size_gb)
    };
    let usage_str = format!("{:.0} GB / {:.0} GB", result.disk_used_gb, result.disk_size_gb);
    let avail_str = format!("{:.0} GB", result.disk_available_gb);

    let has_smart = result.smart.is_some();
    let verbose = has_smart;

    println!("┌──────────────────────────────────────────────────────────┐");
    println!("│ 💿 {} {:>width$} │", header_text, status_icon, width = header_padding + 2);
    println!("├──────────────────────────────────────────────────────────┤");
    // Hardware info
    println!("{}", table_row("disk", &result.disk_name));
    if let Some(ref device) = result.disk_device {
        println!("{}", table_row(text.device(), device));
    }
    println!("{}", table_row("size", &size_str));
    println!("{}", table_row(text.usage(), &usage_str));
    println!("{}", table_row(text.available(), &avail_str));
    println!("{}", table_row("fs", &result.disk_fs));
    println!("{}", table_row("type", disk_type));

    // Verbose mode: Add separator and SMART section
    if verbose {
        println!("├──────────────────────────────────────────────────────────┤");
        println!("{}", table_row(text.performance_test(), ""));
        println!("{}", table_row(text.write_speed(), &format!("{:.1} MB/s", result.write_speed_mb_s)));
        println!("{}", table_row(text.read_speed(), &format!("{:.1} MB/s", result.read_speed_mb_s)));
        println!("{}", table_row(text.seek_time(), &format!("{:.1} ms", result.seek_time_ms)));
        println!("{}", table_row(text.bad_sectors(), &format!("{}", result.bad_sectors)));

        if let Some(ref smart) = result.smart {
            println!("├──────────────────────────────────────────────────────────┤");
            println!("{}", table_row(text.smart_health(), ""));

            let status_str = match smart.status {
                crate::stress::disk::smart::SmartStatus::Verified => "✅ Verified",
                crate::stress::disk::smart::SmartStatus::Failing => "❌ Failing",
                crate::stress::disk::smart::SmartStatus::Unknown => "? Unknown",
            };
            println!("{}", table_row(text.smart_status(), status_str));

            // Health percentage with bar
            if let Some(pct) = smart.health_percentage {
                let bar = create_health_bar(pct);
                println!("{}", table_row("health", &format!("{} {}", bar, pct)));
            }

            // SSD life left with bar
            if let Some(life) = smart.ssd_life_left {
                let bar = create_health_bar(life);
                println!("{}", table_row("ssd life", &format!("{} {}", bar, life)));
            }

            if let Some(temp) = smart.temperature_c {
                println!("{}", table_row(text.temperature(), &format!("{:.0}°C", temp)));
            }
            if let Some(hours) = smart.power_on_hours {
                println!("{}", table_row(text.power_on_hours(), &format!("{} hrs", hours)));
            }
            if let Some(cycles) = smart.power_cycle_count {
                println!("{}", table_row(text.power_cycles(), &format!("{}", cycles)));
            }
            if let Some(ref model) = smart.model {
                println!("{}", table_row(text.model(), model));
            }
            if let Some(ref serial) = smart.serial {
                println!("{}", table_row("serial", serial));
            }
            if let Some(ref firmware) = smart.firmware {
                println!("{}", table_row("firmware", firmware));
            }
            // Extended SMART attributes
            if let Some(realloc) = smart.realloc_sectors {
                if realloc > 0 {
                    println!("{}", table_row("realloc sectors", &format!("{}", realloc)));
                }
            }
            if let Some(pending) = smart.pending_sectors {
                if pending > 0 {
                    println!("{}", table_row("pending sectors", &format!("{}", pending)));
                }
            }
            if let Some(events) = smart.reallocated_events {
                if events > 0 {
                    println!("{}", table_row("realloc events", &format!("{}", events)));
                }
            }
            // Total bytes written/read
            if let Some(lbas_written) = smart.total_lbas_written {
                let tb_written = (lbas_written as f64 * 512.0) / (1024.0 * 1024.0 * 1024.0 * 1024.0);
                println!("{}", table_row("total written", &format!("{:.1} TB", tb_written)));
            }
            if let Some(lbas_read) = smart.total_lbas_read {
                let tb_read = (lbas_read as f64 * 512.0) / (1024.0 * 1024.0 * 1024.0 * 1024.0);
                println!("{}", table_row("total read", &format!("{:.1} TB", tb_read)));
            }
        }
    } else {
        // Normal mode: just show performance
        println!("{}", table_row(text.write_speed(), &format!("{:.1} MB/s", result.write_speed_mb_s)));
        println!("{}", table_row(text.read_speed(), &format!("{:.1} MB/s", result.read_speed_mb_s)));
        println!("{}", table_row(text.seek_time(), &format!("{:.1} ms", result.seek_time_ms)));
        println!("{}", table_row(text.bad_sectors(), &format!("{}", result.bad_sectors)));
    }

    println!("└──────────────────────────────────────────────────────────┘");

    (healthy, issues)
}
