# PCHECKER Plans Directory

## Structure

```
plans/
├── README.md                # This file
├── completed/               # Completed versions
│   └── v0.1.0-mvp-detection/
├── roadmap/                 # Project roadmap & future versions
│   ├── master-plan.md
│   └── versions/
└── reports/                 # Research reports
```

---

## Completed Versions

### v0.1.0 - MVP: Hardware Info Detection ✅

**Status:** COMPLETED (2025-12-25)

**Scope:** Detect và hiển thị thông tin phần cứng
- Platform detection (Windows/macOS/Linux)
- CPU info (model, cores, vendor)
- GPU info (model, VRAM if available)
- RAM info (total, used)
- Disk info (name, size)
- Multi-language (Vietnamese/English)

**Files:** `completed/v0.1.0-mvp-detection/`

---

## Roadmap

### Master Plan

**File:** `roadmap/master-plan.md`

Full roadmap từ MVP → complete stress testing tool.

**Versions Timeline:**

| Version | Description | Status |
|---------|-------------|--------|
| v0.1.0 | Hardware Info Detection | ✅ Done |
| v0.2.0 | CPU + RAM Stress Test | 🚧 In Planning |
| v0.3.0 | Disk + GPU Stress Test | ⏳ Pending |
| v0.4.0 | Analysis & Scoring Engine | ⏳ Pending |
| v0.5.0 | Report Generation | ⏳ Pending |
| v1.0.0 | Full Release | ⏳ Pending |

---

## Version Plans

### v0.2.0 - CPU + RAM Stress Test

**File:** `roadmap/versions/v0.2.0-cpu-ram-stress-test.md`

**Features:**
- CPU load testing (1-2 minutes)
- RAM memory testing (30-60 seconds)
- Score calculation (0-100)
- Pass/Fail status
- Multi-language UI

---

## Research Reports

Research reports from exploration phase: `reports/`

- Rust hardware detection crates
- Platform-specific hardware commands
- GPU detection challenges

---

## Quick Links

- [Master Plan](./roadmap/master-plan.md)
- [v0.1.0 MVP](./completed/v0.1.0-mvp-detection/plan.md)
- [v0.2.0 CPU+RAM Stress Test](./roadmap/versions/v0.2.0-cpu-ram-stress-test.md)
