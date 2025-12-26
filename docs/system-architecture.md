# PChecker System Architecture

**Version:** 0.3.0
**Last Updated:** 2025-12-25

---

## Overview

PChecker is a cross-platform hardware detection and health check CLI tool built with Rust. The architecture follows a modular design with clear separation of concerns, platform-specific implementations, and concurrent execution for stress testing.

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│                    (main.rs + clap)                          │
│  - Argument parsing                                          │
│  - Language selection                                        │
│  - Mode orchestration (info/stress)                          │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                ▼                           ▼
    ┌───────────────────┐       ┌──────────────────────┐
    │    Info Mode      │       │   Stress Mode        │
    │  (hw modules)     │       │ (stress modules)     │
    └───────────────────┘       └──────────────────────┘
                │                           │
                ▼                           ▼
    ┌───────────────────┐       ┌──────────────────────┐
    │ Hardware Detection│       │  Health Checks       │
    │  - CPU, GPU       │       │  - CPU Stress Test   │
    │  - RAM, Disk      │       │  - RAM Stress Test   │
    └───────────────────┘       └──────────────────────┘
                │                           │
                └─────────────┬─────────────┘
                              ▼
                ┌──────────────────────────┐
                │    Platform Layer        │
                │  (platform modules)      │
                │  - macOS, Windows, Linux │
                └──────────────────────────┘
                              │
                              ▼
                ┌──────────────────────────┐
                │   Output Formatting      │
                │   (fmt module)           │
                │  - ANSI colors           │
                │  - Progress bars         │
                │  - Tables                │
                └──────────────────────────┘
```

---

## Module Architecture

### Core Modules

#### 1. Main Entry Point (main.rs)
**Responsibilities:**
- CLI argument parsing via clap derive
- Language selection orchestration
- Mode routing (info vs stress)
- Result aggregation and display

**Key Functions:**
```rust
fn main()                          // Entry point
fn select_language_standalone()    // Interactive language selection
fn run_info_mode(text: &Text)      // Hardware detection mode
fn run_health_check_mode(...)      // Stress test mode
fn print_cpu_result(...)           // Format CPU test results
fn print_ram_result(...)           // Format RAM test results
```

**Data Flow:**
```
CLI Args → Parse → Select Language → Route to Mode
    ↓
Info Mode: Detect HW → Print Info → Exit
    ↓
Stress Mode: Run CPU Test → Run RAM Test → Print Summary → Exit
```

#### 2. Hardware Detection (hw/)
**Purpose:** Detect and display system hardware information

**Submodules:**
- `cpu.rs`: CPU model, core count
- `ram.rs`: RAM total, used, free
- `disk.rs`: Disk name, capacity
- `gpu.rs`: GPU model, VRAM (platform-specific)

**Architecture:**
```
CpuInfo::new()
    ↓
sysinfo::System::new_all()
    ↓
Extract CPU info (model, cores)
    ↓
Return CpuInfo struct
```

**Data Structures:**
```rust
pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
}

pub struct RamInfo {
    pub total_gb: f64,
    pub used_gb: f64,
    pub free_gb: f64,
}

pub struct DiskInfo {
    pub name: String,
    pub total_gb: f64,
}

pub struct GpuInfo {
    pub model: String,
    pub vram_gb: Option<f64>,  // macOS only
}
```

#### 3. Stress Testing (stress/)
**Purpose:** Run health checks on CPU and RAM

**Submodules:**
- `cpu.rs`: CPU stress test (prime calculation)
- `ram.rs`: RAM stress test (write/read verify)
- `mod.rs`: Health status enum, test runners

**CPU Test Architecture:**
```
CpuTestConfig { duration_secs, thread_count, verbose }
    ↓
run_cpu_test(config)
    ↓
┌───────────────────────────────────────┐
│  Spawn N threads (N = logical CPUs)   │
│  Each thread:                         │
│    - Calculate primes (CPU intensive) │
│    - Track operation count            │
│    - Track timing                     │
└───────────────────────────────────────┘
    ↓
Background monitor: Track per-core CPU usage
    ↓
Collect results from all threads
    ↓
Calculate metrics (ops/sec, variance, etc.)
    ↓
Evaluate health (Healthy/IssuesDetected/Failed)
    ↓
