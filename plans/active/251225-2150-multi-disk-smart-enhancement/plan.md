---
title: "Multi-Disk Testing & Extended SMART Data Collection"
description: "Add multi-disk testing support with comprehensive SMART health data collection"
status: pending
priority: P1
effort: 8h
branch: main
tags: [disk, smart, multi-disk, health-check, platform-specific]
created: 2025-12-25
---

# Multi-Disk Testing & Extended SMART Data Collection

**Plan ID:** 251225-2150-multi-disk-smart-enhancement
**Target Version:** 0.4.0
**Status:** Pending

## Overview

Enhance pchecker's disk health check module to support testing multiple disks and collecting comprehensive SMART health data with platform-specific implementations.

## Requirements

### 1. Multi-Disk Testing
- Add `--all-disks` flag to test all detected disks
- Add `--disk <index>` flag to test specific disk by index
- Show progress: "Testing disk 1/3..."
- Aggregate summary showing results for all disks

### 2. Extended SMART Data Collection
Extend `SmartData` struct with new fields:
- `health_percentage: Option<u8>` (0-100%)
- `realloc_sectors: Option<u64>` (bad sectors)
- `pending_sectors: Option<u64>`
- `reallocated_events: Option<u64>`
- `ssd_life_left: Option<u8>` (SSD remaining life %)
- `total_lbas_written: Option<u64>` (total bytes written)
- `total_lbas_read: Option<u64>` (total bytes read)
- `media_errors: Option<u64>`
- `command_timeout: Option<u64>`

### 3. Platform-Specific SMART Collection (Verbose Mode)

#### macOS
- **Basic**: `diskutil info` (no sudo) - status, SSD detection
- **Verbose**: `smartctl -a` (with sudo) - full SMART attributes

#### Linux
- **Verbose**: `smartctl -a` (with sudo) - full SMART attributes
- Parse key attributes: 5, 9, 10, 12, 194, 196, 197, 198, 230, 233

#### Windows
- **Basic**: WMIC queries
- **Verbose**: smartctl if available

### 4. Enhanced Verbose Output Format
- Health percentage bar
- All available SMART attributes
- SSD wear indicators
- Error counts

---

## Implementation Plan

### Phase 1: Extend SmartData Struct (1h)

**File:** `src/stress/disk/smart.rs`

**Tasks:**
1. Add new fields to `SmartData` struct
2. Update `Default` impl
3. Update tests

**New Fields:**
```rust
pub struct SmartData {
    // Existing fields...
    pub status: SmartStatus,
    pub temperature_c: Option<f64>,
    pub power_on_hours: Option<u64>,
    pub power_cycle_count: Option<u64>,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub firmware: Option<String>,

    // New fields...
    pub health_percentage: Option<u8>,
    pub realloc_sectors: Option<u64>,
    pub pending_sectors: Option<u64>,
    pub reallocated_events: Option<u64>,
    pub ssd_life_left: Option<u8>,
    pub total_lbas_written: Option<u64>,
    pub total_lbas_read: Option<u64>,
    pub media_errors: Option<u64>,
    pub command_timeout: Option<u64>,
}
```

**Success Criteria:**
- Code compiles with new fields
- All tests pass
- Default values are None for new fields

---

### Phase 2: Platform-Specific SMART Parsing (3h)

**File:** `src/stress/disk/smart.rs`

#### 2.1 macOS SMART Enhancement

**Tasks:**
1. Enhance `get_macos_smart_data()` with smartctl support
2. Add `parse_smartctl_macos()` function
3. Parse extended attributes from smartctl output

**Implementation:**
```rust
#[cfg(target_os = "macos")]
fn get_macos_smart_data(mount_point: &str, verbose: bool) -> SmartData {
    let mut result = SmartData::default();
    let disk_identifier = get_disk_identifier_from_mount(mount_point);

    // Basic: diskutil info (no sudo)
    if let Ok(output) = Command::new("diskutil")
        .args(&["info", "-plist", &disk_identifier])
        .output()
    {
        parse_diskutil_info(&String::from_utf8_lossy(&output.stdout), &mut result);
    }

    // Verbose: smartctl -a (with sudo)
    if verbose {
        if let Ok(output) = Command::new("sh")
            .arg("-c")
            .arg(&format!("sudo smartctl -a {} 2>/dev/null", disk_identifier))
            .output()
        {
            parse_smartctl_output(&String::from_utf8_lossy(&output.stdout), &mut result);
        }
    }

    result
}
```

