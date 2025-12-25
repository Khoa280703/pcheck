---
title: "Phase 03 - CPU/RAM/Disk Detection: sysinfo Integration"
description: "Implement CPU, RAM, and Disk detection using sysinfo crate"
status: pending
priority: P1
effort: 2h
tags: [sysinfo, hardware, detection]
depends_on: [phase-02-platform-detection.md]
created: 2025-12-25
---

## Context

CPU, RAM, and Disk detection can use the `sysinfo` crate which provides cross-platform APIs for these components.

## Overview

Implement detection modules for CPU, RAM, and Disk using sysinfo 0.37. Each module returns structured data for formatting.

## Requirements

- Use sysinfo 0.37 for cross-platform detection
- Return structured data (not strings) for flexibility
- Handle edge cases (no disks detected, missing info)

## Data Structures

```rust
pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub frequency: Option<u64>, // MHz
}

pub struct RamInfo {
    pub total: u64,      // bytes
    pub available: u64,  // bytes
    pub used: u64,       // bytes
}

pub struct DiskInfo {
    pub name: String,
    pub model: Option<String>,
    pub total: u64,      // bytes
    pub available: u64,  // bytes
    pub file_system: String,
}
```

## Implementation Steps

### Step 1: CPU Module (src/cpu.rs)

```rust
//! CPU information detection using sysinfo

use sysinfo::{System, RefreshKind, CpuRefreshKind};

pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub frequency: Option<u64>,
}

/// Get CPU information
pub fn get_cpu_info() -> CpuInfo {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything())
    );

    sys.refresh_cpu();

    // Get CPU info from first processor
    let processors = sys.cpus();
    let model = if let Some(cpu) = processors.first() {
        cpu.brand().to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let cores = sys.physical_core_count().unwrap_or(0);

    // Try to get CPU frequency (not always available)
    let frequency = processors.first().and_then(|c| {
        let freq = c.frequency();
        if freq > 0 { Some(freq as u64) } else { None }
    });

    CpuInfo { model, cores, frequency }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_info() {
        let info = get_cpu_info();
        assert!(!info.model.is_empty());
        assert!(info.cores > 0);
    }
}
```

### Step 2: RAM Module (src/ram.rs)

```rust
//! RAM information detection using sysinfo

use sysinfo::{System, RefreshKind};

pub struct RamInfo {
    pub total: u64,
    pub available: u64,
    pub used: u64,
}

impl RamInfo {
    /// Convert bytes to GB
    pub fn total_gb(&self) -> f64 {
        self.total as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn available_gb(&self) -> f64 {
        self.available as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn used_gb(&self) -> f64 {
        self.used as f64 / 1024.0 / 1024.0 / 1024.0
    }
}

/// Get RAM information
pub fn get_ram_info() -> RamInfo {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_memory()
    );

    sys.refresh_memory();

    let total = sys.total_memory();
    let available = sys.available_memory();
    let used = total.saturating_sub(available);

    RamInfo { total, available, used }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_info() {
        let info = get_ram_info();
        assert!(info.total > 0);
        assert!(info.used >= 0);
        assert!(info.available <= info.total);
    }
}
```

### Step 3: Disk Module (src/disk.rs)

