// pchecker - Cross-platform hardware detection & health check CLI tool
// https://github.com/Khoa280703/pcheck

mod platform;
mod hw;
mod fmt;
mod lang;
mod prompt;
mod stress;
mod sensors;
mod ai;

use std::time::Instant;
use std::io::{self, Write};
use clap::Parser;
use hw::{CpuInfo, RamInfo, DiskInfo, GpuInfo};
use hw::deep::{get_platform_probe, PlatformProbe};
use lang::{Text, Language};
use fmt::{print_header_with_text, print_section, print_footer_with_text};
use stress::{CpuTestConfig, RamTestConfig, DiskTestConfig, HealthStatus};
use ai::AiTechnician;

/// pchecker - Hardware detection and health check tool
#[derive(Parser, Debug)]
#[command(name = "pchecker")]
#[command(version = "0.3.0")]
#[command(about = "Hardware detection and health check tool", long_about = None)]
struct Args {
    /// Show hardware info only (no tests)
    #[arg(long)]
    info: bool,

    /// Run CPU test (optional duration in seconds)
    #[arg(long, value_name = "SECONDS")]
    cpu: Option<u64>,

    /// Run RAM test
    #[arg(long)]
    ram: bool,

    /// Run Disk test (ALL disks)
    #[arg(long)]
    disk: bool,

    /// Run GPU test (optional duration in seconds)
    #[arg(long, value_name = "SECONDS")]
    gpu: Option<u64>,

    /// Run torture test - all components simultaneously (optional duration in seconds)
    #[arg(short = 'a', long, value_name = "SECONDS")]
    all: Option<u64>,
}

fn main() {
    let args = Args::parse();

    // Select language first
    let lang = select_language_standalone();
    let text = Text::new(lang);

    // Determine mode
    let has_component_flags = args.cpu.is_some() || args.ram || args.disk || args.gpu.is_some();
    let is_info_mode = args.info;
    let is_torture_mode = args.all.is_some();
    let is_auto_mode = !is_info_mode && !has_component_flags && !is_torture_mode;

    // Handle --info
    if is_info_mode {
        let ai = AiTechnician::new(text.lang);
        ai.greet(&text);
        run_info_mode_all(&text, &ai);
        return;
    }

    // Handle --all (torture test)
    if is_torture_mode {
        let duration = args.all.unwrap_or(60);
        run_torture_mode(duration, &text);
        return;
    }

    // Handle component-specific tests
    if has_component_flags {
        run_component_tests(&args, &text);
        return;
    }

    // Full auto mode - prompt for level
    if is_auto_mode {
        run_auto_mode(&text);
    }
}

/// Run torture test mode (all components simultaneously)
fn run_torture_mode(duration: u64, text: &Text) {
    let config = stress::torture::TortureConfig {
        duration_secs: duration,
        _verbose: false,
        language: text.lang,
        skip_confirm: false,  // Ask for confirmation when using --all flag
    };

    let _result = stress::torture::run_torture_test(config);
}

/// Run component-specific tests (--cpu, --ram, --disk, --gpu)
fn run_component_tests(args: &Args, text: &Text) {
    let cpu_duration = args.cpu.unwrap_or(60);
    let gpu_duration = args.gpu.unwrap_or(60);

    // Create AI technician for component tests
    let ai = AiTechnician::new(text.lang);

    // Print header
    println!();
    println!("============================================================");
    println!("🧪 PCHECKER {} - v0.3.0", text.health_check());
    println!("============================================================");
    println!();

    let platform_probe = get_platform_probe();

    // CPU: Show deep info before test
    if args.cpu.is_some() {
        show_cpu_deep_info(text, &platform_probe);
    }

    // RAM: Show deep info before test
    if args.ram {
        show_ram_deep_info(text, &platform_probe);
    }

    // Disk: Show deep info before test
    if args.disk {
        show_disk_deep_info(text, &platform_probe);
    }

    // GPU: Show deep info before test
    if args.gpu.is_some() {
        show_gpu_deep_info(text, &platform_probe);
    }

    run_health_check_mode(
        cpu_duration,
        text,
        &ai,
        args.cpu.is_some(),
        args.ram,
        args.disk,
        args.gpu.is_some(),
        gpu_duration,
    );
}