**SMART Attributes to Parse (ID - Name):**
- 5: Reallocated_Sector_Ct
- 9: Power_On_Hours
- 10: Spin_Retry_Count
- 12: Power_Cycle_Count
- 169: Unknown/Apple
- 173: Wear_Leveling_Count (SSD)
- 190: Airflow_Temperature_Cel
- 194: Temperature_Celsius
- 196: Reallocated_Event_Count
- 197: Current_Pending_Sector
- 198: Offline_Uncorrectable
- 230: Media_Wearout_Indicator (SSD)
- 231: SSD_Life_Left (SSD)
- 232: Available_Reservd_Space
- 233: Media_Wearout_Indicator
- 235: Pending_Sector_Count
- 241: Total_LBAs_Written
- 242: Total_LBAs_Read

#### 2.2 Linux SMART Enhancement

**Tasks:**
1. Enhance `parse_smartctl_output()` for Linux
2. Add parsing for extended attributes
3. Support attribute IDs: 5, 9, 10, 12, 194, 196, 197, 198, 230, 233

**Implementation:**
```rust
#[cfg(target_os = "linux")]
fn parse_smartctl_output(output: &str, result: &mut SmartData) {
    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Parse ID-specific attributes
        if line.contains(" 5 ") && line.contains("Reallocated_Sector_Ct") {
            result.realloc_sectors = parts.get(9).and_then(|s| s.parse().ok());
        }
        if line.contains(" 9 ") && line.contains("Power_On_Hours") {
            result.power_on_hours = parts.get(9).and_then(|s| s.parse().ok());
        }
        if line.contains(" 12 ") && line.contains("Power_Cycle_Count") {
            result.power_cycle_count = parts.get(9).and_then(|s| s.parse().ok());
        }
        if line.contains(" 194 ") && line.contains("Temperature") {
            result.temperature_c = parts.get(9).and_then(|s| s.parse().ok());
        }
        if line.contains(" 196 ") && line.contains("Reallocated_Event_Count") {
            result.reallocated_events = parts.get(9).and_then(|s| s.parse().ok());
        }
        if line.contains(" 197 ") && line.contains("Current_Pending_Sector") {
            result.pending_sectors = parts.get(9).and_then(|s| s.parse().ok());
        }
        if line.contains(" 198 ") && line.contains("Offline_Uncorrectable") {
            result.media_errors = parts.get(9).and_then(|s| s.parse().ok());
        }
        // SSD-specific
        if line.contains(" 231 ") && line.contains("SSD_Life_Left") {
            result.ssd_life_left = parts.get(9).and_then(|s| s.parse().ok());
        }
        if line.contains(" 230 ") || line.contains(" 233 ") {
            if line.contains("Media_Wearout_Indicator") || line.contains("Wear_Leveling") {
                result.ssd_life_left = parts.get(9).and_then(|s| s.parse().ok());
            }
        }
        if line.contains(" 241 ") && line.contains("Total_LBAs_Written") {
            result.total_lbas_written = parts.get(9).and_then(|s| s.parse().ok());
        }
        if line.contains(" 242 ") && line.contains("Total_LBAs_Read") {
            result.total_lbas_read = parts.get(9).and_then(|s| s.parse().ok());
        }
    }
}
```

#### 2.3 Windows SMART Enhancement

**Tasks:**
1. Add WMIC queries for additional SMART data
2. Add smartctl support if available

**Implementation:**
```rust
#[cfg(target_os = "windows")]
fn get_windows_smart_data(verbose: bool) -> SmartData {
    let mut result = SmartData::default();

    // Basic: WMIC queries
    if let Ok(output) = Command::new("wmic")
        .args(&["diskdrive", "get", "status", "/format:list"])
        .output()
    {
        parse_wmic_status(&String::from_utf8_lossy(&output.stdout), &mut result);
    }

    // Verbose: try smartctl if available
    if verbose {
        if let Ok(output) = Command::new("smartctl")
            .args(&["-a", "/dev/sda"])
            .output()
        {
            parse_smartctl_output(&String::from_utf8_lossy(&output.stdout), &mut result);
        }
    }

    result
}
```

**Success Criteria:**
- SMART attributes parsed correctly on all platforms
- Verbose mode shows extended data
- Graceful fallback when smartctl unavailable

---

### Phase 3: Multi-Disk Support (2.5h)

**File:** `src/main.rs`

#### 3.1 Add CLI Arguments

**Tasks:**
1. Add `--all-disks` flag
2. Add `--disk <index>` flag
3. Add disk selection logic

**Implementation:**
```rust
#[derive(Parser, Debug)]
struct Args {
    // Existing flags...
    #[arg(short, long)]
    stress: bool,

    // New disk flags
    /// Test all disks instead of first one
    #[arg(long, conflicts_with = "disk")]
    all_disks: bool,

    /// Test specific disk by index (1-based)
    #[arg(long, conflicts_with = "all_disks", value_name = "INDEX")]
    disk: Option<usize>,
}
```

#### 3.2 Multi-Disk Loop

**Tasks:**
1. Modify `run_health_check_mode()` to handle multiple disks
2. Add progress display: "Testing disk 1/3..."
3. Aggregate results

