# PChecker Codebase Structure Analysis

**Report ID:** scout-external-251226-1417-pchecker-codebase-structure  
**Date:** 2025-12-26  
**Version:** 0.3.0  
**Total Files:** 34 Rust source files  
**Total Lines:** ~5,156 lines of code  

---

## Executive Summary

PChecker is a **cross-platform hardware detection and health check CLI tool** written in Rust. The codebase demonstrates excellent modular organization with clear separation of concerns: hardware detection (`hw/`), stress testing (`stress/`), sensor monitoring (`sensors/`), platform-specific implementations (`platform/`), and user interface (`fmt.rs`, `lang.rs`).

**Key Architectural Patterns:**
- **Strategy Pattern** for platform-specific code (macOS/Windows/Linux)
- **Module-per-component** organization
- **Trait-based abstractions** for hardware interfaces
- **Compile-time platform selection** via `cfg(target_os)`
- **Multi-language support** via translation struct

---

## Directory Structure

```
src/
├── main.rs                      (780 lines)  - CLI entry point, orchestration
├── platform/mod.rs              (65 lines)   - Platform detection (trait-based)
├── hw/                          - Hardware detection modules
│   ├── mod.rs                   (12 lines)   - Module exports
│   ├── cpu/mod.rs               (28 lines)   - CPU model & core detection
│   ├── ram/mod.rs               (24 lines)   - RAM total/used/free detection
│   ├── disk/mod.rs              (40 lines)   - Disk name, capacity, usage detection
│   ├── gpu.rs                   (29 lines)   - GPU detection orchestrator
│   └── gpu/
│       ├── common.rs            (87 lines)   - GpuInfo, GpuType enum
│       └── platform/            - Platform-specific GPU detection
│           ├── mod.rs           (11 lines)
│           ├── macos.rs         - system_profiler SPDisplaysDataType
│           ├── windows.rs       - PowerShell Get-WmiObject
│           └── linux.rs         - lspci -vnnn
├── stress/                      - Health check modules
│   ├── mod.rs                   (22 lines)   - HealthStatus enum, exports
│   ├── cpu/mod.rs               (497 lines)  - CPU stress test (verbose mode)
│   ├── ram/mod.rs               (209 lines)  - RAM write/read verify test
│   ├── disk/mod.rs              (640 lines)  - Disk I/O + SMART data
│   ├── gpu.rs                   (372 lines)  - GPU thermal + compute stress
│   ├── gpu_compute.rs           - wgpu compute shader (optional)
│   ├── cpu/platform/            - Platform-specific CPU display
│   │   ├── mod.rs               (21 lines)
│   │   ├── macos.rs             - 4 cores/row, avg freq
│   │   ├── windows.rs           - 3 cores/row, per-core freq
│   │   └── linux.rs             - 3 cores/row, per-core freq
│   ├── gpu/platform/            - Platform-specific GPU metrics
│   │   ├── mod.rs               (29 lines)
│   │   ├── macos.rs             - powermetrics, SMC
│   │   ├── windows.rs           - (stub)
│   │   └── linux.rs             - (stub)
│   └── disk/
│       ├── mod.rs               (640 lines)  - Main disk test
│       └── smart.rs             - SMART data parsing
├── sensors/                     - Hardware monitoring
│   ├── mod.rs                   (11 lines)   - Sensor exports
│   ├── temp.rs                  (~100 lines) - CPU temperature reading
│   ├── frequency.rs             (~70 lines)  - CPU frequency per-core
│   └── monitor.rs               (~90 lines)  - Background CPU usage monitor
├── fmt.rs                       (~200 lines) - Output formatting, ANSI colors
├── lang.rs                      (568 lines)  - Multi-language support
└── prompt.rs                    (~30 lines)  - Interactive prompts
```

---

## Main Modules and Their Purposes

### 1. **main.rs** (780 lines)
**Purpose:** CLI entry point, argument parsing, test orchestration

**Key Types:**
```rust
struct Args {
    stress: bool,           // Run health check mode
    cpu_stress: bool,       // CPU test only
    ram_stress: bool,       // RAM test only
    disk_stress: bool,      // Disk test only
    gpu_stress: bool,       // GPU test only
    all_disks: bool,        // Test all disks
    disk_index: Option<usize>, // Test specific disk
    list_disks: bool,       // List available disks
    duration: u64,          // Test duration (default 60s)
    quick: bool,            // Quick 15s test
    verbose: bool,          // Detailed output
}
```

