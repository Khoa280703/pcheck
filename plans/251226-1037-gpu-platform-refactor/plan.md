---
title: "Platform-Specific GPU Code Refactoring"
description: "Separate platform-specific GPU code into dedicated modules for better maintainability"
status: pending
priority: P2
effort: 6h
branch: main
tags: [refactoring, gpu, platform-specific, module-structure]
created: 2025-12-26
---

# Platform-Specific GPU Code Refactoring Plan

## Problem Statement

Current GPU code mixes platform-specific logic in single files with heavy use of `#[cfg(target_os = "...")]` attributes scattered throughout:

- **src/hw/gpu.rs**: GPU detection (macOS, Windows, Linux) in one file
- **src/stress/gpu.rs**: GPU stress testing with macOS-specific powermetrics/SMC code embedded
- **src/stress/gpu_compute.rs**: Cross-platform wgpu code (already clean)

### Issues

1. **Maintainability**: Platform code intermixed, hard to modify one platform without affecting others
2. **Readability**: Many `#[cfg]` blocks reduce code clarity
3. **Testability**: Difficult to test platform-specific code in isolation
4. **Extensibility**: Adding new platforms requires modifying existing files
5. **Apple-specific types**: `AppleGpuInfo`, `AppleGpuMetrics`, `ThermalPressure` defined in general GPU module

## Analysis Summary

### src/hw/gpu.rs (270 lines)
- `GpuType` enum (cross-platform)
- `GpuInfo` struct (cross-platform)
- `detect_macos()` - uses `system_profiler`
- `detect_windows()` - uses PowerShell WMI
- `detect_linux()` - uses `lspci` + sysfs parsing
- `try_get_vram_from_sysfs()` - Linux helper

### src/stress/gpu.rs (579 lines)
- Cross-platform: `GpuTemp`, `GpuTestConfig`, `GpuTestResult`, `run_stress_test()`, `evaluate_gpu_health()`, `get_gpu_temp()`
- macOS-only: `ThermalPressure`, `AppleGpuInfo`, `AppleGpuMetrics`, `get_apple_gpu_metrics()`, `get_apple_gpu_info()`, `get_smc_temperature()`
- macOS SMC feature gating with `#[cfg(all(target_os = "macos", feature = "apple-smc"))]`

### src/stress/gpu_compute.rs (188 lines)
- Already well-structured with feature-gating
- No platform-specific logic (uses wgpu abstraction)

## Recommended Module Structure

```
src/
├── hw/
│   ├── gpu/
│   │   ├── mod.rs              # Public API, trait definitions
│   │   ├── common.rs           # Cross-platform types (GpuType, GpuInfo)
│   │   └── platform/
│   │       ├── mod.rs          # Platform module exports
│   │       ├── macos.rs        # macOS GPU detection (system_profiler)
│   │       ├── windows.rs      # Windows GPU detection (WMI)
│   │       └── linux.rs        # Linux GPU detection (lspci, sysfs)
│
├── stress/
│   ├── gpu/
│   │   ├── mod.rs              # Public API, common test logic
│   │   ├── common.rs           # Cross-platform types & functions
│   │   ├── temp.rs             # Cross-platform temperature reading (sysinfo)
│   │   └── platform/
│   │       ├── mod.rs          # Platform module exports
│   │       ├── macos.rs        # macOS: powermetrics, SMC, system_profiler
│   │       ├── windows.rs      # Windows: WMI thermal queries
│   │       └── linux.rs        # Linux: sysfs thermal zones
│   │
│   └── gpu_compute.rs          # Keep as-is (already clean)
```

## Trait-Based Abstraction

### hw::gpu::PlatformDetector

```rust
pub trait PlatformDetector {
    /// Detect GPU on this platform
    fn detect() -> Vec<GpuInfo>;
}
```

### stress::gpu::PlatformMetrics

```rust
pub trait PlatformMetrics {
    /// Get platform-specific GPU metrics
    fn get_metrics(verbose: bool) -> Option<PlatformGpuMetrics>;
}
```

## Migration Steps

### Phase 1: Create New Module Structure (30 min)

1. Create directory structure under `src/hw/gpu/`
2. Create directory structure under `src/stress/gpu/`
3. Add module declarations

### Phase 2: Refactor hw::gpu (1.5h)

**Step 1: Create common types (hw/gpu/common.rs)**
- Move `GpuType` enum
- Move `GpuInfo` struct
- Keep `display()` method

**Step 2: Create platform trait (hw/gpu/mod.rs)**
- Define `PlatformDetector` trait
- Create platform dispatcher function

**Step 3: Extract macOS code (hw/gpu/platform/macos.rs)**
- Move `detect_macos()` to `MacosDetector` impl
- Add `#[cfg(target_os = "macos")]` at module level

**Step 4: Extract Windows code (hw/gpu/platform/windows.rs)**
- Move `detect_windows()` to `WindowsDetector` impl
- Add `#[cfg(target_os = "windows")]` at module level

**Step 5: Extract Linux code (hw/gpu/platform/linux.rs)**
- Move `detect_linux()` and `try_get_vram_from_sysfs()` to `LinuxDetector` impl
- Add `#[cfg(target_os = "linux")]` at module level

**Step 6: Update hw/gpu/mod.rs**
- Re-export public types
- Implement platform dispatcher

### Phase 3: Refactor stress::gpu (2.5h)

