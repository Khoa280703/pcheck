# Rust Hardware Detection Research Report
**Date**: 2025-12-25
**Project**: pchecker - Cross-platform hardware detection CLI
**Researcher**: researcher subagent

## Executive Summary

Building a cross-platform Rust CLI for hardware detection (CPU, GPU, RAM, Disk) requires a **multi-crate strategy**. No single crate covers all needs. GPU detection is the primary challenge - `sysinfo` does **not** support GPU detection.

---

## 1. Recommended Crates

### Core System Info (CPU, RAM, Disk)

| Crate | Version | Purpose | Platforms |
|-------|---------|---------|-----------|
| **sysinfo** | 0.37+ | CPU, RAM, Disk, Network, Processes | Win, macOS, Linux, FreeBSD, iOS, Android |
| **hw** | 0.2+ | Rich hardware metrics (CPU, GPU, drives) | Cross-platform |
| **hardware_query** | latest | Simple hardware detection API | Cross-platform |

**sysinfo** is the mature, battle-tested choice. **hw** offers richer metrics but less mature.

### GPU Detection (Critical Gap)

| Crate | Version | GPU Support | Platforms | Notes |
|-------|---------|-------------|-----------|-------|
| **all-smi** | 0.7+ | NVIDIA, AMD, Apple Silicon, Jetson | Win, macOS, Linux | **Best for detection** |
| **wgpu** | 24.0+ | Vulkan, Metal, D3D12, OpenGL | Cross-platform | Graphics-focused, heavy |
| **gfxinfo** | latest | NVIDIA, AMD, Intel (feature flags) | Cross-platform | Modular flags |

**sysinfo DOES NOT support GPU** - confirmed 2024.

### CLI Parsing

| Crate | Version | Notes |
|-------|---------|-------|
| **clap** | 4.x | Industry standard, use Derive API |

---

## 2. Crate Deep Dives

### sysinfo (0.37+)

**Strengths:**
- Active development, 2025 updates
- Cross-platform (Windows, macOS, Linux, FreeBSD, iOS, Android)
- CPU: usage, frequency, vendor, cores
- RAM: total, free, available, swap
- Disks: type, filesystem, space, removable

**Limitations:**
- **NO GPU SUPPORT** (critical gap)
- CPU frequency: required fixes for M4 Macs
- Multi-threaded by default (higher memory on macOS)
- Android: restricted for non-system apps
- Virtual memory reporting can confuse

**Gotchas:**
- Must call `refresh()` twice for accurate CPU usage
- Distinguish "free" vs "available" RAM
- WASM not supported

### all-smi (0.7+) - GPU Detection

**Strengths:**
- Cross-platform GPU monitoring
- NVIDIA, AMD, Apple Silicon, Jetson support
- Real-time: utilization, memory, temperature, power
- Alternative to nvidia-smi

**Limitations:**
- GPU-focused (not general hardware)
- May require vendor-specific drivers
- Less mature than sysinfo

### wgpu (24.0+) - GPU Abstraction

**Strengths:**
- Pure Rust, safe, cross-platform
- Automatic backend detection (Vulkan/Metal/D3D12/OpenGL)
- WebGPU support

**Limitations:**
- Graphics/rendering focused (overkill for info-only)
- Heavy dependency (shader translation via Naga)
- May require GPU context creation for detection

**Use if:** You need GPU enumeration + future graphics capabilities

---

## 3. Platform-Specific GPU Detection

### Windows
- **NVIDIA**: NVML wrappers, `all-smi`
- **AMD**: ADLX, `all-smi`
- **Intel**: Intel GPU computing APIs
- **DirectX/D3D12**: Device enumeration via DXGI

### macOS
- **Apple Silicon**: Metal API via `metal` crate or `objc` bindings
- **Unified Memory**: GPU/CPU share memory pool
- **Recommended**: `all-smi` (has Apple Silicon support)

### Linux
- **NVIDIA**: NVML via CUDA wrappers
- **AMD**: DRM, AMDGPU kernel interfaces
- **Intel**: i915 DRM interfaces
- **Vulkan**: Device enumeration via ash/vulkan-rs

---

## 4. CLI Best Practices (clap 4.x)

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "pchecker")]
#[command(about = "Hardware detection CLI", long_about = None)]
struct Args {
    /// Show CPU information
    #[arg(short, long)]
    cpu: bool,

    /// Show GPU information
    #[arg(short, long)]
    gpu: bool,

    /// Show RAM information
    #[arg(short, long)]
    ram: bool,