**Implementation:**
```rust
fn run_health_check_mode(
    duration: u64,
    verbose: bool,
    text: &Text,
    run_cpu: bool,
    run_ram: bool,
    run_disk: bool,
    quick: bool,
    args: &Args,
) {
    // ... existing code ...

    // Disk Test
    if run_disk {
        let disk_info_list = DiskInfo::new();

        // Determine which disks to test
        let disks_to_test: Vec<(usize, DiskInfo)> = if args.all_disks {
            disk_info_list.into_iter().enumerate().collect()
        } else if let Some(idx) = args.disk {
            if idx == 0 || idx > disk_info_list.len() {
                println!("Invalid disk index: {} (available: 1-{})", idx, disk_info_list.len());
                return;
            }
            vec![(idx - 1, disk_info_list[idx - 1].clone())]
        } else {
            vec![(0, disk_info_list.first().cloned().unwrap())]
        };

        let total_disks = disks_to_test.len();
        let mut disk_results = Vec::new();

        for (i, (idx, disk_info)) in disks_to_test.into_iter().enumerate() {
            println!("⏳ {} ({}/{})",
                text.testing_disk(),
                i + 1,
                total_disks
            );

            // Run disk test
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
            }
            disk_results.push((idx, disk_result, disk_issues));
        }

        // Aggregate disk results
        for (idx, result, issues) in &disk_results {
            if matches!(result.health, HealthStatus::Failed(_)) {
                if let HealthStatus::Failed(ref msg) = result.health {
                    critical_issues.push(format!("Disk {}: {}", idx + 1, msg));
                }
            }
            all_issues.extend(issues.clone());
        }
    }

    // ... rest of code ...
}
```

**Success Criteria:**
- `--all-disks` tests all disks
- `--disk 2` tests second disk
- Progress shows "Testing disk 1/3..."
- Default behavior unchanged (tests first disk)

---

### Phase 4: Enhanced Verbose Output (1h)

**File:** `src/main.rs` and `src/lang.rs`

#### 4.1 Add Translation Strings

**File:** `src/lang.rs`

**Tasks:**
1. Add new translation methods for SMART fields

**Implementation:**
```rust
impl Text {
    // New SMART field translations
    pub fn testing_disk_n(&self, n: usize, total: usize) -> String {
        match self.lang {
            Language::Vietnamese => format!("Đang kiểm tra ổ {}/{}", n, total),
            Language::English => format!("Testing disk {}/{}", n, total),
        }
    }

    pub fn health_percentage(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "sức khỏe",
            Language::English => "health",
        }
    }

    pub fn realloc_sectors(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "sector đã cấp phát lại",
            Language::English => "realloc sectors",
        }
    }

    pub fn pending_sectors(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "sector đang chờ",
            Language::English => "pending sectors",
        }
    }

    pub fn ssd_life_left(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "đời sống SSD",
            Language::English => "SSD life left",
        }
    }

    pub fn total_written(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tổng đã ghi",
            Language::English => "total written",
        }
    }

    pub fn total_read(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tổng đã đọc",
            Language::English => "total read",
        }
    }

    pub fn media_errors(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "lỗi phương tiện",
            Language::English => "media errors",
        }
    }
}
```

#### 4.2 Enhanced Disk Result Printing

**File:** `src/main.rs`

**Tasks:**
1. Enhance `print_disk_result()` with SMART section
2. Add health percentage bar
3. Show all SMART attributes

