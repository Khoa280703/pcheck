# PChecker Codebase Summary

**Version:** 0.2.0
**Last Updated:** 2025-12-25
**Total Lines:** ~1,980 lines of Rust code

---

## Overview

PChecker is a cross-platform hardware detection and health check CLI tool written in Rust. The codebase is organized into clear modules with distinct responsibilities: hardware detection, stress testing, sensor monitoring, platform-specific implementations, and user interface.

---

## File Structure

```
src/
├── main.rs                   (334 lines) - CLI entry point, orchestration
│
├── hw/                       - Hardware detection modules
│   ├── mod.rs                (40 lines) - Module exports
│   ├── cpu.rs                (50 lines) - CPU model & core detection
│   ├── ram.rs                (40 lines) - RAM total/used/free detection
│   ├── disk.rs               (45 lines) - Disk name & capacity detection
│   └── gpu.rs                (120 lines) - GPU detection (platform-specific)
│
├── stress/                   - Health check modules
│   ├── mod.rs                (60 lines) - HealthStatus enum, test runners
│   ├── cpu.rs                (520 lines) - CPU stress test (with verbose mode)
│   └── ram.rs                (205 lines) - RAM stress test (write/read verify)
│
├── sensors/                  - Hardware monitoring
│   ├── mod.rs                (30 lines) - Sensor exports
│   ├── temp.rs               (100 lines) - CPU temperature reading
│   ├── frequency.rs          (70 lines) - CPU frequency per-core & average
│   └── monitor.rs            (90 lines) - Background CPU usage monitor
│
├── platform/
│   └── mod.rs                (25 lines) - Platform detection (macOS/Windows/Linux)
│
├── fmt.rs                    (200 lines) - Output formatting, ANSI colors, progress bars
├── lang.rs                   (120 lines) - Multi-language support (Vietnamese/English)
└── prompt.rs                 (30 lines) - Interactive prompts
```

---

## Module Details

### main.rs (334 lines)
**Purpose:** CLI entry point and orchestration

**Key Components:**
- `Args` struct: Clap-derived CLI arguments
  - `--stress` / `-s`: Run health check mode
  - `--duration` / `-d`: Test duration in seconds
  - `--quick`: Quick 15-second test
  - `--verbose` / `-v`: Show detailed per-core metrics

**Functions:**
- `main()`: Entry point, parses args, selects language, routes to mode
- `select_language_standalone()`: Interactive language selection (Vietnamese/English)
- `run_info_mode()`: Hardware detection mode, displays CPU, GPU, RAM, Disk info
- `run_health_check_mode()`: Stress test mode, runs CPU and RAM tests
- `print_cpu_result()`: Formats and displays CPU test results
- `print_ram_result()`: Formats and displays RAM test results

**Dependencies:** clap, hw, stress, fmt, lang, platform

---

### hw/ - Hardware Detection

#### hw/mod.rs (40 lines)
**Purpose:** Module exports for hardware detection

**Exports:**
- `CpuInfo`, `RamInfo`, `DiskInfo`, `GpuInfo`
- `pub use` for all submodules

---

#### hw/cpu.rs (50 lines)
**Purpose:** CPU model and core count detection

**Struct:**
```rust
pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
}
```

**Implementation:**
- Uses `sysinfo::System` to detect CPU
- Extracts CPU brand string and physical core count
- Returns `CpuInfo` with model name and core count

---

#### hw/ram.rs (40 lines)
**Purpose:** RAM total, used, and free memory detection

**Struct:**
```rust
pub struct RamInfo {
    pub total_gb: f64,
    pub used_gb: f64,
    pub free_gb: f64,
}
```

**Implementation:**
- Uses `sysinfo::System` to read memory info
- Converts bytes to GB
- Calculates free from total - used

---

#### hw/disk.rs (45 lines)
**Purpose:** Disk name and total capacity detection

**Struct:**
```rust
pub struct DiskInfo {
    pub name: String,
    pub total_gb: f64,
}

impl DiskInfo {
    pub fn display(&self) -> String { /* format */ }
}
```

**Implementation:**
- Uses `sysinfo::System` to list disks
- Returns first disk found
- Formats display with name and capacity

---

#### hw/gpu.rs (120 lines)
**Purpose:** GPU detection with platform-specific implementations

**Struct:**
```rust
pub struct GpuInfo {
    pub model: String,
    pub vram_gb: Option<f64>,
}

impl GpuInfo {
    pub fn display(&self) -> String { /* format */ }
}
```

