# PChecker

**Cross-platform hardware detection and health check CLI tool**

PChecker detects system hardware information and runs stress tests to identify potential hardware issues. Supports macOS (Apple Silicon), Windows, and Linux.

**Version:** 0.3.0
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
- **Health Evaluation:** Automatic assessment with detailed metrics
- **Temperature Monitoring:** Real-time CPU temperature tracking
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
- Dependencies: `sysinfo`, `clap`, `num_cpus`

---

## Usage

### Basic Commands

```bash
# Info mode (default) - Detect and display hardware info
pchecker

# Health check mode (60s CPU test, 30s RAM test)
pchecker --stress

# Health check with custom duration
pchecker --stress --duration 120

# Quick health check (15 seconds)
pchecker --stress --quick

# Verbose mode - Show detailed per-core metrics
pchecker --stress --verbose

# Combined flags (short form)
pchecker -s -d 30 -v
```

### CLI Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--stress` | `-s` | Run health check mode (CPU + RAM) | - |
| `--duration` | `-d` | Test duration in seconds | 60 |
| `--quick` | - | Quick health check (15s) | - |
| `--verbose` | `-v` | Show detailed per-core metrics | - |

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
│   ├── hw/          # Hardware detection (cpu, ram, disk, gpu)
│   ├── stress/      # Health check (cpu, ram, disk)
│   ├── sensors/     # Monitoring (temp, frequency, monitor)
│   ├── platform/    # Platform-specific code
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

### v0.3.0 (Current)
- Disk health check (read/write speed test)
- SSD vs HDD detection
- Multi-disk support
- Disk type and health-specific evaluation

### v0.2.0
- Added `--verbose` flag for detailed per-core metrics
- Visual bar charts for CPU usage
- Temperature sensors list
- Platform-specific output formatting

### v0.1.0
- Initial release
- Hardware detection (CPU, GPU, RAM, Disk)
- Basic health check mode
- Multi-language support

---

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

---

**Author:** Khoa280703
**Repository:** https://github.com/Khoa280703/pcheck
