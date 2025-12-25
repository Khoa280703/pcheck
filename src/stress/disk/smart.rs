// SMART data module for disk health diagnostics
// Platform-specific: macOS diskutil, Linux smartctl, Windows WMI

use std::process::Command;

/// SMART health data collected from disk
#[derive(Debug, Clone)]
pub struct SmartData {
    /// Overall SMART status (Verified/Failing/Unknown)
    pub status: SmartStatus,
    /// Disk temperature in Celsius (if available)
    pub temperature_c: Option<f64>,
    /// Power on hours (if available)
    pub power_on_hours: Option<u64>,
    /// Number of power cycles (if available)
    pub power_cycle_count: Option<u64>,
    /// Device model (if available)
    pub model: Option<String>,
    /// Serial number (if available)
    pub serial: Option<String>,
    /// Firmware version (if available)
    pub firmware: Option<String>,

    // Extended SMART attributes (verbose mode)
    /// Overall health percentage (0-100)
    pub health_percentage: Option<u8>,
    /// Reallocated sector count (bad sectors)
    pub realloc_sectors: Option<u64>,
    /// Pending sector count
    pub pending_sectors: Option<u64>,
    /// Reallocated event count
    pub reallocated_events: Option<u64>,
    /// SSD life left percentage (SSD only)
    pub ssd_life_left: Option<u8>,
    /// Total LBAs written (in 512-byte blocks)
    pub total_lbas_written: Option<u64>,
    /// Total LBAs read (in 512-byte blocks)
    pub total_lbas_read: Option<u64>,
    /// Media errors count
    pub media_errors: Option<u64>,
    /// Command timeout count
    pub command_timeout: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SmartStatus {
    Verified,
    Failing,
    Unknown,
}

/// Get SMART data for the given disk
/// On verbose mode, attempts privileged commands
pub fn get_smart_data(mount_point: &str, verbose: bool) -> SmartData {
    #[cfg(target_os = "macos")]
    {
        get_macos_smart_data(mount_point, verbose)
    }

    #[cfg(target_os = "linux")]
    {
        get_linux_smart_data(mount_point, verbose)
    }

    #[cfg(target_os = "windows")]
    {
        get_windows_smart_data(verbose)
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        SmartData::default()
    }
}

impl Default for SmartData {
    fn default() -> Self {
        Self {
            status: SmartStatus::Unknown,
            temperature_c: None,
            power_on_hours: None,
            power_cycle_count: None,
            model: None,
            serial: None,
            firmware: None,
            health_percentage: None,
            realloc_sectors: None,
            pending_sectors: None,
            reallocated_events: None,
            ssd_life_left: None,
            total_lbas_written: None,
            total_lbas_read: None,
            media_errors: None,
            command_timeout: None,
        }
    }
}

// =============================================================================
// macOS implementation
// =============================================================================

#[cfg(target_os = "macos")]
fn get_macos_smart_data(mount_point: &str, verbose: bool) -> SmartData {
    let mut result = SmartData::default();

    // Try to get disk identifier from mount point
    let disk_identifier = get_disk_identifier_from_mount(mount_point);

    // Get disk info using diskutil (no sudo needed for basic info)
    if let Ok(output) = Command::new("diskutil")
        .args(&["info", "-plist", &disk_identifier])
        .output()
    {
        let plist = String::from_utf8_lossy(&output.stdout);
        parse_diskutil_info(&plist, &mut result);
    }

    // Verbose mode: try to get more detailed SMART info
    if verbose {
        // Try smartctl first (best source for detailed SMART data)
        // macOS smartctl provides similar output to Linux version
        let rdisk = disk_identifier.replace("disk", "rdisk");
        if let Ok(output) = Command::new("sh")
            .arg("-c")
            .arg(&format!("sudo smartctl -a /dev/{} 2>/dev/null || smartctl -a /dev/{} 2>/dev/null", rdisk, rdisk))
            .output()
        {
            let smartctl = String::from_utf8_lossy(&output.stdout);
            if !smartctl.trim().is_empty() && smartctl.contains("SMART") {
                parse_smartctl_output(&smartctl, &mut result);
            }
        }

        // Fallback: try diskutil info with more details
        if result.model.is_none() {
            if let Ok(output) = Command::new("diskutil")
                .args(&["info", &disk_identifier])
                .output()
            {
                let info = String::from_utf8_lossy(&output.stdout);
                parse_diskutil_verbose_info(&info, &mut result);
            }
        }

        // Try ioreg for temperature (may work without sudo)
        if result.temperature_c.is_none() {
            if let Ok(output) = Command::new("ioreg")
                .args(&["-rn", "AppleARMIODevice"])
                .output()
            {
                let ioreg = String::from_utf8_lossy(&output.stdout);
                result.temperature_c = parse_ioreg_temp(&ioreg);
            }
        }
    }

    result
}

#[cfg(target_os = "macos")]
fn get_disk_identifier_from_mount(mount_point: &str) -> String {
    // Get disk identifier from mount point using df
    if let Ok(output) = Command::new("df")
        .arg(mount_point)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().nth(1) {
            if let Some(disk) = line.split_whitespace().next() {
                return disk.to_string();
            }
        }
    }
    // Fallback to disk0
    "disk0".to_string()
}

