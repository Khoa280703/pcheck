// Disk health check module
// Tests disk by sequential/random I/O operations

pub mod smart;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Instant;
use std::io::{self, BufWriter};

use super::HealthStatus;
use smart::SmartData;

pub struct DiskTestConfig {
    pub test_path: Option<String>,
    pub test_size_mb: u64,
    pub include_seek_test: bool,
    pub verbose: bool,
    // AI commentary callbacks (optional, for real-time comments)
    pub on_comment: Option<Box<dyn Fn(&str) + Send>>,
}

impl Default for DiskTestConfig {
    fn default() -> Self {
        Self {
            test_path: None,
            test_size_mb: 100,
            include_seek_test: true,
            verbose: false,
            on_comment: None,
        }
    }
}

pub struct DiskTestResult {
    // Hardware info
    pub disk_name: String,
    pub disk_size_gb: f64,
    pub disk_used_gb: f64,
    pub disk_available_gb: f64,
    pub disk_fs: String,
    pub disk_device: Option<String>,
    // Test metrics
    pub write_speed_mb_s: f64,
    pub read_speed_mb_s: f64,
    pub seek_time_ms: f64,
    pub bad_sectors: u64,
    pub is_ssd: bool,
    // SMART data (verbose mode)
    pub smart: Option<SmartData>,
    pub health: HealthStatus,
}

/// Run disk health check
/// Tests sequential write/read, random access, and data integrity
pub fn run_stress_test(
    config: DiskTestConfig,
    disk_name: String,
    disk_size_gb: f64,
    disk_used_gb: f64,
    disk_available_gb: f64,
    disk_fs: String,
    disk_mount: &str,
) -> DiskTestResult {
    // Determine test file path
    let test_path = if let Some(ref path) = config.test_path {
        PathBuf::from(path)
    } else {
        std::env::temp_dir().join("pchecker_disk_test.tmp")
    };

    let test_path_str = test_path.to_string_lossy().to_string();
    let test_size_bytes = config.test_size_mb * 1024 * 1024;
    let chunk_size = 1024 * 1024; // 1MB chunks

    // Detect disk type (SSD/HDD) based on mount point
    let is_ssd = detect_disk_type(&test_path);

    // Clone callback for use
    let comment_callback = config.on_comment;

    if config.verbose {
        println!("📁 Test file: {}", test_path_str);
        println!("💿 Disk: {} ({})", disk_name, if is_ssd { "SSD" } else { "HDD" });
        println!("📏 Test size: {} MB", config.test_size_mb);
        println!();
    }

    // === PHASE 1: Write Test ===
    print!("⏳ ");
    if config.verbose {
        print!("Writing {} MB... ", config.test_size_mb);
    } else {
        print!("Disk: Writing... ");
    }
    io::stdout().flush().unwrap();

    let (write_speed, write_success) = write_test(&test_path, test_size_bytes, chunk_size, config.verbose);

    if !write_success {
        cleanup_test_file(&test_path);
        return DiskTestResult {
            disk_name: disk_name.clone(),
            disk_size_gb,
            disk_used_gb,
            disk_available_gb,
            disk_fs: disk_fs.clone(),
            disk_device: get_disk_device(disk_mount),
            write_speed_mb_s: 0.0,
            read_speed_mb_s: 0.0,
            seek_time_ms: 0.0,
            bad_sectors: 0,
            is_ssd,
            smart: None,
            health: HealthStatus::Failed("Cannot write to disk - check permissions or disk space".to_string()),
        };
    }

    // === PHASE 2: Read Test ===
    print!("\r⏳ ");
    if config.verbose {
        print!("Reading {} MB... ", config.test_size_mb);
    } else {
        print!("Disk: Reading... ");
    }
    io::stdout().flush().unwrap();

    let (read_speed, bad_sectors, read_success) = read_test(&test_path, test_size_bytes, chunk_size, config.verbose);

    if !read_success {
        cleanup_test_file(&test_path);
        return DiskTestResult {
            disk_name: disk_name.clone(),
            disk_size_gb,
            disk_used_gb,
            disk_available_gb,
            disk_fs: disk_fs.clone(),
            disk_device: get_disk_device(disk_mount),
            write_speed_mb_s: write_speed,
            read_speed_mb_s: 0.0,
            seek_time_ms: 0.0,
            bad_sectors: 0,
            is_ssd,
            smart: None,
            health: HealthStatus::Failed("Read test failed - possible disk failure".to_string()),
        };
    }

    // AI commentary on disk speed
    if let Some(ref callback) = comment_callback {
        if is_ssd {
            if read_speed > 500.0 {
                callback(&format!("{} SSD read speed: {:.1} MB/s - excellent", disk_name, read_speed));
            } else if read_speed > 200.0 {
                callback(&format!("{} SSD read speed: {:.1} MB/s - good", disk_name, read_speed));
            } else {
                callback(&format!("{} SSD read speed: {:.1} MB/s - below average", disk_name, read_speed));
            }
        } else if read_speed > 100.0 {
            callback(&format!("{} HDD read speed: {:.1} MB/s - excellent", disk_name, read_speed));
        } else if read_speed > 50.0 {
            callback(&format!("{} HDD read speed: {:.1} MB/s - good", disk_name, read_speed));
        } else {
            callback(&format!("{} HDD read speed: {:.1} MB/s", disk_name, read_speed));
        }
    }

    // === PHASE 3: Seek Test (optional) ===
    let mut seek_time = 0.0;
    if config.include_seek_test {
        print!("\r⏳ ");
        if config.verbose {
            print!("Testing seek time... ");
        } else {
            print!("Disk: Seeking... ");
        }
        io::stdout().flush().unwrap();

        seek_time = seek_test(&test_path, test_size_bytes, 1000, config.verbose);
    }

    // Cleanup test file
    cleanup_test_file(&test_path);

    // Collect SMART data in verbose mode
    let smart = if config.verbose {
        Some(smart::get_smart_data(disk_mount, true))
    } else {
        None
    };

    // Print final progress
    if config.verbose {
        println!();
        println!("✅ Tests completed");
    } else {
        print!("\r");
    }

    // Evaluate health
    let health = evaluate_disk_health(write_speed, read_speed, seek_time, bad_sectors, is_ssd);

    DiskTestResult {
        disk_name,
        disk_size_gb,
        disk_used_gb,
        disk_available_gb,
        disk_fs,
        disk_device: get_disk_device(disk_mount),
        write_speed_mb_s: write_speed,
        read_speed_mb_s: read_speed,
        seek_time_ms: seek_time,
        bad_sectors,
        is_ssd,
        smart,
        health,
    }
}