/// Show CPU deep info before test
fn show_cpu_deep_info(text: &Text, probe: &PlatformProbe) {
    let cpu = CpuInfo::new();
    println!("🧠 {} - {}", text.cpu(), cpu.model);
    println!("   {} {}", cpu.cores, text.cores_label());

    if let Some(cache) = probe.get_cache_info() {
        if cache.l1_kb.is_some() || cache.l2_kb.is_some() || cache.l3_kb.is_some() {
            println!("   Cache:");
            if let Some(l1) = cache.l1_kb { println!("     L1: {} KB", l1); }
            if let Some(l2) = cache.l2_kb { println!("     L2: {} KB", l2); }
            if let Some(l3) = cache.l3_kb { println!("     L3: {} KB", l3); }
        }
    }
    if let Some(isa) = probe.get_instruction_sets() {
        println!("   Features: {}", isa.features.join(", "));
    }
    if let Some(tdp) = probe.get_tdp(&cpu.model) {
        println!("   TDP: {} W", tdp);
    }
    println!();
}

/// Show RAM deep info before test
fn show_ram_deep_info(text: &Text, probe: &PlatformProbe) {
    let ram = RamInfo::new();
    println!("💾 {} - {:.1} GB", text.ram(), ram.total_gb);

    let dimm_slots = probe.get_dimm_slots();
    if !dimm_slots.is_empty() {
        println!("   DIMM Slots:");
        for slot in &dimm_slots {
            println!("     - Slot {}: {} GB {} ({})", slot.id, slot.size_gb, slot.type_, slot.bank);
            if let Some(speed) = slot.speed_mhz {
                println!("       Speed: {} MHz", speed);
            }
            if let Some(ref mfr) = slot.manufacturer {
                println!("       Manufacturer: {}", mfr);
            }
            if let Some(ref pn) = slot.part_number {
                println!("       Part Number: {}", pn);
            }
        }
    }
    println!();
}

/// Show disk deep info before test
fn show_disk_deep_info(text: &Text, probe: &PlatformProbe) {
    let disks = DiskInfo::new();
    if disks.len() > 1 {
        for (idx, disk) in disks.iter().enumerate() {
            println!("💿 {} #{} - {}", text.disk(), idx, disk.name);
            println!("   Size: {:.1} GB", disk.total_gb);
        }
    } else if let Some(disk) = disks.first() {
        println!("💿 {} - {}", text.disk(), disk.name);
        println!("   Size: {:.1} GB", disk.total_gb);
    }

    if let Some(health) = probe.get_disk_health() {
        println!("   Health:");
        println!("     Status: {}", health.status);
        if let Some(ref fw) = health.firmware {
            println!("     Firmware: {}", fw);
        }
        if let Some(tbw) = health.tbw {
            println!("     TBW: {:.1} TB", tbw);
        }
        if let Some(hours) = health.hours {
            println!("     Power-On Hours: {}", hours);
        }
        if let Some(pct) = health.percentage_used {
            println!("     Life Used: {}%", pct);
        }
    }
    println!();
}

/// Show GPU deep info before test
fn show_gpu_deep_info(text: &Text, probe: &PlatformProbe) {
    let gpus = GpuInfo::new();
    if gpus.len() > 1 {
        for (idx, gpu) in gpus.iter().enumerate() {
            println!("🎮 {} #{} - {}", text.gpu(), idx, gpu.model);
        }
    } else if let Some(gpu) = gpus.first() {
        println!("🎮 {} - {}", text.gpu(), gpu.model);
    }

    if let Some(driver) = probe.get_gpu_driver() {
        if let Some(metal) = driver.metal {
            println!("   Metal: {}", metal);
        }
    }
    println!();
}

