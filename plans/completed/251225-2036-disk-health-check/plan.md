# Plan: Disk Health Check Feature

**Version:** 0.3.0
**Date:** 2025-12-25
**Status:** Planning

---

## Overview

Add comprehensive disk health check functionality to pchecker, including:
- Sequential read/write speed tests
- Random access (seek) time measurement
- Bad sector detection
- Optional individual test flags

---

## User Requirements (from /plan command)

1. **Full Diagnostics:** Speed + seek time + bad sector detection
2. **Integration:**
   - `--stress` runs ALL tests (CPU + RAM + Disk)
   - Add individual flags: `--cpu-stress`, `--ram-stress`, `--disk-stress`
3. **GPU:** Skip for now (deferred to future version)

---

## Architecture

### File Structure

```
src/stress/
├── mod.rs           # Update: add disk module export
├── cpu.rs           # Existing (no changes)
├── ram.rs           # Existing (no changes)
└── disk.rs          # NEW: disk health check implementation
```

### New Module: `src/stress/disk.rs`

```rust
// Disk health check module
// Tests disk by sequential/random I/O operations

pub struct DiskTestConfig {
    pub test_path: Option<String>,  // Custom path (default: temp dir)
    pub test_size_mb: u64,          // Test file size (default: 100MB)
    pub include_seek_test: bool,    // Random access test (default: true)
    pub verbose: bool,              // Show detailed progress
}

pub struct DiskTestResult {
    pub disk_name: String,
    pub test_path: String,
    pub read_speed_mb_s: f64,
    pub write_speed_mb_s: f64,
    pub seek_time_ms: f64,          // Average random access time
    pub bad_sectors: u64,           // Count of read errors
    pub health: HealthStatus,
}

pub fn run_stress_test(config: DiskTestConfig) -> DiskTestResult;
```

---

## Implementation Details

### 1. Sequential Write Speed Test

**Method:**
- Create test file in temp directory
- Write sequential blocks (1MB chunks)
- Measure time and calculate MB/s

**Platform notes:**
- macOS/Linux: `/tmp` or `std::env::temp_dir()`
- Windows: `%TEMP%`

```rust
fn write_test(path: &Path, size_mb: u64) -> Result<(f64, u64), Error> {
    // Write sequential 1MB blocks
    // Return: (speed_mb_s, bytes_written)
}
```

### 2. Sequential Read Speed Test

**Method:**
- Read entire test file sequentially
- Measure time and calculate MB/s
- Verify data integrity

```rust
fn read_test(path: &Path, expected_size: u64) -> Result<(f64, u64), Error> {
    // Read sequential 1MB blocks
    // Verify data integrity
    // Return: (speed_mb_s, errors)
}
```

### 3. Random Access (Seek) Time Test

**Method:**
- Perform random seeks + reads
- Measure average latency
- Detect slow seeks (indicating mechanical issues)

```rust
fn seek_test(path: &Path, iterations: u32) -> Result<f64, Error> {
    // Random position seeks (4KB reads)
    // Measure time for each seek+read
    // Return: average seek time in ms
}
```

### 4. Bad Sector Detection

**Method:**
- During read test, verify data patterns
- Count mismatches as potential bad sectors
- Multiple retries for failed reads

```rust
fn verify_pattern(path: &Path) -> Result<u64, Error> {
    // Read and verify written patterns
    // Return: count of errors
}
```

---

## Health Evaluation Criteria

### Disk Health Rules

| Condition | Result |
|-----------|--------|
| Write failed (permission/IO error) | `Failed: Cannot write to disk - check permissions` |
| Read speed < 10 MB/s (HDD) / < 50 MB/s (SSD) | `Failed: Extremely slow read speed - dying disk` |
| Write speed < 10 MB/s (HDD) / < 30 MB/s (SSD) | `Failed: Extremely slow write speed - dying disk` |
| Bad sectors > 0 | `Failed: Bad sectors detected - disk failure imminent` |
| Seek time > 20ms (HDD) / > 5ms (SSD) | `IssuesDetected: Slow seek time - possible mechanical issue` |
| Large speed variance (>50%) | `IssuesDetected: Unstable performance - check disk health` |
| Otherwise | `Healthy` |

### Disk Type Detection

Auto-detect SSD vs HDD:
- macOS: `diskutil info` (Solid State: Yes)
- Linux: `/sys/block/sdX/queue/rotational`
- Windows: WMI `Win32_DiskDrive` (MediaType)
- Fallback: Use generic thresholds

---

## CLI Integration

### New Flags

```rust
#[derive(Parser, Debug)]
struct Args {
    // Existing
    #[arg(short, long)]
    stress: bool,           // Run ALL tests (CPU+RAM+Disk)

    #[arg(short, long, default_value = "60")]
    duration: u64,

    #[arg(long)]
    quick: bool,

    #[arg(short, long)]
    verbose: bool,

    // NEW: Individual test flags
    #[arg(long)]
    cpu_stress: bool,       // CPU test only

    #[arg(long)]
    ram_stress: bool,       // RAM test only

    #[arg(long)]
    disk_stress: bool,      // Disk test only
}
```

