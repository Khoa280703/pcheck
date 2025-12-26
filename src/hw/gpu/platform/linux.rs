// Linux GPU detection
// Uses lspci and /sys/class/drm for VRAM

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::hw::gpu::common::{GpuInfo, GpuType};

pub fn detect_gpus() -> Vec<GpuInfo> {
    let output = Command::new("lspci")
        .args(&["-vnnn"])
        .output();

    match output {
        Ok(result) => {
            let content = String::from_utf8_lossy(&result.stdout);

            // Find VGA line and parse GPU name
            // Format: "VGA compatible controller [0300]: NVIDIA Corporation ... [10de:XXXX]"
            let gpu_name = content
                .lines()
                .find(|l| l.contains("VGA compatible controller"))
                .and_then(|l| l.split(':').nth(1))
                .and_then(|s| s.split('[').next())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            // Detect GPU type from model name
            let gpu_type = GpuType::from_model(&gpu_name);

            // Try to get VRAM from /sys/class/drm/card*/device/mem_info_vram_total
            // This works for AMD and some NVIDIA GPUs
            let vram = try_get_vram_from_sysfs();

            vec![GpuInfo {
                model: gpu_name,
                vram_gb: vram,
                gpu_type,
            }]
        }
        Err(_) => vec![GpuInfo {
            model: "Detection failed (lspci not found)".to_string(),
            vram_gb: None,
            gpu_type: GpuType::Unknown,
        }],
    }
}

fn try_get_vram_from_sysfs() -> Option<f64> {
    // Try to find VRAM from /sys/class/drm
    // Paths: /sys/class/drm/card0/device/mem_info_vram_total (AMD)
    //        /sys/class/drm/card0/device/memory/vram_total (NVIDIA)
    let drm_path = Path::new("/sys/class/drm");

    if let Ok(entries) = drm_path.read_dir() {
        for entry in entries.flatten() {
            let card_path = entry.path();

            // Try AMD path first
            let vram_path = card_path.join("device/mem_info_vram_total");
            if vram_path.exists() {
                if let Ok(content) = fs::read_to_string(&vram_path) {
                    if let Ok(bytes) = content.trim().parse::<u64>() {
                        let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                        if gb > 0.0 && gb < 256.0 {
                            return Some(gb);
                        }
                    }
                }
            }

            // Try NVIDIA path
            let vram_path_nvidia = card_path.join("device/memory/vram_total");
            if vram_path_nvidia.exists() {
                if let Ok(content) = fs::read_to_string(&vram_path_nvidia) {
                    if let Ok(bytes) = content.trim().parse::<u64>() {
                        let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                        if gb > 0.0 && gb < 256.0 {
                            return Some(gb);
                        }
                    }
                }
            }
        }
    }

    None
}
