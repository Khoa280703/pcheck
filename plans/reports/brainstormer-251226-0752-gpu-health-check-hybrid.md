# Brainstorming Report: GPU Health Check - Hybrid Approach

**Date:** 2025-12-26
**Status:** AGREED - Ready for implementation
**Report:** `plans/reports/brainstormer-251226-0752-gpu-health-check-hybrid.md`

---

## Problem Statement

Implement GPU health check for pchecker v0.3.0 with:
- Hybrid approach: lightweight (normal/quick) + full stress test (--verbose)
- Cross-platform support (Windows, Linux, macOS)
- Windows-first priority (most used laptop buyers)
- Development constraint: macOS-only testing for now

---

## Requirements

| Feature | Normal/Quick Mode | --verbose Mode |
|---------|-------------------|----------------|
| GPU Info | ✅ Model, VRAM, Type | ✅ Same + driver info |
| GPU Type | ✅ iGPU vs dGPU | ✅ Same |
| Temperature | ✅ Check only | ✅ Monitor during load |
| Stress Test | ❌ No | ✅ wgpu compute shader "burn GPU" |

**Temperature thresholds:**
- ⚠️ Warning: ≥ 85°C
- 🔴 FAIL: > 95°C

---

## Evaluated Approaches

### Option 1: Full GPU Stress Test (REJECTED)
- Compute shader + artifact detection + thermal monitoring
- ❌ Adds heavy dependencies (wgpu + backends)
- ❌ Binary size 5MB+
- ❌ Overkill for normal mode

### Option 2: Simple Thermal Only (REJECTED)
- GPU info + temperature check
- ✅ Lightweight
- ❌ Doesn't test GPU under load
- ❌ Misses potential issues

### Option 3: Hybrid Approach ✅ AGREED
- Normal/Quick: Info + thermal check
- --verbose: Info + thermal + wgpu stress test
- ✅ Best of both worlds
- ✅ Optional wgpu feature flag

---

## Final Solution

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    GPU Health Check Strategy                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────┐    ┌─────────────────┐                     │
│  │  Normal / Quick │    │  --verbose Mode │                     │
│  └────────┬────────┘    └────────┬────────┘                     │
│           │                      │                               │
│           ▼                      ▼                               │
│  ┌─────────────────┐    ┌─────────────────┐                     │
│  │ 1. GPU Info     │    │ 1. GPU Info     │                     │
│  │ 2. Type (iGPU/dGPU)│ │ 2. Type (iGPU/dGPU)│                  │
│  │ 3. Temperature  │    │ 3. Temperature  │                     │
│  │ 4. Verdict      │    │ 4. wgpu Load    │                     │
│  └─────────────────┘    │ 5. Temp Monitor │                     │
│                         │ 6. Stability    │                     │
│                         │ 7. Verdict      │                     │
│                         └─────────────────┘                     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Files to Create/Modify

```
src/hw/gpu.rs              # MODIFY - Add type detection, VRAM for Win/Linux
src/stress/gpu.rs          # NEW - GPU thermal check (all platforms)
src/stress/gpu_stress.rs   # NEW - wgpu compute test (optional feature)
src/stress/mod.rs          # MODIFY - Add GPU modules
src/main.rs                # MODIFY - Integrate GPU test
src/lang.rs                # MODIFY - Add GPU translations
Cargo.toml                 # MODIFY - Add optional wgpu dependency
```

### Platform Priority

```
Priority: Windows > Linux > macOS
Reason: Windows = most used laptop buyers market
```

### GPU Type Detection Logic

```rust
fn gpu_type(name: &str) -> GpuType {
    let name_lower = name.to_lowercase();

    // Integrated GPUs
    if name_lower.contains("intel")
        || name_lower.contains("integrated")
        || name_lower.contains("uhd")
        || name_lower.contains("iris")
        || name_lower.contains("apple m")  // Apple Silicon
    {
        GpuType::Integrated
    }
    // Discrete GPUs
    else if name_lower.contains("nvidia")
        || name_lower.contains("amd")
        || name_lower.contains("radeon")
        || name_lower.contains("geforce")
        || name_lower.contains("rtx")
        || name_lower.contains("gtx")
        || name_lower.contains("rx ")
    {
        GpuType::Discrete
    }
    else {
        GpuType::Unknown
    }
}
```

### Temperature Thresholds

