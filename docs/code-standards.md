# PChecker Code Standards & Conventions

**Version:** 0.2.0
**Last Updated:** 2025-12-26

---

## Codebase Structure

### Directory Organization
```
pcheck/                    # Project root (flattened from pcheck/pchecker/)
├── Cargo.toml              # Project manifest
├── Cargo.lock              # Dependency lock file
├── .gitignore              # Git ignore patterns
│
├── src/                    # Source code
│   ├── main.rs             # CLI entry point, orchestration
│   ├── hw/                 # Hardware detection modules
│   ├── stress/             # Health check modules
│   ├── sensors/            # Hardware monitoring
│   ├── platform/           # Platform-specific code
│   ├── fmt.rs              # Output formatting
│   ├── lang.rs             # Multi-language support
│   └── prompt.rs           # Interactive prompts
│
├── docs/                   # Documentation
│   ├── project-overview-pdr.md
│   ├── code-standards.md
│   ├── codebase-summary.md
│   ├── system-architecture.md
│   └── project-roadmap.md
│
├── plans/                  # Project plans
│   ├── active/             # Active development plans
│   └── completed/          # Completed plans
│
├── reports/                # Agent reports
│   └── *.md                # Generated agent reports
│
├── examples/               # Example programs
│   └── test_sysinfo.rs     # sysinfo capability test
│
├── ROADMAP.md              # Single roadmap file
├── README.md               # User guide
└── DEV_GUIDE.md            # Development commands
```

### Module Responsibilities

#### src/main.rs
- CLI argument parsing via clap derive
- Language selection orchestration
- Mode selection (info vs stress)
- Result printing and formatting
- Entry point only (no business logic)

#### src/hw/
**Purpose:** Hardware detection and information gathering

**Modular Platform Structure (v0.2.0):**
- Each component has its own directory with `mod.rs` and `platform/` subdirectory
- Platform-specific implementations: `macos.rs`, `windows.rs`, `linux.rs`

| Module | Responsibility |
|--------|----------------|
| mod.rs | Module exports |
| cpu/ | CPU model, core count detection + platform/ |
| ram/ | RAM total/used/free detection + platform/ |
| disk/ | Disk name and capacity detection + platform/ |
| gpu.rs | GPU model, VRAM, type detection + platform/ |

**Conventions:**
- Struct name: `{Component}Info` (e.g., `CpuInfo`, `RamInfo`)
- Constructor: `new()` method in `mod.rs`
- Platform-specific code in `platform/{macos,windows,linux}.rs`
- Display: `display()` or `to_string()` for formatted output
- Use sysinfo crate for data
- GPU type: "Integrated"/"Discrete" (English), "Tích hợp"/"Rời" (Vietnamese)

#### src/stress/
**Purpose:** Health check and stress testing

**Modular Platform Structure (v0.2.0):**
- Each component has its own directory with `mod.rs` and `platform/` subdirectory (except GPU)

| Module | Responsibility |
|--------|----------------|
| mod.rs | HealthStatus enum, test runners |
| cpu/ | CPU stress test, prime calculation + platform/ |
| ram/ | RAM stress test, write/read verify + platform/ |
| disk/ | Disk stress test, read/write speed + smart.rs |
| gpu.rs | GPU thermal + compute test (no platform/ subdirs) |
| gpu_compute.rs | wgpu-based compute shader (optional) |

**Conventions:**
- Config struct: `{Component}TestConfig`
- Result struct: `{Component}TestResult`
- Test runner: `run_{component}_test(config)`
- Health evaluator: `evaluate_{component}_health(result)`
- Return `HealthStatus` enum (Healthy, IssuesDetected, Failed)
- GPU test: Optional feature flag `gpu-compute` for wgpu dependencies

#### src/sensors/
**Purpose:** Real-time hardware monitoring

| Module | Responsibility |
|--------|----------------|
| mod.rs | Sensor exports, utilities |
| temp.rs | CPU temperature reading |
| frequency.rs | CPU frequency per-core & average |
| monitor.rs | Background CPU usage monitor thread |

