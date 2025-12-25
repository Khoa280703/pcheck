# PChecker Rust Project Scout Report
**Date:** 2025-12-25  
**Project:** pchecker v0.2.0  
**Location:** /Users/khoa2807/development/pcheck/pchecker  
**Total Lines:** ~1,980 lines of Rust code

---

## 1. PROJECT STRUCTURE

### File Tree with Purposes

```
pchecker/
├── Cargo.toml                    # Project manifest & dependencies
├── Cargo.lock                    # Dependency lock file
├── .gitignore                    # Git ignore patterns
│
├── src/
│   ├── main.rs                   # CLI entry point, argument parsing, orchestration
│   │
│   ├── hw/                       # Hardware detection modules
│   │   ├── mod.rs                # Module exports
│   │   ├── cpu.rs                # CPU model & core detection
│   │   ├── ram.rs                # RAM total/used memory detection
│   │   ├── disk.rs               # Disk storage detection
│   │   └── gpu.rs                # GPU detection (platform-specific)
│   │
│   ├── stress/                   # Health check modules
│   │   ├── mod.rs                # Health status enum, exports
│   │   ├── cpu.rs                # CPU stress test (prime calculation)
│   │   └── ram.rs                # RAM stress test (write/read verify)
│   │
│   ├── sensors/                  # Hardware monitoring
│   │   ├── mod.rs                # Sensor module exports
│   │   ├── temp.rs               # CPU temperature reading (sysinfo Components)
│   │   ├── frequency.rs          # CPU frequency per-core & average
│   │   └── monitor.rs            # Background CPU usage monitor thread
│   │
│   ├── platform/
│   │   └── mod.rs                # Platform detection (macOS/Windows/Linux)
│   │
│   ├── fmt.rs                    # Output formatting, ANSI colors, progress bars
│   ├── lang.rs                   # Multi-language support (Vietnamese/English)
│   └── prompt.rs                 # Interactive language selection (placeholder)
│
└── examples/
    └── test_sysinfo.rs           # sysinfo capability test program
```

---

## 2. MAIN FEATURES & CAPABILITIES

### 2.1 Hardware Detection (Info Mode)
**Purpose:** Detect and display system hardware information

- **CPU:** Model name, core count
- **GPU:** Model name, VRAM (macOS only), platform-specific detection
- **RAM:** Total GB, used GB, free GB
- **Disk:** Disk name, total capacity
- **Platform:** macOS (Apple Silicon), Windows, Linux

### 2.2 Health Check Mode (--stress flag)
**Purpose:** Stress test CPU and RAM to detect hardware issues

#### CPU Test
- Multi-threaded prime number calculation
- Spawns threads = logical CPU cores
- Metrics tracked:
  - Total operations count
  - Operations per second
  - Average operation time (ms)
  - Variance percentage (instability detection)
  - CPU temperature (start/end)
  - CPU frequency (start/end)
  - Frequency drop percentage (throttling detection)

#### RAM Test
- Memory allocation (80% of available, max 16GB)
- Write test: fills buffer with pattern (0xAA55_AA55_AA55_AA55)
- Read test: verifies all data matches pattern
- Metrics tracked:
  - Tested GB
  - Write speed (GB/s)
  - Read speed (GB/s)
  - Error count (any mismatch = BAD RAM)

### 2.3 Verbose Mode (--verbose flag)
- Real-time per-core CPU usage display
- Platform-specific formatting:
  - **macOS:** 4 cores/row, shows usage % only
  - **Windows/Linux:** 3 cores/row, shows usage %@frequency
- Detailed sensor list (up to 8 temperature sensors)
- Updates every 5 seconds to avoid flicker

### 2.4 Language Support
- Vietnamese (default option 1)
- English (option 2)
- Interactive selection at startup
- Affects all output text

---

## 3. DEPENDENCIES & VERSIONS

### Cargo.toml
```toml
[package]
name = "pchecker"
version = "0.2.0"
edition = "2021"
description = "Cross-platform hardware detection CLI tool"
license = "MIT"
repository = "https://github.com/Khoa280703/pcheck"
authors = ["Khoa280703"]

[dependencies]
sysinfo = "0.37"           # System info (CPU, RAM, components)
clap = { version = "4.5", features = ["derive"] }  # CLI parsing
num_cpus = "1.16"          # CPU count detection

[profile.release]
opt-level = "z"            # Optimize for size
lto = true                 # Link-time optimization
codegen-units = 1          # Single codegen unit for better optimization
strip = true               # Strip symbols from binary
panic = "abort"            # Abort on panic (reduces binary size)
```

---

## 4. CLI OPTIONS & FLAGS

### Arguments (via clap derive)
```rust
struct Args {
    /// Run health check mode (CPU + RAM)
    #[arg(short, long)]
    stress: bool,

    /// Health check duration in seconds (default: 60)
    #[arg(short, long, default_value = "60")]
    duration: u64,

    /// Quick health check (15 seconds)
    #[arg(long, conflicts_with = "duration")]
    quick: bool,

    /// Verbose output (show detailed per-core metrics)
    #[arg(short, long)]
    verbose: bool,
}
```

### Usage Examples
```bash
# Info mode (default)
pchecker

# Health check (60s CPU, 30s RAM)
pchecker --stress

# Health check with custom duration
pchecker --stress --duration 120

# Quick health check (15s)
pchecker --stress --quick

# Verbose mode (per-core details)
pchecker --stress --verbose

# Combined flags
pchecker -s -d 30 -v
```

---

## 5. PLATFORM SUPPORT

