// macOS Deep Hardware Probe
// Uses system_profiler and sysctl to get detailed hardware info

use std::process::Command;
use crate::hw::deep::{
    common::{CacheInfo, InstructionSets, DimmSlot, DiskHealth, GpuDriver, PcieLink},
    {DeepCpuInfo, DeepRamInfo, DeepDiskInfo, DeepGpuInfo},
    estimate_tdp_from_model,
};

/// macOS deep hardware probe
pub struct MacOsDeepProbe;

// ========== CPU Implementation ==========

impl DeepCpuInfo for MacOsDeepProbe {
    fn get_cache_info(&self) -> Option<CacheInfo> {
        // Use sysctl to get cache sizes (returns bytes, convert to KB)
        let l1 = get_sysctl("hw.l1icachesize").or_else(|| get_sysctl("hw.l1dcachesize"));
        let l2 = get_sysctl("hw.l2cachesize");
        let l3 = get_sysctl("hw.l3cachesize");

        // Convert bytes strings to KB
        let bytes_to_kb = |s: &String| s.trim().parse::<u32>().ok().map(|b| b / 1024);

        Some(CacheInfo {
            l1_kb: l1.as_ref().and_then(bytes_to_kb),
            l2_kb: l2.as_ref().and_then(bytes_to_kb),
            l3_kb: l3.as_ref().and_then(bytes_to_kb),
        })
    }

    fn get_instruction_sets(&self) -> Option<InstructionSets> {
        // Determine CPU architecture and return appropriate features
        let arch = get_sysctl("hw.cpu64capability")?;

        let features = if arch.contains("hw.cpu64capability: ") {
            // Apple Silicon - common ARM features
            vec![
                "ARM64".to_string(),
                "NEON".to_string(),
                "FP16".to_string(),
                "AES".to_string(),
                "SHA".to_string(),
                "PMU".to_string(),
            ]
        } else {
            // Intel macOS features
            vec![
                "x86-64".to_string(),
                "SSE4.2".to_string(),
                "AVX".to_string(),
                "AVX2".to_string(),
                "VT-x".to_string(),
            ]
        };

        Some(InstructionSets { features })
    }

    fn get_tdp(&self, model: &str) -> Option<u32> {
        estimate_tdp_from_model(model)
    }
}

// ========== RAM Implementation ==========

impl DeepRamInfo for MacOsDeepProbe {
    fn get_dimm_slots(&self) -> Vec<DimmSlot> {
        let output = Command::new("system_profiler")
            .args(["SPMemoryDataType", "-json"])
            .output();

        match output {
            Ok(result) => parse_macos_ram_json(&String::from_utf8_lossy(&result.stdout)),
            Err(_) => vec![],
        }
    }
}