| Condition | Output |
|-----------|--------|
| Temp < 85°C | ✅ Normal |
| 85°C ≤ Temp < 95°C | ⚠️ WARNING (thermal throttle possible) |
| Temp ≥ 95°C | 🔴 FAIL (overheating) |

### Dependencies

```toml
[dependencies]
# Existing
sysinfo = "0.37"
clap = { version = "4.5", features = ["derive"] }
num_cpus = "1.16"
fastrand = "2.1"

# New - optional for verbose mode
wgpu = { version = "23", optional = true }

[features]
default = []
gpu-stress = ["wgpu"]  # Enable with: cargo build --features gpu-stress
```

### Binary Size Estimates

| Configuration | Estimated Size |
|---------------|----------------|
| Current (no GPU) | ~550KB |
| + GPU thermal (no wgpu) | ~600KB |
| + wgpu (release, lto+strip) | ~2MB |
| + feature flag (default) | ~600KB (no wgpu) |

---

## Implementation Considerations

### Development Constraint: macOS-Only Testing

**Can implement now (macOS tested):**
- ✅ GPU info detection (all platforms - simple shell commands)
- ✅ GPU type detection (string matching - platform-agnostic)
- ✅ Thermal check (sysinfo - cross-platform)
- ✅ wgpu Metal backend (macOS specific)

**Needs Windows/Linux testing later:**
- ⚠️ wgpu DirectX 12 (Windows)
- ⚠️ wgpu Vulkan (Linux)
- ⚠️ Edge cases (multiple GPUs, driver issues)

**Mitigation:**
- Add runtime warnings for untested platforms
- Use GitHub Actions CI for cross-platform builds
- Graceful degradation if wgpu fails

### Platform-Specific Commands

**Windows:**
```powershell
# GPU Info
Get-WmiObject Win32_VideoController | Select-Object Name, AdapterRAM, DriverVersion

# VRAM in bytes (divide by 1073741824 for GB)
```

**Linux:**
```bash
# GPU Info
lspci -vnnn | grep "VGA compatible controller"

# VRAM
cat /sys/class/drm/card*/device/mem_info_vram_total
```

**macOS:**
```bash
# Already implemented
system_profiler SPDisplaysDataType
```

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| wgpu init fails | GPU stress unavailable | Graceful fallback to thermal-only |
| Windows/Linux untested | Potential bugs | Runtime warnings + CI builds |
| Binary size bloat | User complaints | Feature flag (opt-in) |
| Multiple GPU systems | Detection ambiguity | List all GPUs with index |
| Apple Silicon GPU quirks | Metal-specific issues | Test on M4, handle gracefully |

---

## Success Metrics

- [ ] GPU type detection (iGPU/dGPU) on all platforms
- [ ] Temperature displayed in all modes
- [ ] wgpu stress test only with `--verbose`
- [ ] Default binary < 1MB
- [ ] Graceful degradation if wgpu fails
- [ ] Health verdict based on temperature thresholds
- [ ] Runtime warnings for untested platforms

---

## Next Steps

1. Implement Phase 1: GPU info + type detection (all platforms)
2. Implement Phase 2: Thermal check (reuse sysinfy Components)
3. Implement Phase 3: wgpu stress test (macOS first, add warnings for Win/Linux)
4. Add GitHub Actions CI for cross-platform builds
5. Test on Windows/Linux when available
6. Remove warnings after validation

---

## Unresolved Questions

1. **GPU stress test duration?** (Suggest: 30s fixed for verbose)

2. **GPU buffer size?** (Suggest: 10% of detected VRAM)

3. **Separate `--gpu-stress` flag?** (Currently: `--verbose` enables GPU stress)

---

## Sources

- [Wgpu Compute Examples Reddit](https://www.reddit.com/r/gibnaq/wgpu_compute_examples/)
- [Rust WebGPU Getting Started](https://medium.com/@aleksej.gudkov/rust-webgpu-example-getting-started-with-gpu-programming-in-rust-fc36dace37d6)
- [Checking if GPU is integrated Stack Overflow](https://stackoverflow.com/questions/40061123/checking-if-gpu-is-integrated-or-not)
- [Wgpu binary size discussion](https://github.com/gfx-rs/wgpu/issues/3103)
- [Rust dependencies and binary size HN](https://news.ycombinator.com/item?id=43935067)
