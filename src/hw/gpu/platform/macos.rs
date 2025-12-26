// macOS GPU detection
// Uses system_profiler SPDisplaysDataType

use std::process::Command;

use crate::hw::gpu::common::{GpuInfo, GpuType};

pub fn detect_gpus() -> Vec<GpuInfo> {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output();

    match output {
        Ok(result) => {
            let content = String::from_utf8_lossy(&result.stdout);

            // Parse "Chipset Model: Apple M4"
            let model = content
                .lines()
                .find(|l| l.contains("Chipset Model"))
                .and_then(|l| l.split(':').nth(1))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            // Detect GPU type from model name
            let gpu_type = GpuType::from_model(&model);

            // Parse "VRAM (Dynamic): X GB"
            // For Apple Silicon, VRAM is shared memory (not applicable)
            let vram = if gpu_type == GpuType::Integrated {
                None  // Apple Silicon has unified memory, not separate VRAM
            } else {
                content
                    .lines()
                    .find(|l| l.contains("VRAM"))
                    .and_then(|l| l.split(':').nth(1))
                    .and_then(|s| s.trim().split_whitespace().next())
                    .and_then(|s| s.parse::<f64>().ok())
            };

            vec![GpuInfo { model, vram_gb: vram, gpu_type }]
        }
        Err(_) => vec![GpuInfo {
            model: "Detection failed".to_string(),
            vram_gb: None,
            gpu_type: GpuType::Unknown,
        }],
    }
}