**Conventions:**
- Sensor struct: `{Sensor}Data` or `{Sensor}` (e.g., `CpuTemp`, `CpuFrequency`)
- Reader function: `get_{sensor}()` returns Option or Result
- Monitor handle: `{Sensor}MonitorHandle` for lifecycle management
- Thread-safe: Use Arc<Atomic*> for shared state

#### src/platform/
**Purpose:** Platform-specific implementations

| Module | Responsibility |
|--------|----------------|
| mod.rs | Platform enum, detection, trait |

**Conventions:**
- Platform enum: `Platform` (MacOS, Windows, Linux)
- Detection: `detect()` function
- cfg! macros for compile-time platform selection
- Runtime detection via env variables

#### src/fmt.rs
**Purpose:** Output formatting and terminal UI

**Conventions:**
- ANSI color codes: Reset, color constants (GREEN, YELLOW, RED)
- Helper functions: `progress_bar()`, `temp_color()`, `usage_color()`
- Table formatting: Fixed width (54 chars)
- Minimal dependencies (no external TUI libraries)

#### src/lang.rs
**Purpose:** Multi-language support

**Conventions:**
- Language enum: `Language` (Vietnamese, English)
- Text struct: `Text` holds all translated strings
- Method: `text.{section}()` returns translated string
- No hardcoded user-facing strings in business logic

#### src/prompt.rs
**Purpose:** Interactive user prompts

**Conventions:**
- Standalone functions: `select_{feature}_standalone()`
- Loop until valid input
- Default fallback for errors
- Clear instructions and error messages

---

## Naming Conventions

### Rust Standard Conventions
- **Types:** PascalCase (e.g., `CpuInfo`, `HealthStatus`)
- **Functions:** snake_case (e.g., `get_cpu_temp`, `run_stress_test`)
- **Constants:** SCREAMING_SNAKE_CASE (e.g., `RESET`, `GREEN`)
- **Modules:** snake_case (e.g., `hw`, `stress`, `sensors`)
- **Variables:** snake_case (e.g., `cpu_temp`, `thread_count`)

### Domain-Specific Conventions
- **Config structs:** `{Component}TestConfig` (e.g., `CpuTestConfig`)
- **Result structs:** `{Component}TestResult` (e.g., `CpuTestResult`)
- **Data structs:** `{Component}Info` (e.g., `CpuInfo`, `RamInfo`)
- **Sensor structs:** `{Sensor}Data` or `{Sensor}` (e.g., `CpuTemp`, `CpuFrequency`)
- **Monitor handles:** `{Sensor}MonitorHandle` (e.g., `CpuMonitorHandle`)

### Function Naming
- **Getters:** `get_{property}()` (e.g., `get_cpu_temp()`)
- **Runners:** `run_{component}_test()` (e.g., `run_cpu_test()`)
- **Evaluators:** `evaluate_{component}_health()` (e.g., `evaluate_cpu_health()`)
- **Printers:** `print_{component}_result()` (e.g., `print_cpu_result()`)
- **Formatters:** `format_{property}()` (e.g., `format_large_number()`)

---

## Code Style Guidelines

### Formatting
- Use `rustfmt` with default settings
- Max line width: 100 chars (soft limit)
- Indent: 4 spaces (no tabs)

### Comments & Documentation
```rust
/// Module-level documentation
/// Explains purpose, usage, examples

// Section comments for major blocks
// Function-level comments for non-obvious logic

//! Crate documentation (if publishing)
```

**Requirements:**
- Public structs: Document with `///`
- Public functions: Document purpose, parameters, return values
- Complex algorithms: Explain logic
- Platform-specific code: Document why cfg! is used