Return CpuTestResult
```

**RAM Test Architecture:**
```
RamTestConfig { max_gb }
    ↓
run_ram_test(config)
    ↓
Calculate allocation: min(80% available, 16GB)
    ↓
┌───────────────────────────────────────┐
│  Allocate buffer                       │
│  Write test pattern (0xAA55...)       │
│  Measure write speed                  │
│  Read and verify all data             │
│  Measure read speed                   │
│  Count errors                         │
└───────────────────────────────────────┘
    ↓
Evaluate health (Healthy/IssuesDetected/Failed)
    ↓
Return RamTestResult
```

**Health Evaluation:**
```rust
pub enum HealthStatus {
    Healthy,
    IssuesDetected(Vec<String>),  // Warnings
    Failed(String),                // Critical failure
}

// CPU Health Rules
fn evaluate_cpu_health(result: &CpuTestResult) -> HealthStatus {
    if temp > 95°C → Failed
    if variance > 200% → Failed
    if temp > 85°C → IssuesDetected
    if freq_drop > 10% → IssuesDetected
    else → Healthy
}

// RAM Health Rules
fn evaluate_ram_health(result: &RamTestResult) -> HealthStatus {
    if errors > 0 → Failed
    if write_speed < 0.3 GB/s → Failed
    if read_speed < 0.3 GB/s → Failed
    else → Healthy
}
```

#### 4. Sensors Monitoring (sensors/)
**Purpose:** Real-time hardware monitoring during stress tests

**Submodules:**
- `temp.rs`: CPU temperature reading
- `frequency.rs`: CPU frequency per-core & average
- `monitor.rs`: Background CPU usage monitor

**Temperature Monitoring:**
```
get_cpu_temp()
    ↓
sysinfo::Components::new()
    ↓
Filter CPU-related components (cpu, tdie, core, package, tcal)
    ↓
Read temperature from each sensor
    ↓
Return CpuTemp { current: f64, sensors: Vec<f64> }
```

**Frequency Monitoring:**
```
get_cpu_frequency()
    ↓
sysinfo::System::new()
    ↓
Extract per-core frequency (MHz)
    ↓
Calculate average frequency
    ↓
Return CpuFrequency { cores, per_core_mhz, avg_ghz }
```

**CPU Usage Monitor:**
```
CpuMonitorHandle::start()
    ↓
Spawn background thread
    ↓
Loop every 200ms:
    - Sample CPU usage per core
    - Update shared HashMap<usize, f32>
    - Track max samples (60)
    ↓
Return handle for get_per_core_usage()
```

#### 5. Platform Abstraction (platform/)
**Purpose:** Platform-specific implementations

**Architecture:**
```
pub enum Platform {
    MacOS,
    Windows,
    Linux,
}

pub fn detect() -> Platform {
    if cfg!(target_os = "macos") → Platform::MacOS
    else if cfg!(target_os = "windows") → Platform::Windows
    else → Platform::Linux
}
```

**Platform-Specific Code:**
```rust
// macOS
#[cfg(target_os = "macos")]
fn get_gpu_info() -> Vec<GpuInfo> {
    // Use system_profiler SPDisplaysDataType
}

// Windows
#[cfg(target_os = "windows")]
fn get_gpu_info() -> Vec<GpuInfo> {
    // Use PowerShell Get-WmiObject Win32_VideoController
}

// Linux
#[cfg(target_os = "linux")]
fn get_gpu_info() -> Vec<GpuInfo> {
    // Use lspci -vnnn
}
```

#### 6. Output Formatting (fmt.rs)
**Purpose:** Consistent, color-coded terminal output

**Components:**
- ANSI color codes (GREEN, YELLOW, RED, RESET)
- Progress bars
- Temperature color coding
- Usage color coding
- Table formatting

**Architecture:**
```
User-facing data
    ↓
Apply color codes based on values
    ↓
Format into tables/progress bars
    ↓
Print to stdout with proper spacing
```

**Example:**
```rust
fn temp_color(temp: f64) -> &'static str {
    if temp > 90.0 → RED
    else if temp > 80.0 → ORANGE
    else if temp > 70.0 → YELLOW
    else → GREEN
}

