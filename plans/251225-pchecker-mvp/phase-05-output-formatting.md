---
title: "Phase 05 - Output Formatting: Display Logic"
description: "Create clean, formatted output with emojis and sections"
status: pending
priority: P1
effort: 1.5h
tags: [formatting, display, ui]
depends_on: [phase-04-gpu-detection.md]
created: 2025-12-25
---

## Context

Need consistent, clean output format that's readable and visually appealing across all platforms.

## Overview

Create formatting module that displays hardware info in a structured way with emojis for visual clarity.

## Requirements

- Simple text output (no TUI)
- Emoji indicators for each section
- Consistent spacing and alignment
- Handle missing/optional data gracefully

## Output Format Design

```
💻 System Information
━━━━━━━━━━━━━━━━━━━━━

🖥️  Platform: macOS
📌 OS: macOS 15.2


⚙️  CPU
━━━━━━━━━━━━━━━━━━━━━
Apple M4
  Cores: 10 (8 performance, 2 efficiency)
  Frequency: 4000 MHz


🧠 RAM
━━━━━━━━━━━━━━━━━━━━━
  Total: 16.00 GB
  Available: 12.50 GB
  Used: 3.50 GB (22%)


💾 Disk
━━━━━━━━━━━━━━━━━━━━━
Macintosh HD (/)
  Total: 512.00 GB
  Available: 342.50 GB
  Used: 169.50 GB (33%)


🎮 GPU
━━━━━━━━━━━━━━━━━━━━━
Apple M4 (Integrated)
  VRAM: Shared (System RAM)
```

## Implementation Steps

### Step 1: Create Formatting Module (src/fmt.rs)

```rust
//! Output formatting for hardware information

use crate::platform::Platform;
use crate::cpu::CpuInfo;
use crate::ram::RamInfo;
use crate::disk::DiskInfo;
use crate::gpu::get_gpu_info;

const SEPARATOR: &str = "━━━━━━━━━━━━━━━━━━━━━";

/// Format and print all system information
pub fn print_system_info(platform: &Box<dyn Platform>) {
    print_header();
    println!();

    print_platform_info(platform);
    println!();

    print_cpu_info();
    println!();

    print_ram_info();
    println!();

    print_disk_info();
    println!();

    print_gpu_info();
}

fn print_header() {
    println!("💻 System Information");
    println!("{}", SEPARATOR);
}

fn print_platform_info(platform: &Box<dyn Platform>) {
    println!("🖥️  Platform: {}", platform.name());
    println!("📌 OS: {}", platform.os_info());
}

fn print_cpu_info() {
    let cpu = crate::cpu::get_cpu_info();

    println!("⚙️  CPU");
    println!("{}", SEPARATOR);
    println!("{}", cpu.model);

    // Print cores with breakdown if available (Apple Silicon)
    if cpu.model.contains("M1") || cpu.model.contains("M2") || cpu.model.contains("M3") || cpu.model.contains("M4") {
        println!("  Cores: {}", cpu.cores);
    } else {
        println!("  Cores: {}", cpu.cores);
    }

    if let Some(freq) = cpu.frequency {
        println!("  Frequency: {} MHz", freq);
    }
}

fn print_ram_info() -> Result<String, String> {
    let ram = crate::ram::get_ram_info();
    let percent = if ram.total > 0 {
        (ram.used as f64 / ram.total as f64 * 100.0) as u32
    } else {
        0
    };

    println!("🧠 RAM");
    println!("{}", SEPARATOR);
    println!("  Total: {:.2} GB", ram.total_gb());
    println!("  Available: {:.2} GB", ram.available_gb());
    println!("  Used: {:.2} GB ({}%)", ram.used_gb(), percent);

    Ok(())
}

fn print_disk_info() {
    println!("💾 Disk");

    if let Some(disk) = crate::disk::get_main_disk_info() {
        let used = disk.total.saturating_sub(disk.available);
        let percent = if disk.total > 0 {
            (used as f64 / disk.total as f64 * 100.0) as u32
        } else {
            0
        };

        println!("{}", SEPARATOR);
        println!("{} ({})", disk.name, disk.mount_point);
        println!("  Total: {:.2} GB", disk.total_gb());
        println!("  Available: {:.2} GB", disk.available_gb());
        println!("  Used: {:.2} GB ({}%)",
            disk.total_gb() - disk.available_gb(),
            percent
        );
    } else {
        println!("{}", SEPARATOR);
        println!("  No disk detected");
    }
}

fn print_gpu_info() {
    println!("🎮 GPU");
    println!("{}", SEPARATOR);

    match get_gpu_info() {
        Ok(gpu) => {
            println!("{}", gpu.name);

            // Format VRAM display
            if let Some(vram_gb) = gpu.vram_gb() {
                if vram_gb < 0.5 {
                    println!("  VRAM: Shared (System RAM)");
                } else {
                    println!("  VRAM: {:.1} GB", vram_gb);
                }
            } else if gpu.name.to_lowercase().contains("integrated") ||
                      gpu.name.to_lowercase().contains("intel") {
                println!("  VRAM: Shared (System RAM)");
            }
            // Else: no VRAM info available, don't print
        }
        Err(e) => {
            println!("  Detection failed: {}", e);
        }
    }
}

/// Compact format (single line)
pub fn print_compact() {
    let platform = crate::platform::detect();
    let cpu = crate::cpu::get_cpu_info();
    let ram = crate::ram::get_ram_info();
    let gpu = get_gpu_info().ok();

    println!("{} | {} | {} | {:.1} GB RAM | {}",
        platform.name(),
        cpu.model,
        cpu.cores,
        ram.total_gb(),
        gpu.map(|g| g.name).unwrap_or_else(|| "Unknown GPU".to_string())
    );
}
```

### Step 2: Update main.rs

```rust
mod platform;
mod cpu;
mod ram;
mod disk;
mod gpu;
mod fmt;

use clap::Parser;
use platform::detect;

#[derive(Parser, Debug)]
#[command(name = "pck")]
#[command(
    name = "pck",
    about = "Cross-platform hardware detection CLI",
    long_about = "Displays basic system information including CPU, GPU, RAM, and Disk details.",
    version = "0.1.0"
)]
struct Args {
    /// Compact output format (single line)
    #[arg(short = 'c', long = "compact")]
    compact: bool,
}

fn main() {
    let args = Args::parse();

    let platform = detect();

    if args.compact {
        fmt::print_compact();
    } else {
        fmt::print_system_info(&platform);
    }
}
```

## Success Criteria

- [ ] Clean output with proper spacing
- [ ] Emojis display correctly
- [ ] Optional data handled gracefully
- [ ] Compact format works
- [ ] No text wrapping issues

## Testing Checklist

- [ ] Test standard output
- [ ] Test compact flag
- [ ] Test with missing GPU info
- [ ] Test emoji rendering on different terminals
- [ ] Verify alignment

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Emojis don't render | Low | Low | Use text fallback option |
| Line wrapping on narrow terminals | Medium | Low | Keep lines < 80 chars |
| Terminal color codes interfere | Very Low | Low | No ANSI codes used |

## Next Steps

→ [Phase 06: Testing & Build](./phase-06-testing-build.md)

Cross-platform testing and binary optimization.

## Todos

- [ ] Create fmt.rs module
- [ ] Implement print functions for each component
- [ ] Add compact format
- [ ] Update main.rs with args
- [ ] Test standard output
- [ ] Test compact flag
- [ ] Verify spacing/alignment
