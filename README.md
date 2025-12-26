# PChecker

**Cross-platform hardware detection and health check CLI tool**

PChecker detects system hardware information and runs stress tests to identify potential hardware issues. Supports macOS (Apple Silicon), Windows, and Linux.

**Version:** 0.2.0
**Repository:** https://github.com/Khoa280703/pcheck
**License:** MIT

---

## Features

### Hardware Detection (Info Mode)
- **CPU:** Model name and core count
- **GPU:** Model name and VRAM (macOS), platform-specific detection
- **RAM:** Total, used, and free memory
- **Disk:** Disk name and total capacity
- **Platform:** Automatic detection (macOS/Windows/Linux)

### Health Check (Stress Mode)
- **CPU Stress Test:** Multi-threaded prime calculation to detect instability, overheating, and throttling
- **RAM Stress Test:** Memory allocation with write/read verification to detect faulty RAM
- **Disk Stress Test:** Read/write speed testing with SMART data (optional)
- **GPU Stress Test:** wgpu-based compute shader testing (optional, requires feature flag)
- **Health Evaluation:** Automatic assessment with detailed metrics
- **Temperature Monitoring:** Real-time CPU/GPU temperature tracking
- **Frequency Tracking:** Detects thermal throttling via frequency drops

### Verbose Mode (New in v0.2.0)
- **Per-Core Usage Display:** Visual bar charts for each CPU core
- **Platform-Specific Formatting:**
  - **macOS:** `C00: [██████████] 100%` (4 cores/row)
  - **Windows/Linux:** `C00: [██████████] 100% @4.5GHz` (3 cores/row)
- **Temperature Sensors:** Detailed list (up to 8 sensors)
- **Real-Time Updates:** Every second during stress test

### Platform-Specific Features
- **macOS:** 4 cores/row in verbose mode, frequency average only
- **Windows:** 3 cores/row with per-core frequency
- **Linux:** 3 cores/row with per-core frequency

### Multi-Language Support
- Vietnamese (default)
- English
- Interactive selection at startup

---

## Installation

### Build from Source

```bash
# Clone repository
git clone https://github.com/Khoa280703/pcheck.git
cd pcheck

# Build release binary
cargo build --release

# Binary location: target/release/pchecker
```

### Requirements
- Rust 1.70+ (edition 2021)
- Dependencies: `sysinfo`, `clap`, `num_cpus`, `fastrand`
- Optional: `wgpu`, `pollster`, `bytemuck` (for GPU compute test)
- Optional: `smc` (for Apple SMC temperature reading on macOS)

---

## Usage

### Basic Commands

```bash
# Info mode (default) - Detect and display hardware info
pchecker

# Health check mode (CPU + RAM + Disk + GPU)
pchecker --stress

# Run individual stress tests
pchecker --cpu-stress              # CPU only
pchecker --ram-stress              # RAM only
pchecker --disk-stress             # Disk only
pchecker --gpu-stress              # GPU only (requires feature flag)

# Health check with custom duration
pchecker --stress --duration 120

# Quick health check (15 seconds)
pchecker --stress --quick

# Verbose mode - Show detailed per-core metrics
pchecker --stress --verbose

# Disk-specific options
pchecker --list-disks              # List all available disks
pchecker --disk-stress --all-disks # Test all disks
pchecker --disk-stress --disk-index 1  # Test specific disk

# Combined flags (short form)
pchecker -s -d 30 -v
```

### CLI Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--stress` | `-s` | Run all health checks (CPU + RAM + Disk + GPU) | - |
| `--cpu-stress` | - | Run CPU stress test only | - |
| `--ram-stress` | - | Run RAM stress test only | - |
| `--disk-stress` | - | Run Disk stress test only | - |
| `--gpu-stress` | - | Run GPU stress test only | - |
| `--duration` | `-d` | Test duration in seconds | 60 |
| `--quick` | - | Quick health check (15s) | - |
| `--verbose` | `-v` | Show detailed metrics | - |
| `--all-disks` | - | Test all disks (disk stress) | First disk only |
| `--disk-index` | - | Test specific disk by index | - |
| `--list-disks` | - | List available disks and exit | - |

### Output Modes

#### Normal Mode (Default)
Single progress line showing:
- Overall CPU status (progress bar, operations count)
- Temperature and frequency
- Clean, minimal output

#### Verbose Mode (`--verbose`)
- Per-core usage with visual bar charts
- Temperature sensors list
- Platform-specific formatting (4 cores/row on macOS, 3 on Windows/Linux)
- Updates every second