#[cfg(target_os = "macos")]
fn parse_diskutil_info(plist: &str, result: &mut SmartData) {
    // Parse SMART status from plist
    let lines: Vec<&str> = plist.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let line = line.trim();

        // Check for SMART status
        if line.contains("<key>SMARTStatus</key>") {
            if i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if next_line.contains("Verified") || next_line.contains("true") {
                    result.status = SmartStatus::Verified;
                } else if next_line.contains("Failing") || next_line.contains("false") {
                    result.status = SmartStatus::Failing;
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn parse_diskutil_verbose_info(info: &str, result: &mut SmartData) {
    for line in info.lines() {
        let line = line.trim();

        // SMART Status
        if line.contains("SMART Status:") {
            if line.contains("Verified") {
                result.status = SmartStatus::Verified;
            } else if line.contains("Failing") {
                result.status = SmartStatus::Failing;
            }
        }

        // Model
        if line.contains("Model:") {
            if let Some(model) = line.split("Model:").nth(1) {
                result.model = Some(model.trim().to_string());
            }
        }

        // Firmware
        if line.contains("Firmware:") {
            if let Some(firmware) = line.split("Firmware:").nth(1) {
                result.firmware = Some(firmware.trim().to_string());
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn parse_ioreg_temp(ioreg: &str) -> Option<f64> {
    // Parse temperature from ioreg output
    // Looking for "temperature" key in AppleARMIODevice
    for line in ioreg.lines() {
        if line.contains("\"temperature\"") {
            // Format: "temperature" = <number>
            if let Some(eq_pos) = line.find('=') {
                let value_str = line[eq_pos + 1..].trim();
                // ioreg temperature is often in deci-celsius or similar
                let cleaned = value_str.trim_matches(|c: char| c == '"' || c == '<' || c == '>');
                if let Ok(value) = cleaned.parse::<f64>() {
                    // Convert from whatever scale ioreg uses to Celsius
                    if value > 1000.0 {
                        return Some(value / 100.0);
                    } else if value > 100.0 {
                        return Some(value / 10.0);
                    } else {
                        return Some(value);
                    }
                }
            }
        }
    }
    None
}

// =============================================================================
// Linux implementation
// =============================================================================

#[cfg(target_os = "linux")]
fn get_linux_smart_data(mount_point: &str, verbose: bool) -> SmartData {
    let mut result = SmartData::default();

    // Find device from mount point
    let device = find_device_for_mount(mount_point);

    // Verbose mode: try smartctl (requires sudo)
    if verbose {
        if let Ok(output) = Command::new("sh")
            .arg("-c")
            .arg(&format!("sudo smartctl -a {} 2>/dev/null || smartctl -a {} 2>/dev/null", device, device))
            .output()
        {
            let smartctl = String::from_utf8_lossy(&output.stdout);
            parse_smartctl_output(&smartctl, &mut result);
        }
    }

    // Try to get basic info from /sys/block (no sudo needed)
    if result.status == SmartStatus::Unknown {
        result.status = SmartStatus::Verified; // Assume healthy if no errors
    }

    result
}

#[cfg(target_os = "linux")]
fn find_device_for_mount(mount_point: &str) -> String {
    if let Ok(mounts) = std::fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == mount_point {
                return parts[0].to_string();
            }
        }
    }
    "/dev/sda".to_string()
}

// =============================================================================
// Shared smartctl parser (used by both Linux and macOS)
// =============================================================================

// Make this function available on both platforms
fn parse_smartctl_output(output: &str, result: &mut SmartData) {
    for line in output.lines() {
        let line = line.trim();

        // SMART overall health
        if line.contains("SMART overall-health self-assessment test result") {
            if line.contains("PASSED") {
                result.status = SmartStatus::Verified;
            } else if line.contains("FAILED") {
                result.status = SmartStatus::Failing;
            }
        }

        // Helper to parse attribute value (column 10, 0-indexed) - returns owned String
        let parse_attr_value = |line: &str| -> Option<String> {
            line.split_whitespace().nth(9).map(|s| s.to_string())
        };

        // ID 5: Reallocated_Sector_Ct
        if line.contains(" 5 ") && line.contains("Reallocated_Sector_Ct") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    result.realloc_sectors = Some(val);
                }
            }
        }

        // ID 9: Power_On_Hours
        if line.contains(" 9 ") && line.contains("Power_On_Hours") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    result.power_on_hours = Some(val);
                }
            }
        }

        // ID 10: Spin_Retry_Count
        if line.contains(" 10 ") && line.contains("Spin_Retry_Count") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    if val > 0 {
                        result.command_timeout = Some(val);
                    }
                }
            }
        }

        // ID 12: Power_Cycle_Count
        if line.contains(" 12 ") && line.contains("Power_Cycle_Count") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    result.power_cycle_count = Some(val);
                }
            }
        }

        // ID 194: Temperature_Celsius
        if line.contains("194") && line.contains("Temperature") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<f64>() {
                    result.temperature_c = Some(val);
                }
            }
        }

        // ID 196: Reallocated_Event_Count
        if line.contains("196") && line.contains("Reallocated_Event_Count") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    result.reallocated_events = Some(val);
                }
            }
        }

        // ID 197: Current_Pending_Sector
        if line.contains("197") && line.contains("Current_Pending_Sector") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    result.pending_sectors = Some(val);
                }
            }
        }

        // ID 198: Offline_Uncorrectable
        if line.contains("198") && line.contains("Offline_Uncorrectable") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    result.media_errors = Some(val);
                }
            }
        }

        // ID 230: Media_Wearout_Indicator (SSD)
        if line.contains("230") && (line.contains("Media_Wearout_Indicator") || line.contains("SSD") || line.contains("Wear_Level")) {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    // Wear level usually goes from 100 to 0, or 0 to 100
                    // Normalize to percentage remaining
                    if val <= 100 {
                        result.ssd_life_left = Some(100 - val as u8);
                    }
                }
            }
        }

        // ID 233: Media_Wearout_Indicator (alternative)
        if line.contains("233") && (line.contains("Media_Wearout_Indicator") || line.contains("SSD_Life_Left")) {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u8>() {
                    if val <= 100 {
                        result.ssd_life_left = Some(val);
                    }
                }
            }
        }

        // ID 241: Total_LBAs_Written
        if line.contains("241") && line.contains("Total_LBAs_Written") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    result.total_lbas_written = Some(val);
                }
            }
        }

        // ID 242: Total_LBAs_Read
        if line.contains("242") && line.contains("Total_LBAs_Read") {
            if let Some(val_str) = parse_attr_value(line) {
                if let Ok(val) = val_str.parse::<u64>() {
                    result.total_lbas_read = Some(val);
                }
            }
        }

        // Model
        if line.contains("Device Model:") {
            if let Some(model) = line.split("Device Model:").nth(1) {
                result.model = Some(model.trim().to_string());
            }
        }

        // Serial
        if line.contains("Serial Number:") {
            if let Some(serial) = line.split("Serial Number:").nth(1) {
                result.serial = Some(serial.trim().to_string());
            }
        }

        // Firmware
        if line.contains("Firmware Version:") {
            if let Some(firmware) = line.split("Firmware Version:").nth(1) {
                result.firmware = Some(firmware.trim().to_string());
            }
        }
    }
}

