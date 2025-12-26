// Common GPU types and traits

use crate::lang::Text;

/// GPU type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuType {
    Integrated,
    Discrete,
    Unknown,
}

impl GpuType {
    pub fn as_str(&self) -> &str {
        match self {
            GpuType::Integrated => "Integrated",
            GpuType::Discrete => "Discrete",
            GpuType::Unknown => "Unknown",
        }
    }

    pub fn as_localized_str<'a>(&'a self, text: &'a Text) -> &'a str {
        match self {
            GpuType::Integrated => text.gpu_type_integrated(),
            GpuType::Discrete => text.gpu_type_discrete(),
            GpuType::Unknown => text.gpu_type_unknown(),
        }
    }

    /// Detect GPU type from model name
    pub fn from_model(model: &str) -> Self {
        let model_lower = model.to_lowercase();

        // Integrated GPUs
        if model_lower.contains("intel")
            || model_lower.contains("integrated")
            || model_lower.contains("uhd")
            || model_lower.contains("iris")
            || model_lower.contains("graphics")
            || model_lower.contains("apple m")  // Apple Silicon = integrated
        {
            GpuType::Integrated
        }
        // Discrete GPUs
        else if model_lower.contains("nvidia")
            || model_lower.contains("amd")
            || model_lower.contains("radeon")
            || model_lower.contains("geforce")
            || model_lower.contains("rtx")
            || model_lower.contains("gtx")
            || model_lower.contains("rx ")
        {
            GpuType::Discrete
        }
        else {
            GpuType::Unknown
        }
    }
}

pub struct GpuInfo {
    pub model: String,
    pub vram_gb: Option<f64>,
    pub gpu_type: GpuType,
}

impl GpuInfo {
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        let type_str = format!("[{}]", self.gpu_type.as_str());
        if let Some(vram) = self.vram_gb {
            format!("{} {} ({:.0} GB)", self.model, type_str, vram)
        } else {
            format!("{} {}", self.model, type_str)
        }
    }

    pub fn display_localized(&self, text: &Text) -> String {
        let type_str = format!("[{}]", self.gpu_type.as_localized_str(text));
        if let Some(vram) = self.vram_gb {
            format!("{} {} ({:.0} GB)", self.model, type_str, vram)
        } else {
            format!("{} {}", self.model, type_str)
        }
    }
}