    /// Show all information
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let args = Args::parse();
    // Handle logic
}
```

**Best Practices:**
- Use **Derive API** (type-safe, cleaner)
- Subcommands for modular design
- Custom validators for input
- `Option<T>` for optional args, `Vec<T>` for multiple

---

## 5. Single Binary & Static Linking

### Targets

| Platform | Target Triple | Static Linking |
|----------|---------------|----------------|
| Linux | `x86_64-unknown-linux-musl` | Fully static via musl |
| Windows | `x86_64-pc-windows-msvc` | Partial (CRT dependencies) |
| macOS | `x86_64-apple-darwin`, `aarch64-apple-darwin` | Default is static-ish |

### Tools

| Tool | Purpose | Limitations |
|------|---------|-------------|
| **cargo-zigbuild** | Zig linker for cross-compilation | Linux/macOS targets only (2024) |
| **cross** | Docker-based cross-compilation | Requires Docker |
| **eyra** | Pure Rust static linking | Experimental, pure-Rust only |

### Cross-Compilation from macOS M4

**To Linux (Recommended):**
```bash
# Install musl cross-compiler
brew install musl-cross

# Build static binary
cargo build --release --target x86_64-unknown-linux-musl
```

**To Windows:**
```bash
# Install Windows target
rustup target add x86_64-pc-windows-msvc

# Build (partial static)
cargo build --release --target x86_64-pc-windows-msvc
```

**Gotchas:**
- OpenSSL: use `vendored` feature flag in dependencies
- C dependencies: require cross-compilation toolchains
- cargo-zigbuild doesn't support Windows from macOS (2024)
- M4 to x86_64 requires Rosetta or cross-toolchain

### Binary Size Optimization

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit
strip = true         # Strip symbols
panic = "abort"      # Remove panic unwinding code
```

---

## 6. Recommended Architecture

```rust
// lib.rs
pub mod cpu;
pub mod gpu;
pub mod ram;
pub mod disk;

// cpu.rs - uses sysinfo
pub fn get_cpu_info() -> CpuInfo {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    // Extract CPU details
}

// gpu.rs - uses all-smi or platform-specific
pub fn get_gpu_info() -> Vec<GpuInfo> {
    #[cfg(target_os = "windows")]
    return windows_gpu_detection();
    #[cfg(target_os = "macos")]
    return macos_gpu_detection();
    #[cfg(target_os = "linux")]
    return linux_gpu_detection();
}
```

---

## 7. Dependency Recommendations

```toml
[dependencies]
sysinfo = "0.37"           # CPU, RAM, Disk
all-smi = "0.7"            # GPU detection (primary)
clap = { version = "4.5", features = ["derive"] }

# Optional: richer GPU enumeration
#wgpu = "24"               # Only if need graphics context

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

---

## 8. Code Example

```rust
use clap::Parser;
use sysinfo::System;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let args = Args::parse();
    let mut sys = System::new_all();
    sys.refresh_all();

    if args.all {
        println!("CPUs: {}", sys.cpus().len());
        println!("Total RAM: {} MB", sys.total_memory() / 1024);
        println!("Disks: {:?}", sys.disks());
        // GPU: call all-smi or platform-specific code
    }
}
```

---

## 9. Unresolved Questions

1. **GPU Detection on Windows without Admin**: Can `all-smi` detect GPUs without elevated privileges?
2. **Apple Silicon GPU Memory**: How to extract unified memory pool allocation for GPU?
3. **Linux GPU Driverless**: Can we detect GPUs without vendor drivers installed?
4. **Single Binary Windows**: True static linking on Windows (avoiding CRT dependency)?
5. **Cross-Compilation M4→Win**: Working toolchain for Windows builds on M4 macOS?

---

## Sources

- [sysinfo crate](https://crates.io/crates/sysinfo)
- [sysinfo CHANGELOG](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)
- [sysinfo System docs](https://docs.rs/sysinfo/latest/sysinfo/struct.System.html)
- [all-smi crate](https://crates.io/crates/all-smi)
- [all-smi lib.rs](https://lib.rs/crates/all-smi)
- [wgpu GitHub](https://github.com/gfx-rs/wgpu)
- [wgpu crate](https://crates.io/crates/wgpu/24.0.5)
- [gfxinfo crate](https://lib.rs/crates/gfxinfo)
- [hw crate](https://docs.rs/crate/hw/latest)
- [clap 4.0 release](https://epage.github.io/blog/2022/09/clap4/)
- [cargo-zigbuild GitHub](https://github.com/rust-cross/cargo-zigbuild)
- [cargo-zigbuild crate](https://crates.io/crates/cargo-zigbuild/0.19.5)
- [eyra crate](https://crates.io/crates/eyra)
- [Cross-compilation guide 2024](https://sebi.io/posts/2024-05-02-guide-cross-compiling-rust-from-macos-to-raspberry-pi-2024-apple-silicon/)
- [Rust-GPU blog](https://rust-gpu.github.io/blog/2025/07/25/rust-on-every-gpu/)
- [LogRocket wgpu tutorial](https://blog.logrocket.com/rust-wgpu-cross-platform-graphics/)
- [metal-candle crate](https://lib.rs/crates/metal-candle)
- [GPU type Rust forum discussion](https://users.rust-lang.org/t/gpu-type-and-driver-version/77247)