```rust
//! Disk information detection using sysinfo

use sysinfo::{System, RefreshKind};

pub struct DiskInfo {
    pub name: String,
    pub model: Option<String>,
    pub total: u64,
    pub available: u64,
    pub file_system: String,
    pub mount_point: String,
}

impl DiskInfo {
    /// Convert bytes to GB
    pub fn total_gb(&self) -> f64 {
        self.total as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn available_gb(&self) -> f64 {
        self.available as f64 / 1024.0 / 1024.0 / 1024.0
    }
}

/// Get disk information (main disk where OS is installed)
pub fn get_main_disk_info() -> Option<DiskInfo> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_disks_list()
    );

    sys.refresh_disks_list();

    let disks = sys.disks();

    // Find main disk (root or C:)
    let main_disk = disks.iter().find(|d| {
        let mount = d.mount_point().to_string_lossy();
        #[cfg(target_os = "windows")]
        return mount == "C:\\" || mount.starts_with("C:");

        #[cfg(not(target_os = "windows"))]
        return mount == "/";
    });

    if let Some(disk) = main_disk {
        Some(DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            model: None, // sysinfo doesn't provide model
            total: disk.total_space(),
            available: disk.available_space(),
            file_system: disk.file_system().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
        })
    } else {
        // Fallback to first disk
        disks.first().map(|disk| DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            model: None,
            total: disk.total_space(),
            available: disk.available_space(),
            file_system: disk.file_system().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
        })
    }
}

/// Get all disk information
pub fn get_all_disks() -> Vec<DiskInfo> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_disks_list()
    );

    sys.refresh_disks_list();

    sys.disks().iter().map(|disk| DiskInfo {
        name: disk.name().to_string_lossy().to_string(),
        model: None,
        total: disk.total_space(),
        available: disk.available_space(),
        file_system: disk.file_system().to_string_lossy().to_string(),
        mount_point: disk.mount_point().to_string_lossy().to_string(),
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_disk() {
        let disk = get_main_disk_info();
        assert!(disk.is_some());
        let info = disk.unwrap();
        assert!(info.total > 0);
    }
}
```

### Step 4: Update main.rs

```rust
mod platform;
mod cpu;
mod ram;
mod disk;

use clap::Parser;
use platform::detect;

#[derive(Parser, Debug)]
#[command(name = "pck")]
#[command(about = "Cross-platform hardware detection CLI", long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();

    let platform = detect();
    println!("Platform: {}", platform.name());
    println!("OS: {}", platform.os_info());
    println!();

    // CPU
    let cpu = cpu::get_cpu_info();
    println!("CPU: {}", cpu.model);
    println!("  Cores: {}", cpu.cores);
    if let Some(freq) = cpu.frequency {
        println!("  Frequency: {} MHz", freq);
    }
    println!();

    // RAM
    let ram = ram::get_ram_info();
    println!("RAM: {:.2} GB Total", ram.total_gb());
    println!("  Available: {:.2} GB", ram.available_gb());
    println!("  Used: {:.2} GB", ram.used_gb());
    println!();

    // Disk
    if let Some(disk) = disk::get_main_disk_info() {
        println!("Disk: {} ({})", disk.name, disk.mount_point);
        println!("  Total: {:.2} GB", disk.total_gb());
        println!("  Available: {:.2} GB", disk.available_gb());
    }

    println!();

    // GPU
    match platform.detect_gpu() {
        Ok(gpu) => println!("GPU: {}", gpu),
        Err(e) => println!("GPU Error: {}", e),
    }
}
```

## Success Criteria

- [ ] CPU info displays correctly (model, cores)
- [ ] RAM info displays correctly (total, available)
- [ ] Disk info displays correctly (name, size)
- [ ] All tests pass
- [ ] Works on all 3 platforms

## Testing Checklist

- [ ] Test on macOS M4 (Apple Silicon specific handling)
- [ ] Test on Linux
- [ ] Test on Windows
- [ ] Verify RAM calculations (no overflow)
- [ ] Verify disk detection finds main disk

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| sysinfo returns 0 cores | Low | Medium | Fallback to logical core count |
| RAM overflow on systems >16TB | Very Low | Low | Use u64 (sufficient for 16M TB) |
| Main disk detection fails | Low | Low | Fallback to first disk |

## Next Steps

→ [Phase 04: GPU Detection](./phase-04-gpu-detection.md)

Refine GPU detection with better parsing and VRAM info.

## Todos

- [ ] Implement cpu.rs module
- [ ] Implement ram.rs module
- [ ] Implement disk.rs module
- [ ] Update main.rs with detection calls
- [ ] Test on current platform
- [ ] Verify data accuracy