/// Sequential write test - returns (speed_mb_s, success)
fn write_test(path: &PathBuf, size_bytes: u64, chunk_size: usize, verbose: bool) -> (f64, bool) {
    let file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
    {
        Ok(f) => f,
        Err(_) => return (0.0, false),
    };

    let mut writer = BufWriter::new(file);
    let pattern_byte: u8 = 0xA5;
    let buffer = vec![pattern_byte; chunk_size];

    let chunks = (size_bytes / chunk_size as u64) as usize;
    let start = Instant::now();

    for i in 0..chunks {
        if writer.write_all(&buffer).is_err() {
            return (0.0, false);
        }

        // Progress update every 10%
        if verbose && (i + 1) % (chunks / 10 + 1).min(10) == 0 {
            let progress = ((i + 1) * 100 / chunks) as u8;
            print!("{}% ", progress);
            io::stdout().flush().unwrap();
        }
    }

    if writer.flush().is_err() {
        return (0.0, false);
    }

    let elapsed = start.elapsed();
    let seconds = elapsed.as_secs_f64();
    let mb_written = size_bytes as f64 / (1024.0 * 1024.0);
    let speed = if seconds > 0.0 { mb_written / seconds } else { 0.0 };

    (speed, true)
}

/// Sequential read test with verification - returns (speed_mb_s, bad_sectors, success)
fn read_test(path: &PathBuf, size_bytes: u64, chunk_size: usize, verbose: bool) -> (f64, u64, bool) {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return (0.0, 0, false),
    };

    let pattern_byte: u8 = 0xA5;
    let expected_buffer = vec![pattern_byte; chunk_size];
    let mut read_buffer = vec![0u8; chunk_size];

    let chunks = (size_bytes / chunk_size as u64) as usize;
    let start = Instant::now();
    let mut bad_sectors = 0u64;

    for i in 0..chunks {
        match file.read(&mut read_buffer) {
            Ok(n) if n == chunk_size => {
                // Verify pattern
                if read_buffer != expected_buffer {
                    // Count byte mismatches as potential bad sectors
                    for (a, b) in read_buffer.iter().zip(expected_buffer.iter()) {
                        if a != b {
                            bad_sectors += 1;
                        }
                    }
                }
            }
            Ok(_) | Err(_) => return (0.0, 0, false),
        }

        // Progress update
        if verbose && (i + 1) % (chunks / 10 + 1).min(10) == 0 {
            let progress = ((i + 1) * 100 / chunks) as u8;
            print!("{}% ", progress);
            io::stdout().flush().unwrap();
        }
    }

    let elapsed = start.elapsed();
    let seconds = elapsed.as_secs_f64();
    let mb_read = size_bytes as f64 / (1024.0 * 1024.0);
    let speed = if seconds > 0.0 { mb_read / seconds } else { 0.0 };

    // Convert byte errors to sector errors (4KB sectors)
    let sector_errors = bad_sectors / 4096;

    (speed, sector_errors, true)
}