/// Run full auto mode (prompt for level)
fn run_auto_mode(text: &Text) {
    let duration = select_level_prompt(text);

    // Run full test: Info → CPU → RAM → Disk → GPU → Summary
    run_full_auto_test(duration, text);
}

/// Level selection prompt
fn select_level_prompt(text: &Text) -> u64 {
    println!();
    println!("============================================================");
    println!("{} - pchecker v0.3.0", text.select_test_level());
    println!("============================================================");
    println!();
    println!("[1] {} (~90s)", text.level_quick());
    println!("[2] {} (~240s)", text.level_normal());
    println!("[3] {} (~480s)", text.level_deep());
    println!();

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("{} [1-3]: ", text.your_choice());
        io::stdout().flush().unwrap();

        input.clear();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "1" => return 15,
                    "2" => return 60,
                    "3" => return 120,
                    _ => {
                        println!("⚠️  {}", text.invalid_choice());
                        continue;
                    }
                }
            }
            Err(_) => return 60,
        }
    }
}

/// Run full auto test with selected duration
fn run_full_auto_test(duration: u64, text: &Text) {
    // Create AI technician
    let ai = AiTechnician::new(text.lang);

    // AI greeting
    ai.greet(text);

    // First show info
    run_info_mode_all(text, &ai);

    // Then run individual tests
    run_health_check_mode(
        duration,
        text,
        &ai,
        true,  // CPU
        true,  // RAM
        true,  // Disk
        true,  // GPU
        duration,
    );

    // Finally run torture test (test tổng - all components simultaneously)
    println!();
    println!("============================================================");
    println!("⚡ {} - pchecker v0.3.0", text.torture_final());
    println!("============================================================");
    println!();

    let torture_config = stress::torture::TortureConfig {
        duration_secs: duration,
        _verbose: false,
        language: text.lang,
        skip_confirm: true,  // Skip confirmation in auto mode
    };

    let _result = stress::torture::run_torture_test(torture_config);
}

