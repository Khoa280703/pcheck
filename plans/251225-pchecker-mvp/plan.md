---
title: "pchecker MVP - Cross-Platform Hardware Detection CLI"
description: "MVP for cross-platform hardware detection CLI tool showing basic system info (CPU, GPU, RAM, Disk) on Windows/macOS/Linux"
status: pending
priority: P1
effort: 10h
branch: main
tags: [rust, cli, hardware-detection, mvp]
created: 2025-12-25
---

## Project Summary

**pchecker** is a cross-platform hardware detection CLI tool that displays basic system information in a simple text format. The MVP focuses on information display only (no stress testing) with a single `pck` command that shows:
- OS information
- CPU (model, cores)
- GPU (model, VRAM if available)
- RAM (total, available)
- Disk (model, size)

## Tech Stack

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust | Performance, safety, single-binary distribution |
| CPU/RAM/Disk | sysinfo 0.37+ | Cross-platform, well-maintained |
| CLI | clap 4.x (Derive API) | Simple, type-safe argument parsing |
| GPU | Platform-specific commands | No good Rust crate, OS native tools reliable |
| Build Optimization | LTO + strip | Reduce binary size (<5MB target) |

## Architecture

**Hybrid modular approach:**
- Modules by function: `cpu.rs`, `gpu.rs`, `ram.rs`, `disk.rs`, `platform.rs`, `fmt.rs`
- Platform-specific code inside each module using conditional compilation (`cfg`)
- Shared traits for consistent interface across platforms

```
src/
├── main.rs           # CLI entry point
├── platform/
│   ├── mod.rs        # Platform trait + detection
│   ├── windows.rs    # Windows-specific
│   ├── macos.rs      # macOS-specific
│   └── linux.rs      # Linux-specific
├── cpu.rs            # CPU detection (sysinfo)
├── gpu.rs            # GPU detection (platform commands)
├── ram.rs            # RAM detection (sysinfo)
├── disk.rs           # Disk detection (sysinfo)
└── fmt.rs            # Output formatting
```

## Implementation Phases

| Phase | Description | Status | File |
|-------|-------------|--------|------|
| 01 | MVP Setup - Project initialization | Pending | [phase-01-mvp-setup.md](./phase-01-mvp-setup.md) |
| 02 | Platform Detection - OS detection module | Pending | [phase-02-platform-detection.md](./phase-02-platform-detection.md) |
| 03 | CPU/RAM/Disk Detection - sysinfo integration | Pending | [phase-03-cpu-ram-disk.md](./phase-03-cpu-ram-disk.md) |
| 04 | GPU Detection - Platform-specific commands | Pending | [phase-04-gpu-detection.md](./phase-04-gpu-detection.md) |
| 05 | Output Formatting - Display logic | Pending | [phase-05-output-formatting.md](./phase-05-output-formatting.md) |
| 06 | Testing & Build - Cross-platform validation | Pending | [phase-06-testing-build.md](./phase-06-testing-build.md) |

## MVP Success Criteria

- [x] Single `pck` command runs without arguments
- [x] Displays all required info on Windows, macOS, and Linux
- [x] Single binary < 5MB after optimization
- [x] Clean, readable text output with emoji indicators
- [x] No external dependencies at runtime (static linking)

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| GPU detection varies by OS | Medium | Use platform-specific commands, fallback to generic detection |
| Binary size bloat | Low | LTO + strip, minimize dependencies |
| macOS M4 compatibility | Low | sysinfo supports Apple Silicon, test on target |
| Windows PowerShell version | Low | Use PowerShell 5+ (available on Win10+) |

## Next Steps

1. Begin with [Phase 01: MVP Setup](./phase-01-mvp-setup.md)
2. Execute phases sequentially
3. Test on all 3 platforms in Phase 06

## Unresolved Questions

- Final output format design (emoji choices, section order)
- Error handling strategy (missing info, command failures)
- Binary distribution method (cargo install, GitHub releases, homebrew)
