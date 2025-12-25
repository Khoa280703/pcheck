# Multi-Disk & SMART Enhancement - Research Report

**Report ID:** planner-251225-2150-multi-disk-smart-enhancement
**Date:** 2025-12-25
**Plan:** plans/251225-2150-multi-disk-smart-enhancement
**Status:** Complete

---

## Executive Summary

Comprehensive implementation plan created for multi-disk testing support and extended SMART data collection in pchecker. The plan spans 5 phases over 8 hours, covering struct extensions, platform-specific parsing, multi-disk CLI support, and enhanced verbose output.

---

## Current Codebase Analysis

### Disk Module Structure
```
src/stress/disk/
├── mod.rs         - Main disk test logic (640 lines)
└── smart.rs       - SMART data collection (392 lines)
```

### Key Files Identified

| File | Purpose | Current State |
|------|---------|---------------|
| `src/stress/disk/smart.rs` | SMART data collection | Basic fields only (status, temp, hours, cycles, model, serial) |
| `src/stress/disk/mod.rs` | Disk test runner | Tests single disk only |
| `src/main.rs` | CLI entry point | First disk only, no --all-disks flag |
| `src/hw/disk.rs` | Disk detection | Returns `Vec<DiskInfo>` - multi-disk ready |
| `src/lang.rs` | Translations | Missing new SMART field strings |

### Existing SmartData Struct
```rust
pub struct SmartData {
    pub status: SmartStatus,
    pub temperature_c: Option<f64>,
    pub power_on_hours: Option<u64>,
    pub power_cycle_count: Option<u64>,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub firmware: Option<String>,
}
```

### Disk Detection (hw/disk.rs)
- Uses `sysinfo::Disks` - returns vector of all disks
- Already multi-disk capable
- Returns: `Vec<DiskInfo>` with name, total_gb, mount_point

---

## Design Decisions

### 1. SmartData Extension
**Decision:** Add 8 new fields as Option<T>

**Rationale:**
- Maintains backward compatibility
- Allows graceful degradation when data unavailable
- Follows existing pattern

### 2. Multi-Disk CLI Design
**Decision:** Two mutually exclusive flags
- `--all-disks`: Test all detected disks
- `--disk <index>`: Test specific disk (1-based index)
- Default: First disk (existing behavior)

**Rationale:**
- `--all-disks` clearer than `--disks all`
- 1-based indexing more user-friendly
- Conflicts prevent ambiguity

### 3. Sequential vs Parallel Testing
**Decision:** Sequential disk testing

**Rationale:**
- Avoids I/O contention
- Simpler progress tracking
- Predictable execution time
- Disk tests are I/O bound, not CPU bound

### 4. Platform-Specific SMART Parsing
**Decision:** Tiered approach
- Basic mode: Built-in tools (diskutil, WMIC)
- Verbose mode: smartctl with graceful fallback

**Rationale:**
- No sudo requirement for basic mode
- Extended data available when privileged
- Handles missing smartctl gracefully

---

## SMART Attributes Reference

### Common SMART Attribute IDs

| ID | Name | Description | Platform |
|----|------|-------------|----------|
| 1 | Raw_Read_Error_Rate | Hardware read errors | All |
| 5 | Reallocated_Sector_Ct | Bad sectors reallocated | All |
| 9 | Power_On_Hours | Lifetime hours | All |
| 10 | Spin_Retry_Count | Spin retry attempts | HDD |
| 12 | Power_Cycle_Count | Power on count | All |
| 190 | Airflow_Temperature_Cel | Air temperature | All |
| 194 | Temperature_Celsius | Drive temperature | All |
| 196 | Reallocated_Event_Count | Reallocation events | All |
| 197 | Current_Pending_Sector | Pending sectors | All |
| 198 | Offline_Uncorrectable | Uncorrectable sectors | All |
| 230 | Media_Wearout_Indicator | SSD wear (inverse) | SSD |
| 231 | SSD_Life_Left | SSD remaining life % | SSD |
| 233 | Media_Wearout_Indicator | SSD wear indicator | SSD |
| 241 | Total_LBAs_Written | Total bytes written | SSD |
| 242 | Total_LBAs_Read | Total bytes read | SSD |

### LBA to TB Conversion
```
TB = (LBA * 512) / (1024^4)
```

---

## Implementation Notes

### smartctl Output Format (Linux)
```
ID# ATTRIBUTE_NAME          FLAG     VALUE WORST THRESH TYPE      UPDATED  WHEN_FAILED RAW_VALUE
  5 Reallocated_Sector_Ct   0x0033   100   100   010    Pre-fail  Always       -       0
  9 Power_On_Hours          0x0032   099   099   000    Old_age   Always       -       1234
 12 Power_Cycle_Count       0x0032   100   100   000    Old_age   Always       -       56
```

### diskutil Output Format (macOS)
```
   Device Identifier:      disk0
   SMART Status:           Verified
   Solid State:            Yes
```

### Parsing Strategy
1. Split by whitespace
2. Match by ID + partial attribute name
3. Extract RAW_VALUE (last column)
4. Handle hex values (0x0033)

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| smartctl not installed | Extended data unavailable | Graceful fallback to basic mode |
| sudo timeout | Verbose mode fails | Handle timeout, show basic data |
| Platform variations | Parsing errors | Test on each platform, robust regex |
| Disk index out of range | Panic | Validate before access |
| Many disks | Long execution | Document, let user decide |

---

## Dependencies Analysis

### No New Dependencies Required
- `std::process::Command` sufficient for external commands
- `sysinfo` already provides disk enumeration
- `clap` already supports all needed argument types

### External Tools (Optional)
- `smartctl` - Part of smartmontools package
  - macOS: `brew install smartmontools`
  - Linux: `apt install smartmontools`
  - Windows: Download from sourceforge

---

## Testing Strategy

### Unit Tests
- SmartData default values
- Attribute parsing functions
- Index validation

### Integration Tests
- Default (first disk)
- --all-disks (multiple)
- --disk N (specific)
- --verbose (SMART output)

### Platform Testing
- macOS (Apple Silicon)
- Linux (Ubuntu/Debian)
- Windows (10/11)

---

## File Changes Summary

| File | Lines Added | Lines Modified | Complexity |
|------|-------------|----------------|------------|
| `src/stress/disk/smart.rs` | ~150 | ~30 | High |
| `src/main.rs` | ~80 | ~40 | Medium |
| `src/lang.rs` | ~60 | 0 | Low |
| `Cargo.toml` | 1 | 1 | Trivial |
| `README.md` | ~30 | 10 | Low |

---

## Open Questions

1. **Health percentage calculation**: Should we derive from SMART or use as-is?
   - Resolution: Use as-is from drive (if available), otherwise None

2. **Error thresholds**: Should warnings be raised for specific SMART values?
   - Resolution: Document in plan, defer to future enhancement

3. **Test file location**: Multi-disk tests on same mount point?
   - Resolution: Use temp dir for each, system manages

---

## Approval Checklist

- [x] Requirements analyzed
- [x] Codebase reviewed
- [x] Design decisions documented
- [x] Implementation plan created
- [x] Risks assessed
- [x] Success criteria defined

---

## Next Steps

1. **Stakeholder Review** - Get approval on plan
2. **Phase 1 Start** - Begin SmartData struct extension
3. **Progressive Implementation** - Complete phases sequentially
4. **Testing** - Validate after each phase
5. **Documentation** - Update README when complete

---

**Report Generated:** 2025-12-25
**Planner:** planning subagent
**Status:** Ready for implementation