/// Run info mode - show ALL hardware info (including deep info)
fn run_info_mode_all(text: &Text, ai: &AiTechnician) {
    let start_time = Instant::now();

    // Print header
    print_header_with_text("v0.3.0", text.header());

    // Detect platform
    let platform = platform::detect();
    print_section("💻", text.system(), &platform.to_string());

    // AI intro
    ai.intro_detect(text);

    // Get platform probe for deep info
    let platform_probe = get_platform_probe();

    // Detect CPU + Deep Info
    let cpu = CpuInfo::new();
    let cpu_display = format!("{} ({} {})", cpu.model, cpu.cores, text.cores_label());
    print_section("🧠", text.cpu(), &cpu_display);

    // CPU Deep Info
    if let Some(cache) = platform_probe.get_cache_info() {
        if cache.l1_kb.is_some() || cache.l2_kb.is_some() || cache.l3_kb.is_some() {
            println!("   Cache:");
            if let Some(l1) = cache.l1_kb { println!("     L1: {} KB", l1); }
            if let Some(l2) = cache.l2_kb { println!("     L2: {} KB", l2); }
            if let Some(l3) = cache.l3_kb { println!("     L3: {} KB", l3); }
        }
    }
    if let Some(isa) = platform_probe.get_instruction_sets() {
        println!("   Features: {}", isa.features.join(", "));
    }
    if let Some(tdp) = platform_probe.get_tdp(&cpu.model) {
        println!("   TDP: {} W", tdp);
    }
    println!();

    // Detect GPU + Deep Info
    let gpus = GpuInfo::new();
    if gpus.len() > 1 {
        for (idx, gpu) in gpus.iter().enumerate() {
            print_section("🎮", &format!("{} #{}", text.gpu(), idx), &gpu.display_localized(text));
        }
    } else if let Some(gpu) = gpus.first() {
        print_section("🎮", text.gpu(), &gpu.display_localized(text));
    } else {
        print_section("🎮", text.gpu(), text.no_gpu());
    }

    // GPU Deep Info (Metal version)
    if let Some(driver) = platform_probe.get_gpu_driver() {
        if let Some(metal) = driver.metal {
            println!("   Metal: {}", metal);
        }
    }
    println!();

    // Detect RAM + Deep Info
    let ram = RamInfo::new();
    let ram_display = format!("{:.1} GB ({:.1} GB {})", ram.total_gb, ram.used_gb, text.ram_free());
    print_section("💾", text.ram(), &ram_display);

    // RAM Deep Info (DIMM slots)
    let dimm_slots = platform_probe.get_dimm_slots();
    if !dimm_slots.is_empty() {
        println!("   DIMM Slots:");
        for slot in &dimm_slots {
            println!("     - Slot {}: {} GB {} ({})", slot.id, slot.size_gb, slot.type_, slot.bank);
            if let Some(speed) = slot.speed_mhz {
                println!("       Speed: {} MHz", speed);
            }
            if let Some(ref mfr) = slot.manufacturer {
                println!("       Manufacturer: {}", mfr);
            }
            if let Some(ref pn) = slot.part_number {
                println!("       Part Number: {}", pn);
            }
        }
    }
    println!();

    // Detect ALL disks + Deep Info
    let disks = DiskInfo::new();
    if disks.len() > 1 {
        for (idx, disk) in disks.iter().enumerate() {
            print_section("💿", &format!("{} #{}", text.disk(), idx), &disk.display());
        }
    } else if let Some(disk) = disks.first() {
        print_section("💿", text.disk(), &disk.display());
    }

    // Disk Deep Info (Health)
    if let Some(health) = platform_probe.get_disk_health() {
        println!("   Health:");
        println!("     Status: {}", health.status);
        if let Some(ref fw) = health.firmware {
            println!("     Firmware: {}", fw);
        }
        if let Some(tbw) = health.tbw {
            println!("     TBW: {:.1} TB", tbw);
        }
        if let Some(hours) = health.hours {
            println!("     Power-On Hours: {}", hours);
        }
        if let Some(pct) = health.percentage_used {
            println!("     Life Used: {}%", pct);
        }
    }
    println!();

    // AI reaction to specs
    let is_good_config = cpu.cores >= 8 || ram.total_gb >= 16.0;
    ai.react_specs(text, is_good_config);

    // Print footer
    print_footer_with_text(start_time, text.done_in());
}

/// Standalone language selection
fn select_language_standalone() -> Language {
    // Create a default Text for Vietnamese (default language)
    let default_text = Text::new(Language::Vietnamese);

    println!();
    println!("============================================================");
    println!("🤖 PCHECKER v0.2.0");
    println!("============================================================");
    println!();
    println!("{}", default_text.language_select_prompt());
    println!();
    println!("  [1] {}", default_text.language_option_vi());
    println!("  [2] {}", default_text.language_option_en());
    println!();

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("{}", default_text.language_choice_prompt());
        io::stdout().flush().unwrap();

        input.clear();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "1" | "vi" | "VI" | "vietnamese" => return Language::Vietnamese,
                    "2" | "en" | "EN" | "english" => return Language::English,
                    _ => {
                        println!("{}", default_text.language_invalid_choice());
                        continue;
                    }
                }
            }
            Err(_) => return Language::Vietnamese,
        }
    }
}

