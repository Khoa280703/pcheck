// Deep CPU Information Trait
// Defines interface for low-level CPU details

use crate::hw::deep::common::{CacheInfo, InstructionSets};

/// Trait for deep CPU information
pub trait DeepCpuInfo {
    /// Get CPU cache information (L1, L2, L3)
    fn get_cache_info(&self) -> Option<CacheInfo>;

    /// Get CPU instruction sets / features
    fn get_instruction_sets(&self) -> Option<InstructionSets>;

    /// Get estimated TDP from model name (heuristic)
    fn get_tdp(&self, model: &str) -> Option<u32>;
}

/// Estimate TDP from CPU model name using heuristic parsing
///
/// Parses CPU model suffix to estimate TDP:
/// - U/G1-G7/P = 15-28W (Ultrabook/Office)
/// - H/HK/HX = 45W+ (High performance/Gaming)
/// - HQ/MQ = 45W (Old chips)
pub fn estimate_tdp_from_model(model: &str) -> Option<u32> {
    // Apple Silicon - check first to avoid matching "U" in "Ultra"
    if model == "Apple M1 Pro" || model == "Apple M2 Pro" || model == "Apple M3 Pro" {
        return Some(15);
    }
    if model == "Apple M1 Max" || model == "Apple M2 Max" || model == "Apple M3 Max" {
        return Some(30);
    }
    if model == "Apple M1 Ultra" || model == "Apple M2 Ultra" || model == "Apple M3 Ultra" {
        return Some(60);
    }

    let model_lower = model.to_lowercase();

    // Ultrabook / Office (15-28W) - check after Apple Silicon to avoid false matches
    // Check for Intel/AMD specific patterns to avoid matching random "U" in model names
    let is_intel_amu = model_lower.contains("intel") || model_lower.contains("amd") || model_lower.contains("core") || model_lower.contains("ryzen");
    if is_intel_amu && (model_lower.ends_with("u") || model_lower.ends_with("g7")
        || model_lower.ends_with("g5") || model_lower.ends_with("g1")
        || model_lower.contains("/p")) {
        return Some(15);
    }

    // High performance / Gaming (45W+)
    if model_lower.ends_with("h") || model_lower.ends_with("hk")
        || model_lower.ends_with("hx") || model_lower.contains("hq")
        || model_lower.ends_with("mq") {
        return Some(45);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tdp_intel_u_series() {
        assert_eq!(estimate_tdp_from_model("Intel Core i7-1165G7"), Some(15));
        assert_eq!(estimate_tdp_from_model("Intel Core i5-1035G1"), Some(15));
    }

    #[test]
    fn test_estimate_tdp_intel_h_series() {
        assert_eq!(estimate_tdp_from_model("Intel Core i7-11800H"), Some(45));
        assert_eq!(estimate_tdp_from_model("Intel Core i9-11980HK"), Some(45));
    }

    #[test]
    fn test_estimate_tdp_apple_silicon() {
        assert_eq!(estimate_tdp_from_model("Apple M1 Pro"), Some(15));
        assert_eq!(estimate_tdp_from_model("Apple M2 Max"), Some(30));
        assert_eq!(estimate_tdp_from_model("Apple M3 Ultra"), Some(60));
    }

    #[test]
    fn test_estimate_tdp_unknown() {
        assert_eq!(estimate_tdp_from_model("Unknown CPU Model XYZ"), None);
    }
}