**Implementation:**
- `get_gpu_info()`: Platform-specific GPU detection
  - **macOS:** Uses `system_profiler SPDisplaysDataType`
  - **Windows:** Uses PowerShell `Get-WmiObject Win32_VideoController`
  - **Linux:** Uses `lspci -vnnn`
- VRAM detection: macOS only (Windows/Linux TODO)

---

### stress/ - Health Check Modules

#### stress/mod.rs (60 lines)
**Purpose:** Health status enum and test runner exports

**Enum:**
```rust
pub enum HealthStatus {
    Healthy,
    IssuesDetected(Vec<String>),
    Failed(String),
}
```

**Exports:**
- `CpuTestConfig`, `CpuTestResult`
- `RamTestConfig`, `RamTestResult`
- `run_cpu_test()`, `run_ram_test()`

---

#### stress/cpu.rs (520 lines)
**Purpose:** CPU stress test with verbose mode support

**Structs:**
```rust
pub struct CpuTestConfig {
    pub duration_secs: u64,
    pub thread_count: Option<usize>,
    pub verbose: bool,  // Verbose mode (v0.2.0)
}

pub struct CpuTestResult {
    pub operations: u64,
    pub ops_per_second: f64,
    pub avg_op_time_ms: f64,
    pub variance_pct: f64,
    pub temperature: Option<CpuTemp>,
    pub frequency_start: CpuFrequency,
    pub frequency_end: CpuFrequency,
    pub freq_drop_pct: f64,
    pub health: HealthStatus,
}
```

**Key Functions:**
- `run_stress_test(config)`: Main CPU stress test
  - Spawns threads equal to logical CPU cores
  - Runs prime number calculation on each thread
  - Tracks operations, timing, temperature, frequency
  - Background monitor tracks per-core CPU usage

- `calculate_primes(n)`: CPU-intensive prime number calculation
  - Finds first n prime numbers
  - Used as workload for stress test

- `is_prime(n)`: Trial division primality test

- `evaluate_cpu_health(result)`: Health evaluation logic
  - Failed: Thread panic, temp > 95°C, variance > 200%
  - Issues: Temp > 85°C, frequency drop > 10%
  - Healthy: Otherwise

**Verbose Mode (v0.2.0):**
- `print_cpu_progress_box()`: Prints real-time progress
  - Normal mode: Single progress line
  - Verbose mode: Per-core usage bars + temperature sensors
- `build_per_core_display()`: Formats per-core usage rows
  - Platform-specific: 4 cores/row (macOS), 3 cores/row (Windows/Linux)
  - Visual bar charts: `[████████░░] 80%`
  - Frequency included on Windows/Linux: `@4.5GHz`

**Tests (lines 461-519):**
- `test_cpu_test_short`: 1-second CPU test
- `test_prime_calculation`: Verify prime count
- `test_is_prime`: Prime detection logic
- `test_evaluate_cpu_health`: All health rule branches

---

#### stress/ram.rs (205 lines)
**Purpose:** RAM stress test with write/read verification

**Structs:**
```rust
pub struct RamTestConfig {
    pub max_gb: Option<f64>,
}

pub struct RamTestResult {
    pub tested_gb: f64,
    pub write_speed_gb_s: f64,
    pub read_speed_gb_s: f64,
    pub errors: u64,
    pub health: HealthStatus,
}
```

**Key Functions:**
- `run_ram_test(config)`: Main RAM stress test
  - Allocates 80% of available RAM (max 16GB)
  - Write test: Fill buffer with pattern (0xAA55_AA55_AA55_AA55)
  - Read test: Verify all data matches pattern
  - Measure write/read speeds

- `evaluate_ram_health(result)`: Health evaluation logic
  - Failed: Errors > 0, allocation < 0.1GB, speeds < 0.3 GB/s
  - Healthy: Otherwise

**Tests (lines 162-204):**
- `test_ram_test_small`: 100MB RAM test
- `test_evaluate_ram_health`: All health rule branches

---

### sensors/ - Hardware Monitoring

#### sensors/mod.rs (30 lines)
**Purpose:** Sensor module exports

**Exports:**
- `CpuTemp`, `CpuFrequency`
- `get_cpu_temp()`, `get_cpu_frequency()`
- `CpuMonitorHandle`, `get_all_sensors()`

---

#### sensors/temp.rs (100 lines)
**Purpose:** CPU temperature reading

**Struct:**
```rust
pub struct CpuTemp {
    pub current: f64,
    pub sensors: Vec<String>,
}
```