**Step 1: Create common types (stress/gpu/common.rs)**
- Move `GpuTemp` struct
- Move `GpuTestConfig` struct
- Move `GpuTestResult` struct

**Step 2: Create temp module (stress/gpu/temp.rs)**
- Move `get_gpu_temp()` (cross-platform using sysinfo)

**Step 3: Create platform trait (stress/gpu/mod.rs)**
- Define `PlatformMetrics` trait
- Define platform-specific metrics struct

**Step 4: Extract macOS code (stress/gpu/platform/macos.rs)**
- Move `ThermalPressure` enum
- Move `AppleGpuInfo` struct
- Move `AppleGpuMetrics` struct
- Move `get_apple_gpu_metrics()` to `MacosMetrics` impl
- Move `get_apple_gpu_info()` to `MacosMetrics` impl
- Move `get_smc_temperature()` to `MacosMetrics` impl
- Handle `#[cfg(feature = "apple-smc")]` for SMC

**Step 5: Create Windows stub (stress/gpu/platform/windows.rs)**
- Implement `WindowsMetrics` returning None
- Add WMI thermal queries as future enhancement

**Step 6: Create Linux stub (stress/gpu/platform/linux.rs)**
- Implement `LinuxMetrics` returning None
- Add sysfs thermal queries as future enhancement

**Step 7: Update stress/gpu/mod.rs**
- Keep `run_stress_test()` as main orchestrator
- Keep `evaluate_gpu_health()` (cross-platform logic)
- Use trait for platform metrics

### Phase 4: Update Imports & Tests (1h)

**Update main.rs**
- Change `use hw::GpuInfo` to `use hw::gpu::GpuInfo`
- Update stress module imports if needed

**Update stress/mod.rs**
- Update pub use statements for new paths

**Update tests**
- Update module paths in test imports
- Verify all tests pass

### Phase 5: Documentation & Cleanup (30 min)

1. Add module-level documentation
2. Document trait methods
3. Remove cfg attributes from file content (use module-level cfg)
4. Run `cargo clippy` and fix warnings
5. Run `cargo fmt` for consistent formatting

## File Changes Summary

### Files to Create

| File | Purpose |
|------|---------|
| `src/hw/gpu/mod.rs` | GPU detection module root |
| `src/hw/gpu/common.rs` | Cross-platform types |
| `src/hw/gpu/platform/mod.rs` | Platform module exports |
| `src/hw/gpu/platform/macos.rs` | macOS GPU detection |
| `src/hw/gpu/platform/windows.rs` | Windows GPU detection |
| `src/hw/gpu/platform/linux.rs` | Linux GPU detection |
| `src/stress/gpu/mod.rs` | GPU stress module root |
| `src/stress/gpu/common.rs` | Cross-platform test types |
| `src/stress/gpu/temp.rs` | Temperature reading (sysinfo) |
| `src/stress/gpu/platform/mod.rs` | Platform metrics exports |
| `src/stress/gpu/platform/macos.rs` | macOS metrics (powermetrics, SMC) |
| `src/stress/gpu/platform/windows.rs` | Windows metrics stub |
| `src/stress/gpu/platform/linux.rs` | Linux metrics stub |

### Files to Modify

| File | Changes |
|------|---------|
| `src/main.rs` | Update import paths |
| `src/stress/mod.rs` | Update pub use paths |
| `src/hw/mod.rs` | Update GPU module path |

### Files to Delete

| File | Action |
|------|--------|
| `src/hw/gpu.rs` | Delete after migration |
| `src/stress/gpu.rs` | Delete after migration |

## Dependencies

No new dependencies required. Existing dependencies remain:
- `sysinfo` for cross-platform temperature
- `smc` (feature-gated) for Apple SMC

## Testing Strategy

1. Run existing tests: `cargo test`
2. Test on macOS (primary platform)
3. Test on Windows (via CI or cross-platform testing)
4. Test on Linux (via CI or cross-platform testing)
5. Verify feature flags: `cargo test --features apple-smc`

## Benefits

1. **Separation of Concerns**: Each platform in its own module
2. **Easier Testing**: Can mock platform traits
3. **Better Readability**: No `#[cfg]` spam in logic files
4. **Easier Extension**: Add new platform without touching existing code
5. **Clear Ownership**: Platform-specific logic clearly isolated

## Risk Mitigation

1. **Incremental Migration**: Phase-by-phase approach
2. **Test Coverage**: Run tests after each phase
3. **Backward Compatibility**: Public API remains unchanged
4. **Feature Preservation**: All existing features maintained

## Future Enhancements

1. **Windows**: Implement WMI thermal queries in `windows.rs`
2. **Linux**: Implement sysfs thermal zone reading in `linux.rs`
3. **BSD**: Add FreeBSD support via `sysctl`
4. **Metrics**: Extend `PlatformMetrics` trait with more methods

## Unresolved Questions

1. Should `PlatformMetrics` return a unified `GpuMetrics` struct or platform-specific types?
   - **Recommendation**: Use unified enum `enum GpuMetrics { Apple(AppleGpuMetrics), Generic(...) }`

2. Should SMC be a separate crate dependency or inline code?
   - **Current**: Keep as feature-gated dependency (already working)

3. Should we add `std::error::Error` implementations for platform detection errors?
   - **Recommendation**: Yes, for better error handling in CLI