**Key Functions:**
- `main()`: Parses args, selects language, routes to mode
- `select_language_standalone()`: Interactive language picker
- `run_info_mode()`: Hardware detection display
- `run_health_check_mode()`: Stress test orchestration
- `print_cpu_result()`, `print_ram_result()`, `print_disk_result()`, `print_gpu_result()`: Result formatting
- `list_disks_mode()`: Disk listing utility

---

### 2. **Platform Module** (`platform/mod.rs`)
**Purpose:** OS detection and trait-based abstraction

**Key Types:**
```rust
pub trait Platform: fmt::Display {}

pub struct MacOS;  // macOS (Apple Silicon)
pub struct Windows;
pub struct Linux;

pub fn detect() -> Box<dyn Platform>
```

**Pattern:** Compile-time platform selection via `cfg(target_os)`

---

### 3. **Hardware Detection Module** (`hw/`)

#### **hw/cpu/mod.rs** (28 lines)
**Purpose:** CPU model and core count detection

**Key Types:**
```rust
pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
}
```

**Implementation:**
- Uses `sysinfo::System` with `CpuRefreshKind::everything()`
- Extracts CPU brand string from first logical core
- Returns total logical core count

#### **hw/ram/mod.rs** (24 lines)
**Purpose:** RAM total, used, free memory detection

**Key Types:**
```rust
pub struct RamInfo {
    pub total_gb: f64,
    pub used_gb: f64,
}
```

**Implementation:**
- Uses `sysinfo::System.refresh_memory()`
- Converts bytes to GB

#### **hw/disk/mod.rs** (40 lines)
**Purpose:** Disk name, capacity, usage detection (multi-disk support)

**Key Types:**
```rust
#[derive(Clone)]
pub struct DiskInfo {
    pub name: String,
    pub total_gb: f64,
    pub used_gb: f64,
    pub available_gb: f64,
    pub mount_point: String,
}
```

**Implementation:**
- Uses `sysinfo::Disks::new_with_refreshed_list()`
- Returns `Vec<DiskInfo>` for all disks

#### **hw/gpu/** (GPU Detection)
**Purpose:** Cross-platform GPU detection with platform-specific implementations

**Key Types:**
```rust
// common.rs
pub enum GpuType {
    Integrated,
    Discrete,
    Unknown,
}

pub struct GpuInfo {
    pub model: String,
    pub vram_gb: Option<f64>,
    pub gpu_type: GpuType,
}

impl GpuType {
    pub fn from_model(model: &str) -> Self  // Auto-detect from name
}
```

**Platform Implementations:**
- **macOS:** `system_profiler SPDisplaysDataType`
- **Windows:** PowerShell `Get-WmiObject Win32_VideoController`
- **Linux:** `lspci -vnnn`

---

### 4. **Stress Test Module** (`stress/`)

#### **stress/mod.rs** (22 lines)
**Purpose:** Health status enum and test exports

**Key Types:**
```rust
pub enum HealthStatus {
    Healthy,
    IssuesDetected(Vec<String>),
    Failed(String),
}
```

#### **stress/cpu/mod.rs** (497 lines)
**Purpose:** CPU stress test with verbose mode support

**Key Types:**
```rust
pub struct CpuTestConfig {
    pub duration_secs: u64,
    pub thread_count: Option<usize>,
    pub verbose: bool,
}

pub struct CpuTestResult {
    pub cpu_model: String,
    pub cpu_cores: usize,
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

**Algorithm:**
1. Spawns threads equal to logical CPU cores
2. Runs prime number calculation (10,000 primes/iteration)
3. Tracks operations, timing, temperature, frequency
4. Background monitor samples per-core CPU usage every 200ms

**Verbose Mode:**
- Per-core usage bars: `C00: [████████░░] 80% @4.5GHz`
- Platform-specific: 4 cores/row (macOS), 3 cores/row (Win/Linux)
- Temperature sensors list (up to 8 sensors)

**Health Evaluation Rules:**
- **Failed:** Thread panic, temp > 95°C, variance > 200%
- **Issues:** Temp > 85°C, frequency drop > 10%
- **Healthy:** Otherwise

#### **stress/ram/mod.rs** (209 lines)
**Purpose:** RAM stress test with write/read verification

**Key Types:**
```rust
pub struct RamTestConfig {
    pub max_gb: Option<f64>,
}

