---
title: "Phase 01 - MVP Setup: Project Initialization"
description: "Initialize Rust project with required dependencies and basic structure"
status: pending
priority: P1
effort: 1h
tags: [setup, rust, cargo]
depends_on: []
created: 2025-12-25
---

## Context

First phase of pchecker MVP. Establish the foundation: Rust project structure, dependencies, and build system.

## Overview

Create a minimal but complete Rust CLI project that compiles and runs "Hello World". Set up Cargo.toml with all required dependencies for the MVP.

## Requirements

- Rust 1.70+ (2021 edition)
- Cargo package manager
- Git repository initialized

## Dependencies

```toml
[dependencies]
sysinfo = "0.37"
clap = { version = "4.5", features = ["derive"] }
```

## Implementation Steps

### Step 1: Create Project Structure

```bash
# Initialize Rust project
cargo new pchecker --name pchecker
cd pchecker

# Create module directories
mkdir -p src/platform
```

**Expected structure:**
```
pchecker/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── src/
    ├── main.rs
    └── platform/
        └── mod.rs
```

### Step 2: Configure Cargo.toml

```toml
[package]
name = "pchecker"
version = "0.1.0"
edition = "2021"
description = "Cross-platform hardware detection CLI"
authors = ["Your Name"]

[dependencies]
sysinfo = "0.37"
clap = { version = "4.5", features = ["derive"] }

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Strip symbols
panic = "abort"     # Reduce binary size
```

### Step 3: Create Placeholder Modules

**src/main.rs:**
```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "pck")]
#[command(about = "Cross-platform hardware detection CLI", long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();
    println!("Hello from pchecker! Hardware detection coming soon...");
}
```

**src/platform/mod.rs:**
```rust
//! Platform detection module
//! Placeholder for platform-specific code

pub fn detect_platform() -> &'static str {
    #[cfg(target_os = "windows")]
    return "Windows";

    #[cfg(target_os = "macos")]
    return "macOS";

    #[cfg(target_os = "linux")]
    return "Linux";

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "Unknown"
}
```

**src/cpu.rs:**
```rust
//! CPU detection module
//! Placeholder

pub fn get_cpu_info() -> String {
    "CPU detection coming soon".to_string()
}
```

**src/ram.rs:**
```rust
//! RAM detection module
//! Placeholder

pub fn get_ram_info() -> String {
    "RAM detection coming soon".to_string()
}
```

**src/disk.rs:**
```rust
//! Disk detection module
//! Placeholder

pub fn get_disk_info() -> String {
    "Disk detection coming soon".to_string()
}
```

**src/gpu.rs:**
```rust
//! GPU detection module
//! Placeholder

pub fn get_gpu_info() -> String {
    "GPU detection coming soon".to_string()
}
```

**src/fmt.rs:**
```rust
//! Output formatting module
//! Placeholder

pub fn format_output(info: &str) -> String {
    info.to_string()
}
```

### Step 4: Initialize Git

```bash
git init
git add .
git commit -m "Initial commit: Project setup with placeholder modules"
```

### Step 5: Verify Build

```bash
# Debug build
cargo run

# Release build (test optimization)
cargo build --release

# Check binary size
ls -lh target/release/pchecker
```

## Success Criteria

- [ ] `cargo run` prints "Hello from pchecker!"
- [ ] `cargo build --release` succeeds
- [ ] All placeholder modules compile without errors
- [ ] Project structure matches specification
- [ ] Binary created successfully

## Testing Checklist

- [ ] Debug build runs
- [ ] Release build runs
- [ ] No compiler warnings
- [ ] Git repository initialized

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Rust not installed | Low | High | Document installation requirement |
| Cargo version mismatch | Low | Low | Use 2021 edition (stable) |

## Next Steps

→ [Phase 02: Platform Detection](./phase-02-platform-detection.md)

Implement OS detection logic with platform traits.

## Todos

- [ ] Create cargo project
- [ ] Add dependencies to Cargo.toml
- [ ] Create all module files
- [ ] Initialize git repo
- [ ] Verify cargo run works
- [ ] Verify release build
