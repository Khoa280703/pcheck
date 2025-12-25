# pchecker

**Cross-platform hardware detection CLI tool**

One command to check your hardware: CPU, GPU, RAM, Disk.

## Features

- Cross-platform: Windows, macOS, Linux
- Single binary - no installation needed
- Auto-detect all hardware
- Simple, human-readable output

## Installation

```bash
# Download binary (coming soon)
curl -sL https://github.com/Khoa280703/pcheck/releases/download/v0.1.0/pck -o pck
chmod +x pck
./pck
```

## Build from source

```bash
cargo build --release
```

## Usage

```bash
pck
```

## Roadmap

- [x] Research phase
- [ ] MVP: Hardware info detection
- [ ] Phase 2: Stress testing
- [ ] Phase 3: GPU benchmarking
- [ ] Phase 4: Report export (JSON/HTML)

## Tech Stack

- Rust
- sysinfo - CPU/RAM/Disk detection
- clap - CLI parsing
- Platform-specific commands for GPU

## License

MIT
