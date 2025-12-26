# pchecker - Development Commands Guide

## Build Commands

```bash
# Debug build (fastest, for development)
cargo build

# Release build (optimized, for production)
cargo build --release

# Clean + rebuild (fixes cache issues)
cargo clean
cargo build
```

## Test Commands

```bash
# Run all tests
cargo test

# Run specific test module
cargo test stress::cpu
cargo test stress::ram
cargo test stress::disk

# Run test with output
cargo test -- --nocapture

# Run single test
cargo test test_cpu_test_short
```

## Run Commands

```bash
# Run with debug build (slower)
cargo run

# Run with release binary (faster)
./target/release/pchecker

# Run with specific args
cargo run -- --help
cargo run -- --quick --cpu-stress
./target/release/pchecker --stress --verbose
```

## Common Debug Scenarios

### After code changes, output doesn't update
```bash
# Force rebuild
cargo clean && cargo build --release
# Then run
./target/release/pchecker
```

### Test specific health check
```bash
# CPU only
cargo run -- --cpu-stress --quick

# RAM only
cargo run -- --ram-stress

# Disk only
cargo run -- --disk-stress --quick

# All tests (quick mode)
cargo run -- --stress --quick
```

### Verbose mode (detailed per-core output)
```bash
cargo run -- --cpu-stress --verbose
```

### Info mode (no stress test)
```bash
cargo run
# Just press Enter, shows hardware info only
```

## Useful Aliases (add to ~/.zshrc or ~/.bashrc)

```bash
# For this project
alias pcbuild='cd ~/development/pcheck && cargo build --release'
alias pctest='cd ~/development/pcheck && cargo test'
alias pcrun='cd ~/development/pcheck && ./target/release/pchecker'
alias pcclean='cd ~/development/pcheck && cargo clean'
alias pc-rebuild='cd ~/development/pcheck && cargo clean && cargo build --release'
```

## Language Selection (auto-prompt)

When running, select:
- `1` or `vi` → Tiếng Việt
- `2` or `en` → English

## Quick Test Matrix

| Command | Duration | What it tests |
|---------|----------|---------------|
| `--quick --cpu-stress` | ~15s | CPU only |
| `--quick --ram-stress` | ~5s | RAM only |
| `--quick --disk-stress` | ~5s | Disk only |
| `--quick --stress` | ~20s | All 3 |
| `--cpu-stress` (default 60s) | ~60s | CPU only |
| `--ram-stress` | ~30s | RAM only |
| `--disk-stress` | ~30s | Disk only |
| `--stress` | ~2min | All 3 |
| `--stress --verbose` | ~2min | All + detailed |

## Troubleshooting

| Problem | Solution |
|---------|----------|
| Output shows old behavior | `cargo clean && cargo build --release` |
| Binary not found | Run `cargo build --release` first |
| Test fails | Check specific test with `cargo test <name>` |
| SSD detection wrong | Check platform-specific code in `src/stress/disk.rs` |
| Temperature N/A | Check `src/sensors/temp.rs` for platform support |
| Wrong project path | Project root is `pcheck/`, not `pcheck/pchecker/` |
