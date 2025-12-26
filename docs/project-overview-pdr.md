# PChecker Project Overview & PDR

**Version:** 0.2.0
**Last Updated:** 2025-12-26
**Status:** Active Development

---

## Project Overview

### Purpose
PChecker is a cross-platform hardware detection and health check CLI tool designed to:
- Detect system hardware information (CPU, GPU, RAM, Disk)
- Run stress tests to identify potential hardware issues
- Provide real-time monitoring during tests
- Support multiple languages (Vietnamese, English)
- Work across macOS (Apple Silicon), Windows, and Linux

### Target Users
- System administrators diagnosing hardware issues
- Developers testing system stability
- Hardware enthusiasts monitoring system performance
- IT technicians performing diagnostics

### Key Value Propositions
- **Cross-Platform:** Single codebase supports macOS, Windows, Linux
- **Zero Dependencies:** No external tools required (uses only Rust crates)
- **Fast:** Optimized binary with small footprint (~500KB)
- **User-Friendly:** Clear output with color-coded status indicators
- **Comprehensive:** Tests CPU stability, RAM integrity, thermal performance

---

## Product Development Requirements (PDR)

### Functional Requirements

#### FR-1: Hardware Detection
**Priority:** High
**Status:** Implemented (v0.1.0)

The system shall detect and display:
- CPU model name and core count
- GPU model name and VRAM (platform-specific)
- RAM total, used, and free memory in GB
- Disk name and total capacity
- Operating system platform

**Acceptance Criteria:**
- Detection completes within 2 seconds
- Information accuracy >= 95%
- Graceful handling of missing sensors

#### FR-2: CPU Health Check
**Priority:** High
**Status:** Implemented (v0.1.0)

The system shall:
- Spawn threads equal to logical CPU cores
- Run intensive prime number calculations
- Track operation count, speed, and variance
- Monitor temperature changes
- Detect frequency drops (throttling)

**Acceptance Criteria:**
- Configurable test duration (default: 60s)
- Metrics accuracy within 5% margin
- Detects crashes, overheating, instability
- Variance calculation identifies performance inconsistency

#### FR-3: RAM Health Check
**Priority:** High
**Status:** Implemented (v0.1.0)

The system shall:
- Allocate up to 80% of available RAM (max 16GB)
- Write test pattern (0xAA55_AA55_AA55_AA55)
- Read and verify all data
- Measure write/read speeds

**Acceptance Criteria:**
- Detects single-bit errors
- Speed measurement accurate within 10%
- Handles allocation failures gracefully
- Max test duration: 30 seconds

#### FR-4: Verbose Mode
**Priority:** Medium
**Status:** Implemented (v0.2.0)

The system shall provide detailed output when `--verbose` flag is used:
- Per-core CPU usage with visual bar charts
- Platform-specific formatting (4 cores/row on macOS, 3 on Windows/Linux)
- Temperature sensors list (up to 8 sensors)
- Real-time updates every second

**Acceptance Criteria:**
- Visual bars accurately represent usage percentage
- Platform-specific formatting matches OS conventions
- Updates do not cause flicker (proper terminal clearing)
- Sensor information available when supported by hardware

#### FR-5: Disk Health Check
**Priority:** Medium
**Status:** Implemented (v0.2.0)

The system shall:
- Run disk read/write speed tests
- Detect SSD vs HDD type
- Support multi-disk selection
- Display SMART data when available

**Acceptance Criteria:**
- Sequential read/write speed measurement
- Disk type detection (SSD/HDD)
- Support for --all-disks, --disk-index, --list-disks flags
- Verbose mode shows SMART attributes

#### FR-6: GPU Health Check
**Priority:** Medium
**Status:** Implemented (v0.2.0)

The system shall:
- Run GPU thermal monitoring during stress
- Optional wgpu-based compute shader test
- Detect integrated vs discrete GPU
- Translate GPU type to Vietnamese/English

**Acceptance Criteria:**
- Temperature tracking (start, end, max)
- Compute shader test via feature flag (gpu-compute)
- GPU type detection: "Integrated"/"Tích hợp" vs "Discrete"/"Rời"
- Graceful fallback when compute unavailable

#### FR-7: Multi-Language Support
**Priority:** Medium
**Status:** Implemented (v0.1.0)

The system shall support:
- Vietnamese (default)
- English
- Interactive language selection at startup

**Acceptance Criteria:**
- All user-facing text translatable
- Language selection persistent during session
- No hardcoded strings in output paths

#### FR-8: CLI Interface
**Priority:** High
**Status:** Implemented (v0.2.0)