### 5.1 Platform Detection (Strategy Pattern)
- **File:** `src/platform/mod.rs`
- Uses compile-time `cfg!` macros
- Runtime detection via `detect()` function

### 5.2 Platform-Specific Code

#### macOS (target_os = "macos")
- GPU: Uses `system_profiler SPDisplaysDataType`
- CPU display: 4 cores/row in verbose mode
- Frequency: Shows average only (no per-core)
- Temperature: Reads from PMU tdie components (Apple Silicon)

#### Windows (target_os = "windows")
- GPU: Uses PowerShell `Get-WmiObject Win32_VideoController`
- CPU display: 3 cores/row with frequency
- VRAM detection: TODO comment

#### Linux (target_os = "linux")
- GPU: Uses `lspci -vnnn`
- CPU display: 3 cores/row with frequency
- VRAM detection: TODO comment via `/sys`

---

## 6. HEALTH EVALUATION CRITERIA

### 6.1 CPU Health Rules
**File:** `src/stress/cpu.rs:evaluate_cpu_health()`

| Condition | Result |
|-----------|--------|
| Thread panicked (crash) | `Failed: CPU crashed during test - FAULTY HARDWARE` |
| Temperature > 95°C | `Failed: CPU overheating - cooling system failure` |
| Variance > 200% | `Failed: Extreme instability detected - possible CPU fault` |
| Temperature > 85°C | `IssuesDetected: CPU running hot - check cooling` |
| Frequency drop > 10% | `IssuesDetected: CPU throttled - thermal/power limit` |
| Otherwise | `Healthy` |

### 6.2 RAM Health Rules
**File:** `src/stress/ram.rs:evaluate_ram_health()`

| Condition | Result |
|-----------|--------|
| Any errors > 0 | `Failed: Memory errors detected - BAD RAM` |
| Allocation < 0.1 GB | `Failed: Memory allocation failed` |
| Write speed < 0.3 GB/s | `Failed: Extremely low write speed - faulty RAM` |
| Read speed < 0.3 GB/s | `Failed: Extremely low read speed - faulty RAM` |
| Otherwise | `Healthy` |

---

## 7. KEY IMPLEMENTATION DETAILS

### 7.1 Multi-threading
- CPU stress test spawns threads = logical CPU cores
- Uses `Arc<AtomicBool>` for thread coordination
- Uses `Arc<AtomicU64>` for shared operation counter
- Background monitor thread for real-time CPU usage

### 7.2 Sensors Monitoring
- **Temperature:** Uses sysinfo `Components` API
  - Filters for CPU-related labels (cpu, tdie, core, package, tcal)
  - Handles negative temp readings on Apple Silicon
- **Frequency:** Per-core + average from sysinfo
- **Usage Monitor:** Background thread with 200ms refresh cycle

### 7.3 Output Formatting
- ANSI color codes for terminal:
  - GREEN, YELLOW, ORANGE, RED for temperature
  - GREEN, DARK_GRAY for usage bars
- Progress bar with filled/empty blocks
- Box drawing for result tables (54 chars wide)

### 7.4 Prime Calculation Algorithm
**File:** `src/stress/cpu.rs`
- Used as CPU-intensive workload
- `calculate_primes(n)`: Finds first n prime numbers
- `is_prime(n)`: Trial division up to sqrt(n)
- Called 10,000 iterations per loop

---

## 8. TESTS IN CODEBASE

### Test Locations
1. **src/stress/cpu.rs** (lines 461-519)
   - `test_cpu_test_short`: 1-second CPU test
   - `test_prime_calculation`: Verify prime count
   - `test_is_prime`: Prime detection logic
   - `test_evaluate_cpu_health`: All health rule branches

2. **src/stress/ram.rs** (lines 162-204)
   - `test_ram_test_small`: 100MB RAM test
   - `test_evaluate_ram_health`: All health rule branches

3. **src/sensors/temp.rs** (lines 86-99)
   - `test_get_cpu_temp`: Validates temp reading range

4. **src/sensors/frequency.rs** (lines 55-67)
   - `test_get_cpu_frequency`: Validates frequency detection

### Running Tests
```bash
cargo test
```

---

## 9. UNRESOLVED QUESTIONS

1. **VRAM Detection:** Windows/Linux GPU detection has TODO comments for VRAM
2. **Error Handling:** Some GPU detection failures return generic "Detection failed"
3. **Temperature Availability:** May return None on some systems (no fallback)
4. **Language Selection:** Currently interactive only - no `--lang` flag support
5. **Test Coverage:** No integration tests for full workflow
6. **Disk Health:** No stress test for disks (only detection)
7. **GPU Stress:** No GPU stress testing implemented

---

## 10. ARCHITECTURAL PATTERNS

### Design Patterns Used
1. **Strategy Pattern:** Platform trait for OS-specific implementations
2. **Module Pattern:** Clear separation (hw, stress, sensors, platform)
3. **Builder Pattern:** clap derive for CLI argument parsing
4. **Monitor Pattern:** Background thread for CPU usage tracking
5. **Facade Pattern:** `CpuInfo::new()`, `RamInfo::new()` hide sysinfo complexity

### Code Quality
- Clean module organization
- Consistent error handling (HealthStatus enum)
- Platform-specific code isolated via cfg macros
- Good use of Rust type system (Option, Result)
- Comprehensive inline documentation

---

## END OF REPORT

**Analysis completed:** 2025-12-25  
**Files analyzed:** 19 Rust files  
**Dependencies:** 3 crates (sysinfo, clap, num_cpus)
