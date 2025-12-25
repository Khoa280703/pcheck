# Research Report: Platform-Specific Hardware Detection

**Research Date:** 2025-12-25
**Target:** Cross-platform CLI tool (pchecker)
**Platforms:** Windows, macOS (Intel + Apple Silicon), Linux

---

## Executive Summary

Hardware detection requires platform-specific approaches:
- **macOS**: Use `sysctl` + `system_profiler` for reliable info. Apple Silicon GPUs need Metal/IORegistry.
- **Windows**: WMI via PowerShell or Win32 APIs. Comprehensive but verbose.
- **Linux**: Parse `/proc` filesystem + `lspci`. Simple, file-based.

**Key Recommendation**: Use Rust `sysinfo` crate as foundation, supplement with platform-specific commands where needed.

---

## 1. macOS (Darwin)

### 1.1 CPU Detection

**Commands:**
```bash
# CPU model (Intel Macs only)
sysctl machdep.cpu.brand_string
# Output: machdep.cpu.brand_string: Intel(R) Core(TM) i7-8559U CPU @ 2.70GHz

# CPU core count
sysctl hw.physicalcpu
sysctl hw.logicalcpu

# CPU frequency
sysctl hw.cpufrequency
```

**Apple Silicon Note:** `machdep.cpu.brand_string` unavailable. Use:
```bash
system_profiler SPHardwareDataType | grep "Chip"
# Output: Chip: Apple M1 Pro
```

**Edge Cases:**
- Intel vs Apple Silicon: Different sysctl keys
- Rosetta 2: May affect reporting in some contexts
- Performance cores vs Efficiency cores on M1/M2/M3

### 1.2 GPU Detection

**Basic Detection:**
```bash
system_profiler SPDisplaysDataType
# Output: GPU info including Apple Silicon integrated GPU
```

**Apple Silicon GPU Details:**
- Unified memory: CPU/GPU share 60-70% of total RAM
- Metal API for runtime detection
- IORegistry for low-level access (requires IOKit framework)

**Key Commands:**
```bash
# GPU core count (limited info)
system_profiler SPDisplaysDataType | grep "Total Number of Cores"

# GPU memory (unified - not separate VRAM)
# Metal typically has access to 60-70% of total RAM
```

**Edge Cases:**
- External GPUs on Intel Macs (rare on Apple Silicon)
- Multiple GPUs (integrated + discrete) on older Intel Macs
- Apple Silicon GPU core counts not fully exposed via sysctl

### 1.3 RAM Detection

**Commands:**
```bash
# Total physical memory (bytes)
sysctl hw.memsize
# Output: hw.memsize: 17179869184 (16 GB)

# Alternative
sysctl hw.physmem

# Memory type/slots
system_profiler SPMemoryDataType
```

**Apple Silicon Note:** Unified memory architecture - no separate VRAM

**Edge Cases:**
- Swap memory not included in hw.memsize
- Unified memory on Apple Silicon (CPU + GPU share pool)

### 1.4 Disk Detection

**Commands:**
```bash
# Disk list
diskutil list

# Disk info
system_profiler SPStorageDataType

# Specific disk size
diskutil info / | grep "Disk Size"
```

---

## 2. Windows

### 2.1 CPU Detection

**PowerShell/WMI:**
```powershell
# CPU info
Get-WmiObject Win32_Processor | Select-Object Name, NumberOfCores, NumberOfLogicalProcessors, MaxClockSpeed

# Alternative (CIM cmdlet - newer)
Get-CimInstance Win32_Processor | Select-Object Name, NumberOfCores
```

**Win32 API (Rust):**
- Use `windows` crate or `winapi` crate
- Call `GetSystemInfo()` or `GetLogicalProcessorInformation()`

**Edge Cases:**
- Hyper-threading: Logical cores > physical cores
- Hybrid CPUs (Intel P-cores + E-cores, AMD 3D V-Cache)
- CPU affinity masks

### 2.2 GPU Detection

**PowerShell/WMI:**
```powershell
# GPU info
Get-WmiObject Win32_VideoController | Select-Object Name, AdapterRAM, DriverVersion

# GPU usage (modern Windows)
Get-Counter "\GPU Engine(*)\Utilization Percentage"
```

**Edge Cases:**
- Multiple GPUs (integrated + discrete)
- GPU vendor-specific tools (nvidia-smi, AMD tools)
- Virtual GPU scenarios

### 2.3 RAM Detection

**PowerShell/WMI:**
```powershell
# Total physical memory
Get-WmiObject Win32_ComputerSystem | Select-Object TotalPhysicalMemory

# Memory details
Get-WmiObject Win32_PhysicalMemory | Select-Object Capacity, Speed
```

