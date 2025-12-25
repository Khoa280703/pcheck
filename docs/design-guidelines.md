# pchecker - Design Guidelines

## Output Format Specification

### Terminal Output Structure

```
============================================================
HEADER (Brand + Version)
============================================================

SECTION 1 (Icon + Label + Value)
SECTION 2 (Icon + Label + Value)
SECTION 3 (Icon + Label + Value)
...

============================================================
FOOTER (Status + Time)
============================================================
```

### MVP Output Sections (Order)

| Order | Icon | Label | Description |
|-------|------|-------|-------------|
| 1 | 💻 | SYSTEM | OS name, version, kernel |
| 2 | 🧠 | CPU | Model name, cores |
| 3 | 🎮 | GPU | Model name, memory |
| 4 | 💾 | RAM | Total, available |
| 5 | 💿 | DISK | Model, capacity |

### Field Alignment Rules

```
💻 SYSTEM     macOS 15.2 (Darwin 24.6.0)
│   │         │
│   │         └── Value (left-aligned, wraps if needed)
│   └────────────── Label (SYSTEM, CPU, GPU...)
└────────────────── Emoji icon (2 chars)
    └────────────── Separator (5 spaces minimum)
```

- **Icon column**: Fixed 2 chars (emoji)
- **Separator**: 5 spaces
- **Label column**: Fixed 12 chars (left-aligned, padded with spaces)
- **Value**: Left-aligned, no truncation (allow wrap)

## Emoji Usage

### Primary Icons (Consistent Set)

| Category | Emoji | Usage |
|----------|-------|-------|
| Brand | 🤖 | Header logo |
| System | 💻 | OS/Platform info |
| CPU | 🧠 | Processor info |
| GPU | 🎮 | Graphics card |
| RAM | 💾 | Memory |
| Disk | 💿 | Storage |
| Network | 🌐 | Network info |
| Success | ✅ | Pass/good |
| Warning | ⚠️ | Warning/caution |
| Error | ❌ | Fail/error |
| Info | ℹ️ | Information |
| Clock | ⏱️ | Execution time |
| Loading | ⠋, ⠙, ⠹, ⠸, ⠼, ⠴ | Spinner frames |

### Terminal Compatibility

- Use **Unicode emoji only** (no image-based)
- Avoid: Windows-only emoji, skin tone variants, flags
- Test on: Windows Terminal, iTerm2, Linux TTY

## Color Coding

### MVP: No Colors (Text Only)

For maximum terminal compatibility, MVP uses **plain text** with emoji indicators.

### Phase 2: Optional ANSI Colors

When colors are added, use this scheme:

| Element | ANSI Code | Hex | Usage |
|---------|-----------|-----|-------|
| Green (Success) | `\x1b[32m` | #10B981 | ✅ Pass, good values |
| Yellow (Warning) | `\x1b[33m` | #F59E0B | ⚠️ Caution, old hardware |
| Red (Error) | `\x1b[31m` | #EF4444 | ❌ Fail, critical issues |
| Cyan (Label) | `\x1b[36m` | #06B6D4 | Section labels |
| Reset | `\x1b[0m` | - | Reset to default |

### Color Detection

```rust
// Detect color support
fn supports_color() -> bool {
    // Check NO_COLOR env var
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }
    // Check terminal type
    match std::env::var("TERM") {
        Ok(term) if term.contains("color") || term.contains("ansi") => true,
        _ => false,
    }
}
```

## Error Message Format

### Error States

```
============================================================
❌ PCHECKER v0.1.0 - Error
============================================================

⚠️  ERROR: Failed to detect GPU

Cause: Could not access GPU information on this platform.
       GPU detection requires administrator privileges.

Suggestion: Run with elevated privileges:
            sudo pck

============================================================
Done in 0.03s
============================================================
```

### Error Types

| Error Type | Icon | Message Format |
|------------|------|----------------|
| Permission | 🔒 | `ERROR: Permission denied` |
| Not Found | 🔍 | `ERROR: Hardware not detected` |
| Timeout | ⏱️ | `ERROR: Detection timed out` |
| Unknown | ❓ | `ERROR: Unknown error` |

## Border/Divider Rules

### Primary Divider

```
============================================================
```

- 60 equals signs (`=`)
- Used for top and bottom borders
- Used before footer

### Section Separator

```
------------------------------------------------------------
```

- 60 hyphens (`-`)
- Used to separate major sections (if needed)

## Number Formatting

### Memory Units

| Unit | Bytes | Display |
|------|-------|---------|
| GB | 1024^3 | `16.0 GB` |
| MB | 1024^2 | `512.0 MB` |
| TB | 1024^4 | `1.5 TB` |

- Always show **1 decimal place** for memory
- Use space between number and unit

### CPU Core Display

```
8 cores          -> "8 cores"
8 cores / 16 threads -> "8 cores (16 threads)"
```

## Execution Time Display

```
Done in 0.12s
Done in 1.5s
Done in 12s
```

- Show decimal if < 1 second
- No decimal if >= 1 second
- Always include "s" suffix

## Platform-Specific Formats

### macOS (Apple Silicon)

```
💻 SYSTEM      macOS 15.2 (Darwin 24.6.0)
🧠 CPU         Apple M4 (8 cores)
🎮 GPU         Apple M4 GPU (10 cores)
💾 RAM         16.0 GB (12.4 GB available)
💿 DISK        APPLE SSD AP1024Z 512 GB
```

### Windows (NVIDIA GPU)

```
💻 SYSTEM      Windows 11 Pro (Build 22631)
🧠 CPU         Intel Core i7-12700H (12 cores)
🎮 GPU         NVIDIA GeForce RTX 3070 (8 GB)
💾 RAM         32.0 GB (24.2 GB available)
💿 DISK        Samsung 980 PRO 1 TB
```

### Linux Server

```
💻 SYSTEM      Ubuntu 22.04 LTS (Kernel 6.5.0)
🧠 CPU         AMD EPYC 7763 (64 cores)
🎮 GPU         No dedicated GPU detected
💾 RAM         128.0 GB (116.8 GB available)
💿 DISK        NVMe Samsung 970 EVO 2 TB
```

## Width Constraints

| Terminal Width | Behavior |
|----------------|----------|
| < 60 chars | Error: "Terminal too narrow (min 60 chars)" |
| 60-80 chars | Standard layout |
| > 80 chars | No expansion (fixed 60-char width) |

## Language

- **English only** for MVP
- Future: Vietnamese translation option
- Technical terms (CPU, GPU, RAM) remain uppercase

## Version Display

```
🤖 PCHECKER v0.1.0 - Hardware Info Tool
```

Format: `{emoji} {NAME} v{VERSION} - {TAGLINE}`