/// Random access (seek) test - returns average seek time in ms
fn seek_test(path: &PathBuf, file_size: u64, iterations: u32, verbose: bool) -> f64 {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return 0.0,
    };

    let mut read_buffer = vec![0u8; 4096]; // 4KB reads
    let mut total_seek_time = 0.0;
    let mut successful_seeks = 0u32;

    for i in 0..iterations {
        // Random position aligned to 4KB
        let max_pos = file_size.saturating_sub(4096);
        if max_pos == 0 {
            break;
        }
        let random_pos = (fastrand::u64(..) % max_pos) & !4095;

        let seek_start = Instant::now();

        if file.seek(SeekFrom::Start(random_pos)).is_err() {
            continue;
        }
        if file.read_exact(&mut read_buffer).is_err() {
            continue;
        }

        let seek_time_ns = seek_start.elapsed().as_nanos() as f64;
        total_seek_time += seek_time_ns;
        successful_seeks += 1;

        // Progress update
        if verbose && (i + 1) % 200 == 0 {
            let progress = ((i + 1) * 100 / iterations) as u8;
            print!("{}% ", progress);
            io::stdout().flush().unwrap();
        }
    }

    if successful_seeks == 0 {
        return 0.0;
    }

    // Average seek time in milliseconds
    (total_seek_time / successful_seeks as f64) / 1_000_000.0
}

/// Detect disk type (SSD/HDD)
/// Platform-specific detection
fn detect_disk_type(path: &PathBuf) -> bool {
    // Try to get the mount point
    let default_mount = PathBuf::from("/");
    let mount_ref = path.ancestors().find(|p| {
        p.exists() && !p.as_os_str().is_empty()
    }).unwrap_or(&default_mount);

    // Clone to owned for platform-specific use
    let mount = mount_ref.to_path_buf();

    // Platform-specific detection
    #[cfg(target_os = "macos")]
    {
        check_macos_ssd(&mount)
    }

    #[cfg(target_os = "linux")]
    {
        check_linux_ssd(&mount)
    }

    #[cfg(target_os = "windows")]
    {
        true // Default to SSD on Windows
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        true // Default assumption
    }
}

#[cfg(target_os = "macos")]
fn check_macos_ssd(mount: &PathBuf) -> bool {
    use std::process::Command;

    // Try to get disk info from mount point
    let mount_str = mount.to_string_lossy().to_string();

    // Get disk identifier from mount
    let output = Command::new("df")
        .arg(&mount_str)
        .output();

    let disk_identifier = match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.lines()
                .nth(1)
                .and_then(|line| line.split_whitespace().next())
                .unwrap_or("disk0")
                .to_string()
        }
        Err(_) => return true, // Default to SSD
    };

    // Use diskutil to check if solid state
    let output = Command::new("diskutil")
        .args(["info", "-plist", &disk_identifier])
        .output();

    match output {
        Ok(out) => {
            let plist = String::from_utf8_lossy(&out.stdout);
            // Check for SolidState key with true value
            plist.contains("SolidState") &&
            (plist.contains("Yes") || plist.contains("true") || plist.contains("<true/>"))
        }
        Err(_) => true, // Default to SSD
    }
}

#[cfg(target_os = "linux")]
fn check_linux_ssd(mount: &PathBuf) -> bool {
    use std::path::Path;

    // Try to find the block device for this mount
    let mount_str = mount.to_string_lossy().to_string();

    // Read /proc/mounts to find device
    if let Ok(mounts) = std::fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == mount_str {
                let device = parts[0];
                // Extract device name (e.g., /dev/sda1 -> sda)
                if let Some(name) = Path::new(device).file_name() {
                    let device_name = name.to_string_lossy();
                    // Trim partition number
                    let base_name = device_name.trim_end_matches('0'..='9');

                    // Check /sys/block/.../queue/rotational
                    let rotational_path = format!("/sys/block/{}/queue/rotational",
                        base_name.trim_end_matches('p'));
                    if let Ok(rotational) = std::fs::read_to_string(&rotational_path) {
                        return rotational.trim() == "0";
                    }
                }
                break;
            }
        }
    }

    true // Default to SSD
}

/// Clean up test file
fn cleanup_test_file(path: &PathBuf) {
    let _ = std::fs::remove_file(path);
}

