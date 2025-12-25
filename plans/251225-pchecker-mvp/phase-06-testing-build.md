---
title: "Phase 06 - Testing & Build: Cross-Platform Validation"
description: "Test on all platforms, optimize binary size, prepare for release"
status: pending
priority: P1
effort: 2h
tags: [testing, build, optimization, release]
depends_on: [phase-05-output-formatting.md]
created: 2025-12-25
---

## Context

Final phase: validate on all target platforms and optimize binary for distribution.

## Overview

Test on macOS M4, Linux, and Windows. Optimize binary size using LTO and strip. Create release artifacts.

## Requirements

- Test on all 3 platforms
- Binary size < 5MB
- All features working correctly
- No panics or errors in normal operation

## Testing Matrix

| Platform | Tester | Status | Notes |
|----------|--------|--------|-------|
| macOS M4 | @khoa2807 | Pending | Main dev machine |
| Linux | @khoa2807 | Pending | Server |
| Windows | @khoa2807 | Pending | When available |

## Implementation Steps

### Step 1: Create Test Checklist

**tests/integration_test.rs:**
```rust
//! Integration tests for pchecker

#[cfg(test)]
mod tests {
    #[test]
    fn test_runs_without_panic() {
        // Basic smoke test
        assert!(true);
    }
}
```

### Step 2: Optimize Cargo.toml

Already configured in Phase 01, verify settings:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Strip symbols
panic = "abort"     # Reduce binary size
```

### Step 3: Build Release Binaries

```bash
# Build release
cargo build --release

# Check binary size
ls -lh target/release/pchecker

# Run release binary
./target/release/pchecker
./target/release/pchecker --compact

# Test all features work
./target/release/pcker --help
```

### Step 4: Platform Testing

**macOS M4 Testing:**
```bash
# Run standard
./target/release/pcker

# Run compact
./target/release/pcker -c

# Verify CPU info (Apple Silicon)
# Expected: Apple M4/M3 with cores
# Expected: Shared GPU VRAM

# Verify RAM info
# Expected: Total GB displayed correctly

# Verify Disk info
# Expected: Macintosh HD detected
```

**Linux Testing:**
```bash
# Copy to Linux machine
scp target/release/pcker user@linux-server:/tmp/

# SSH and test
ssh user@linux-server
cd /tmp
./pcker

# Verify GPU detection (lspci)
# Verify disk detection (/ mount point)
```

**Windows Testing:**
```bash
# Build on Windows or cross-compile
# Run in PowerShell
.\pcker.exe

# Verify GPU detection (PowerShell WMI)
# Verify disk detection (C:\)
```

### Step 5: Create Release Notes Template

**RELEASE_NOTES.md:**
```markdown
# pcker v0.1.0

First MVP release of pcker - cross-platform hardware detection CLI.

## Features
- CPU detection (model, cores, frequency)
- RAM detection (total, available, used)
- Disk detection (name, size, available)
- GPU detection (model, VRAM where available)
- Platform detection (Windows, macOS, Linux)
- Compact output mode (-c flag)

## Usage
```bash
# Standard output
pcker

# Compact output
pcker -c

# Help
pcker --help
```

## Supported Platforms
- macOS 11+ (Intel and Apple Silicon)
- Linux (kernel 3.2+)
- Windows 10+

## Installation
```bash
# Cargo
cargo install pcker

# Manual download
# Download binary from releases, make executable
chmod +x pcker
```

## Known Limitations
- GPU VRAM detection varies by platform
- Disk model not always available
- No stress testing features (info-only)
```

### Step 6: Binary Distribution Setup

```bash
# Create release directory
mkdir -p release

# Create release archives for each platform
# macOS
cp target/release/pcker release/pcker-macos-x64
cp target/release/pcker release/pcker-macos-arm64

# Linux
cp target/release/pcker release/pcker-linux-x64

# Windows
cp target/release/pcker.exe release/pcker-windows-x64.exe

# Create checksums
cd release
shasum -a 256 pcker-* > checksums.txt
```

## Success Criteria

- [ ] All tests pass on current platform
- [ ] Manual testing on at least 2 platforms
- [ ] Binary size < 5MB (expected ~2-3MB)
- [ ] No panics in normal operation
- [ ] All required info displayed correctly

## Testing Checklist

### macOS M4
- [ ] CPU: Apple M4 detected
- [ ] RAM: Total GB correct
- [ ] Disk: Macintosh HD detected
- [ ] GPU: Integrated GPU detected
- [ ] Output: Clean formatting

### Linux
- [ ] CPU: Model and cores correct
- [ ] RAM: Values accurate
- [ ] Disk: / mount point found
- [ ] GPU: lspci detection works
- [ ] Output: Clean formatting

### Windows
- [ ] CPU: Model and cores correct
- [ ] RAM: Values accurate
- [ ] Disk: C:\ detected
- [ ] GPU: WMI detection works
- [ ] Output: Clean formatting

## Binary Size Targets

| Platform | Target | Expected |
|----------|--------|----------|
| macOS (x64) | < 5MB | ~2.5MB |
| macOS (ARM) | < 5MB | ~2.5MB |
| Linux (x64) | < 5MB | ~2.5MB |
| Windows (x64) | < 5MB | ~3MB |

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Binary exceeds 5MB | Low | Low | Add more optimizations |
| GPU detection fails on Windows | Medium | Medium | Fallback to simpler command |
| macOS codesign required | Low | Low | Document unsigned status |
| Windows SmartScreen warning | Low | Medium | Document unsigned status |

## Post-MVP Enhancements (Out of Scope)

- JSON output format
- Stress testing features
- Temperature monitoring
- Network speed test
- Battery information
- Custom themes/colors
- TUI interface
- Install scripts (brew, apt, chocolatey)

## Next Steps

After MVP complete:
1. Create GitHub release
2. Upload binaries
3. Publish to crates.io
4. Write documentation

## Todos

- [ ] Run release build
- [ ] Verify binary size
- [ ] Test on macOS M4
- [ ] Test on Linux
- [ ] Test on Windows
- [ ] Document any platform-specific issues
- [ ] Create release notes
- [ ] Package binaries for distribution