fn progress_bar(percent: u8, width: usize) -> String {
    let filled = (percent as usize * width) / 100;
    let empty = width - filled;
    format!("{}{} {}", "█".repeat(filled), "░".repeat(empty), percent)
}
```

#### 7. Multi-Language Support (lang.rs)
**Purpose:** Support for Vietnamese and English

**Architecture:**
```
Language enum (Vietnamese, English)
    ↓
Text struct holds all translations
    ↓
text.method() returns translated string
    ↓
Output uses translated strings
```

**Example:**
```rust
pub enum Language {
    Vietnamese,
    English,
}

pub struct Text {
    lang: Language,
}

impl Text {
    pub fn cpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese → "CPU",
            Language::English → "CPU",
        }
    }

    pub fn testing_cpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese → "Đang kiểm tra CPU",
            Language::English → "Testing CPU",
        }
    }
}
```

---

## Concurrency Model

### CPU Stress Test Concurrency
```
Main Thread
    ↓
Spawn N Worker Threads (N = logical CPUs)
    ↓
┌─────────┬─────────┬─────────┬─────────┐
│ Thread 1│ Thread 2│ Thread 3│  ...    │
│ Primes  │ Primes  │ Primes  │ Primes  │
└─────────┴─────────┴─────────┴─────────┘
    │         │         │         │
    └─────────┴─────────┴─────────┘
              │
        Shared State:
        - Arc<AtomicBool> running
        - Arc<AtomicU64> total_ops
              │
        Background Monitor Thread
        - Samples CPU usage every 200ms
        - Updates HashMap<usize, f32>
              │
Main Thread:
- Collects results every 1s
- Prints progress (normal or verbose)
- Joins all threads
```

**Thread Safety:**
- `Arc<AtomicBool>`: Shared flag for thread coordination
- `Arc<AtomicU64>`: Shared counter for total operations
- `Arc<Mutex<HashMap>>`: Not needed (monitor has own HashMap)

### RAM Test Concurrency
```
Main Thread (Single-threaded)
    ↓
Allocate buffer
    ↓
Write test pattern (measure speed)
    ↓
Read and verify (measure speed)
    ↓
Return results
```

**Note:** RAM test is single-threaded to avoid memory contention.

---

## Data Flow Diagrams

### Info Mode Flow
```
User runs: pchecker
    ↓
Select language (interactive)
    ↓
For each component (CPU, GPU, RAM, Disk):
    - Detect hardware
    - Format output
    - Print section
    ↓
Print footer with execution time
```

### Stress Mode Flow (Normal)
```
User runs: pchecker --stress
    ↓
Select language (interactive)
    ↓
CPU Test:
    - Spawn threads
    - Every 1s: Print progress line (overwrite)
    - Collect results
    - Clear progress line
    - Print result table
    ↓
RAM Test:
    - Allocate buffer
    - Write/read verify
    - Print result table
    ↓
Print summary:
    - Critical issues (if any)
    - Issues detected (if any)
    - Overall health status
```

### Stress Mode Flow (Verbose)
```
User runs: pchecker --stress --verbose
    ↓
Select language (interactive)
    ↓
CPU Test:
    - Spawn threads
    - Every 1s:
        - Print main progress line
        - Print per-core usage rows (visual bars)
        - Print temperature sensors list
        - Move cursor back to overwrite
    - Collect results
    - Clear all lines
    - Print result table
    ↓
RAM Test:
    - (Same as normal mode)
    ↓
Print summary (same as normal mode)
```

---

## Verbose Mode Architecture (v0.2.0)

### Implementation
**Location:** `src/stress/cpu.rs`

**Key Components:**
1. **Per-Core Usage Display:**
   - Background monitor samples CPU usage per core
   - HashMap<usize, f32> maps core index to usage percentage
   - Visual bar chart: `[████████░░] 80%`

2. **Platform-Specific Formatting:**
   - macOS: 4 cores/row, shows usage %
   - Windows/Linux: 3 cores/row, shows usage %@frequency

3. **Temperature Sensors:**
   - List up to 8 temperature sensors
   - Format: `Sensor name: 45.0°C`

**Data Flow:**
```
Background Monitor Thread
    ↓
Sample CPU usage every 200ms
    ↓
Update HashMap<usize, f32>
    ↓
Main Thread (every 1s)
    ↓
get_per_core_usage() → HashMap<usize, f32>
    ↓