### Error Handling
```rust
// Use Result for recoverable errors
pub fn get_cpu_temp() -> Option<CpuTemp> {
    // Returns None if temperature unavailable
}

// Use Option for missing data
pub struct CpuTestResult {
    pub temperature: Option<CpuTemp>,  // May be unavailable
}

// Use HealthStatus enum for test results
pub enum HealthStatus {
    Healthy,
    IssuesDetected(Vec<String>),
    Failed(String),  // Error message
}
```

**Conventions:**
- Prefer `Result<T, E>` over panic
- Use `Option<T>` for nullable values
- Use domain-specific enums (e.g., `HealthStatus`)
- Avoid unwrap() in production code (use expect() with message)

### Concurrency
```rust
// Use Arc for shared ownership
let running = Arc::new(AtomicBool::new(true));

// Clone Arc for each thread
let running_clone = Arc::clone(&running);

// Use atomic types for counters
let total_ops = Arc::new(AtomicU64::new(0));
total_ops.fetch_add(1, Ordering::Relaxed);

// Join threads and handle results
for thread in threads {
    thread.join().expect("Thread panicked");
}
```

**Conventions:**
- `Arc<T>` for shared ownership across threads
- `AtomicBool`, `AtomicU64` for shared state
- `Ordering::Relaxed` for counters (strict ordering not required)
- Join all threads before returning results

### Platform-Specific Code
```rust
// Compile-time platform detection
let cores_per_row = if cfg!(target_os = "macos") { 4 } else { 3 };

// Runtime platform detection
match platform {
    Platform::MacOS => { /* macOS code */ }
    Platform::Windows => { /* Windows code */ }
    Platform::Linux => { /* Linux code */ }
}

// Platform-specific functions
#[cfg(target_os = "macos")]
fn get_gpu_info() -> String { /* macOS impl */ }

#[cfg(target_os = "windows")]
fn get_gpu_info() -> String { /* Windows impl */ }
```

**Conventions:**
- Use `cfg!` for compile-time branches (same binary)
- Use `#[cfg]` for platform-specific functions
- Isolate platform code in separate modules when possible
- Document platform differences clearly

---

## Testing Standards

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_test_short() {
        // 1-second CPU test for rapid testing
    }

    #[test]
    fn test_evaluate_cpu_health() {
        // Test all health rule branches
        let result = create_test_result(/* ... */);
        let health = evaluate_cpu_health(&result);
        assert!(matches!(health, HealthStatus::Healthy));
    }
}
```

**Requirements:**
- Test public APIs
- Test edge cases (empty, zero, max values)
- Test error conditions
- Use short durations for stress tests (1-2 seconds)
- Mock external dependencies when possible

### Test Organization
- Place tests in same file as code (module-level `#[cfg(test)]`)
- Test file: `{module}.rs` (e.g., `cpu.rs` has tests at bottom)
- Integration tests: `tests/` directory (future)
- Examples: `examples/` directory for manual testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_cpu_test_short

# Run with output
cargo test -- --nocapture

# Run only unit tests (ignore doc tests)
cargo test --lib
```

---

## Dependencies Guidelines

### Adding Dependencies
1. Check if existing crates can suffice
2. Prefer widely-used, well-maintained crates
3. Minimize dependency count
4. Review license compatibility
5. Update Cargo.toml and Cargo.lock

### Current Dependencies
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

### Dependency Criteria
- **sysinfo:** Cross-platform system info (required)
- **clap:** CLI argument parsing (required)
- **num_cpus:** CPU core detection (required)
- **fastrand:** Random number generation (required)
- **wgpu/pollster/bytemuck:** GPU compute test (optional)
- **smc:** Apple SMC temperature (optional, macOS only)
- **Future:** TUI library, logging, serialization

### Forbidden Dependencies
- CLI frameworks other than clap (use clap for consistency)
- Heavy TUI libraries (keep it simple for now)
- Async runtimes (tokio, async-std) - not needed yet

---

## Build & Release Standards

### Development Build
```bash
cargo build
# Binary: target/debug/pchecker
# Size: ~5MB (with debug symbols)
```

### Release Build
```toml
[profile.release]
opt-level = "z"            # Optimize for size
lto = true                 # Link-time optimization
codegen-units = 1          # Single codegen unit
strip = true               # Strip symbols
panic = "abort"            # Abort on panic
```

```bash
cargo build --release
# Binary: target/release/pchecker
# Size: ~500KB-1MB (stripped, optimized)
```

### Version Management
- Update `Cargo.toml` version field
- Update version in `src/main.rs` (clap command)
- Tag release in Git: `v0.2.0`
- Create GitHub release with binaries

### Cross-Compilation
```bash
# macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# Windows (via cross or on Windows)
cargo build --release --target x86_64-pc-windows-msvc

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

