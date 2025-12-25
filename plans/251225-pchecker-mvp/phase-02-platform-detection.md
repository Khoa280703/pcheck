---
title: "Phase 02 - Platform Detection: OS Detection Module"
description: "Implement runtime platform detection with trait-based architecture"
status: pending
priority: P1
effort: 1.5h
tags: [platform, traits, cfg]
depends_on: [phase-01-mvp-setup.md]
created: 2025-12-25
---

## Context

Hardware detection commands vary by OS. Need a platform abstraction layer to dispatch to correct implementation.

## Overview

Create a `Platform` trait with platform-specific implementations (Windows, macOS, Linux). Detect current platform at runtime and provide info.

## Requirements

- Detect Windows, macOS, Linux at runtime
- Trait-based abstraction for extensibility
- Compile platform-specific code conditionally

## Architecture

```rust
trait Platform {
    fn name(&self) -> &str;
    fn detect_gpu(&self) -> Result<String, String>;
    // GPU uses platform-specific commands
}

struct WindowsPlatform;
struct MacosPlatform;
struct LinuxPlatform;
```

## Implementation Steps

### Step 1: Define Platform Trait

**src/platform/mod.rs:**
```rust
//! Platform detection and abstraction
//!
//! Provides platform-specific implementations for hardware detection.

use std::process::Command;

/// Platform trait for hardware detection operations
pub trait Platform {
    /// Get platform name
    fn name(&self) -> &'static str;

    /// Get detailed OS info
    fn os_info(&self) -> String;

    /// Detect GPU (platform-specific commands)
    fn detect_gpu(&self) -> Result<String, String>;
}

/// Detect current platform at runtime
pub fn detect() -> Box<dyn Platform> {
    #[cfg(target_os = "windows")]
    return Box::new(windows::WindowsPlatform);

    #[cfg(target_os = "macos")]
    return Box::new(macos::MacosPlatform);

    #[cfg(target_os = "linux")]
    return Box::new(linux::LinuxPlatform);

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    compile_error!("Unsupported platform");
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;
```

### Step 2: Implement Windows Platform

**src/platform/windows.rs:**
```rust
//! Windows-specific platform implementation

use super::Platform;

pub struct WindowsPlatform;

impl Platform for WindowsPlatform {
    fn name(&self) -> &'static str {
        "Windows"
    }

    fn os_info(&self) -> String {
        // Use ver command for basic info
        match std::process::Command::new("cmd")
            .args(["/c", "ver"])
            .output()
        {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(_) => "Windows (version unknown)".to_string(),
        }
    }

    fn detect_gpu(&self) -> Result<String, String> {
        // PowerShell WMI query
        match std::process::Command::new("powershell")
            .args([
                "-Command",
                "Get-WmiObject Win32_VideoController | Select-Object -ExpandProperty Name"
            ])
            .output()
        {
            Ok(output) => {
                let gpu = String::from_utf8_lossy(&output.stdout);
                let gpu_name = gpu.lines().next().unwrap_or("Unknown GPU");
                Ok(gpu_name.trim().to_string())
            }
            Err(e) => Err(format!("Failed to detect GPU: {}", e)),
        }
    }
}
```

### Step 3: Implement macOS Platform

**src/platform/macos.rs:**
```rust
//! macOS-specific platform implementation

use super::Platform;

pub struct MacosPlatform;

impl Platform for MacosPlatform {
    fn name(&self) -> &'static str {
        "macOS"
    }

    fn os_info(&self) -> String {
        // Use sw_vers for macOS version
        match std::process::Command::new("sw_vers")
            .arg("-productVersion")
            .output()
        {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout).trim();
                format!("macOS {}", version)
            }
            Err(_) => "macOS (version unknown)".to_string(),
        }
    }

    fn detect_gpu(&self) -> Result<String, String> {
        // Use system_profiler
        match std::process::Command::new("system_profiler")
            .args(["SPDisplaysDataType"])
            .output()
        {
            Ok(output) => {
                let output = String::from_utf8_lossy(&output.stdout);
                // Parse GPU name from output
                for line in output.lines() {
                    if line.contains("Chipset Model") || line.contains("GPU") {
                        if let Some(name) = line.split(':').nth(1) {
                            return Ok(name.trim().to_string());
                        }
                    }
                }
                Ok("Unknown GPU".to_string())
            }
            Err(e) => Err(format!("Failed to detect GPU: {}", e)),
        }
    }
}
```

### Step 4: Implement Linux Platform

**src/platform/linux.rs:**
```rust
//! Linux-specific platform implementation

use super::Platform;

pub struct LinuxPlatform;

impl Platform for LinuxPlatform {
    fn name(&self) -> &'static str {
        "Linux"
    }

    fn os_info(&self) -> String {
        // Try /etc/os-release first
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    if let Some(name) = line.split('=').nth(1) {
                        return name.trim_matches('"').to_string();
                    }
                }
            }
        }
        "Linux (distribution unknown)".to_string()
    }

    fn detect_gpu(&self) -> Result<String, String> {
        // Use lspci
        match std::process::Command::new("lspci")
            .args(["-nn", "-d", "::0300"])
            .output()
        {
            Ok(output) => {
                let output = String::from_utf8_lossy(&output.stdout);
                let first_gpu = output.lines().next().unwrap_or("");
                // Extract GPU name
                if let Some(name) = first_gpu.split(':').last() {
                    Ok(name.trim().to_string())
                } else {
                    Ok("Unknown GPU".to_string())
                }
            }
            Err(e) => Err(format!("Failed to detect GPU: {}", e)),
        }
    }
}
```

### Step 5: Update main.rs

```rust
mod platform;
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
    println!("OS Info: {}", platform.os_info());

    match platform.detect_gpu() {
        Ok(gpu) => println!("GPU: {}", gpu),
        Err(e) => println!("GPU Error: {}", e),
    }
}
```

## Success Criteria

- [ ] Correctly detects Windows/macOS/Linux
- [ ] OS info displays correctly on each platform
- [ ] GPU detection works on at least 1 platform
- [ ] Code compiles with `cfg` attributes
- [ ] No dead code warnings

## Testing Checklist

- [ ] Test on macOS M4
- [ ] Test on Linux
- [ ] Test on Windows (when available)
- [ ] Verify correct platform name
- [ ] Verify OS info accuracy

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| PowerShell not on PATH | Low | Medium | Use full path to powershell.exe |
| system_profiler format changes | Low | Low | Parse flexibly, handle missing data |
| lspci not installed | Medium | Low | Check for command, return error |

## Next Steps

→ [Phase 03: CPU/RAM/Disk Detection](./phase-03-cpu-ram-disk.md)

Implement sysinfo-based hardware detection for CPU, RAM, and Disk.

## Todos

- [ ] Define Platform trait
- [ ] Implement WindowsPlatform
- [ ] Implement MacosPlatform
- [ ] Implement LinuxPlatform
- [ ] Update main.rs with platform detection
- [ ] Test on current platform
- [ ] Verify GPU detection command output