/// Evaluate disk health based on test results
fn evaluate_disk_health(write: f64, read: f64, seek: f64, bad_sectors: u64, is_ssd: bool) -> HealthStatus {
    let mut issues = Vec::new();

    // Critical: bad sectors detected
    if bad_sectors > 0 {
        return HealthStatus::Failed(format!(
            "Bad sectors detected ({} sectors) - disk failure imminent",
            bad_sectors
        ));
    }

    // Speed thresholds differ for SSD vs HDD
    let (min_read, min_write, max_seek) = if is_ssd {
        (50.0, 30.0, 5.0)  // SSD: faster, lower seek
    } else {
        (10.0, 10.0, 20.0) // HDD: slower, higher seek
    };

    // Critical: extremely slow speeds
    if read < min_read {
        return HealthStatus::Failed(format!(
            "Extremely slow read speed ({:.1} MB/s) - dying disk",
            read
        ));
    }

    if write < min_write {
        return HealthStatus::Failed(format!(
            "Extremely slow write speed ({:.1} MB/s) - dying disk",
            write
        ));
    }

    // Issues: slow seek time (if seek test was run)
    if seek > 0.0 && seek > max_seek {
        issues.push(format!(
            "Slow seek time ({:.1}ms) - possible mechanical issue",
            seek
        ));
    }

    // Issues: speed warning (slower than expected but not critical)
    if is_ssd && read < 100.0 {
        issues.push(format!(
            "SSD read speed below average ({:.1} MB/s)",
            read
        ));
    } else if !is_ssd && read < 50.0 {
        issues.push(format!(
            "HDD read speed below average ({:.1} MB/s)",
            read
        ));
    }

    if !issues.is_empty() {
        HealthStatus::IssuesDetected(issues)
    } else {
        HealthStatus::Healthy
    }
}

/// Get disk device identifier from mount point
/// Platform-specific implementation
fn get_disk_device(mount_point: &str) -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        get_disk_device_macos(mount_point)
    }

    #[cfg(target_os = "linux")]
    {
        get_disk_device_linux(mount_point)
    }

    #[cfg(target_os = "windows")]
    {
        get_disk_device_windows()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

#[cfg(target_os = "macos")]
fn get_disk_device_macos(mount_point: &str) -> Option<String> {
    use std::process::Command;
    if let Ok(output) = Command::new("df")
        .arg(mount_point)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().nth(1) {
            if let Some(disk) = line.split_whitespace().next() {
                return Some(disk.to_string());
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn get_disk_device_linux(mount_point: &str) -> Option<String> {
    if let Ok(mounts) = std::fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == mount_point {
                return Some(parts[0].to_string());
            }
        }
    }
    None
}

#[cfg(target_os = "windows")]
fn get_disk_device_windows() -> Option<String> {
    // On Windows, we could return drive letter or WMI identifier
    // For now, return None to avoid complexity
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_test_small() {
        let config = DiskTestConfig {
            test_path: Some("/tmp/pchecker_test.tmp".to_string()),
            test_size_mb: 1,  // Only 1MB for quick test
            include_seek_test: false,
            verbose: false,
        };
        let result = run_stress_test(
            config,
            "Test Disk".to_string(),
            512.0,
            200.0,
            300.0,
            "APFS".to_string(),
            "/tmp",
        );

        assert!(result.write_speed_mb_s > 0.0);
        assert!(result.read_speed_mb_s > 0.0);
        assert!(matches!(result.health, HealthStatus::Healthy));
    }

    #[test]
    fn test_evaluate_disk_health() {
        // Healthy SSD
        assert!(matches!(
            evaluate_disk_health(500.0, 2000.0, 0.5, 0, true),
            HealthStatus::Healthy
        ));

        // Healthy HDD
        assert!(matches!(
            evaluate_disk_health(100.0, 80.0, 10.0, 0, false),
            HealthStatus::Healthy
        ));

        // Failed - bad sectors
        assert!(matches!(
            evaluate_disk_health(500.0, 2000.0, 0.5, 1, true),
            HealthStatus::Failed(_)
        ));

        // Failed - extremely slow read (SSD)
        assert!(matches!(
            evaluate_disk_health(500.0, 20.0, 0.5, 0, true),
            HealthStatus::Failed(_)
        ));

        // Failed - extremely slow write (HDD)
        assert!(matches!(
            evaluate_disk_health(5.0, 80.0, 10.0, 0, false),
            HealthStatus::Failed(_)
        ));

        // Issues - slow seek (HDD)
        assert!(matches!(
            evaluate_disk_health(100.0, 80.0, 25.0, 0, false),
            HealthStatus::IssuesDetected(_)
        ));
    }
}
