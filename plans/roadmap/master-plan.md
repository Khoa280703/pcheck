---
title: "pchecker MASTER PLAN - Hardware Detection & Health Check CLI"
description: "Full roadmap: Detect → Health Check → Report for used computer buyers"
status: active
priority: P0
branch: main
tags: [rust, cli, hardware-detection, health-check, mvp]
created: 2025-12-25
updated: 2025-12-25
---

## Project Vision

**pchecker** = Công cụ kiểm tra phần cứng khi mua máy cũ

**Philosophy:** Health Check, NOT Benchmark

| Health Check (What we do) | Benchmark (What we DON'T do) |
|---------------------------|------------------------------|
| Detect hardware faults | Compare performance |
| Find defective parts | Rank which is faster |
| Pass/Fail based on health | Score 0-100 |

**Use case:** Người mua máy cũ cần biết máy có **HỎNG** không - không cần biết máy mạnh bao nhiêu.

**Flow hoàn chỉnh:**
```
Detect Info → Health Check Each Component → Generate Health Report → Conclusion
```

**Target users:** Người không rành công nghệ mua máy cũ.

---

## Master Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              pchecker CLI Flow                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌──────────────┐    ┌─────────────┐    ┌────────────┐ │
│  │  1. DETECT  │ -> │ 2. HEALTH    │ -> │ 3. GENERATE │ -> │ 4. REPORT  │ │
│  │     Info    │    │    CHECK     │    │  STATUS    │    │            │ │
│  └─────────────┘    └──────────────┘    └─────────────┘    └────────────┘ │
│         │                  │                   │                │        │
│         ▼                  ▼                   ▼                ▼        │
│  ┌───────────┐      ┌──────────┐        ┌───────────┐    ┌───────────┐  │
│  │ • CPU     │      │ • CPU    │        │ • Health   │    │ • Healthy │  │
│  │ • GPU     │      │ • RAM    │        │   Status   │    │ • Issues  │  │
│  │ • RAM     │      │ • Disk   │        │ • Warnings │    │ • Failed  │  │
│  │ • Disk    │      │ • GPU    │        │           │    │           │  │
│  │ • Platform│      └──────────┘        └───────────┘    └───────────┘  │
│  └───────────┘                                                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Full Implementation Roadmap

| Version | Description | Status | Effort |
|---------|-------------|--------|--------|
| **v0.1.0** | Hardware Info Detection | ✅ Done | 4h |
| **v0.2.1** | CPU + RAM Health Check + Temp/Freq | ✅ Done | 8h |
| **v0.3.0** | Disk + GPU Health Check | ⏳ Pending | 8h |
| **v0.4.0** | Enhanced Report & Recommendations | ⏳ Pending | 3h |
| **v0.5.0** | CLI UX Improvements | ⏳ Pending | 2h |
| **v1.0.0** | Full Release | ⏳ Pending | - |

---

## Version History

### v0.1.0 - Hardware Info Detection ✅

**Status:** COMPLETED (2025-12-25)

**Features:**
- Platform detection (macOS/Windows/Linux)
- CPU info (model, cores)
- GPU info (model, VRAM if available)
- RAM info (total, used)
- Disk info (name, size)
- Multi-language UI (Vietnamese/English)

**Binary:** 550KB

---

### v0.2.1 - CPU + RAM Health Check + Temp/Freq ✅

**Status:** COMPLETED (2025-12-25)

**Features:**
- CPU health check (stability, temperature, frequency monitoring)
- RAM health check (bit error detection, speed test)
- Real temperature sensors via sysinfo Components
- CPU frequency tracking (start/end, drop %)
- Health status: HEALTHY / FAILED (no "issues" - only actual faults)
- Multi-language UI
- CLI flags: `--stress`, `--quick`, `-d <seconds>`

**Health Criteria (from Check.md):**

| Component | Healthy | Failed |
|-----------|---------|--------|
| **CPU** | No crash, temp ≤ 95°C, variance < 200% | Crash OR temp > 95°C OR variance > 200% |
| **RAM** | 0 errors, speed ≥ 0.3 GB/s | Errors > 0 OR speed < 0.3 GB/s |

**Files:**
- `src/stress/cpu.rs` - CPU health check with sensors
- `src/stress/ram.rs` - RAM health check
- `src/sensors/mod.rs` - Sensor module
- `src/sensors/temp.rs` - Temperature reading
- `src/sensors/frequency.rs` - CPU frequency reading
- `src/main.rs` - Stress test mode integration

**Output Example:**
```
┌──────────────────────────────────────────────────────────┐
│ 🧠 CPU Check                                     ✅ │
├──────────────────────────────────────────────────────────┤
│ operations:                                       170,207 │
│ ops/sec:                                            5674 │
│ avg time:                                         2.115ms │
│ variance:                                            83.9% │
│ temperature:                                        57.0°C │
│ frequency:                  4.51 GHz ->        4.51 GHz          │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│ 💾 RAM Check                                     ✅ │
├──────────────────────────────────────────────────────────┤
│ tested:                                            4.7 GB │
│ write speed:                                       7.9 GB/s │
│ read speed:                                        3.5 GB/s │
│ errors detected:                                           0 │
└──────────────────────────────────────────────────────────┘

SUMMARY: ✅ Hardware appears to be in good condition
```

---

### v0.3.0 - Disk + GPU Health Check (Planning)

**Status:** PENDING

**Planned Features:**

#### Disk Health Check
- Sequential read/write speed test
- Detect slow disks (possible failure)
- SSD vs HDD detection
- SMART status check (if available)

**Health Criteria:**
- SSD: Seq > 100 MB/s
- HDD: Seq > 50 MB/s
- Very slow = possibly failing disk

#### GPU Health Check
- Compute shader stress test
- Artifact detection
- Crash detection
- Thermal monitoring (if available)

**Platform-specific:**
- macOS: Metal compute
- Windows: DirectX/OpenGL
- Linux: OpenGL/Vulkan

---

### v0.4.0 - Enhanced Report (Planning)

**Status:** PENDING

**Planned Features:**
- Better issue descriptions
- Recommendations for each issue type
- Export report to file (TXT/JSON)
- Historical comparison (save previous results)

---

### v0.5.0 - CLI UX Improvements (Planning)

**Status:** PENDING

**Planned Features:**
- Progress bar during tests
- Ctrl+C graceful cancellation
- Select specific tests (`--test cpu,ram`)
- Verbose vs quiet mode

---

## Known Limitations

1. **Temperature monitoring**: Works on Linux/Windows/macOS (x86), limited on Apple Silicon (PMU sensors only)
2. **Throttling detection**: Frequency drop measurement (now implemented)
3. **RAM patterns**: Single pattern (0xAA55)
4. **GPU testing**: Platform-specific commands required
5. **Benchmark database**: None (health check only)

---

## Next Steps

1. Implement v0.3.0 (Disk + GPU Health Check)
2. Enhanced reporting
3. CLI/UX improvements
4. Cross-platform testing (Windows, Linux)

---

## Unresolved Questions

1. Should we add config file to save preferences?
2. Should we add JSON export for automation?
3. Should we add `--test` flag to run specific components only?