/// Parse macOS system_profiler RAM JSON output (pure function for testing)
fn parse_macos_ram_json(json_str: &str) -> Vec<DimmSlot> {
    use serde_json::Value;

    let json: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    // Navigate to SPMemoryDataType
    // The structure varies by macOS version:
    // - Older: SPMemoryDataType[0].spm_memory_item_array[]
    // - Newer (Apple Silicon): SPMemoryDataType[0] with direct fields

    let memory_data = json
        .get("SPMemoryDataType")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .unwrap_or(&serde_json::Value::Null);

    // Try the older structure with spm_memory_item_array first
    if let Some(items) = memory_data
        .get("spm_memory_item_array")
        .and_then(|v| v.as_array())
    {
        return items
            .iter()
            .filter_map(|item| {
                let size_str = item.get("dimm_size")?.as_str()?;
                let size_gb = parse_size_gb(size_str)?;

                Some(DimmSlot {
                    id: 0,
                    bank: item.get("dimm_manufacturer")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string(),
                    size_gb,
                    type_: item.get("dimm_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string(),
                    speed_mhz: item.get("dimm_speed")
                        .and_then(|v| v.as_str())
                        .and_then(parse_speed_mhz),
                    manufacturer: item.get("dimm_manufacturer")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    part_number: item.get("dimm_part_number")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                })
            })
            .collect();
    }

    // Try newer Apple Silicon structure (direct fields)
    let size_str = memory_data.get("SPMemoryDataType")
        .or_else(|| memory_data.get("dimm_size"))
        .and_then(|v| v.as_str());
    let manufacturer = memory_data.get("dimm_manufacturer")
        .or_else(|| memory_data.get("dimm_type"))
        .and_then(|v| v.as_str());
    let ram_type = memory_data.get("dimm_type")
        .and_then(|v| v.as_str());

    if let (Some(size), Some(mfr), Some(type_)) = (size_str, manufacturer, ram_type) {
        let size_gb = parse_size_gb(size).unwrap_or(0.0);
        if size_gb > 0.0 {
            return vec![DimmSlot {
                id: 0,
                bank: mfr.to_string(),
                size_gb,
                type_: type_.to_string(),
                speed_mhz: None,
                manufacturer: Some(mfr.to_string()),
                part_number: None,
            }];
        }
    }

    vec![]
}

/// Parse size string like "16 GB" to f64 GB
fn parse_size_gb(s: &str) -> Option<f64> {
    let s = s.trim();
    let num: f64 = s.split_whitespace().next()?.parse().ok()?;
    let unit = s[s.len().saturating_sub(2)..].to_lowercase();

    match unit.as_str() {
        "gb" => Some(num),
        "tb" => Some(num * 1024.0),
        "mb" => Some(num / 1024.0),
        _ => None,
    }
}

/// Parse speed string like "3200 MHz" to u32 MHz
fn parse_speed_mhz(s: &str) -> Option<u32> {
    s.split_whitespace()
        .next()?
        .parse()
        .ok()
}

// ========== Disk Implementation ==========

impl DeepDiskInfo for MacOsDeepProbe {
    fn get_firmware(&self) -> Option<String> {
        // Use diskutil to get firmware version
        let output = Command::new("diskutil")
            .args(["info", "-plist", "/"])
            .output()
            .ok()?;

        let _plist = String::from_utf8_lossy(&output.stdout);
        // Parse plist for "diskuuid" or similar firmware info
        // For now, return None as this requires more complex parsing
        None
    }

    fn get_tbw(&self) -> Option<f64> {
        // TBW data from smartctl (requires sudo or specific access)
        None
    }

    fn get_power_hours(&self) -> Option<u64> {
        // Power-on hours from smartctl
        None
    }

    fn get_disk_health(&self) -> Option<DiskHealth> {
        Some(DiskHealth {
            status: "Unknown".to_string(),
            firmware: self.get_firmware(),
            tbw: self.get_tbw(),
            hours: self.get_power_hours(),
            percentage_used: None,
        })
    }
}

// ========== GPU Implementation ==========

impl DeepGpuInfo for MacOsDeepProbe {
    fn get_driver_version(&self) -> Option<String> {
        // Driver version not applicable for Apple Silicon
        None
    }

    fn get_metal_version(&self) -> Option<String> {
        // Get Metal version from system_profiler
        let output = Command::new("system_profiler")
            .args(["SPDisplaysDataType", "-json"])
            .output()
            .ok()?;

        let json = String::from_utf8_lossy(&output.stdout);

        // Parse for "Metal" family support string
        // e.g., "Metal Family: Supported, Metal GPUFamily: Apple M4 Pro"
        json.lines()
            .find(|line| line.contains("Metal"))
            .and_then(|line| {
                line.split(':')
                    .nth(1)
                    .map(|s| s.trim().to_string())
            })
    }

    fn get_pcie_link(&self) -> Option<PcieLink> {
        // Apple Silicon has unified memory, no traditional PCIe
        None
    }

    fn get_gpu_driver(&self) -> Option<GpuDriver> {
        Some(GpuDriver {
            version: None,
            metal: self.get_metal_version(),
        })
    }
}

// ========== Helper Functions ==========

/// Get sysctl value by key
fn get_sysctl(key: &str) -> Option<String> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg(key)
        .output()
        .ok()?;

    let value = String::from_utf8_lossy(&output.stdout);
    let trimmed = value.trim();

    if trimmed.is_empty() || trimmed.contains("error") {
        None
    } else {
        Some(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_gb() {
        assert_eq!(parse_size_gb("16 GB"), Some(16.0));
        assert_eq!(parse_size_gb("1 TB"), Some(1024.0));
        assert_eq!(parse_size_gb("32000 MB"), Some(31.25));
        assert_eq!(parse_size_gb("invalid"), None);
    }

    #[test]
    fn test_parse_speed_mhz() {
        assert_eq!(parse_speed_mhz("3200 MHz"), Some(3200));
        assert_eq!(parse_speed_mhz("8533 MHz"), Some(8533));
        assert_eq!(parse_speed_mhz("invalid"), None);
    }

    #[test]
    fn test_parse_macos_ram_json_valid() {
        let json = r#"{
            "SPMemoryDataType": [{
                "spm_memory_item_array": [{
                    "dimm_size": "24 GB",
                    "dimm_speed": "8533 MHz",
                    "dimm_type": "LPDDR5",
                    "dimm_manufacturer": "Apple",
                    "dimm_part_number": "Unified"
                }]
            }]
        }"#;

        let slots = parse_macos_ram_json(json);
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].size_gb, 24.0);
        assert_eq!(slots[0].type_, "LPDDR5");
        assert_eq!(slots[0].manufacturer, Some("Apple".to_string()));
        assert_eq!(slots[0].speed_mhz, Some(8533));
    }

    #[test]
    fn test_parse_macos_ram_json_empty() {
        let slots = parse_macos_ram_json("{}");
        assert_eq!(slots.len(), 0);
    }
}