pub struct RamTestResult {
    pub ram_total_gb: f64,
    pub tested_gb: f64,
    pub write_speed_gb_s: f64,
    pub read_speed_gb_s: f64,
    pub errors: u64,
    pub health: HealthStatus,
}
```

**Algorithm:**
1. Allocates 80% of available RAM (max 16GB)
2. Write test: Fill buffer with pattern `0xAA55_AA55_AA55_AA55`
3. Read test: Verify all data matches pattern
4. Measure write/read speeds

**Health Evaluation Rules:**
- **Failed:** Errors > 0, allocation < 0.1GB, speeds < 0.3 GB/s
- **Healthy:** Otherwise

#### **stress/disk/mod.rs** (640 lines)
**Purpose:** Disk I/O test with SMART data

**Key Types:**
```rust
pub struct DiskTestConfig {
    pub test_path: Option<String>,
    pub test_size_mb: u64,
    pub include_seek_test: bool,
    pub verbose: bool,
}

pub struct DiskTestResult {
    pub disk_name: String,
    pub disk_size_gb: f64,
    pub disk_used_gb: f64,
    pub disk_available_gb: f64,
    pub disk_fs: String,
    pub disk_device: Option<String>,
    pub write_speed_mb_s: f64,
    pub read_speed_mb_s: f64,
    pub seek_time_ms: f64,
    pub bad_sectors: u64,
    pub is_ssd: bool,
    pub smart: Option<SmartData>,
    pub health: HealthStatus,
}
```

**Algorithm:**
1. **Write Test:** Sequential write (pattern `0xA5`)
2. **Read Test:** Sequential read with pattern verification
3. **Seek Test:** Random access (1000 iterations, 4KB reads)
4. **SMART Data:** Platform-specific collection (verbose mode)

**Health Evaluation Rules (SSD vs HDD):**
- **SSD:** read >= 50 MB/s, write >= 30 MB/s, seek <= 5ms
- **HDD:** read >= 10 MB/s, write >= 10 MB/s, seek <= 20ms
- **Failed:** Bad sectors > 0, extremely slow speeds
- **Issues:** Slow seek, below-average speeds

#### **stress/gpu.rs** (372 lines)
**Purpose:** GPU thermal and compute stress test

**Key Types:**
```rust
pub struct GpuTestConfig {
    pub duration_secs: u64,
    pub verbose: bool,
}

pub struct GpuTestResult {
    pub gpu_model: String,
    pub gpu_type: String,
    pub vram_gb: Option<f64>,
    pub temperature_start: Option<GpuTemp>,
    pub temperature_end: Option<GpuTemp>,
    pub temperature_max: Option<f32>,
    pub is_apple_silicon: bool,
    pub apple_gpu_metrics: Option<AppleGpuMetrics>,
    pub health: HealthStatus,
}

pub struct AppleGpuMetrics {
    pub frequency_mhz: Option<u32>,
    pub power_mw: Option<u32>,
    pub residency_pct: Option<f32>,
    pub temperature_c: Option<f32>,
    pub thermal_pressure: Option<ThermalPressure>,
    pub smc_temperature_c: Option<f32>,
    pub gpu_cores: Option<u32>,
    pub metal_version: Option<String>,
}

pub enum ThermalPressure {
    Nominal, Moderate, Heavy, Trapping, Sleeping, Unknown,
}
```

**Algorithm:**
1. Attempts wgpu compute shader stress test (optional feature)
2. Falls back to thermal monitoring if compute unavailable
3. Apple Silicon: Uses `powermetrics` + SMC for detailed metrics
4. Other platforms: sysinfo temperature sensors only

**Health Evaluation Rules:**
- **Failed:** Temp > 95°C
- **Issues:** Temp > 85°C, no sensor (non-Apple Silicon)
- **Healthy:** Otherwise (Apple Silicon: no sensor expected)

---

### 5. **Sensors Module** (`sensors/`)

**Purpose:** Real-time hardware monitoring during stress tests

#### **sensors/temp.rs** (~100 lines)
**Key Types:**
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

#### **sensors/frequency.rs** (~70 lines)
**Key Types:**
```rust
pub struct CpuFrequency {
    pub cores: usize,
    pub per_core_mhz: Vec<u64>,
    pub avg_ghz: f64,
}
```

**Implementation:**
- Uses `sysinfo::System` to read per-core frequency
- Calculates average frequency in GHz

#### **sensors/monitor.rs** (~90 lines)
**Key Types:**
```rust
pub struct CpuMonitorHandle {
    receiver: flume::Receiver<HashMap<usize, f32>>,
}

