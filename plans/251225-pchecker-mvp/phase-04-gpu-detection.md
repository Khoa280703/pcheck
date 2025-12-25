---
title: "Phase 04 - GPU Detection: Platform-Specific Commands"
description: "Implement GPU detection using platform-specific commands (PowerShell, system_profiler, lspci)"
status: pending
priority: P1
effort: 2h
tags: [gpu, platform-specific, parsing]
depends_on: [phase-03-cpu-ram-disk.md]
created: 2025-12-25
---

## Context

GPU detection requires platform-specific commands since no single Rust crate supports all platforms well enough for MVP.

## Overview

Implement robust GPU detection for each platform with proper parsing and error handling. Attempt to get VRAM info where available.

## Requirements

- Detect GPU name on all platforms
- Attempt VRAM detection where possible
- Handle multiple GPUs (show first or all)
- Graceful error handling

## Data Structure

```rust
pub struct GpuInfo {
    pub name: String,
    pub vram: Option<u64>, // bytes
}
```

## Implementation Steps

### Step 1: Update Platform Trait

**src/platform/mod.rs:**
```rust
//! ... existing imports ...

pub struct GpuInfo {
    pub name: String,
    pub vram: Option<u64>,
}

impl GpuInfo {
    pub fn vram_gb(&self) -> Option<f64> {
        self.vram.map(|v| v as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}

/// Platform trait for hardware detection operations
pub trait Platform {
    /// ... existing methods ...

    /// Detect GPU (platform-specific commands)
    fn detect_gpu(&self) -> Result<GpuInfo, String>;
}
```

### Step 2: Update Windows GPU Detection

**src/platform/windows.rs:**
```rust
//! ... existing imports ...

impl Platform for WindowsPlatform {
    // ... existing methods ...

    fn detect_gpu(&self) -> Result<super::GpuInfo, String> {
        // PowerShell WMI query for GPU name and VRAM
        match std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Get-WmiObject Win32_VideoController | Select-Object Name, AdapterRAM | ConvertTo-Json"
            ])
            .output()
        {
            Ok(output) => {
                let json = String::from_utf8_lossy(&output.stdout);

                // Parse JSON (simple parsing, MVP doesn't need serde)
                let name = json
                    .split("\"Name\":")
                    .nth(1)
                    .and_then(|s| s.split(',').next())
                    .and_then(|s| s.split('\"').nth(1))
                    .unwrap_or("Unknown GPU")
                    .to_string();

                let vram = json
                    .split("\"AdapterRAM\":")
                    .nth(1)
                    .and_then(|s| s.split('}').next())
                    .and_then(|s| s.trim().trim_start_matches(',').trim().parse::<u64>().ok());

                Ok(super::GpuInfo { name, vram })
            }
            Err(e) => {
                // Fallback: simpler command
                match std::process::Command::new("wmic")
                    .args(["path", "win32_VideoController", "get", "name"])
                    .output()
                {
                    Ok(output) => {
                        let result = String::from_utf8_lossy(&output.stdout);
                        let name = result.lines()
                            .skip(1) // Skip header
                            .map(|l| l.trim())
                            .filter(|l| !l.is_empty())
                            .next()
                            .unwrap_or("Unknown GPU");

                        Ok(super::GpuInfo {
                            name: name.to_string(),
                            vram: None,
                        })
                    }
                    Err(e) => Err(format!("Failed to detect GPU: {}", e)),
                }
            }
        }
    }
}
```

### Step 3: Update macOS GPU Detection

**src/platform/macos.rs:**
```rust
//! ... existing imports ...

impl Platform for MacosPlatform {
    // ... existing methods ...

    fn detect_gpu(&self) -> Result<super::GpuInfo, String> {
        match std::process::Command::new("system_profiler")
            .args(["SPDisplaysDataType", "-json"])
            .output()
        {
            Ok(output) => {
                let json = String::from_utf8_lossy(&output.stdout);

                // Parse GPU name
                let name = json
                    .split("\"sppci_model\"")
                    .nth(1)
                    .and_then(|s| s.split(',').next())
                    .and_then(|s| s.split('\"').nth(1))
                    .unwrap_or_else(|| {
                        // Fallback to text parsing
                        match std::process::Command::new("system_profiler")
                            .args(["SPDisplaysDataType"])
                            .output()
                        {
                            Ok(text_output) => {
                                let text = String::from_utf8_lossy(&text_output.stdout);
                                text.lines()
                                    .find(|l| l.contains("Chipset Model") || l.contains("GPU"))
                                    .and_then(|l| l.split(':').nth(1))
                                    .map(|s| s.trim())
                                    .unwrap_or("Unknown GPU")
                                    .to_string()
                            }
                            Err(_) => "Unknown GPU".to_string(),
                        }
                    });

                // Parse VRAM
                let vram = json
                    .split("\"sppci_vram_mb\"")
                    .nth(1)
                    .and_then(|s| s.split(',').next())
                    .and_then(|s| s.trim().trim_start_matches(':').trim().parse::<u64>().ok())
                    .map(|mb| mb * 1024 * 1024); // Convert MB to bytes

                Ok(super::GpuInfo { name, vram })
            }
            Err(e) => Err(format!("Failed to detect GPU: {}", e)),
        }
    }
}
```