**Win32 API:**
- `GlobalMemoryStatusEx()` function

**Edge Cases:**
- Reserved memory for hardware
- Memory-mapped I/O regions

### 2.4 Disk Detection

**PowerShell/WMI:**
```powershell
# Logical disks
Get-WmiObject Win32_LogicalDisk | Select-Object DeviceID, Size, FreeSpace, DriveType

# Physical disks
Get-WmiObject Win32_DiskDrive | Select-Object Model, Size
```

**Edge Cases:**
- Network drives
- Removable media
- Drive letters vs mount points

---

## 3. Linux

### 3.1 CPU Detection

**Commands:**
```bash
# Detailed CPU info
cat /proc/cpuinfo

# CPU model
cat /proc/cpuinfo | grep "model name" | head -n 1

# Core count
cat /proc/cpuinfo | grep "processor" | wc -l

# Physical CPUs
cat /proc/cpuinfo | grep "physical id" | sort -u | wc -l

# Structured output
lscpu
```

**Key Files:**
- `/proc/cpuinfo`: Full CPU details
- `/sys/devices/system/cpu/present`: CPU range

**Edge Cases:**
- Hyper-threading: siblings > cores per socket
- CPU hotplug (offline CPUs)
- ARM vs x86 differences in /proc/cpuinfo
- Frequency scaling

### 3.2 GPU Detection

**Commands:**
```bash
# PCI devices with GPU info
lspci | grep -i vga
lspci | grep -i nvidia

# Detailed GPU info
lspci -v -s $(lspci | grep -i vga | cut -d' ' -f1)

# DRM devices (modern Linux)
ls /sys/class/drm/

# Vendor-specific
nvidia-smi
rocm-smi  # AMD
```

**Key Files:**
- `/sys/class/drm/card*/device/`: GPU device info
- `/sys/bus/pci/devices/*/vendor`: PCI vendor IDs

**Edge Cases:**
- Multiple GPUs
- Headless systems
- Containerized environments (limited GPU access)
- Open-source vs proprietary drivers

### 3.3 RAM Detection

**Commands:**
```bash
# Memory info
cat /proc/meminfo

# Total memory (KB)
grep MemTotal /proc/meminfo

# Human-readable
free -h
```

**Key Fields:**
- `MemTotal`: Total RAM
- `MemAvailable`: Available for new apps
- `MemFree`: Free (unused) memory
- `Buffers`, `Cached`: Reclaimable memory

**Edge Cases:**
- Shared memory (SHM)
- Huge pages
- ZRAM (compressed swap)
- Container memory limits (cgroups)

### 3.4 Disk Detection

**Commands:**
```bash
# Partition info
cat /proc/partitions

# Mount points and usage
df -h

# Block device details
lsblk

# Disk size (bytes)
cat /sys/block/sda/size
```

**Key Files:**
- `/proc/partitions`: Block devices
- `/proc/mounts`: Mounted filesystems
- `/sys/block/*/size`: Device sizes

**Edge Cases:**
- Network filesystems (NFS, SMB)
- Bind mounts
- LVM / RAID devices
- Loop devices

---

## 4. Rust Implementation Strategy

### 4.1 Recommended Crates

**Primary:**
```toml
[dependencies]
sysinfo = "0.30"  # Cross-platform system info
```

**Platform-Specific (if needed):**
```toml
# macOS
[target.'cfg(target_os = "macos")'.dependencies]
mach = "0.4"  # Mach kernel APIs

# Windows
[target.'cfg(windows)'.dependencies]
windows = "0.52"  # Win32 APIs

# Linux
[target.'cfg(target_os = "linux")'.dependencies]
procfs = "0.16"  # /proc filesystem parsing
```

### 4.2 Code Structure (Rust Pseudo-code)

```rust
use sysinfo::{System, SystemExt};

fn get_hardware_info() -> HardwareInfo {
    let mut sys = System::new_all();

    // CPU
    sys.refresh_cpu();
    let cpu_name = sys.cpus().first()?.name();
    let cpu_count = sys.cpus().len();

    // Memory
    sys.refresh_memory();
    let total_mem = sys.total_memory();
    let available_mem = sys.available_memory();

    // Disks
    sys.refresh_disks_list();
    let disks = sys.disks();

    // Platform-specific GPU
    #[cfg(target_os = "macos")]
    let gpu = get_macos_gpu();

    #[cfg(windows)]
    let gpu = get_windows_gpu();

    #[cfg(target_os = "linux")]
    let gpu = get_linux_gpu();
}
```