/// Run health check mode (v0.3.0 feature)
fn run_health_check_mode(duration: u64, text: &Text, ai: &AiTechnician, run_cpu: bool, run_ram: bool, run_disk: bool, run_gpu: bool, gpu_duration: u64) {
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
    let gpu_info_list = GpuInfo::new();

    // Test ALL disks
    let disks_to_test: Vec<(usize, crate::hw::DiskInfo)> = disk_info_list
        .iter()
        .enumerate()
        .map(|(i, d)| (i, d.clone()))
        .collect();

    // CPU Test
    if run_cpu {
        println!("⏳ {} ({}s)", text.testing_cpu(), duration);
        io::stdout().flush().unwrap();

        // Create AI callback for CPU
        let ai_clone = (*ai).clone();
        let cpu_config = CpuTestConfig {
            duration_secs: duration,
            thread_count: None,
            verbose: false,
            on_comment: Some(Box::new(move |msg| {
                ai_clone.comment_realtime(msg);
            })),
        };
        let cpu_result = stress::run_cpu_test(cpu_config, cpu_info.model.clone(), cpu_info.cores);

        let (cpu_healthy, cpu_issues) = print_cpu_result(&cpu_result, text);

        // AI post-test reaction
        let has_warning = matches!(cpu_result.health, HealthStatus::IssuesDetected(_));
        ai.react_result(text, cpu_healthy, has_warning);

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

        // Create AI callback for RAM
        let ai_clone = (*ai).clone();
        let ram_config = RamTestConfig {
            max_gb: None,
            on_comment: Some(Box::new(move |msg| {
                ai_clone.comment_realtime(msg);
            })),
        };
        let ram_result = stress::run_ram_test(ram_config, ram_info.total_gb);

        let (ram_healthy, ram_issues) = print_ram_result(&ram_result, text);

        // AI post-test reaction
        let has_warning = matches!(ram_result.health, HealthStatus::IssuesDetected(_));
        ai.react_result(text, ram_healthy, has_warning);

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
        for (idx, disk_info) in &disks_to_test {
            if disks_to_test.len() > 1 {
                println!("⏳ {} #{} (~30s)", text.testing_disk(), idx);
            } else {
                println!("⏳ {} (~30s)", text.testing_disk());
            }
            io::stdout().flush().unwrap();

            // Create AI callback for Disk
            let ai_clone = (*ai).clone();
            let disk_config = DiskTestConfig {
                test_path: None,
                test_size_mb: 100,
                include_seek_test: true,
                text: text.clone(),
                verbose: false,
                on_comment: Some(Box::new(move |msg| {
                    ai_clone.comment_realtime(msg);
                })),
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

            // AI post-test reaction
            let has_warning = matches!(disk_result.health, HealthStatus::IssuesDetected(_));
            ai.react_result(text, disk_healthy, has_warning);

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

    // GPU Test
    if run_gpu {
        if gpu_info_list.is_empty() {
            println!("⏠️  {}", text.no_gpu());
            println!();
        } else {
            for (idx, gpu_info) in gpu_info_list.iter().enumerate() {
                if gpu_info_list.len() > 1 {
                    println!("⏳ {} #{} (~{}s)", text.testing_gpu(), idx, gpu_duration);
                } else {
                    println!("⏳ {} (~{}s)", text.testing_gpu(), gpu_duration);
                }
                io::stdout().flush().unwrap();

                let gpu_config = stress::GpuTestConfig {
                    duration_secs: gpu_duration,
                    verbose: false,
                };
                let gpu_result = stress::run_gpu_test(
                    gpu_config,
                    gpu_info.model.clone(),
                    gpu_info.gpu_type.as_str().to_string(),
                    gpu_info.vram_gb,
                );

                let (gpu_healthy, gpu_issues) = print_gpu_result(&gpu_result, text);
                if !gpu_healthy {
                    all_healthy = false;
                    if matches!(gpu_result.health, HealthStatus::Failed(_)) {
                        if let HealthStatus::Failed(ref msg) = gpu_result.health {
                            if gpu_info_list.len() > 1 {
                                critical_issues.push(format!("GPU #{} ({}): {}", idx, gpu_info.model, msg));
                            } else {
                                critical_issues.push(format!("GPU: {}", msg));
                            }
                        }
                    }
                }
                all_issues.extend(gpu_issues);
                println!();
            }
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

    // Reset any colors from progress bars before printing result box
    print!("\x1b[0m");

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
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
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
    // Reset any colors from progress bars before printing result box
    print!("\x1b[0m");

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
    // Reset any colors from progress bars before printing result box
    print!("\x1b[0m");

    let (status_icon, healthy, issues) = match &result.health {
        HealthStatus::Healthy => ("✅", true, vec![]),
        HealthStatus::IssuesDetected(issues) => ("⚠️", false, issues.clone()),
        HealthStatus::Failed(msg) => ("❌", false, vec![msg.clone()]),
    };

    // Calculate header padding
    let header_text = text.disk_health_check();
    let header_len = header_text.chars().count();
    let header_padding = 52 - header_len - 4; // 4 for emoji + spaces

    let disk_type = if result.is_ssd { text.ssd() } else { text.hdd() };
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
    println!("{}", table_row(text.disk_label(), &result.disk_name));
    if let Some(ref device) = result.disk_device {
        println!("{}", table_row(text.device(), device));
    }
    println!("{}", table_row(text.size(), &size_str));
    println!("{}", table_row(text.usage(), &usage_str));
    println!("{}", table_row(text.available(), &avail_str));
    println!("{}", table_row(text.fs(), &result.disk_fs));
    println!("{}", table_row(text.type_label(), disk_type));

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
                println!("{}", table_row(text.health(), &format!("{} {}", bar, pct)));
            }

            // SSD life left with bar
            if let Some(life) = smart.ssd_life_left {
                let bar = create_health_bar(life);
                println!("{}", table_row(text.ssd_life(), &format!("{} {}", bar, life)));
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
                println!("{}", table_row(text.serial(), serial));
            }
            if let Some(ref firmware) = smart.firmware {
                println!("{}", table_row(text.firmware(), firmware));
            }
            // Extended SMART attributes
            if let Some(realloc) = smart.realloc_sectors {
                if realloc > 0 {
                    println!("{}", table_row(text.realloc_sectors(), &format!("{}", realloc)));
                }
            }
            if let Some(pending) = smart.pending_sectors {
                if pending > 0 {
                    println!("{}", table_row(text.pending_sectors(), &format!("{}", pending)));
                }
            }
            if let Some(events) = smart.reallocated_events {
                if events > 0 {
                    println!("{}", table_row(text.realloc_events(), &format!("{}", events)));
                }
            }
            // Total bytes written/read
            if let Some(lbas_written) = smart.total_lbas_written {
                let tb_written = (lbas_written as f64 * 512.0) / (1024.0 * 1024.0 * 1024.0 * 1024.0);
                println!("{}", table_row(text.total_written(), &format!("{:.1} TB", tb_written)));
            }
            if let Some(lbas_read) = smart.total_lbas_read {
                let tb_read = (lbas_read as f64 * 512.0) / (1024.0 * 1024.0 * 1024.0 * 1024.0);
                println!("{}", table_row(text.total_read(), &format!("{:.1} TB", tb_read)));
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

fn print_gpu_result(result: &stress::GpuTestResult, text: &Text) -> (bool, Vec<String>) {
    // Reset any colors from progress bars before printing result box
    print!("\x1b[0m");

    let (status_icon, healthy, issues) = match &result.health {
        HealthStatus::Healthy => ("✅", true, vec![]),
        HealthStatus::IssuesDetected(issues) => ("⚠️", false, issues.clone()),
        HealthStatus::Failed(msg) => ("❌", false, vec![msg.clone()]),
    };

    // Calculate header padding
    let header_text = text.gpu_health_check();
    let header_len = header_text.chars().count();
    let header_padding = 52 - header_len - 4; // 4 for emoji + spaces

    // Format VRAM
    let vram_str = if let Some(vram) = result.vram_gb {
        format!("{:.0} GB", vram)
    } else if result.is_apple_silicon {
        text.unified_memory().to_string()
    } else {
        text.not_available().to_string()
    };

    // Format temperature with color coding
    // Priority: SMC temp > powermetrics temp > sysinfo temperature > SoC message > N/A
    let temp_val = result.apple_gpu_metrics
        .as_ref()
        .and_then(|m| m.smc_temperature_c.or(m.temperature_c))
        .or(result.temperature_max)
        .or_else(|| result.temperature_end.as_ref().map(|t| t.current));

    let temp_str = if let Some(temp) = temp_val {
        format!("{:.1}°C", temp)
    } else if result.is_apple_silicon {
        result.apple_gpu_metrics
            .as_ref()
            .and_then(|m| m.thermal_pressure.as_ref().map(|p| {
                match p {
                    stress::gpu::ThermalPressure::Nominal => "✅ Nominal".to_string(),
                    stress::gpu::ThermalPressure::Moderate => "⚠️ Moderate".to_string(),
                    stress::gpu::ThermalPressure::Heavy => "❌ Heavy".to_string(),
                    _ => text.soc_see_cpu().to_string(),
                }
            }))
            .unwrap_or_else(|| text.soc_see_cpu().to_string())
    } else {
        text.not_available().to_string()
    };

    println!("┌──────────────────────────────────────────────────────────┐");
    println!("│ 🎮 {} {:>width$} │", header_text, status_icon, width = header_padding + 2);
    println!("├──────────────────────────────────────────────────────────┤");
    // Hardware info
    println!("{}", table_row(text.model(), &result.gpu_model));
    println!("{}", table_row(text.type_label(), &text.translate_gpu_type(&result.gpu_type)));
    println!("{}", table_row(text.ram(), &vram_str));
    println!("{}", table_row(text.temperature(), &temp_str));

    // Apple Silicon GPU metrics (verbose mode)
    if let Some(ref metrics) = result.apple_gpu_metrics {
        println!("├──────────────────────────────────────────────────────────┤");
        println!("{}", table_row(text.gpu_freq(), &metrics.frequency_mhz.map_or(text.not_available().to_string(), |f| format!("{} MHz", f))));
        println!("{}", table_row(text.gpu_power(), &metrics.power_mw.map_or(text.not_available().to_string(), |p| format!("{} mW", p))));
        println!("{}", table_row(text.gpu_usage(), &metrics.residency_pct.map_or(text.not_available().to_string(), |r| format!("{:.1}%", r))));
        // Show GPU cores if available
        if let Some(cores) = metrics.gpu_cores {
            println!("{}", table_row(text.gpu_cores(), &format!("{}", cores)));
        }
        // Show Metal version if available
        if let Some(ref metal) = metrics.metal_version {
            println!("{}", table_row(text.metal(), metal));
        }
        // Show thermal pressure if available
        if let Some(ref pressure) = metrics.thermal_pressure {
            let pressure_str = match pressure {
                stress::gpu::ThermalPressure::Nominal => "✅ Nominal",
                stress::gpu::ThermalPressure::Moderate => "⚠️ Moderate",
                stress::gpu::ThermalPressure::Heavy => "❌ Heavy",
                stress::gpu::ThermalPressure::Trapping => "🔥 Trapping",
                stress::gpu::ThermalPressure::Sleeping => "💤 Sleeping",
                stress::gpu::ThermalPressure::Unknown => "?",
            };
            println!("{}", table_row(text.thermal_state(), pressure_str));
        }
        // Show SMC temperature if different from powermetrics
        if let Some(smc_temp) = metrics.smc_temperature_c {
            if metrics.temperature_c.is_some() && Some(smc_temp) != metrics.temperature_c {
                println!("{}", table_row(text.smc_temp(), &format!("{:.1}°C", smc_temp)));
            }
        }
    }

    println!("└──────────────────────────────────────────────────────────┘");

    (healthy, issues)
}