---

## Health Evaluation Criteria

### CPU Health Rules

| Condition | Result |
|-----------|--------|
| Thread panicked (crash) | `Failed: CPU crashed during test - FAULTY HARDWARE` |
| Temperature > 95°C | `Failed: CPU overheating - cooling system failure` |
| Variance > 200% | `Failed: Extreme instability detected - possible CPU fault` |
| Temperature > 85°C | `IssuesDetected: CPU running hot - check cooling` |
| Frequency drop > 10% | `IssuesDetected: CPU throttled - thermal/power limit` |
| Otherwise | `Healthy` |

### RAM Health Rules

| Condition | Result |
|-----------|--------|
| Any errors > 0 | `Failed: Memory errors detected - BAD RAM` |
| Allocation < 0.1 GB | `Failed: Memory allocation failed` |
| Write speed < 0.3 GB/s | `Failed: Extremely low write speed - faulty RAM` |
| Read speed < 0.3 GB/s | `Failed: Extremely low read speed - faulty RAM` |
| Otherwise | `Healthy` |

---

## Platform Support

### macOS (Apple Silicon)
- GPU: `system_profiler SPDisplaysDataType`
- CPU: 4 cores/row in verbose mode
- Frequency: Average only
- Temperature: PMU tdie components

### Windows
- GPU: PowerShell `Get-WmiObject Win32_VideoController`
- CPU: 3 cores/row with frequency
- VRAM detection: TODO

### Linux
- GPU: `lspci -vnnn`
- CPU: 3 cores/row with frequency
- VRAM detection: TODO via `/sys`

---

## Examples

### Hardware Info Detection
```bash
$ pchecker
# Interactive language selection
# Displays CPU, GPU, RAM, Disk, Platform info
```

### Standard Health Check
```bash
$ pchecker --stress
# Runs 60s CPU test + 30s RAM test
# Shows overall health status
```

### Quick Test with Verbose Output
```bash
$ pchecker --stress --quick --verbose
# 15s quick test with per-core metrics
# Visual bar charts for each core
# Detailed temperature sensor list
```

---

## Development

### Running Tests
```bash
cargo test
```

### Building Release Binary
```bash
cargo build --release
```

### Code Structure
```
pcheck/              # Project root
├── src/
│   ├── main.rs      # CLI entry point, argument parsing
│   ├── hw/          # Hardware detection with platform modules
│   │   ├── cpu/     # CPU detection + platform/{macos,windows,linux}.rs
│   │   ├── gpu/     # GPU detection + platform/{macos,windows,linux}.rs
│   │   ├── ram/     # RAM detection + platform/{macos,windows,linux}.rs
│   │   └── disk/    # Disk detection + platform/{macos,windows,linux}.rs
│   ├── stress/      # Health tests with platform modules
│   │   ├── cpu/     # CPU test + platform/
│   │   ├── ram/     # RAM test + platform/
│   │   ├── disk/    # Disk test + smart.rs
│   │   ├── gpu.rs   # GPU test (thermal + compute)
│   │   └── gpu_compute.rs  # wgpu-based compute shader test
│   ├── sensors/     # Monitoring (temp, frequency, monitor)
│   ├── platform/    # Platform detection
│   ├── fmt.rs       # Output formatting
│   ├── lang.rs      # Multi-language support
│   └── prompt.rs    # Interactive prompts
├── docs/            # Documentation
├── plans/           # Project plans (active/, completed/)
├── reports/         # Agent reports
├── ROADMAP.md       # Single roadmap file
└── Cargo.toml       # Project manifest
```

---

## Version History

### v0.2.0 (Current)
- Modular platform structure (hw/*/mod.rs + platform/ subdirs)
- GPU compute stress test (wgpu-based, optional feature flag)
- GPU type translations ("Tích hợp"/"Integrated", "Rời"/"Discrete")
- New CLI flags: --cpu-stress, --ram-stress, --disk-stress, --gpu-stress
- Disk selection: --all-disks, --disk-index, --list-disks
- Full Vietnamese translation for all result boxes

### v0.1.0
- Initial release
- Hardware detection (CPU, GPU, RAM, Disk)
- Basic health check mode (CPU, RAM)
- Multi-language support
- Verbose mode with per-core metrics

---

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

---

**Author:** Khoa280703
**Repository:** https://github.com/Khoa280703/pcheck