**Implementation:**
- Uses `sysinfo::Components` to read temperature sensors
- Filters for CPU-related labels (cpu, tdie, core, package, tcal)
- Handles negative temp readings on Apple Silicon
- Returns up to 8 temperature sensors

**Tests (lines 86-99):**
- `test_get_cpu_temp`: Validates temp reading range

---

#### sensors/frequency.rs (70 lines)
**Purpose:** CPU frequency per-core and average

**Struct:**
```rust
pub struct CpuFrequency {
    pub cores: usize,
    pub per_core_mhz: Vec<u64>,
    pub avg_ghz: f64,
}

impl CpuFrequency {
    pub fn current_ghz(&self) -> f64 { /* return avg */ }
}
```

**Implementation:**
- Uses `sysinfo::System` to read per-core frequency
- Calculates average frequency in GHz
- Returns frequency for all cores

**Tests (lines 55-67):**
- `test_get_cpu_frequency`: Validates frequency detection

---

#### sensors/monitor.rs (90 lines)
**Purpose:** Background CPU usage monitor thread

**Struct:**
```rust
pub struct CpuMonitorHandle {
    receiver: flume::Receiver<HashMap<usize, f32>>,
}

impl CpuMonitorHandle {
    pub fn start() -> Self { /* spawn background thread */ }
    pub fn get_per_core_usage(&self) -> HashMap<usize, f32> { /* ... */ }
}
```

**Implementation:**
- Spawns background thread on `start()`
- Samples CPU usage every 200ms
- Maintains rolling buffer of 60 samples (12 seconds)
- Returns HashMap mapping core index to usage percentage

**Usage in Stress Test:**
- Provides real-time per-core usage for verbose mode
- Thread-safe via channels

---

### platform/mod.rs (25 lines)
**Purpose:** Platform detection

**Enum:**
```rust
pub enum Platform {
    MacOS,
    Windows,
    Linux,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { /* ... */ }
}

pub fn detect() -> Platform {
    if cfg!(target_os = "macos") { Platform::MacOS }
    else if cfg!(target_os = "windows") { Platform::Windows }
    else { Platform::Linux }
}
```

---

### fmt.rs (200 lines)
**Purpose:** Output formatting, ANSI colors, progress bars

**Constants:**
```rust
pub const RESET: &str = "\x1b[0m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const ORANGE: &str = "\x1b[38;5;208m";
pub const RED: &str = "\x1b[31m";
pub const DARK_GRAY: &str = "\x1b[90m";
pub const CYAN: &str = "\x1b[36m";
```

**Functions:**
- `temp_color(temp)`: Returns color code based on temperature
- `temp_status(temp)`: Returns status text (OK, HOT, CRITICAL)
- `usage_color(usage)`: Returns color code based on CPU usage
- `progress_bar(percent, width)`: Generates visual progress bar
- `format_large_number(n)`: Formats numbers with thousands separator
- `print_header_with_text()`: Prints header with version and title
- `print_section()`: Prints hardware info section
- `print_footer_with_text()`: Prints footer with execution time

---

### lang.rs (120 lines)
**Purpose:** Multi-language support (Vietnamese, English)

**Enum:**
```rust
pub enum Language {
    Vietnamese,
    English,
}
```

**Struct:**
```rust
pub struct Text {
    lang: Language,
}

impl Text {
    // Methods return translated strings
    pub fn header(&self) -> &str { /* ... */ }
    pub fn cpu(&self) -> &str { /* ... */ }
    pub fn gpu(&self) -> &str { /* ... */ }
    pub fn ram(&self) -> &str { /* ... */ }
    pub fn disk(&self) -> &str { /* ... */ }
    // ... many more translation methods
}
```

**Languages:**
- Vietnamese (Tiếng Việt): Default option 1
- English: Option 2

---

### prompt.rs (30 lines)
**Purpose:** Interactive prompts (placeholder for future use)

Currently minimal - language selection is handled in `main.rs`.

---

## Dependencies

### Cargo.toml
```toml
[package]
name = "pchecker"
version = "0.2.0"
edition = "2021"

[dependencies]
sysinfo = "0.37"           # System info (CPU, RAM, components)
clap = { version = "4.5", features = ["derive"] }  # CLI parsing
num_cpus = "1.16"          # CPU count detection

[profile.release]
opt-level = "z"            # Optimize for size
lto = true                 # Link-time optimization
codegen-units = 1          # Single codegen unit
strip = true               # Strip symbols
panic = "abort"            # Abort on panic
```

