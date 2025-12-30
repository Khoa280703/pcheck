// Deep RAM Information Trait
// Defines interface for per-DIMM RAM details

use crate::hw::deep::common::DimmSlot;

/// Trait for deep RAM information
pub trait DeepRamInfo {
    /// Get all DIMM slots information
    fn get_dimm_slots(&self) -> Vec<DimmSlot>;
}