impl CpuMonitorHandle {
    pub fn start() -> Self;  // Spawns background thread
    pub fn get_per_core_usage(&self) -> HashMap<usize, f32>;
}
```

**Implementation:**
- Spawns background thread sampling CPU usage every 200ms
- Maintains rolling buffer of 60 samples (12 seconds)
- Returns HashMap mapping core index to usage percentage

---

### 6. **Language System** (`lang.rs` - 568 lines)
**Purpose:** Multi-language support (Vietnamese, English)

**Key Types:**
```rust
pub enum Language {
    Vietnamese,
    English,
}

pub struct Text {
    pub lang: Language,
}

impl Text {
    // 50+ translation methods
    pub fn header(&self) -> &str;
    pub fn cpu(&self) -> &str;
    pub fn gpu(&self) -> &str;
    pub fn ram(&self) -> &str;
    pub fn disk(&self) -> &str;
    // ... many more
}
```

**Design Pattern:**
- Static translation methods per string
- No runtime loading (compile-time embedded)
- Simple and maintainable for CLI tools

---

### 7. **Formatting Module** (`fmt.rs` - ~200 lines)
**Purpose:** Output formatting, ANSI colors, progress bars

**Constants:**
```rust
pub const RESET: &str = "\x1b[0m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const ORANGE: &str = "\x1b[38;5;208m";
pub const RED: &str = "\x1b[31m";
pub const CYAN: &str = "\x1b[36m";
```

**Key Functions:**
- `temp_color(temp)`: Color code based on temperature
- `progress_bar(percent, width)`: Visual progress bar
- `print_header_with_text()`, `print_section()`, `print_footer_with_text()`

---

## Platform-Specific Code Organization

### Strategy Pattern Implementation

**1. Platform Detection (`platform/mod.rs`):**
```rust
pub trait Platform: fmt::Display {}
pub fn detect() -> Box<dyn Platform>
```

**2. CPU Display Formatting (`stress/cpu/platform/`):**
- **macOS:** 4 cores/row, average frequency only
- **Windows/Linux:** 3 cores/row, per-core frequency

**3. GPU Detection (`hw/gpu/platform/`):**
- **macOS:** `system_profiler SPDisplaysDataType`
- **Windows:** PowerShell `Get-WmiObject Win32_VideoController`
- **Linux:** `lspci -vnnn`

**4. GPU Metrics (`stress/gpu/platform/`):**
- **macOS:** `powermetrics` + SMC (Apple Silicon)
- **Windows/Linux:** Stub implementations

**5. Disk Type Detection (`stress/disk/mod.rs`):**
- **macOS:** `diskutil info -plist` (SolidState key)
- **Linux:** `/sys/block/*/queue/rotational`
- **Windows:** Default to SSD

**6. Disk Device Mapping (`stress/disk/mod.rs`):**
- **macOS:** `df` command
- **Linux:** `/proc/mounts`
- **Windows:** Returns None

---

## Key Data Structures

### Core Info Structs
```rust
// Hardware detection
CpuInfo { model: String, cores: usize }
RamInfo { total_gb: f64, used_gb: f64 }
DiskInfo { name, total_gb, used_gb, available_gb, mount_point }
GpuInfo { model, vram_gb: Option<f64>, gpu_type: GpuType }

// Test configs
CpuTestConfig { duration_secs, thread_count, verbose }
RamTestConfig { max_gb: Option<f64> }
DiskTestConfig { test_path, test_size_mb, include_seek_test, verbose }
GpuTestConfig { duration_secs, verbose }

// Test results
CpuTestResult { cpu_model, cpu_cores, operations, ops_per_second, 
                avg_op_time_ms, variance_pct, temperature, 
                frequency_start, frequency_end, freq_drop_pct, health }
RamTestResult { ram_total_gb, tested_gb, write_speed_gb_s, 
                read_speed_gb_s, errors, health }
DiskTestResult { disk_name, disk_size_gb, write_speed_mb_s, 
                 read_speed_mb_s, seek_time_ms, bad_sectors, 
                 is_ssd, smart: Option<SmartData>, health }
GpuTestResult { gpu_model, gpu_type, vram_gb, temperature_max, 
                is_apple_silicon, apple_gpu_metrics, health }
```

### Sensor Structs
```rust
CpuTemp { current: f64, sensors: Vec<String> }
CpuFrequency { cores, per_core_mhz: Vec<u64>, avg_ghz: f64 }
```

### Health Status
```rust
enum HealthStatus {
    Healthy,
    IssuesDetected(Vec<String>),
    Failed(String),
}
```

---

## Hardware Detection Patterns

### 1. **sysinfo Crate** (Primary Detection)
- **CPU:** `sysinfo::System.cpus()`
- **RAM:** `sysinfo::System.total_memory()`, `used_memory()`
- **Disk:** `sysinfo::Disks::new_with_refreshed_list()`
- **Temperature:** `sysinfo::Components`

### 2. **Platform Commands** (GPU)
- **macOS:** `system_profiler SPDisplaysDataType`
- **Windows:** PowerShell `Get-WmiObject Win32_VideoController`
- **Linux:** `lspci -vnnn`

### 3. **Platform Metrics** (Verbose Mode)
- **macOS:** `powermetrics`, `diskutil`, SMC
- **Linux:** `/proc/mounts`, `/sys/block/*/queue/rotational`
- **Windows:** WMI queries (stubs in some areas)

---

## Stress Test Architecture

### CPU Stress Test
1. **Workload:** Prime number calculation (trial division)
2. **Threads:** Equal to logical CPU cores
3. **Metrics:** Operations, timing, temperature, frequency
4. **Verbose:** Per-core usage bars + sensor list

### RAM Stress Test
1. **Allocation:** 80% of available RAM, max 16GB
2. **Write:** Pattern `0xAA55_AA55_AA55_AA55`
3. **Read:** Verify all data matches pattern
4. **Metrics:** Tested GB, write/read speeds, errors

### Disk Stress Test
1. **Write:** Sequential write (pattern `0xA5`)
2. **Read:** Sequential read with pattern verification
3. **Seek:** Random access (1000 iterations)
4. **Metrics:** Write/read speeds, seek time, bad sectors
5. **Verbose:** SMART data collection

### GPU Stress Test
1. **Compute:** wgpu compute shader (optional)
2. **Fallback:** Thermal monitoring only
3. **Metrics:** Temperature, frequency, power, usage (Apple Silicon)
4. **Verbose:** Detailed Apple GPU metrics (powermetrics, SMC)

---

## Translation/i18n System

### Design
- **Enum-based:** `Language { Vietnamese, English }`
- **Struct-based:** `Text { lang: Language }`
- **Method-based:** 50+ translation methods
- **No runtime loading:** All translations compile-time embedded

### Usage Pattern
```rust
let text = Text::new(Language::English);
println!("{}", text.cpu_health_check());
```

### Coverage
- Header/footer text
- Hardware labels (CPU, GPU, RAM, Disk)
- Test status messages
- Health check results
- SMART data labels
- GPU-specific labels
- Error messages

---

## Recent Changes (Git Status Analysis)

### Deleted (Refactored to module dirs):
- `src/hw/cpu.rs` → `src/hw/cpu/mod.rs`
- `src/hw/ram.rs` → `src/hw/ram/mod.rs`
- `src/hw/disk.rs` → `src/hw/disk/mod.rs`
- `src/stress/cpu.rs` → `src/stress/cpu/mod.rs`
- `src/stress/ram.rs` → `src/stress/ram/mod.rs`

### Added/New:
- `src/hw/gpu/common.rs` - GPU common types
- `src/hw/gpu/platform/` - Platform-specific GPU detection
- `src/stress/cpu/platform/` - Platform-specific CPU display
- `src/stress/gpu.rs` - GPU stress test (new)
- `src/stress/gpu_compute.rs` - wgpu compute shader (new)
- `src/stress/gpu/platform/` - Platform-specific GPU metrics
- `src/stress/disk/smart.rs` - SMART data parsing (new)
- `src/stress/disk/mod.rs` - Disk stress test (new)

### Modified:
- `src/main.rs` - Added GPU test, disk test, verbose flags
- `src/hw/gpu.rs` - Refactored to use platform modules
- `src/lang.rs` - Added GPU/disk translations
- `Cargo.toml` - Added wgpu, pollster, bytemuck, smc (optional)

---

## Dependencies

### Core
```toml
sysinfo = "0.37"        # System info (CPU, RAM, components)
clap = { version = "4.5", features = ["derive"] }  # CLI parsing
num_cpus = "1.16"       # CPU count detection
fastrand = "2.1"        # Random number generation
```

### Optional Features
```toml
[features]
default = []
gpu-compute = ["wgpu", "pollster", "bytemuck"]
apple-smc = ["smc"]

[dependencies]
wgpu = { version = "0.20", optional = true }
pollster = { version = "0.3", optional = true }
bytemuck = { version = "1.14", optional = true }
smc = { version = "0.2", optional = true }
```

---

## Build Configuration

### Release Profile
```toml
[profile.release]
opt-level = "z"         # Optimize for size
lto = true              # Link-time optimization
codegen-units = 1       # Single codegen unit
strip = true            # Strip symbols
panic = "abort"         # Abort on panic
```

**Binary Size:** ~500KB - 1MB (stripped, size-optimized)

---

## Testing Coverage

### Unit Tests
- `src/stress/cpu/mod.rs`: 4 tests (test, prime, health eval)
- `src/stress/ram/mod.rs`: 2 tests (test, health eval)
- `src/stress/gpu.rs`: 3 tests (temp, health eval, short test)
- `src/stress/disk/mod.rs`: 2 tests (test, health eval)
- `src/sensors/temp.rs`: 1 test (temp validation)
- `src/sensors/frequency.rs`: 1 test (frequency validation)

### Running Tests
```bash
cargo test                      # All tests
cargo test --lib                # Unit tests only
cargo test test_cpu_test_short  # Specific test
```

---

## Code Quality Observations

### Strengths
1. **Clear modular organization** - Separate concerns (hw, stress, sensors, platform)
2. **Platform abstraction** - Strategy pattern for OS-specific code
3. **Comprehensive error handling** - HealthStatus enum for test results
4. **Multi-language support** - Clean translation system
5. **Verbose mode** - Detailed diagnostics with platform-specific formatting
6. **Optional features** - GPU compute, SMC via Cargo features

### Areas for Improvement
1. **VRAM detection** - Windows/Linux have TODO comments
2. **Temperature fallback** - May return None without fallback
3. **Language selection** - Interactive only (no `--lang` flag)
4. **Integration tests** - No full workflow tests
5. **GPU compute** - Optional feature (not in default build)

---

## Unresolved Questions

1. **GPU VRAM detection on Windows/Linux** - Currently marked as TODO
2. **SMC temperature reading** - Optional `apple-smc` feature, how widely used?
3. **wgpu compute stress** - Optional feature, what's the fallback strategy?
4. **Disk SMART on Windows** - Implementation status?
5. **GPU stress on non-Apple platforms** - Currently thermal-only, compute not integrated?

---

## Summary

PChecker is a **well-architected Rust CLI tool** with:
- **34 files**, **~5,156 lines** of code
- **Modular design**: hw/, stress/, sensors/, platform/
- **Cross-platform**: macOS (Apple Silicon), Windows, Linux
- **Comprehensive testing**: CPU, RAM, Disk, GPU stress tests
- **Platform-specific optimizations**: Verbose mode formatting, metrics collection
- **Multi-language**: Vietnamese, English
- **Optional features**: GPU compute, Apple SMC

**Key architectural pattern:** **Strategy pattern** for platform-specific code via `cfg(target_os)` and module-per-platform organization.

**Recent refactor:** Flattened structure from single files to module directories (`cpu.rs` → `cpu/mod.rs`) for better organization and platform-specific implementations.