---

## Key Implementation Details

### CPU Stress Test Algorithm
1. **Workload:** Prime number calculation (trial division)
2. **Thread Count:** Equal to logical CPU cores
3. **Metrics Tracked:**
   - Total operations count
   - Operations per second
   - Average operation time (ms)
   - Variance percentage (instability detection)
   - CPU temperature (start/end)
   - CPU frequency (start/end)
   - Frequency drop percentage (throttling detection)

### RAM Stress Test Algorithm
1. **Allocation:** 80% of available RAM, max 16GB
2. **Write Test:** Fill buffer with pattern (0xAA55_AA55_AA55_AA55)
3. **Read Test:** Verify all data matches pattern
4. **Metrics Tracked:**
   - Tested GB
   - Write speed (GB/s)
   - Read speed (GB/s)
   - Error count (any mismatch = BAD RAM)

### Verbose Mode (v0.2.0)
1. **Per-Core Usage Display:**
   - Background monitor samples every 200ms
   - Visual bar charts: `[████████░░] 80%`
   - Platform-specific: 4 cores/row (macOS), 3 cores/row (Windows/Linux)
   - Frequency included on Windows/Linux: `@4.5GHz`

2. **Temperature Sensors:**
   - Lists up to 8 temperature sensors
   - Format: `Sensor name: 45.0°C`

3. **Update Frequency:** Every 1 second

---

## Testing Coverage

### Unit Tests
1. **src/stress/cpu.rs:**
   - `test_cpu_test_short`: 1-second CPU test
   - `test_prime_calculation`: Verify prime count
   - `test_is_prime`: Prime detection logic
   - `test_evaluate_cpu_health`: All health rule branches

2. **src/stress/ram.rs:**
   - `test_ram_test_small`: 100MB RAM test
   - `test_evaluate_ram_health`: All health rule branches

3. **src/sensors/temp.rs:**
   - `test_get_cpu_temp`: Validates temp reading range

4. **src/sensors/frequency.rs:**
   - `test_get_cpu_frequency`: Validates frequency detection

### Running Tests
```bash
cargo test                      # Run all tests
cargo test --lib                # Run unit tests only
cargo test test_cpu_test_short  # Run specific test
```

---

## Platform-Specific Code

### macOS (target_os = "macos")
- GPU: `system_profiler SPDisplaysDataType`
- CPU display: 4 cores/row in verbose mode
- Frequency: Shows average only (no per-core)
- Temperature: Reads from PMU tdie components (Apple Silicon)

### Windows (target_os = "windows")
- GPU: PowerShell `Get-WmiObject Win32_VideoController`
- CPU display: 3 cores/row with frequency
- VRAM detection: TODO comment

### Linux (target_os = "linux")
- GPU: `lspci -vnnn`
- CPU display: 3 cores/row with frequency
- VRAM detection: TODO comment via `/sys`

---

## Binary Size & Performance

### Release Build
```bash
cargo build --release
# Binary: target/release/pchecker
# Size: ~500KB - 1MB (stripped, size-optimized)
```

### Profile Configuration
- `opt-level = "z"`: Optimize for size
- `lto = true`: Link-time optimization
- `codegen-units = 1`: Single codegen unit
- `strip = true`: Strip symbols
- `panic = "abort"`: Abort on panic (reduces size)

### Performance Metrics
- Startup time: <100ms
- Memory overhead: <50MB (info mode)
- CPU usage during stress: 95%+ on all cores

---

## Known Limitations

1. **VRAM Detection:** Windows/Linux GPU detection has TODO comments
2. **Temperature Availability:** May return None on some systems (no fallback)
3. **Language Selection:** Interactive only - no `--lang` flag support
4. **Test Coverage:** No integration tests for full workflow
5. **Disk Health:** No stress test for disks (only detection)
6. **GPU Stress:** No GPU stress testing implemented

---

## Future Enhancements

### v0.3.0 (Planned)
- GPU stress testing
- Disk health check (read/write tests)
- Command-line language selection (`--lang` flag)
- JSON output mode for automation
- Config file support

### v0.4.0 (Planned)
- VRAM detection for Windows/Linux
- Battery health check (laptops)
- Network interface detection
- Export results to file

### v1.0.0 (Future)
- Continuous monitoring mode
- Web dashboard
- Historical data tracking
- Automated benchmarking

---

**Last Updated:** 2025-12-25
**Document Version:** 1.0