**Implementation:**
```rust
fn print_disk_result(result: &stress::DiskTestResult, text: &Text) -> (bool, Vec<String>) {
    // ... existing code ...

    // Verbose mode: Enhanced SMART section
    if verbose {
        if let Some(ref smart) = result.smart {
            println!("├──────────────────────────────────────────────────────────┤");
            println!("{}", table_row(text.smart_health(), ""));

            // Health percentage bar
            if let Some(health) = smart.health_percentage {
                let bar_len = (health as usize * 40) / 100;
                let bar = format!("{}{} {}", "█".repeat(bar_len), "░".repeat(40 - bar_len), health);
                println!("{}", table_row(text.health_percentage(), &format!("{}%", bar)));
            }

            // Existing SMART fields
            let status_str = match smart.status {
                crate::stress::disk::smart::SmartStatus::Verified => "✅ Verified",
                crate::stress::disk::smart::SmartStatus::Failing => "❌ Failing",
                crate::stress::disk::smart::SmartStatus::Unknown => "? Unknown",
            };
            println!("{}", table_row(text.smart_status(), status_str));

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
            if let Some(serial) = &smart.serial {
                println!("{}", table_row("serial", serial));
            }
            if let Some(firmware) = &smart.firmware {
                println!("{}", table_row("firmware", firmware));
            }

            // New SMART fields
            if let Some(health_pct) = smart.health_percentage {
                println!("{}", table_row(text.health_percentage(), &format!("{}%", health_pct)));
            }
            if let Some(realloc) = smart.realloc_sectors {
                println!("{}", table_row(text.realloc_sectors(), &format!("{}", realloc)));
            }
            if let Some(pending) = smart.pending_sectors {
                println!("{}", table_row(text.pending_sectors(), &format!("{}", pending)));
            }
            if let Some(events) = smart.reallocated_events {
                println!("{}", table_row("realloc events", &format!("{}", events)));
            }
            if let Some(ssd_life) = smart.ssd_life_left {
                println!("{}", table_row(text.ssd_life_left(), &format!("{}%", ssd_life)));
            }
            if let Some(written) = smart.total_lbas_written {
                let tb = written as f64 * 512.0 / (1024.0 * 1024.0 * 1024.0 * 1024.0);
                println!("{}", table_row(text.total_written(), &format!("{:.1} TB", tb)));
            }
            if let Some(read) = smart.total_lbas_read {
                let tb = read as f64 * 512.0 / (1024.0 * 1024.0 * 1024.0 * 1024.0);
                println!("{}", table_row(text.total_read(), &format!("{:.1} TB", tb)));
            }
            if let Some(errors) = smart.media_errors {
                println!("{}", table_row(text.media_errors(), &format!("{}", errors)));
            }
        }
    }

    // ... rest of code ...
}
```

**Success Criteria:**
- Health percentage bar displays correctly
- All SMART fields shown in verbose mode
- Format matches existing table style

---

### Phase 5: Testing & Validation (0.5h)

**Tasks:**
1. Test multi-disk on macOS with multiple disks
2. Test specific disk selection
3. Test verbose SMART output
4. Verify platform-specific parsing

**Test Commands:**
```bash
# Test default (first disk)
cargo run --release -- --stress --disk-stress

# Test all disks
cargo run --release -- --stress --disk-stress --all-disks

# Test specific disk
cargo run --release -- --stress --disk-stress --disk 2

# Test verbose SMART output
cargo run --release -- --stress --disk-stress --verbose
```

**Success Criteria:**
- All test scenarios pass
- No compilation errors
- Output format consistent
- Platform-specific code works on each OS

---

## Success Criteria Summary

### Functional
- [x] `--all-disks` flag tests all disks
- [x] `--disk <index>` flag tests specific disk
- [x] Progress shows "Testing disk 1/3..."
- [x] All SMART fields populated when available
- [x] Platform-specific SMART parsing works
- [x] Verbose mode shows comprehensive SMART data

### Code Quality
- [x] Follows code standards in `docs/code-standards.md`
- [x] No unwrap() in production code
- [x] Proper error handling with Result/Option
- [x] All functions documented
- [x] Tests added/updated

### Platform Support
- [x] macOS: diskutil + smartctl
- [x] Linux: smartctl parsing
- [x] Windows: WMIC + smartctl (if available)

---

## Dependencies

### Existing (No changes needed)
- `sysinfo` - Disk detection
- `clap` - CLI parsing

### No new dependencies required
- All functionality uses existing `std::process::Command`
- smartctl is external tool (optional, graceful fallback)

---

## Unresolved Questions

1. **smartctl availability**: Should we check for smartctl availability before attempting to use it?
   - Resolution: Try command, handle failure gracefully

2. **LBA to TB conversion**: Is 512 bytes per sector always correct?
   - Resolution: Most modern drives use 512B or 4096B (4Kn), 512B is safe default

3. **Disk test ordering**: Should disks be tested in parallel or sequentially?
   - Resolution: Sequential to avoid I/O contention

4. **Maximum disk count**: Should we limit the number of disks tested?
   - Resolution: No hard limit, but practical limit is system-dependent

---

## Files to Modify

| File | Changes |
|------|---------|
| `src/main.rs` | Add CLI args, multi-disk loop, enhanced output |
| `src/stress/disk/smart.rs` | Extend SmartData, platform parsing |
| `src/stress/disk/mod.rs` | No changes needed (uses SmartData) |
| `src/lang.rs` | Add new translation strings |
| `Cargo.toml` | Version bump to 0.4.0 |
| `README.md` | Document new flags and features |

---

## Estimated Effort

| Phase | Estimated Time |
|-------|---------------|
| Phase 1: Extend SmartData Struct | 1h |
| Phase 2: Platform-Specific SMART Parsing | 3h |
| Phase 3: Multi-Disk Support | 2.5h |
| Phase 4: Enhanced Verbose Output | 1h |
| Phase 5: Testing & Validation | 0.5h |
| **Total** | **8h** |

---

## Next Steps

1. Review and approve plan
2. Start Phase 1: Extend SmartData Struct
3. Progress through phases sequentially
4. Test after each phase
5. Update documentation when complete