### Flag Logic

```
if --cpu-stress or --ram-stress or --disk-stress:
    Run only specified tests
else if --stress:
    Run all tests (CPU + RAM + Disk)
else:
    Info mode (default)
```

### Example Usage

```bash
# Run all tests
pchecker --stress

# Run only disk test
pchecker --disk-stress

# Run CPU + Disk only
pchecker --cpu-stress --disk-stress

# Quick disk test
pchecker --disk-stress --quick
```

---

## Progress Display

### Normal Mode

```
⏳ Disk: [██████████████] 100% | Write: 512 MB/s | Read: 2.1 GB/s | Seek: 0.3ms
```

### Verbose Mode (`--verbose`)

```
⏳ Disk: [██████████████] 100% | 256 MB tested

💿 Disk Health Check:
   📁 Test file: /tmp/pchecker_disk_test.tmp
   ✏️  Write: 512.3 MB/s
   📖 Read:  2,145.7 MB/s
   🔍 Seek:  0.3ms avg (1000 random reads)
   🔥 Errors: 0 bad sectors

   Disk Type: SSD (NVMe)
   Status: ✅ Healthy
```

---

## Language Support

Add to `src/lang.rs`:

```rust
impl Text {
    pub fn testing_disk(&self) -> &str { ... }
    pub fn disk_health_check(&self) -> &str { ... }
    pub fn write_speed(&self) -> &str { ... }
    pub fn read_speed(&self) -> &str { ... }
    pub fn seek_time(&self) -> &str { ... }
    pub fn bad_sectors(&self) -> &str { ... }
    // ...
}
```

---

## Error Handling

### Test Failures

| Error Type | Handling |
|------------|----------|
| Permission denied | `Failed: Cannot write to test location` |
| Disk full | `Failed: Insufficient disk space` |
| File too large | Reduce test size, retry |
| IO error during test | `Failed: Disk I/O error - hardware failure` |

### Cleanup

Always delete test file after test completes:
```rust
fn cleanup_test_file(path: &Path) {
    let _ = std::fs::remove_file(path);  // Ignore errors
}
```

---

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_disk_test_small() {
    // 10MB test in temp dir
    let config = DiskTestConfig {
        test_size_mb: 10,
        include_seek_test: false,
        verbose: false,
    };
    let result = run_stress_test(config);
    assert!(result.write_speed_mb_s > 0.0);
    assert!(matches!(result.health, HealthStatus::Healthy));
}

#[test]
fn test_evaluate_disk_health() {
    // Test various health scenarios
}
```

### Platform-Specific Tests

- macOS: Test on APFS/HFS
- Linux: Test on ext4/btrfs
- Windows: Test on NTFS

---

## Dependencies

**No new dependencies required** - use Rust stdlib:
- `std::fs::{File, OpenOptions}`
- `std::io::{Write, Read, Seek, SeekFrom}`
- `std::time::Instant`

---

## Implementation Order

1. **Phase 1: Core Module**
   - Create `src/stress/disk.rs`
   - Implement `DiskTestConfig`, `DiskTestResult`
   - Implement write test (sequential)
   - Implement read test (sequential)
   - Basic health evaluation

2. **Phase 2: Advanced Tests**
   - Implement seek test (random access)
   - Implement bad sector detection
   - Disk type detection (SSD/HDD)

3. **Phase 3: CLI Integration**
   - Add flags to `Args` struct
   - Update `main.rs` logic for selective testing
   - Update `run_health_check_mode()` to include disk
   - Add result printing function

4. **Phase 4: Language & Polish**
   - Add Vietnamese/English strings
   - Verbose mode display
   - Error messages
   - Documentation update

---

## Output Format

### Result Table

```
┌──────────────────────────────────────────────────────────┐
│ 💾 Disk Health Check                           ✅       │
├──────────────────────────────────────────────────────────┤
│ tested: 100 MB                                           │
│ write speed: 512.3 MB/s                                  │
│ read speed: 2,145.7 MB/s                                 │
│ seek time: 0.3ms                                         │
│ bad sectors: 0                                           │
└──────────────────────────────────────────────────────────┘
```

---

## Risks & Mitigation

| Risk | Mitigation |
|------|------------|
| Test file too large for available space | Default to 100MB, configurable |
| SSD wear from repeated writes | Keep test size minimal, warn user |
| Permission issues on some systems | Fallback to user home directory |
| Slow tests on HDD | Optional skip for seek test |
| Platform-specific path issues | Use `std::env::temp_dir()` |

---

## Post-Implementation

1. Update README.md with disk test documentation
2. Update project-roadmap.md (v0.3.0 → completed)
3. Run full test suite on all platforms
4. Create release notes for v0.3.0

---

**Estimated Implementation Time:** 3-4 hours
**Test Coverage Target:** 80%
**Breaking Changes:** None (backward compatible)
