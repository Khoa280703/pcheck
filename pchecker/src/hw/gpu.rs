// GPU detection module
// Uses platform-specific commands via std::process::Command

use std::process::Command;

pub struct GpuInfo {
    pub model: String,
    pub vram_gb: Option<f64>,
}

impl GpuInfo {
    /// Detect GPU using platform-specific commands
    pub fn new() -> Vec<Self> {
        #[cfg(target_os = "macos")]
        return Self::detect_macos();

        #[cfg(target_os = "windows")]
        return Self::detect_windows();

        #[cfg(target_os = "linux")]
        return Self::detect_linux();

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        return vec![Self {
            model: "Unknown platform".to_string(),
            vram_gb: None,
        }];
    }

    #[cfg(target_os = "macos")]
    fn detect_macos() -> Vec<Self> {
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

                // Parse "VRAM (Dynamic): X GB"
                let vram = content
                    .lines()
                    .find(|l| l.contains("VRAM"))
                    .and_then(|l| l.split(':').nth(1))
                    .and_then(|s| s.trim().split_whitespace().next())
                    .and_then(|s| s.parse::<f64>().ok());

                vec![Self { model, vram_gb: vram }]
            }
            Err(_) => vec![Self {
                model: "Detection failed".to_string(),
                vram_gb: None,
            }],
        }
    }

    #[cfg(target_os = "windows")]
    fn detect_windows() -> Vec<Self> {
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                "(Get-WmiObject Win32_VideoController).Name"
            ])
            .output();

        match output {
            Ok(result) => {
                let model = String::from_utf8_lossy(&result.stdout)
                    .trim()
                    .to_string();

                if model.is_empty() {
                    vec![Self {
                        model: "No GPU detected".to_string(),
                        vram_gb: None,
                    }]
                } else {
                    vec![Self {
                        model,
                        vram_gb: None, // TODO: Implement VRAM detection
                    }]
                }
            }
            Err(_) => vec![Self {
                model: "Detection failed".to_string(),
                vram_gb: None,
            }],
        }
    }

    #[cfg(target_os = "linux")]
    fn detect_linux() -> Vec<Self> {
        let output = Command::new("lspci")
            .args(&["-vnnn"])
            .output();

        match output {
            Ok(result) => {
                let content = String::from_utf8_lossy(&result.stdout);

                // Find VGA line and parse GPU name
                let gpu_name = content
                    .lines()
                    .find(|l| l.contains("VGA compatible controller"))
                    .and_then(|l| l.split(':').nth(1))
                    .and_then(|s| s.split('[').next())
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                vec![Self {
                    model: gpu_name,
                    vram_gb: None, // TODO: Implement VRAM detection via /sys
                }]
            }
            Err(_) => vec![Self {
                model: "Detection failed (lspci not found)".to_string(),
                vram_gb: None,
            }],
        }
    }

    pub fn display(&self) -> String {
        if let Some(vram) = self.vram_gb {
            format!("{} ({:.0} GB)", self.model, vram)
        } else {
            self.model.clone()
        }
    }
}