---

## Git Commit Conventions

### Commit Message Format
```
<type>: <brief description>

<extended description (optional)>
```

### Types
- `feat`: New feature (e.g., `feat: add verbose mode`)
- `fix`: Bug fix (e.g., `fix: handle missing temperature sensor`)
- `docs`: Documentation (e.g., `docs: update README`)
- `refactor`: Code refactor (e.g., `refactor: extract platform module`)
- `test`: Test updates (e.g., `test: add cpu health test`)
- `chore`: Build/config (e.g., `chore: update dependencies`)

### Examples
```
feat: add verbose mode for per-core CPU metrics

- Add --verbose flag
- Display per-core usage with visual bars
- Platform-specific formatting (4 cores/row on macOS)
- Show temperature sensors list
```

---

## Code Review Checklist

### Before Submitting PR
- [ ] Code follows naming conventions
- [ ] Public functions documented
- [ ] Tests added/updated
- [ ] No hardcoded user-facing strings
- [ ] Platform-specific code isolated
- [ ] Error handling uses Result/Option
- [ ] No unwrap() in production code
- [ ] Thread safety considered (if concurrent)
- [ ] Cargo.toml updated (if new deps)
- [ ] README updated (if user-facing change)

### Review Criteria
- **Correctness:** Does it work as intended?
- **Style:** Does it follow conventions?
- **Documentation:** Is it clear and complete?
- **Testing:** Are tests comprehensive?
- **Performance:** Is it efficient?
- **Safety:** Is it memory-safe?
- **Portability:** Does it work on all platforms?

---

## Best Practices

### Do's
- Use Rust's type system to prevent errors
- Leverage Option/Result for error handling
- Write tests for critical paths
- Document public APIs
- Use meaningful variable names
- Keep functions focused and short
- Prefer composition over inheritance
- Use platform abstractions

### Don'ts
- Don't use unwrap() in production code
- Don't hardcode user-facing strings
- Don't mix platform code (isolate it)
- Don't ignore compiler warnings
- Don't use unsafe unless necessary
- Don't create God functions (keep them focused)
- Don't duplicate code (DRY principle)
- Don't optimize prematurely (measure first)

---

## Performance Guidelines

### CPU Stress Test
- Use all cores (thread count = logical CPUs)
- Prime calculation is CPU-intensive (good for stress test)
- Track operations/second for performance metric
- Calculate variance to detect instability

### RAM Stress Test
- Allocate 80% of available RAM (max 16GB)
- Write pattern: 0xAA55_AA55_AA55_AA55 (alternating bits)
- Measure write/read speeds for performance metric
- Verify all data to detect errors

### Output Formatting
- Minimize allocations in hot loops
- Use fixed-size buffers where possible
- Avoid string concatenation in loops
- Use ANSI codes directly (no library overhead)

---

## Security Considerations

### Input Validation
- Validate CLI arguments (clap derive handles most)
- Check for reasonable ranges (duration > 0)
- Handle invalid user input gracefully

### Resource Limits
- Max RAM allocation: 16GB (prevent OOM)
- Max test duration: No hard limit (user-controlled)
- Thread count: Limited to logical CPUs

### Platform Code
- Avoid shell injection (use Rust APIs)
- Validate external command output (if any)
- Handle platform-specific errors gracefully

---

**Last Updated:** 2025-12-26
**Document Version:** 1.1