The system shall support:
- `--stress` / `-s`: Run all health checks (CPU + RAM + Disk + GPU)
- `--cpu-stress`, `--ram-stress`, `--disk-stress`, `--gpu-stress`: Individual tests
- `--duration` / `-d`: Set test duration in seconds
- `--quick`: Quick 15-second test
- `--verbose` / `-v`: Detailed metrics
- `--all-disks`, `--disk-index`, `--list-disks`: Disk selection
- Flag combinations (e.g., `-s -d 30 -v`)

**Acceptance Criteria:**
- Help text available via `--help`
- Version info available via `--version`
- Conflicting flags properly detected (`--quick` vs `--duration`)
- Default values clearly documented

### Non-Functional Requirements

#### NFR-1: Performance
**Priority:** High
**Requirements:**
- Startup time < 100ms
- Binary size < 1MB (release build)
- Memory overhead < 50MB during info mode
- CPU usage during stress test: 95%+ on all cores

**Implementation:**
- Release profile optimized for size (`opt-level = "z"`)
- LTO enabled for better optimization
- Symbol stripping reduces binary size
- Panic = abort reduces runtime overhead

#### NFR-2: Reliability
**Priority:** High
**Requirements:**
- No crashes during normal operation
- Graceful handling of missing sensors
- Thread-safe concurrent operations
- Proper cleanup of resources

**Implementation:**
- Result types for error handling
- Arc<AtomicBool> for thread coordination
- Option types for missing data
- Comprehensive test coverage

#### NFR-3: Usability
**Priority:** Medium
**Requirements:**
- Clear output with color coding
- Progress indicators during long operations
- Self-explanatory status messages
- Consistent formatting

**Implementation:**
- ANSI color codes for terminal output
- Progress bars for visual feedback
- Table formatting for results
- Icon indicators (✅ ⚠️ ❌)

#### NFR-4: Maintainability
**Priority:** Medium
**Requirements:**
- Clear module separation
- Comprehensive inline documentation
- Test coverage for critical paths
- Follows Rust best practices

**Implementation:**
- Module organization: hw, stress, sensors, platform
- Unit tests in each module
- Type-safe enums for health status
- Platform-specific code isolated via cfg macros

#### NFR-5: Portability
**Priority:** High
**Requirements:**
- Support macOS (Apple Silicon), Windows, Linux
- No external dependencies beyond Rust crates
- Conditional compilation for platform-specific code
- Consistent behavior across platforms

**Implementation:**
- `cfg!` macros for platform detection
- Platform trait for OS-specific implementations
- Cross-platform libraries (sysinfo, clap)
- Fallback behavior for unsupported features

### Technical Constraints

#### TC-1: Dependencies
- Must use only Rust crates from crates.io
- Preferred crates: sysinfo, clap, num_cpus
- No external system tools (e.g., no PowerShell, bash commands)

#### TC-2: Rust Edition
- Minimum Rust version: 1.70
- Edition: 2021
- Target platforms: x86_64, aarch64 (Apple Silicon)

#### TC-3: Build System
- Cargo for package management
- No build scripts (use pure Rust)
- Release profile: size-optimized, stripped binary

---

## Architecture Overview

### Module Structure
```
pcheck/                    # Project root
├── src/
│   ├── main.rs           # CLI entry point, orchestration
│   ├── hw/               # Hardware detection modules (modular)
│   │   ├── mod.rs        # Module exports
│   │   ├── cpu/          # CPU detection + platform/{macos,windows,linux}.rs
│   │   ├── ram/          # RAM detection + platform/{macos,windows,linux}.rs
│   │   ├── disk/         # Disk detection + platform/{macos,windows,linux}.rs
│   │   └── gpu.rs        # GPU detection + platform/{macos,windows,linux}.rs
│   ├── stress/           # Health check modules (modular)
│   │   ├── mod.rs        # Health status enum, exports
│   │   ├── cpu/          # CPU test + platform/{macos,windows,linux}.rs
│   │   ├── ram/          # RAM test + platform/{macos,windows,linux}.rs
│   │   ├── disk/         # Disk test + smart.rs
│   │   ├── gpu.rs        # GPU thermal + compute test
│   │   └── gpu_compute.rs # wgpu-based compute shader
│   ├── sensors/          # Hardware monitoring
│   │   ├── mod.rs        # Sensor module exports
│   │   ├── temp.rs       # CPU temperature reading
│   │   ├── frequency.rs  # CPU frequency per-core & average
│   │   └── monitor.rs    # Background CPU usage monitor thread
│   ├── platform/
│   │   └── mod.rs        # Platform detection (macOS/Windows/Linux)
│   ├── fmt.rs            # Output formatting, ANSI colors, progress bars
│   ├── lang.rs           # Multi-language support (Vietnamese/English)
│   └── prompt.rs         # Interactive language selection (placeholder)
├── docs/                 # Project documentation
│   ├── project-overview-pdr.md
│   ├── code-standards.md
│   ├── codebase-summary.md
│   ├── system-architecture.md
│   └── project-roadmap.md
├── plans/                # Project plans
│   ├── active/           # Active plans
│   └── completed/        # Completed plans
├── reports/              # Agent reports
├── ROADMAP.md            # Single roadmap file
├── README.md             # User guide
├── DEV_GUIDE.md          # Development commands
└── Cargo.toml            # Project manifest
```

