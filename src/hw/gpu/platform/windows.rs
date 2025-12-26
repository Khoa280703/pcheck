// Windows GPU detection
// Uses PowerShell WMI Win32_VideoController

use std::process::Command;

use crate::hw::gpu::common::{GpuInfo, GpuType};

pub fn detect_gpus() -> Vec<GpuInfo> {
    // Get GPU name and VRAM in one command
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-WmiObject Win32_VideoController | Select-Object Name, AdapterRAM | Format-List"
        ])
        .output();

    match output {
        Ok(result) => {
            let content = String::from_utf8_lossy(&result.stdout);

            // Parse "Name : NVIDIA GeForce RTX 3060"
            let model = content
                .lines()
                .find(|l| l.trim().starts_with("Name"))
                .and_then(|l| l.split(':').nth(1))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Unknown GPU".to_string());

            // Detect GPU type from model name
            let gpu_type = GpuType::from_model(&model);

            // Parse "AdapterRAM : 1073741824" (bytes, need to convert to GB)
            let vram = content
                .lines()
                .find(|l| l.trim().starts_with("AdapterRAM"))
                .and_then(|l| l.split(':').nth(1))
                .and_then(|s| s.trim().parse::<u64>().ok())
                .map(|bytes| bytes as f64 / (1024.0 * 1024.0 * 1024.0))
                .filter(|gb| *gb > 0.0 && *gb < 256.0);  // Sanity check: 0-256 GB

            if model == "Unknown GPU" {
                vec![GpuInfo {
                    model: "No GPU detected".to_string(),
                    vram_gb: None,
                    gpu_type: GpuType::Unknown,
                }]
            } else {
                vec![GpuInfo { model, vram_gb: vram, gpu_type }]
            }
        }
        Err(_) => vec![GpuInfo {
            model: "Detection failed".to_string(),
            vram_gb: None,
            gpu_type: GpuType::Unknown,
        }],
    }
}
