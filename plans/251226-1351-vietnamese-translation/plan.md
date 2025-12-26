# Vietnamese Translation Plan

**Date:** 2025-12-26
**Status:** Draft

## Overview

Add missing Vietnamese translations for hardcoded English text throughout the codebase.

## New Text Methods to Add

### Labels (src/lang.rs)
```rust
// Disk labels
pub fn disk_label(&self) -> &str { "đĩa" / "disk" }
pub fn size(&self) -> &str { "kích thước" / "size" }
pub fn fs(&self) -> &str { "fs" / "fs" }
pub fn type_label(&self) -> &str { "kiểu" / "type" }
pub fn ssd(&self) -> &str { "SSD" / "SSD" }
pub fn hdd(&self) -> &str { "HDD" / "HDD" }

// GPU labels
pub fn unified_memory(&self) -> &str { "Unified (chia sẻ)" / "Unified (with CPU)" }
pub fn soc_see_cpu(&self) -> &str { "SoC (xem CPU)" / "SoC (see CPU)" }
pub fn gpu_freq(&self) -> &str { "tần số" / "freq" }
pub fn gpu_power(&self) -> &str { "công suất" / "power" }
pub fn gpu_cores(&self) -> &str { "nhân GPU" / "GPU cores" }
pub fn metal(&self) -> &str { "Metal" / "Metal" }

// Common
pub fn not_available(&self) -> &str { "N/A" / "N/A" }
pub fn sensors(&self) -> &str { "Cảm biến" / "Sensors" }
```

## Implementation Steps

1. **Add new Text methods** to `src/lang.rs`
2. **Update main.rs** - replace hardcoded labels in print functions
3. **Update stress modules** - replace hardcoded error messages
4. **Build and test** both languages

## Files to Modify

- `src/lang.rs` - Add ~30 new translation methods
- `src/main.rs` - Use new methods in print_*_result functions
- `src/stress/disk/mod.rs` - Use Text for labels
- `src/stress/gpu/mod.rs` - Use Text for labels