### Design Patterns
1. **Strategy Pattern:** Platform trait for OS-specific implementations
2. **Module Pattern:** Clear separation (hw, stress, sensors, platform)
3. **Builder Pattern:** clap derive for CLI argument parsing
4. **Monitor Pattern:** Background thread for CPU usage tracking
5. **Facade Pattern:** `CpuInfo::new()`, `RamInfo::new()` hide complexity

### Data Flow
```
User Input (CLI Args)
    ↓
main.rs (Argument Parsing)
    ↓
Language Selection
    ↓
┌───────────────┬────────────────┐
│   Info Mode   │  Stress Mode   │
├───────────────┼────────────────┤
│ Detect HW     │ Run CPU Test   │
│ Display Info  │ Run RAM Test   │
└───────────────┴────────────────┘
    ↓
Formatted Output (Tables, Progress Bars, Color Codes)
```

---

## Current Status & Features

### Implemented (v0.2.0)
- [x] Hardware detection (CPU, GPU, RAM, Disk)
- [x] CPU stress test with prime calculation
- [x] RAM stress test with write/read verify
- [x] Disk stress test with read/write speed check
- [x] GPU stress test (wgpu-based, optional)
- [x] Temperature monitoring
- [x] Frequency tracking and throttling detection
- [x] Verbose mode with per-core metrics
- [x] Multi-language support (Vietnamese, English)
- [x] Platform-specific implementations (macOS, Windows, Linux)
- [x] Modular platform structure (hw/*/mod.rs + platform/ subdirs)
- [x] GPU type translations (Integrated/Discrete)

### Known Limitations
- VRAM detection incomplete on Windows/Linux (TODO)
- Temperature reading may fail on some systems (no fallback)
- Language selection is interactive only (no `--lang` flag)
- No integration tests for full workflow
- GPU compute test requires feature flag (not default)

---

## Future Roadmap

### v0.3.0 (Planned)
- [ ] Command-line language selection (`--lang` flag)
- [ ] JSON output mode for automation
- [ ] Config file support
- [ ] Improved GPU compute test stability

### v0.5.0 (Planned)
- [ ] VRAM detection for Windows/Linux
- [ ] Battery health check (laptops)
- [ ] Network interface detection
- [ ] Export results to file

### v1.0.0 (Future)
- [ ] Continuous monitoring mode
- [ ] Web dashboard
- [ ] Historical data tracking
- [ ] Automated benchmarking

---

## Success Metrics

### Technical Metrics
- Test coverage: >80% for critical modules
- Binary size: <1MB (release build)
- Startup time: <100ms
- Crash rate: <0.1%

### User Metrics
- GitHub stars: Track community interest
- Issues: Bug reports and feature requests
- Downloads: Binary release usage
- Contributions: Community PRs

---

## Documentation

### User Documentation
- README.md: Installation and usage guide
- CLI help: `pchecker --help`
- Examples: `examples/` directory

### Developer Documentation
- Code standards: `docs/code-standards.md`
- Architecture: `docs/system-architecture.md`
- Codebase summary: `docs/codebase-summary.md`
- Roadmap: `docs/project-roadmap.md`

---

## Dependencies

### Runtime Dependencies
```toml
[dependencies]
sysinfo = "0.37"           # System info (CPU, RAM, components)
clap = { version = "4.5", features = ["derive"] }  # CLI parsing
num_cpus = "1.16"          # CPU count detection
fastrand = "2.1"           # Random number generation

# Optional: GPU compute stress test
wgpu = { version = "0.20", optional = true }
pollster = { version = "0.3", optional = true }
bytemuck = { version = "1.14", optional = true }

# Optional: Apple SMC temperature reading (macOS only)
smc = { version = "0.2", optional = true }

[features]
default = []
gpu-compute = ["wgpu", "pollster", "bytemuck"]
apple-smc = ["smc"]
```

### Build Dependencies
- Rust 1.70+ (edition 2021)
- Cargo package manager

---

## License & Distribution

**License:** MIT
**Repository:** https://github.com/Khoa280703/pcheck
**Author:** Khoa280703

Distribution:
- GitHub releases for binary downloads
- Cargo registry for crate publishing
- Cross-platform binaries (macOS, Windows, Linux)

---

**Last Updated:** 2025-12-26
**Document Version:** 1.1
