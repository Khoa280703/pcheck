# pchecker CLI Output Design Report

**Date:** 2025-12-25
**Project:** pchecker - Cross-platform Hardware Detection CLI
**Designer:** UI/UX Subagent

---

## Summary

Designed CLI output format for pchecker MVP. Simple text-based, emoji-enhanced, scannable layout targeting non-technical users (PC buyers) and sysadmins.

---

## Deliverables

### 1. Design Guidelines
**File:** `/Users/khoa2807/development/pcheck/docs/design-guidelines.md`

Complete specification including:
- Output structure & section order
- Emoji icon set (cross-platform Unicode)
- Field alignment rules (12-char label padding)
- Number formatting (1 decimal for memory)
- Error message format
- Platform-specific examples (macOS, Windows, Linux)

### 2. Wireframe HTML
**File:** `/Users/khoa2807/development/pcheck/docs/wireframes/output-mockup.html`

Interactive mockup with tabbed interface showing:
- macOS M4 output (your MacBook)
- Windows NVIDIA GPU output
- Linux server output
- Error state with diagnostics

### 3. Screenshots
**Directory:** `/Users/khoa2807/development/pcheck/docs/wireframes/`

- `output-mockup-all.png` - Full wireframe overview
- `macos-m4-output.png` - Apple Silicon example
- `windows-nvidia-output.png` - Windows gaming PC
- `linux-server-output.png` - Headless server
- `error-state-output.png` - Error handling

---

## Output Format (Final)

```
============================================================
🤖 PCHECKER v0.1.0 - Hardware Info Tool
============================================================

💻 SYSTEM     macOS 15.2 (Darwin 24.6.0)
🧠 CPU        Apple M4 (8 cores)
🎮 GPU        Apple M4 GPU (10 cores)
💾 RAM        16.0 GB (12.4 GB available)
💿 DISK       APPLE SSD AP1024Z 512 GB

============================================================
Done in 0.12s
============================================================
```

---

## Key Design Decisions

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| Width | Fixed 60 chars | Works on all terminals, narrow windows |
| Icons | Unicode emoji only | No font files, works everywhere |
| Colors | None (MVP) | Max compatibility, NO_COLOR support |
| Labels | 12-char padding | Aligned values, easy scanning |
| Numbers | 1 decimal (16.0 GB) | Precision without noise |
| Error style | Cause + Suggestion | Actionable feedback |

---

## Emoji Set (Cross-Platform)

| Icon | Usage |
|------|-------|
| 🤖 | Brand logo |
| 💻 | System/OS |
| 🧠 | CPU |
| 🎮 | GPU |
| 💾 | RAM |
| 💿 | Disk |
| ⚠️ | Warning |
| ❌ | Error |

Avoided: Flags, skin tones, Windows-only emoji

---

## Section Order (MVP)

1. SYSTEM - OS + version
2. CPU - Model + cores
3. GPU - Model + VRAM
4. RAM - Total + available
5. DISK - Model + capacity

---

## Error Handling Pattern

```
⚠️  GPU        Failed to detect GPU
────────────────────────────────────
⚠️  WARNING: GPU detection failed

Cause: Could not access GPU information.
       NVIDIA/AMD drivers may not be installed.

Suggestion: Install GPU drivers or run with admin privileges.
```

Clear separation: problem -> cause -> action

---

## Future Enhancements (Post-MVP)

- ANSI colors (Green/Yellow/Red status)
- Progress bars during detection
- Multi-language support (Vietnamese)
- JSON export flag
- Compact mode (one-line output)

---

## Files Created

```
docs/
├── design-guidelines.md           # Complete output spec
wireframes/
├── output-mockup.html             # Interactive wireframe
├── output-mockup-all.png          # Full screenshot
├── macos-m4-output.png            # macOS example
├── windows-nvidia-output.png      # Windows example
├── linux-server-output.png        # Linux example
└── error-state-output.png         # Error example
```

---

## Unresolved Questions

1. Should terminal width detection trigger "compact mode" or error message?
2. GPU detection on Linux without proprietary drivers - fallback behavior?
3. Multi-disk systems - show all or primary only?