build_per_core_display(freq, usage, cores, verbose)
    ↓
Return Vec<String> of formatted rows
    ↓
print_cpu_progress_box() with verbose=true
    ↓
Print:
    - Main progress line
    - Per-core usage rows
    - Temperature sensors
    ↓
Move cursor back to overwrite
```

**Terminal Management:**
```
Normal mode: 1 line to clear
Verbose mode:
    - Main line: 1
    - Per-core rows: (cores + cores_per_row - 1) / cores_per_row
    - Sensors: up to 8 sensors
    Total: ~1 + core_rows + 9 lines
```

---

## Error Handling Strategy

### Error Types
1. **Recoverable Errors:**
   - Missing sensors → Return None or default value
   - Allocation failure → Reduce allocation size
   - Thread panic → Mark as Failed with message

2. **Unrecoverable Errors:**
   - System call failures → Propagate error
   - Invalid user input → Clap handles (validate before use)

### Error Propagation
```
Low-level function
    ↓
Return Result<T, E> or Option<T>
    ↓
Mid-level function
    ↓
Match on Result/Option
    ↓
Convert to domain-specific type (HealthStatus)
    ↓
High-level function
    ↓
Display to user
```

### Example:
```rust
// Low-level: Returns None if unavailable
pub fn get_cpu_temp() -> Option<CpuTemp> {
    // Try to read temperature
    // Return None on failure
}

// Mid-level: Uses Option
fn print_cpu_progress_box(...) {
    let temp_str = temp.map(|t| format!("{:.1}°C", t.current))
        .unwrap_or_else(|| "N/A".to_string());
}

// High-level: Converts to HealthStatus
fn evaluate_cpu_health(result: &CpuTestResult) -> HealthStatus {
    if result.temperature.is_none() {
        // No temp data → can't evaluate overheating
        // Continue with other checks
    }
}
```

---

## Performance Considerations

### CPU Stress Test
- **Workload:** Prime number calculation (CPU-intensive)
- **Thread Count:** Equal to logical CPUs (max utilization)
- **Duration:** Configurable (default: 60s)
- **Metrics:** Operations/second, variance, temperature, frequency

### RAM Stress Test
- **Allocation:** 80% of available (max 16GB)
- **Write Pattern:** 0xAA55_AA55_AA55_AA55 (alternating bits)
- **Read/Verify:** Full data validation
- **Metrics:** Write/read speed, error count

### Output Performance
- **Update Frequency:** 1 second (avoids flicker)
- **String Allocation:** Minimized in hot loops
- **Terminal Clearing:** Cursor movement (rewrite instead of clear)

---

## Security Architecture

### Input Validation
- Clap derives validate CLI arguments
- Range checks (duration > 0)
- Conflict detection (--quick vs --duration)

### Resource Limits
- Max RAM allocation: 16GB (prevents OOM)
- Thread count: Limited to logical CPUs
- Test duration: User-controlled (no hard limit)

### Platform Code Safety
- No shell injection (use Rust APIs)
- Validate external command output (if any)
- Handle platform-specific errors gracefully

---

## Extensibility

### Adding New Hardware Detection
1. Create new module in `hw/` (e.g., `battery.rs`)
2. Implement `{Component}Info` struct
3. Add detection in `main.rs`
4. Update language support in `lang.rs`

### Adding New Stress Test
1. Create new module in `stress/` (e.g., `gpu.rs`)
2. Implement `run_{component}_test(config)`
3. Implement `evaluate_{component}_health(result)`
4. Add to health check mode in `main.rs`

### Adding New Platform Support
1. Add `Platform` variant in `platform/mod.rs`
2. Add `cfg!(target_os = "...")` detection
3. Implement platform-specific functions with `#[cfg]`
4. Test on target platform

---

## Testing Architecture

### Unit Tests
- Location: In each module (e.g., `cpu.rs`)
- Focus: Single function logic
- Example: `test_evaluate_cpu_health()`

### Integration Tests (Future)
- Location: `tests/` directory
- Focus: End-to-end workflows
- Example: `test_full_stress_test()`

### Manual Testing
- Location: `examples/` directory
- Focus: Specific capabilities
- Example: `test_sysinfo.rs`

---

**Last Updated:** 2025-12-25
**Document Version:** 1.0