### 4.3 Alternative: Shell Commands

```rust
use std::process::Command;

fn exec_command(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

// macOS
let cpu = exec_command("sysctl", &["-n", "machdep.cpu.brand_string"]);
let mem = exec_command("sysctl", &["-n", "hw.memsize"]);

// Linux
let cpu = exec_command("sh", &["-c", "cat /proc/cpuinfo | grep 'model name' | head -n 1"]);
let mem = exec_command("cat", &["/proc/meminfo"]);
```

---

## 5. Edge Cases Summary

### 5.1 Cross-Platform
- Virtual machines: CPU/RAM may be host-reported
- Containers: Limited hardware visibility
- 32-bit vs 64-bit architectures
- Locale-specific output (parse carefully)

### 5.2 macOS
- **Apple Silicon**: Many Intel-specific sysctl keys missing
- **Rosetta 2**: May report x86_64 for some processes
- **External GPUs**: Supported on Intel Macs, limited on Apple Silicon
- **Secure Boot**: May restrict low-level hardware access

### 5.3 Windows
- **WMI Availability**: Not available on Windows PE (WinPE)
- **Permissions**: Some WMI classes require admin
- **WMI vs CIM**: Prefer newer CIM cmdlets (PowerShell 3+)
- **32-bit process on 64-bit Windows**: WMI registry redirection issues

### 5.4 Linux
- **ARM systems**: Different /proc/cpuinfo format
- **Containers**: /proc shows host info, not container limits (use cgroups)
- **SNAP packages**: Confined filesystem access
- **Android (Termux)**: Limited /proc access

---

## 6. Performance & Best Practices

### 6.1 Performance
- **sysinfo**: Refreshes can be expensive, cache results
- **/proc parsing**: Fast, file-based
- **WMI queries**: Can be slow, minimize queries
- **sysctl**: Very fast on macOS

### 6.2 Error Handling
- Always handle missing commands/files
- Validate output before parsing
- Provide fallback methods
- Log errors for debugging

### 6.3 Security
- Avoid command injection in shell commands
- Validate all user inputs
- Limit privilege escalation requests
- Respect sandboxing (especially on macOS)

---

## 7. Quick Reference

| Platform | CPU | GPU | RAM | Disk |
|----------|-----|-----|-----|------|
| **macOS** | `sysctl hw.*` + `system_profiler` | `system_profiler SPDisplaysDataType` | `sysctl hw.memsize` | `diskutil list` |
| **Windows** | `Win32_Processor` | `Win32_VideoController` | `Win32_ComputerSystem` | `Win32_LogicalDisk` |
| **Linux** | `/proc/cpuinfo` | `lspci` + `/sys/class/drm` | `/proc/meminfo` | `/proc/partitions` |

---

## Resources & References

### Official Documentation
- [Metal GPU Detection - Apple Developer](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions)
- [WMI Tasks: Computer Hardware - Microsoft](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--computer-hardware)
- [Linux /proc filesystem - Kernel.org](https://www.kernel.org/doc/html/latest/filesystems/proc.html)

### Crates & Libraries
- [sysinfo - crates.io](https://crates.io/crates/sysinfo)
- [sysinfo docs.rs](https://docs.rs/sysinfo/latest/sysinfo/)
- [hardware-query - crates.io](https://crates.io/crates/hardware-query)

### Community Resources
- [Detect Apple Silicon GPU core count - Stack Overflow](https://stackoverflow.com/questions/72363212/detect-apple-silicon-gpu-core-count)
- [macOS sysctl examples - Superuser](https://serverfault.com/questions/112711/how-can-i-get-cpu-count-and-total-ram-from-the-os-x-command-line)
- [PowerShell Get-WmiObject examples](https://adamtheautomator.com/get-wmiobject/)

### Further Reading
- [Apple Silicon unified memory analysis - Eclectic Light](https://eclecticlight.co/2022/03/01/making-sense-of-m1-memory-use/)
- [asitop - Apple Silicon monitoring tool](https://github.com/tlstommy/asitop)

---

## Unresolved Questions

1. **Apple Silicon GPU VRAM**: How to reliably report GPU memory on unified memory systems? (60-70% heuristic?)
2. **Rust IOKit bindings**: Current state of IOKit framework bindings for macOS IORegistry access?
3. **Performance**: Benchmark comparing sysinfo crate vs direct shell command execution?
4. **Windows ARM**: Detection methods on Windows on ARM (WoA)?
5. **Container environments**: Best practices for detecting hardware limits in Docker/LXC/Podman?

---

**Report Status:** Complete
**Next Steps:** Create implementation plan with prototype code