// =============================================================================
// Windows implementation
// =============================================================================

#[cfg(target_os = "windows")]
fn get_windows_smart_data(verbose: bool) -> SmartData {
    let mut result = SmartData::default();

    // Verbose mode: use wmic/PowerShell to get SMART data
    if verbose {
        // Get disk status
        if let Ok(output) = Command::new("wmic")
            .args(&["diskdrive", "get", "status", "/format:list"])
            .output()
        {
            let wmic = String::from_utf8_lossy(&output.stdout);
            for line in wmic.lines() {
                if line.contains("OK") || line.contains("Degraded") {
                    result.status = SmartStatus::Verified;
                } else if line.contains("Pred Fail") {
                    result.status = SmartStatus::Failing;
                }
            }
        }

        // Get model
        if let Ok(output) = Command::new("wmic")
            .args(&["diskdrive", "get", "model", "/format:list"])
            .output()
        {
            let wmic = String::from_utf8_lossy(&output.stdout);
            for line in wmic.lines() {
                if let Some(model) = line.strip_prefix("Model=") {
                    result.model = Some(model.trim().to_string());
                }
            }
        }

        // Get serial
        if let Ok(output) = Command::new("wmic")
            .args(&["diskdrive", "get", "serialnumber", "/format:list"])
            .output()
        {
            let wmic = String::from_utf8_lossy(&output.stdout);
            for line in wmic.lines() {
                if let Some(serial) = line.strip_prefix("SerialNumber=") {
                    result.serial = Some(serial.trim().to_string());
                }
            }
        }

        // Get firmware
        if let Ok(output) = Command::new("wmic")
            .args(&["diskdrive", "get", "firmwarerevision", "/format:list"])
            .output()
        {
            let wmic = String::from_utf8_lossy(&output.stdout);
            for line in wmic.lines() {
                if let Some(firmware) = line.strip_prefix("FirmwareRevision=") {
                    result.firmware = Some(firmware.trim().to_string());
                }
            }
        }

        // Try PowerShell for detailed SMART attributes
        // Get-PhysicalDisk cmdlet provides health info on Windows 8+
        if let Ok(output) = Command::new("powershell")
            .args(&["-Command", "Get-PhysicalDisk | Select-Object HealthStatus, MediaType, Size | Format-List"])
            .output()
        {
            let ps = String::from_utf8_lossy(&output.stdout);
            for line in ps.lines() {
                if line.contains("HealthStatus") {
                    if line.contains("Healthy") {
                        result.status = SmartStatus::Verified;
                        // Estimate health percentage based on status
                        result.health_percentage = Some(100);
                    } else if line.contains("Warning") {
                        result.health_percentage = Some(70);
                        if result.status == SmartStatus::Unknown {
                            result.status = SmartStatus::Verified;
                        }
                    } else if line.contains("Unhealthy") {
                        result.health_percentage = Some(20);
                        result.status = SmartStatus::Failing;
                    }
                }
            }
        }

        // Try to get SMART data via PowerShell and storage module
        if let Ok(output) = Command::new("powershell")
            .args(&["-Command",
                "Get-StorageReliabilityCounter | Select-Object Temperature, Wear, TotalLbasRead, TotalLbasWritten | Format-List"])
            .output()
        {
            let ps = String::from_utf8_lossy(&output.stdout);
            for line in ps.lines() {
                if line.contains("Temperature") {
                    if let Some(temp_str) = line.split(':').nth(1) {
                        if let Ok(temp) = temp_str.trim().parse::<f64>() {
                            result.temperature_c = Some(temp);
                        }
                    }
                }
                if line.contains("Wear") {
                    if let Some(wear_str) = line.split(':').nth(1) {
                        if let Ok(wear) = wear_str.trim().parse::<u8>() {
                            result.ssd_life_left = Some(wear);
                        }
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_data_default() {
        let data = SmartData::default();
        assert_eq!(data.status, SmartStatus::Unknown);
        assert!(data.temperature_c.is_none());
    }

    #[test]
    fn test_smart_status_equality() {
        assert_eq!(SmartStatus::Verified, SmartStatus::Verified);
        assert_ne!(SmartStatus::Verified, SmartStatus::Failing);
    }
}