### Step 4: Update Linux GPU Detection

**src/platform/linux.rs:**
```rust
//! ... existing imports ...

impl Platform for MacosPlatform {
    // ... existing methods ...

    fn detect_gpu(&self) -> Result<super::GpuInfo, String> {
        // Try lspci first
        match std::process::Command::new("lspci")
            .args(["-nn", "-d", "::0300"])
            .output()
        {
            Ok(output) => {
                let lspci_output = String::from_utf8_lossy(&output.stdout);

                // Parse GPU name
                let name = lspci_output
                    .lines()
                    .next()
                    .and_then(|line| {
                        // Format: "00:02.0 VGA compatible controller [0300]: Intel Corporation ..."
                        line.split(':').last()
                    })
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| {
                        // Try broader search
                        match std::process::Command::new("lspci")
                            .arg("-nn")
                            .output()
                        {
                            Ok(broad_output) => {
                                let broad = String::from_utf8_lossy(&broad_output.stdout);
                                broad.lines()
                                    .find(|l| l.contains("VGA") || l.contains("Display"))
                                    .map(|l| l.split(':').last().unwrap_or("Unknown GPU").trim().to_string())
                                    .unwrap_or("Unknown GPU".to_string())
                            }
                            Err(_) => "Unknown GPU".to_string(),
                        }
                    });

                // Try to get VRAM from various sources
                let vram = self.try_get_vram_linux();

                Ok(super::GpuInfo { name, vram })
            }
            Err(e) => Err(format!("Failed to detect GPU: {}", e)),
        }
    }
}

impl LinuxPlatform {
    fn try_get_vram_linux(&self) -> Option<u64> {
        // Try reading from /sys/class/drm/card*/device/mem_info_vram_total
        if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let path = entry.path();
                let vram_path = path.join("device/mem_info_vram_total");
                if vram_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(vram_path) {
                        if let Ok(bytes) = content.trim().parse::<u64>() {
                            return Some(bytes);
                        }
                    }
                }
            }
        }

        // Try glxinfo (not always available)
        if let Ok(output) = std::process::Command::new("glxinfo")
            .args(["-b"])
            .output()
        {
            let glx = String::from_utf8_lossy(&output.stdout);
            for line in glx.lines() {
                if line.contains("Video memory:") {
                    // Parse "Video memory: 2048 MB"
                    if let Some(mb_str) = line.split(':').nth(1) {
                        let mb_str = mb_str.trim().trim_start_matches(|c: char| !c.is_ascii_digit());
                        if let Ok(mb) = mb_str.split_whitespace().next()?.parse::<u64>() {
                            return Some(mb * 1024 * 1024);
                        }
                    }
                }
            }
        }

        None
    }
}
```

### Step 5: Create GPU Module (src/gpu.rs)

```rust
//! GPU information module
//! Wraps platform-specific GPU detection

use crate::platform::{detect, GpuInfo};

/// Get GPU information using platform-specific detection
pub fn get_gpu_info() -> Result<GpuInfo, String> {
    let platform = detect();
    platform.detect_gpu()
}

/// Get all GPUs (MVP: returns single GPU)
pub fn get_all_gpus() -> Vec<GpuInfo> {
    match get_gpu_info() {
        Ok(gpu) => vec![gpu],
        Err(_) => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_detection() {
        let gpu = get_gpu_info();
        assert!(gpu.is_ok());
        let info = gpu.unwrap();
        assert!(!info.name.is_empty());
        println!("Detected GPU: {}", info.name);
    }
}
```

## Success Criteria

- [ ] GPU name detected on all platforms
- [ ] VRAM detected on at least 2 platforms
- [ ] Graceful error handling for missing commands
- [ ] Tests pass on current platform

## Testing Checklist

- [ ] Test on macOS M4 (integrated GPU)
- [ ] Test on Linux (NVIDIA/AMD)
- [ ] Test on Windows (NVIDIA/AMD/Intel)
- [ ] Verify VRAM parsing
- [ ] Test fallback commands

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| JSON parsing fragile | Medium | Low | Add text fallbacks |
| PowerShell execution policy | Low | Medium | Use -ExecutionPolicy Bypass |
| system_profiler JSON format change | Low | Low | Use text parsing fallback |
| lspci not installed | Medium | Low | Return error message |

## Next Steps

→ [Phase 05: Output Formatting](./phase-05-output-formatting.md)

Create clean, formatted output with emojis and sections.

## Todos

- [ ] Update Platform trait with GpuInfo
- [ ] Implement Windows GPU with VRAM
- [ ] Implement macOS GPU with VRAM
- [ ] Implement Linux GPU with VRAM
- [ ] Create gpu.rs module
- [ ] Add error handling
- [ ] Test on current platform
- [ ] Verify VRAM accuracy
